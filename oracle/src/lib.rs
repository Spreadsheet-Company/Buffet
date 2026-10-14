// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Spreadsheet Company

//! Buffet's oracle (`OMAKASE.md`, `MISE.2`): the game written a second time
//! by hand, the way a person would write it, so that the cartridge's frames
//! are held to a rule written twice; the runners that play it, as examples;
//! and the tests that load the cartridge through the engine and hold every
//! frame to the hand.
//!
//! Nothing of the game is here yet. The crate holds one function and one
//! test, the smoke test of the pin: a two-row cartridge loaded through
//! `alonzo` from crates.io, stepped, its step rows read. The day the pin
//! resolves from the registry (`HOUSE.2`), this is the first green build.

use alonzo::cartridge::Cartridge;
use vla_lang::machine::Step;

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
