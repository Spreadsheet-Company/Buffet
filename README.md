<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- SPDX-FileCopyrightText: 2026 Spreadsheet Company -->

# Buffet: "All You Can Sheet"

*A semi-idle game in which you run an asset-management firm inside a spreadsheet. Written on Alonzo, Spreadsheet Company's game
engine, in VLA, its language: every number on every screen is a cell of a
workbook the player may read, and by the late game the player has written a
trading bot in formulas and nobody told them that was programming.*

---

## What it is

You begin with a pencil, an exam and a small pile of other people's money.
The market breathes in cycles you can learn but not solve. Licenses open
the stations one at a time, stocks first, then bonds, the firm, leverage,
derivatives, the Strategy sheet, the rest of the menu and the rivals, so
that the first five minutes are checkers and the chess arrives one license
at a time. Every mechanic is cells you can read, which is the lesson and
the joke: the game is fun insofar as it is funny, and it is a tutorial
because it cannot help it, the model being open by design.

**Course 0 is being cooked.** The repository was founded on 2026-10-14. The
design reference is [`docs/BUFFET.md`](docs/BUFFET.md); the roadmap is
[`docs/OMAKASE.md`](docs/OMAKASE.md), the chef's order: eight courses, each a
playable game composed on the one before, and the kitchen's families of
work; [`docs/SPIKE.md`](docs/SPIKE.md) is the ledger of what has been served.

## The terms

| What | Where | License |
|---|---|---|
| the game: every cartridge, source and library, the replays and goldens, the game written by hand with its runners and tests, the checks and the builders | `src/`, `oracle/`, `tools/`, the workspace's own files | AGPL-3.0-or-later |
| the game's own files, whose source includes the content and whose cartridge carries it | `src/buffet*.vla` | AGPL-3.0-or-later AND CC-BY-SA-4.0 |
| what the diner reads: the content sheets and the documents | `src/content/`, `*.md`, `docs/` | CC-BY-SA-4.0 |
| the kit, copied from Alonzo | `src/common.vla` | 0BSD |
| the page's files, vendored from Alonzo | `web/` | Apache-2.0 |

Why this shape, in three sentences. The game's cells are open by design, so
nothing in it can be protected by secrecy, and the license is the only
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

The game is a cartridge, rows the engine loads, and all of it is in `src/`:
the source a person edits, `src/buffet.source.vla`; the cartridge the
engine's pass expands it into, `src/buffet.vla`, committed so that a
stranger can load it without building anything; Buffet's own library of
macros, `src/kitchen.vla`; and Alonzo's kit, `src/common.vla`, copied at the
engine's pin. The workspace holds the game's oracle: the game written a
second time by hand, the runners that play it and the tests that hold every
frame of the cartridge to it. It consumes the engine and the language from
crates.io as a stranger does, pinned exactly in `Cargo.toml`:

```text
cargo run --release --example expand      # src/buffet.source.vla into src/buffet.vla
cargo test --workspace
tools\run_checks.ps1
```

Both pins, `alonzo = "=0.4.0"` and `vla-lang = "=0.14.0"`, the language
that release pins, resolve from crates.io, and `Cargo.lock` is committed.

The page is Alonzo's player, its template and scripts vendored unchanged
in `web/` at the tag the pin names, filled with the engine's module and the
game's texts into one file that runs from disk and fetches nothing:

```text
cargo build --release --locked -p alonzo --target wasm32-unknown-unknown
tools\build_web.ps1                       # web/index.html: open it, pick Buffet
```

## Not advice

Buffet is a game, and nothing in it is financial advice: advice starts from
a person's goals, means and timeline, and the game knows none of yours. It
is no brokerage and holds no real money; its market is a formula of a seed
and a day, fed by no real market's data; and every model in it is a toy,
labeled as one on its Caveats sheet, *Things this game gets wrong*.

*Any resemblance to financial advice, living or dead, is purely
coincidental.*
