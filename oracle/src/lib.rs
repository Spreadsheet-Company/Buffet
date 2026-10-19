// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Spreadsheet Company

//! Buffet's oracle (`docs/OMAKASE.md`, `MISE.2`): the game written a second time
//! by hand, the way a person would write it, so that the cartridge's frames
//! are held to a rule written twice; the runners that play it, as examples;
//! and the tests that load the cartridge through the engine and hold every
//! frame to the hand.
//!
//! What it holds since `POT.1` (2026-10-14): the hand, [`buffet`], its
//! calendar and its dice, since `POT.2` its market, since `STATION.1` the
//! stocks' four rungs and their dividend, since `FRONT.1` the Portfolio's
//! date and its dollars spelled by text, since `BACK.1` the side hustle, its
//! presses, its claims and its cap, since `POT.10` the market's hours,
//! the calendar counted in minutes, the session's bells and marks, and its
//! path between two closes (since `POT.11` the calendar in seconds at two
//! paces, a tick every five minutes and the volatility factor), and since
//! `GUARD.1` the Series 777, its draw
//! walked a row at a time, its answers, its score, the firm's capital and
//! style, its keys by view and state, and the bell waiting for it, and since
//! `PLATE.1` the order desk, its buttons and commands, its fills and books,
//! each pocket's cash and the ledger's ring; the host's derived writes, the
//! Write sheet's rows as the host reads them; the door
//! the runners and the tests
//! share, which reads a source of the repository and the texts it includes
//! and hands them to the engine's pass, `alonzo::pass::expand`, as the
//! engine's own runner does; and the smoke test of the pin, a two-row
//! cartridge loaded through `alonzo` from crates.io and stepped. The crate
//! is never published, so its tests read the repository beside it at run
//! time.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use alonzo::cartridge::Cartridge;
use alonzo::pass::{expand, folder_of, includes, Expansion};
use vla_lang::machine::{LineRefusal, Step, Viewed};

pub mod buffet;

/// The repository's root, the folder above this crate.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// A file of the repository, by its path from the root, read whole.
pub fn read(path: &str) -> String {
    let at = root().join(path);
    std::fs::read_to_string(&at).unwrap_or_else(|e| panic!("{}: {e}", at.display()))
}

/// A text with CRLF read as LF, as a `.vla` is stored and as a replay's
/// digest reads it.
pub fn lf(text: &str) -> String {
    text.replace("\r\n", "\n")
}

/// The texts a source may include, read from `base` under the keys the pass
/// resolves them by: its includes, and theirs, each against the source's own
/// folder, each read once. A text that cannot be read is left out, and the
/// pass refuses it by name. Alonzo's runner's own walk, `examples/expand.rs`.
pub fn gather(base: &Path, path: &str, text: &str) -> Vec<(String, String)> {
    let folder = folder_of(path).to_string();
    let mut files = Vec::new();
    let mut seen = BTreeSet::new();
    let mut queue = includes(text, &folder);
    while let Some(key) = queue.pop() {
        if !seen.insert(key.clone()) {
            continue;
        }
        if let Ok(body) = std::fs::read_to_string(base.join(&key)) {
            queue.extend(includes(&body, &folder));
            files.push((key, body));
        }
    }
    files
}

/// A source of the repository, by its path from the root, expanded by the
/// pass with every text it includes.
pub fn expand_source(path: &str) -> Result<Expansion, LineRefusal> {
    let text = read(path);
    let files = gather(&root(), path, &text);
    expand(&text, path, &files)
}

/// A cartridge of the repository, by its path from the root, loaded by the
/// engine under its file's name, or the load's refusal in its words.
pub fn load(path: &str) -> Result<Cartridge, String> {
    let text = read(path);
    let name = alonzo::pass::file_name(path);
    Cartridge::load(&text, name, text.len())
        .map_err(|r| format!("line {}: {} {}", r.line, r.refusal.id, r.refusal.text))
}

