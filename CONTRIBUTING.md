<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- SPDX-FileCopyrightText: 2026 Spreadsheet Company -->

# Contributing to Buffet

*Short on purpose, in Alonzo's shape: what has to exist before the first
outside patch, not after. The cheapest pull request there is, is a joke:
one row in one sheet.*

## Sign your commits (DCO)

Contributions are accepted under the [Developer Certificate of
Origin](https://developercertificate.org/) 1.1. Sign each commit:

```text
git commit -s
```

That adds a `Signed-off-by:` line certifying you wrote the change or have
the right to submit it.

## Two kinds of contribution, two kinds of paperwork

| What you touch | Path | License | What you sign |
|---|---|---|---|
| the content: a joke, an exam question with its why, a headline, a memo, a name, an achievement, a row of the accuracy sheet, a scenario | `src/content/`, `*.md`, `docs/` | CC-BY-SA-4.0 | the DCO alone |
| the code: the game's source and library, its rows, a replay or a golden, the game written by hand, a runner, a check | `src/` (less `src/content/` and the kit's copy, `src/common.vla`), `oracle/`, `tools/`, the workspace's own files | AGPL-3.0-or-later, alone or with CC-BY-SA-4.0 in the game's own files | the DCO and, once, `CLA.md` |

By contributing, you agree your contribution is licensed under the license
of the file it touches, as `REUSE.toml` declares it. The agreement for code
exists so that Spreadsheet Company can offer the game under a commercial
license too (`COMMERCIAL.md`); it promises that every contribution stays
available to everyone under the public terms.

## The content policy

The game is fun insofar as it is funny, and every player-visible string is
a row of a content sheet. A row is reviewed by one named human before it
is merged, under four rules:

- **Punch up.** Institutions, archetypes, jargon and ourselves. Never a
  named individual, never people who lost their savings.
- **Fictional names.** Firms, people and tickers are invented. A parody
  that points at a living company or a real mark is marked `(check)` in
  the row and gets a reader's glance before it ships.
- **Clarity first.** The game teaches. A joke may be the vehicle; it is
  never the fog. An exam question carries its one-line *why*.
- **PG-13.** The audience includes classrooms.

The voice is the design reference's (`docs/BUFFET.md`, section 2): deadpan,
understated, never a wink at the camera. A one-page *How to add a row*
sits beside the content sheets the day they exist (`docs/OMAKASE.md`,
`MENU.1`).

## What a change to the game needs

- **A course is a game** (`docs/OMAKASE.md`, `BD-1`). A change to an earlier
  course's rows re-records that course's replay and says so; a later
  course's build never moves an earlier course's bytes, which the pass's
  check over every frozen course's source holds (`MISE.1`).
- **The hand holds every frame.** A rule changed in the cartridge is
  changed in the game written by hand (`oracle/`, `MISE.2`) and in the
  page's own copy of it, and `cargo test` holds the replay goldens to it.
  A rule changed on purpose re-records the replay whose digest pins the
  cartridge.
- **The market is open and exogenous** (`BD-3`). No `RAND`, no clock but
  the frame, no price moved by an order, no hidden state a rule may read.
- **The day is the clock** (`BD-7`). Only the Day sheet reads the frame;
  everything else reads the day.
- **A roadmap ID** is minted in `docs/OMAKASE.md`, in its family, and never
  reused; an item of Alonzo's or Frazaro's keeps its own ID.
- **A commit** is titled `<ID>: <the claim, in plain English>`; its body
  gives the defect and the fix, then `Pins:`, `Floors:` and `Docs:`.

Run the checks before you open a pull request: `tools/run_checks.ps1`, one
line a check and one total, and `-WithExtras` for the controls that prove
each check on mutants; beside it the Cargo oracle, `cargo build --workspace`,
`cargo fmt --all -- --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`. CI runs both on every push and pull request
(`.github/workflows/checks.yml`): the checks under Windows PowerShell 5.1,
and the Cargo half on Linux with `--locked`, the engine's module built for
the browser beside it and the page built with it and held offline
(`HOUSE.2`, `HOUSE.4`). A page you built before an edit of `src/` fails the
offline check until `tools/build_web.ps1` builds it again.

## Security

Do not open a public issue for a vulnerability. See `SECURITY.md`.

## The names

"Buffet" and "all you can sheet" are names held by Spreadsheet Company; see
`TRADEMARK.md`. A fork is welcome under the licenses above and needs its
own name.
