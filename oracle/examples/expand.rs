// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Spreadsheet Company

//! The pass at the command line (`OMAKASE.md`, `MISE.1`; Alonzo's `SPEC.md`
//! section 7.8): each cartridge's source expanded by the engine's pass,
//! `alonzo::pass::expand`, into the cartridge beside it, then the cartridge
//! loaded, as an act of its own, to say whether it loads. Alonzo's own
//! runner, `examples/expand.rs`, adapted by the same copyright holder under
//! this repository's terms: a dependency's examples are not this workspace's
//! to run, so the door is the oracle crate's, and the pass is the engine's.
//!
//! ```text
//! cargo run --release --example expand                 # every src/<name>.source.vla
//! cargo run --release --example expand -- --check      # the same, compared and written nowhere
//! cargo run --release --example expand -- src/buffet.source.vla
//! ```
//!
//! A source is `<name>.source.vla`, and its cartridge is `<name>.vla` in the
//! same folder, the game's in `src/` (the owner's word of 2026-10-14, the
//! game in one folder). An include is resolved against the source's own
//! folder, every include of an included text too, so the game's source
//! includes the kit and Buffet's library each by its own line.
//!
//! Prints one row a source:
//! `(expanded "<source>" "<cartridge>" (rows <n>) (loads yes))`, or with
//! `--check` `(checked "<source>" "<cartridge>" same|differs|missing)`, or
//! `(refused "<source>" (line <n>) <id> "<words>")`, then
//! `(expand-done <sources>)`. Exits 0 when every source expanded and every
//! cartridge loads (and, checking, is the same), 1 when one did not, and 2
//! for a word it does not know.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use alonzo::cartridge::Cartridge;
use alonzo::pass::{cartridge_path, expand, file_name};
use buffet_oracle::{gather, lf, root};
use vla_lang::form::Form;
use vla_lang::printer::write_datum;

/// A text as a row quotes it.
fn quoted(text: &str) -> String {
    write_datum(&Form::string(text))
}

/// Every source in the repository's `src/`, relative to its root.
fn default_sources(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(root.join("src")) else {
        return out;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if entry.path().is_file() && name.ends_with(".source.vla") {
            out.push(format!("src/{name}"));
        }
    }
    out.sort();
    out
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let check = args.iter().any(|a| a == "--check");
    let mut given = Vec::new();
    for a in &args {
        match a.as_str() {
            "--check" => {}
            a if a.starts_with("--") => {
                eprintln!("expand: an unknown option, {a}");
                return ExitCode::from(2);
            }
            a => given.push(a.replace('\\', "/")),
        }
    }
    let root = root();
    let (base, sources): (PathBuf, Vec<String>) = if given.is_empty() {
        (root.clone(), default_sources(&root))
    } else {
        (PathBuf::from("."), given)
    };
    if sources.is_empty() {
        eprintln!("expand: no <name>.source.vla to expand");
        return ExitCode::from(2);
    }

    let mut fine = true;
    for path in &sources {
        let Some(target) = cartridge_path(path) else {
            eprintln!("expand: {path} is not a <name>.source.vla");
            return ExitCode::from(2);
        };
        let text = match std::fs::read_to_string(base.join(path)) {
            Ok(t) => t,
            Err(e) => {
                println!(
                    "(refused {} (line 0) vla-file-not-found {})",
                    quoted(path),
                    quoted(&e.to_string())
                );
                fine = false;
                continue;
            }
        };
        let files = gather(&base, path, &text);
        let expansion = match expand(&text, path, &files) {
            Ok(e) => e,
            Err(r) => {
                println!(
                    "(refused {} (line {}) {} {})",
                    quoted(path),
                    r.line,
                    r.refusal.id,
                    quoted(&r.refusal.text)
                );
                fine = false;
                continue;
            }
        };
        if check {
            let verdict = match std::fs::read_to_string(base.join(&target)) {
                Ok(held) if lf(&held) == lf(&expansion.text) => "same",
                Ok(_) => "differs",
                Err(_) => "missing",
            };
            fine &= verdict == "same";
            println!("(checked {} {} {verdict})", quoted(path), quoted(&target));
            continue;
        }
        if let Err(e) = std::fs::write(base.join(&target), &expansion.text) {
            println!(
                "(refused {} (line 0) vla-file-not-replaced {})",
                quoted(&target),
                quoted(&e.to_string())
            );
            fine = false;
            continue;
        }
        // The load, an act of its own after the pass's.
        let loads = match Cartridge::load(&expansion.text, file_name(&target), expansion.text.len())
        {
            Ok(_) => "(loads yes)".to_string(),
            Err(r) => {
                fine = false;
                format!(
                    "(loads no (line {}) {} {})",
                    r.line,
                    r.refusal.id,
                    quoted(&r.refusal.text)
                )
            }
        };
        println!(
            "(expanded {} {} (rows {}) {loads})",
            quoted(path),
            quoted(&target),
            expansion.rows
        );
    }
    println!("(expand-done {})", sources.len());
    if fine {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