/// The values of a window of a sheet as the `values` record prints them, by
/// address: a number as its shortest text, a text without its quotes. A cell
/// with no value is not there.
pub fn values(cart: &Cartridge, sheet: &str, window: &str) -> BTreeMap<String, String> {
    let Ok(Viewed::Text(record)) = cart.view("values", sheet, Some(window)) else {
        panic!("the {sheet} sheet's record of {window}");
    };
    let mut out = BTreeMap::new();
    for head in ["(value ", "(cell "] {
        let head = format!("{head}\"{sheet}\" \"");
        for line in record.lines() {
            if let Some((addr, v)) = line
                .strip_prefix(head.as_str())
                .and_then(|rest| rest.split_once("\" "))
            {
                let v = v.strip_suffix(')').unwrap_or(v);
                let v = v
                    .strip_prefix('"')
                    .and_then(|t| t.strip_suffix('"'))
                    .unwrap_or(v);
                out.insert(addr.to_string(), v.to_string());
            }
        }
    }
    out
}

/// The Write sheet's rows as the host reads them after a step (Alonzo's
/// `SPEC.md` section 3.9, `web/host.js`'s `_readWrite`, `PLATE.1`): from row 2,
/// each row whose address is a text not empty, its sheet a text not empty and
/// its value no error, a `derived` row for the next frame, its value spelled as
/// the record spells it; each the host's own write before the next frame's
/// inputs.
pub fn derived(cart: &Cartridge) -> Vec<String> {
    let Ok(Viewed::Text(record)) = cart.view("values", "Write", Some("A2:C101")) else {
        return Vec::new();
    };
    let mut cells: BTreeMap<(u32, char), String> = BTreeMap::new();
    for line in record.lines() {
        let Some(rest) = line
            .strip_prefix("(value \"Write\" \"")
            .or_else(|| line.strip_prefix("(cell \"Write\" \""))
        else {
            continue;
        };
        let Some((addr, spelled)) = rest.split_once("\" ") else {
            continue;
        };
        let column = addr.chars().next().unwrap_or(' ');
        if let Ok(row) = addr[1..].parse::<u32>() {
            let spelled = spelled.strip_suffix(')').unwrap_or(spelled);
            cells.insert((row, column), spelled.to_string());
        }
    }
    let rows: BTreeSet<u32> = cells.keys().map(|(row, _)| *row).collect();
    let text = |s: &String| s.len() > 2 && s.starts_with('"');
    rows.into_iter()
        .filter_map(|row| {
            let at = |c: char| cells.get(&(row, c));
            let (sheet, addr, value) = (at('A')?, at('B')?, at('C')?);
            (text(sheet) && text(addr) && !value.starts_with('('))
                .then(|| format!("(derived {sheet} {addr} {value})"))
        })
        .collect()
}

/// A cartridge's text loaded through the engine and stepped `frames` times
/// at an unbounded budget: each step's row, `(step <frame> <evaluated> <of>
/// done)`, or the load's refusal in the engine's own words.
pub fn step_rows(text: &str, name: &str, frames: u64) -> Result<Vec<String>, String> {
    let mut cart = Cartridge::load(text, name, text.len()).map_err(|e| e.refusal.text)?;
    let mut rows = Vec::with_capacity(frames as usize);
    for _ in 0..frames {
        let step: Step = cart.step(usize::MAX);
        rows.push(step.row());
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::step_rows;

    /// The smallest grid cartridge: a manifest, a Screen of four by two, and
    /// one formula reading the Clock's frame.
    const SMOKE: &str = concat!(
        "; SPDX-License-Identifier: AGPL-3.0-or-later\n",
        "(cartridge \"smoke\"\n",
        "  (spec 3)\n",
        "  (title \"Smoke\")\n",
        "  (rate 30)\n",
        "  (screen grid 4 2)\n",
        "  (seed 1))\n",
        "(sheet \"Screen\" visible)\n",
        "(formula \"Screen\" \"A1\" \"=Clock!B1*2\")\n",
    );

    #[test]
    fn the_pin_loads_and_steps_a_cartridge() {
        let rows = step_rows(SMOKE, "smoke.vla", 3).expect("the smoke cartridge loads");
        assert_eq!(rows.len(), 3);
        for (i, row) in rows.iter().enumerate() {
            let frame = i as u64 + 1;
            assert!(
                row.starts_with(&format!("(step {frame} ")) && row.ends_with(" done)"),
                "frame {frame}: {row}"
            );
        }
    }

    #[test]
    fn a_refusal_comes_back_in_the_engines_words() {
        let err = step_rows("(not a cartridge)\n", "bad.vla", 1).expect_err("refused");
        assert!(!err.is_empty());
    }
}
