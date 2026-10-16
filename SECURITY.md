<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- SPDX-FileCopyrightText: 2026 Spreadsheet Company -->

# SECURITY.md

*A place a report can land, and a stated commitment, before the game
exists.*

## Reporting a vulnerability

**Contact: `english@spreadsheet.company`.** Do not open a public issue for
a vulnerability.

Please include the commit hash or the version you are running, the browser
if the issue is in the page, and the smallest replay or the smallest edit
that reproduces it.

## Response commitment

We will acknowledge a report within three business days. We do not commit
to a fixed remediation timeline, since severity and complexity vary and a
promised deadline the project cannot reliably meet is worse than an honest
one. The reporter is kept informed of our assessment and progress until the
issue is resolved, and a critical, actively exploitable issue is
prioritized above all other work the moment it is confirmed.

## What a reviewer checks on the artifact, not on faith

The game is a cartridge: a VLA program and its sheets, rows the engine
loads. It evaluates nothing itself; every value is the language crate's,
which has no imports at all, and the engine's module imports nothing,
which Alonzo's own checks hold on the module's import section at every
push of the version this repository pins (`Cargo.toml`). What a cartridge
can do is what the engine's devices let it do: draw, sound, read the keys
and the mouse. It makes no outbound network call, ever: Frazaro's `SD-13`
crosses the road into Alonzo and from there into this repository, and the
page's files, when they are vendored here, are held by Alonzo's offline
check as they are there.

A cartridge's formulas are the ones it was loaded with: the step computes
values and never formulas (Alonzo's `AD-7`), so the game cannot write code
at run time; the one place a formula enters at run time is a person's own
act on the Strategy sheet, recorded by the replay as an edit. The market is
a pure function of the seed, the difficulty and the scenario, and a replay
reproduces every frame (`docs/OMAKASE.md`, `BD-3`).

## What this game is not

Not advice, not a brokerage, not a feed. No real money is ever handled, no
real market's data is ever read, and every model in it is a toy, labeled as
one on its Caveats sheet, whose first row says plainly that the game is not
financial advice (`docs/OMAKASE.md`, `BD-5`, `MENU.5`).
