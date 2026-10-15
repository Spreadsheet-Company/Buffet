<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- SPDX-FileCopyrightText: 2026 Spreadsheet Company -->

# How to add a row

*Five minutes, a browser, and no Rust. Every string the game shows is a row
of a content sheet, each sheet a file of this folder, and a row is the
cheapest pull request there is.*

## The five minutes

1. Open the sheet's file in this folder (`disclosures.vla` holds the fine
   print, a disclosure a row) and press the pencil to edit it on GitHub.
2. Add your row on a new line above the last one, the line that holds
   `))` alone. A row is its vintage, its approver and its text:

   ```text
   (0 "" "Your text here.")
   ```

   Write the vintage the rows above yours have; leave the approver empty,
   `""`, for the reviewer; and write your text between the double quotes,
   with single quotes inside it if it needs any.
3. Keep the voice, below.
4. Commit with a sign-off, a last line in the commit's description that
   reads `Signed-off-by: Your Name <you@example.com>`, which certifies the
   Developer Certificate of Origin (`CONTRIBUTING.md`), and open the pull
   request.

## The voice

The game looks and feels like a fund manager's workbook, and a row reads as
its compliance department would write it: deadpan, understated, never a wink
at the camera. The humor is financial and economic. The policy's rules, in
full in `CONTRIBUTING.md`:

- **Punch up.** Institutions, archetypes, jargon and ourselves; never a named
  individual, never people who lost their savings.
- **Fictional names.** A parody of a living company or a real mark gets a
  reader's glance before it ships.
- **Clarity first.** The game teaches; a joke is the vehicle, never the fog.
- **PG-13.** The audience includes classrooms.
- **A fund manager's words.** No kitchen in the game: the title, "Buffet",
  and the tagline, "All You Can Sheet", are the only exceptions.

## What happens next

One check stays red at first, and that is expected. The game's cartridge,
`src/buffet.vla`, is the pass's expansion of the game's source and of every
sheet it includes, committed so that anyone loads the game without building
it, so a row added changes it. A reviewer expands it again on your branch,
reads your row against the policy, sets its vintage if a release came since
the rows above it, writes their GitHub name as its approver, and merges it.
Your row ships with the next release, shown in its turn.

With Rust installed, you can expand the cartridge yourself and run the
game's oracle, which holds every frame of the game:

```text
cargo run --release --example expand
cargo test --workspace
```

## A new sheet

A new kind of string is a new sheet, and a sheet is more than words: the
game's source includes it and says what the game does with it, and the
layout check counts it. That is a dish of `docs/OMAKASE.md` and not a row;
open an issue first.
