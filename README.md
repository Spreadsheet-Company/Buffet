<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- SPDX-FileCopyrightText: 2026 Spreadsheet Company -->

# Buffet: all you can sheet

*An idle game in which you run an asset-management firm, and a spreadsheet
you are allowed to open. Written on Alonzo, Spreadsheet Company's game
engine, in VLA, its language: every number on every screen is a cell of a
workbook the player may read, and by the late game the player has written a
trading bot in formulas and nobody told them that was programming.*

---

## What it is

You begin with a pencil, an exam and a small pile of other people's money.
The market breathes in cycles you can learn but not solve. Licences open
the stations one at a time, stocks first, then bonds, the firm, leverage,
derivatives, the Strategy sheet, the rest of the menu and the rivals, so
that the first five minutes are checkers and the chess arrives one licence
at a time. Every mechanic is cells you can read, which is the lesson and
the joke: the game is fun insofar as it is funny, and it is a tutorial
because it cannot help it, the model being open by design.

**Nothing is built yet.** The repository was founded on 2026-10-14. The
design reference is [`BUFFET.md`](BUFFET.md); the roadmap is
[`OMAKASE.md`](OMAKASE.md), the chef's order: eight courses, each a
playable game composed on the one before, and the kitchen's families of
work; [`SPIKE.md`](SPIKE.md) is the ledger of what has been served, empty.

## The terms

| What | Where | License |
|---|---|---|
| the game: every cartridge, source and library, the replays and goldens, the game written by hand with its runners and tests, the checks and the builders | `cartridges/`, `oracle/`, `tools/`, the workspace's own files | AGPL-3.0-or-later |
| the game's own folder, whose source includes the content and whose cartridge carries it | `cartridges/buffet/` | AGPL-3.0-or-later AND CC-BY-SA-4.0 |
| what the diner reads: the content sheets and the documents | `cartridges/**/content/`, `*.md`, `docs/` | CC-BY-SA-4.0 |
| the kit, copied from Alonzo | `kit/` | 0BSD |
| the page's files, vendored from Alonzo | `web/` | Apache-2.0 |

Why this shape, in three sentences. The game's cells are open by design, so
nothing in it can be protected by secrecy, and the licence is the only
protection there is: the AGPL keeps a fork open under the same terms and
makes a copy served from anyone's machines publish its changes, which is
the deterrent against a rebrand with a larger budget. The content is
share-alike so that a stranger's five-minute joke needs no paperwork but a
signed commit. A commercial license is sold to whoever wants out of the
AGPL ([`COMMERCIAL.md`](COMMERCIAL.md)), which is what the contributor
agreement for code ([`CLA.md`](CLA.md)) exists for, and the names are the
company's ([`TRADEMARK.md`](TRADEMARK.md)): the code forks and the name
does not.

`REUSE.toml` is the per-file map and `tools/check_spdx.ps1` holds the tree
to it on every push. Contributions: [`CONTRIBUTING.md`](CONTRIBUTING.md).
Vulnerabilities: [`SECURITY.md`](SECURITY.md).

## Building

The game is a cartridge, rows the engine loads, and the workspace holds its
oracle: the game written a second time by hand, the runners that play it
and the tests that hold every frame of the cartridge to it. It consumes the
engine and the language from crates.io as a stranger does, pinned exactly
in `Cargo.toml`:

```text
cargo test --workspace
tools\run_checks.ps1
```

Both pins, `alonzo = "=0.1.0"`, Alonzo's first release, and
`vla-lang = "=0.9.3"`, the language it pins, resolve from crates.io, and
`Cargo.lock` is committed.

## Not advice

Not advice, not a brokerage, not a feed: no real money, no real market's
data, and every model a toy, labelled as one.
