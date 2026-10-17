<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- SPDX-FileCopyrightText: 2026 Spreadsheet Company -->

# How to add a row

*Five minutes, a browser, and no Rust. Every string the game shows is a row
of a content sheet, each sheet a file of this folder, and a row is the
cheapest pull request there is.*

## The five minutes

1. Open the sheet's file in this folder and press the pencil to edit it on
   GitHub: `disclosures.vla` holds the fine print, a disclosure a row;
   `questions.vla` the exam's bank, a question a row; `firms.vla` the four
   styles the exam's last question offers, a style a row, of which the exam
   shows the first four; and `caveats.vla` the things this game gets wrong,
   a caveat a row.
2. Add your row above the file's last line, the one that opens with `))`.
   A row is the sheet's fields, in the order the file names them above its
   rows. A disclosure or a style is one line:

   ```text
   ("Your text here.")
   ```

   and a question or a caveat a field a line:

   ```text
   ("The concept it tests"
    "The question, as the proctor reads it:"
    "the first option"
    "the second option"
    "the third option"
    "the fourth option"
    "B"
    "Why the right option is right, in one line.")

   ("What this game simplifies, plainly."
    "What the real world does, plainly and accurately."
    "Management's response."
    "Market!B2")
   ```

   Write each text between the double quotes, with single quotes inside it
   if it needs any.
3. A question has four options, one of them right, and its answer is that
   option's letter, a capital from A to D. Its why is one line of at most
   140 characters that names no option, since a concept card quotes it on
   its own; and its concept is the idea it tests, which the exam never shows
   before the question is answered. A caveat says what the game gets wrong,
   then what the real world does, which a finance educator checks before a
   release, then Management's response, the joke, as a model's owner answers
   a finding, then where: the cell or range it qualifies, as a formula
   writes one, or empty for the game as a whole. Each of its texts is one
   line within its column's width, which the file sets at its top; and it
   goes after the first caveat, the plain statement that the game is not
   advice.
4. Keep the voice, below.
5. Commit with a sign-off, a last line in the commit's description that
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
it, so a row added changes it. The repository's reviewer expands it again on
your branch, reads your row against the policy, keeps the bank's answers
spread over the four letters and a caveat's real world true, and approves
and merges the pull request: that approval is the row's review, and the row
names no reviewer. Your row ships with the next release, shown in its turn.

With Rust installed, you can expand the cartridge yourself and run the
game's oracle, which holds every frame of the game and every row whole:

```text
cargo run --release --example expand
cargo test --workspace
```

## A new sheet

A new kind of string is a new sheet, and a sheet is more than words: the
game's source includes it and says what the game does with it, and the
layout check counts it. That is a dish of `docs/OMAKASE.md` and not a row;
open an issue first.
