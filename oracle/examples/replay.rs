// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Spreadsheet Company

//! The owner's session at the command line (`docs/SPIKE.md`, `HOUSE.4`'s second
//! slice, decision 3): the replay, `src/buffet.replay.vla`, its header
//! restamped with the digest of the cartridge beside it, then played over the
//! engine as the page's host plays it, and its golden, `src/buffet.golden.vla`,
//! and the hand, `src/buffet.hand.js`, written from the play. A dish that moves
//! `src/buffet.vla` moves the three by this one command; a dish that changes
//! what the session's acts do plays the session again.
//!
//! ```text
//! cargo run --release --example replay                 # the three written
//! cargo run --release --example replay -- --check      # the same, compared and written nowhere
//! ```
//!
//! Prints `(played <frames> (held <frames>) (cells <cells>))`, then a row a
//! file, `(wrote "<file>")`, or with `--check` `(checked "<file>"
//! same|differs|missing)`, or `(refused "<file>" "<words>")`, then
//! `(replay-done)`. Exits 0 when the session played and every file was
//! written (or, checking, is the same), 1 when one was not, and 2 for a word
//! it does not know.

use std::process::ExitCode;

use buffet_oracle::replay::{
    cartridge_digest, crlf, golden_text, hand_names, hand_text, play_session, restamped, Replay,
    CARTRIDGE, GOLDEN, HAND, REPLAY,
};
use buffet_oracle::{lf, root};
use vla_lang::form::Form;
use vla_lang::printer::write_datum;

/// A text as a row quotes it.
fn quoted(text: &str) -> String {
    write_datum(&Form::string(text))
}

fn main() -> ExitCode {
    let mut check = false;
    for a in std::env::args().skip(1) {
        match a.as_str() {
            "--check" => check = true,
            other => {
                eprintln!("replay: an unknown word, {other}");
                return ExitCode::from(2);
            }
        }
    }
    let root = root();
    let read = |path: &str| std::fs::read_to_string(root.join(path));
    let refused = |path: &str, words: &str| {
        println!("(refused {} {})", quoted(path), quoted(words));
        println!("(replay-done)");
        ExitCode::from(1)
    };
    let cartridge = match read(CARTRIDGE) {
        Ok(t) => lf(&t),
        Err(e) => return refused(CARTRIDGE, &e.to_string()),
    };
    let replay = match read(REPLAY)
        .map_err(|e| e.to_string())
        .and_then(|t| Replay::read(&t))
    {
        Ok(r) => r,
        Err(e) => return refused(REPLAY, &e),
    };
    // The header names the cartridge beside it, the replay's comment lines and frames as they stand.
    let replay = restamped(&replay, &cartridge_digest(&cartridge));
    let session = match play_session(&replay, &cartridge) {
        Ok(s) => s,
        Err(e) => return refused(REPLAY, &e),
    };
    println!(
        "(played {} (held {}) (cells {}))",
        session.last,
        session.held.len(),
        session.held.len() * hand_names().len()
    );
    // Each `.vla` CRLF as the working tree holds it, the hand LF (`.gitattributes`).
    let files = [
        (REPLAY, crlf(&replay.text())),
        (GOLDEN, crlf(&golden_text(&session))),
        (HAND, hand_text(&session)),
    ];
    let mut fine = true;
    for (path, text) in &files {
        if check {
            let verdict = match read(path) {
                Ok(held) if lf(&held) == lf(text) => "same",
                Ok(_) => "differs",
                Err(_) => "missing",
            };
            fine &= verdict == "same";
            println!("(checked {} {verdict})", quoted(path));
            continue;
        }
        match std::fs::write(root.join(path), text) {
            Ok(()) => println!("(wrote {})", quoted(path)),
            Err(e) => {
                fine = false;
                println!("(refused {} {})", quoted(path), quoted(&e.to_string()));
            }
        }
    }
    println!("(replay-done)");
    if fine {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
