<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- SPDX-FileCopyrightText: 2026 Spreadsheet Company -->

# BUFFET: THE SPIKE

*Where a finished ticket goes. `OMAKASE.md` holds open items only; the day
an item is served, built, its tests passing and the owner's hand test among
them, and committed, its line moves here verbatim under its family, with
its build record beneath it: what landed, what it cost against its
prediction, what it found. The scoping of an item, the decisions the owner
approved, lives here too, under the item's heading, from the day the
scoping is approved. Alonzo keeps the same ledger as `REARVIEW.md`; this
file is its twin for the game. Founded 2026-10-14 with the repository,
empty.*

# THE CLOSED LEDGER

## 🏠 THE HOUSE

- ✅ **`HOUSE.3` — the kit copied and held.** Built 2026-10-14 with `POT.1`, the kit's first reader: `src/common.vla`, Alonzo's `cartridges/common.vla` at the tag the pin names, `v0.1.0`, byte for byte with its 0BSD header, beside the game's source in the one folder (`MISE.1`), its name kept so that it diffs against Alonzo's by name; and `tools/check_kit.ps1` holding the copy to that file's SHA-256 over its LF text, `315D3611…BFFC16C9`, 23,787 bytes, read off the tag once and pinned, since the registry package carries no cartridges; a pattern Buffet hoists to the kit goes by a pull request to Alonzo and the copy follows at the next pin. What the kit gives the first source, so that nothing of it is written twice: `sheets`, `labels`, `headers`, `on`, `cells` and `formulas`; `looks`; `default-keys` and `keys`; `camera`; `put`, the Write sheet's row; and the verbs `first-frame`, `pressed`, `count`, `hold`, `dice`, `every` and `since` (the kit's `dice` reads the frame and rolls every frame, so the market's die is the kit's `hold` over the day instead, `POT.1`). *Needs:* `HOUSE.2`. *Oracle, met:* the check green; `-Control`, a copy with one byte changed and a copy missing each failing by name; `run_checks.ps1`'s floor raised to 2. *Pays into:* every source, which includes the kit, then `MISE.1`'s library. `~hours`

  *Closed 2026-10-14, a commit of its own.* Predicted `~hours`; cost under an hour, inside `POT.1`'s session. The plan's folder, `kit/`, did not outlive the owner's question of the day, why one file wants a folder of its own: the copy stands in `src/` under its own name, the license map names it, and the check holds it by its digest (`MISE.1`'s decision of the day). The check reads the copy's text with CRLF as LF, since the tag stores the file LF and a checkout writes it CRLF.

- ✅ **`HOUSE.1` — the founding: the repository, its terms, its names, the oracle crate, and the first check.** Built 2026-10-14 at the owner's word, the root commit `54bd5ef`. What landed: `LICENSE` and `LICENSES/` (the four texts), `REUSE.toml` and `tools/check_spdx.ps1` (an expression read identifier by identifier; every text file under `cartridges/` and `kit/` carrying the header the map gives it; `-Control` five mutants), `tools/run_checks.ps1` at a floor of 1, `.github/workflows/checks.yml` (the ratchets job), `NOTICE`, `TRADEMARK.md`, `COMMERCIAL.md`, `CLA.md`, `CONTRIBUTING.md` (the content policy stands there since this day), `SECURITY.md`, `README.md`, `SPIKE.md` (the ledger, empty), `BUFFET.md` and this file moved from Alonzo's `cartridges/buffet/` and re-headed, the workspace and the oracle crate (`oracle/`, `buffet-oracle`, `publish = false`, one function and two tests, the smoke test of the pin loading a two-row cartridge through the engine and stepping it), `.gitattributes`, `.gitignore` and `rust-toolchain.toml` in Alonzo's shape. Alonzo's side the same day: `cartridges/buffet/` removed, `CART.11`'s line left its roadmap for its ledger. The owner's decision on the terms, written here so it can be cited: AGPL-3.0-or-later for everything that computes, CC-BY-SA-4.0 for what the diner reads, the expression over the game's own folder, 0BSD kept on the kit and Apache-2.0 on the page's files, a commercial license offered beside the public terms, a contributor agreement for code and the DCO alone for content, and the names held under a trademark policy; the reasoning is `README.md`'s "The terms". Verified that day: `tools/run_checks.ps1 -WithExtras` ALL GREEN; `cargo build`, `fmt --check`, `clippy -D warnings` and `test` green. *Served 2026-10-14:* the remote created under `Spreadsheet-Company/Buffet`, public, by the owner; `main` pushed with its ten commits; CI's ratchets job green at the first push, run 38628730253. *Pays into:* everything; a stranger's path, the checks, the ledger. `~hours` to close

  *Closed 2026-10-14, at the first push.* Predicted `~hours` to close; cost the push and one run. The ratchets job ran the three checks and their three controls ALL GREEN under Windows PowerShell 5.1 in 23 seconds, run 38628730253 (https://github.com/Spreadsheet-Company/Buffet/actions/runs/38628730253).

- ✅ **`HOUSE.2` — the pin on crates.io.** Built 2026-10-14, `c1fd6fa`, the day Alonzo's `REPO.4` published `alonzo` 0.1.0: the local `.cargo/config.toml` patch over `../Alonzo/engine` deleted, `alonzo = "=0.1.0"` and `vla-lang = "=0.9.3"` (the version that release pins, so the lock holds one copy of the language) resolving from the registry with crates.io's checksums, `Cargo.lock` committed, the two tests green over the downloaded crate, `cargo tree` holding no `frazaro-core` (`AD-1`). *Served 2026-10-14:* the crate job in `.github/workflows/checks.yml`, `765e06f`, in Alonzo's shape (build, `clippy -D warnings` and `test`, each `--locked`, and `fmt --check`, then the engine's module built from the registry crate for the page, `cargo build --release --locked -p alonzo --target wasm32-unknown-unknown`), green at the first push. *Oracle, met:* `cargo build --locked` in a clean clone with no patch; CI green on both jobs, run 38628730253. *Pays into:* every build; `HOUSE.4`'s module. `~hours` to close

  *Closed 2026-10-14, at the first push.* Predicted `~hours` to close; cost one commit, `765e06f`, and its first run, 38628730253, green in 31 seconds on Linux: `alonzo` 0.1.0 and `vla-lang` 0.9.3 downloaded from crates.io under the lock's checksums and compiled, the 14 tests passing, and the engine's module built from the registry crate. The oracle's first run on Linux, where the engine's own first run had found a path defect (Alonzo's `fc6dc4f`), found none.

- ✅ **`HOUSE.4`, first slice — the page.** *Scoped, built and hand-tested 2026-10-15: the scoping, its ten decisions approved whole by the owner, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* Alonzo's page vendored unchanged at the tag the pin names, `v0.1.3` (`alonzo` =0.1.3, whose page draws the grid from the `values` view, `ENGINE.8`'s second slice): the template and its three scripts, `web/index.template.html`, `web/viewport.js`, `web/host.js` and `web/fake.js`, Apache-2.0 kept with their headers, and the kit copied again at the same tag; `tools/check_vendored.ps1`, grown from the kit's check, holding every copy to its SHA-256 at the tag and the tag to the pin. `tools/build_web.ps1`, Buffet's own, adapted from Alonzo's, fills the template's five places: the three scripts; the engine's module, built from the registry crate under the workspace's release profile; and `{{BLOCKS}}`, every text the page holds as a block (`ENGINE.15`): the page's name, the cartridge and its source, and the kit and the library at `src/common.vla` and `src/kitchen.vla`, the paths the source's includes resolve to, so that Build in the drawer writes the cartridge's bytes; the replay and golden blocks come at `COURSE.0`'s tasting. `tools/check_web_offline.ps1` adapted: the page loads nothing and links nowhere, and a built page's blocks are `src/`'s files both ways. The page is Alonzo's full-screen player (`ENGINE.14`), opening on the empty sheet with Buffet in the picker after it (`ENGINE.15`, the owner's word of 2026-10-14), and it shows the model's sheets live by their tabs before any panel exists, so first light is `FRONT.1`'s, the dish that completes it. The second slice, the loop oracle's cases and the JavaScript hand, is the tasting's. *Needs:* `HOUSE.2`. *Oracle, met:* the page built here carrying the module, the offline check green over it, and the owner's hand test on it; in CI at the push. *Pays into:* every hand test after it; first light; the playtest pass's browser half; the deploy at service. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test, beside the dev profile's optimization (`00eb51c`, a commit of its own). What landed: the pin at `alonzo` 0.1.3 and its lock; the kit copied again and the page's four files copied, all at `v0.1.3`; `tools/check_vendored.ps1`, grown from `check_kit.ps1`, its control five mutants; `tools/build_web.ps1`; `tools/check_web_offline.ps1`, its control fourteen mutants; `run_checks.ps1`'s floor raised to 4; the page built in both CI jobs, the ratchets job's without the module. The cartridge's bytes and the describe golden stood. One of the seven predictions missed: the module came out 48 bytes larger than the 0.1.0 build it was set against, the engine's catalog citing `docs/SPEC.md` since 0.1.1. CI's numbers (P6) and the Linux half of P2 came with the push, run 38712895205, and were written into their cells with `MENU.1`.

## 🍽 THE COURSES

## 🍲 THE STOCK POT

- ✅ **`POT.1` — the calendar and the dice.** *Scoped, built and hand-tested 2026-10-14: the scoping, the owner's hand test and what they found are under THE ITEMS, SCOPED.* A Day sheet, the one sheet that reads `Clock!B1`, a label in column A beside each value: the frames a day, 30, a value the manifest's `(rate 30)` and a test hold equal, so that the teacher's dial, the Clock's rate, scales the game without touching a formula; the days a year, 365, every day a market day; the day from 0, `=INT((Clock!B1-1)/B1)`, and the phase, `=MOD(Clock!B1-1,B1)`, the frame less one so that the first frame opens the first day and every day holds its thirty; the boundary, `=IF(B4=0,1,0)`; the year and the day of the year, each from 1. A Dice sheet of two streams, each a die and its draws, each draw's uniform beside it. The market's die is the kit's `hold` over the boundary, and not the kit's `dice`, which reads the frame and rolls every frame (`BD-7`, `BD-3`): the seed on the first day, and else yesterday's die jumped 1,017 draws by one multiplication exact in a double, so that the market is a function of the seed and the day alone, and a day's draws are taken by the dishes one by one, a later draw moving no earlier one. The exam's die is the seed jumped 2^30 + 88 draws, half the period away, so that the market and the exam never share a die. Four market draws and two exam draws so far, `POT.2`'s phases and `GUARD.1`'s start and stride the first to read them, by the library's first verb, `draws`; a near-normal draw later as twelve uniforms less six (`POT.5`), each uniform a draw's cell. *Oracle, met:* the owner's hand test in Alonzo's page; every frame of two years and two days held to the hand (`MISE.2`), the calendar at every frame and the dice moving on a day's first frame alone; the same dice on the same days at one frame a day; no formula outside the Day sheet reading the frame or the phase, and the census's `(every-frame 0)` (`BD-7`, whose oracle the scoping corrected); the library's ten-thousandth draw from 1 equal to Park and Miller's published 1,043,618,065; seven mutants of the cartridge, the library and the source, each caught. *Pays into:* every dish after it reads the day and never the frame, so each costs a boundary and nothing between; the exam's die (`GUARD.1`); the phases of `POT.2`; the timers of `PLATE.3`. `~days`

  *Closed 2026-10-14.* Predicted `~days`; cost one session, which also copied the kit (`HOUSE.3`), laid the layout's first pieces (`MISE.1`) and founded the game by hand (`MISE.2`), each of those its own line. What landed: the source, `src/buffet.source.vla`, and its cartridge, `src/buffet.vla`, 41 rows; the library, `src/kitchen.vla`, and its first verb, `draws`; the hand, `oracle/src/buffet.rs`; eight tests of the cartridge in `oracle/tests/cartridge.rs` and four of the hand; the runner, `oracle/examples/expand.rs`; and the describe golden.

- ✅ **`POT.2` — tier 1: four waves and a drift, prices a function of the day.** *Scoped, built and hand-tested 2026-10-14: the scoping, its sixteen decisions approved whole by the owner, every prediction beside its number, the owner's hand test and what it raised are under THE ITEMS, SCOPED.* The Market sheet, its name plain as the model's are: the index `=g^day * (1 + A1*SIN(w1*day+p1) + ... + A4*SIN(w4*day+p4))` over its own value on the first day, so that it opens at exactly 1 and a rung at its price, with Kitchin, Juglar, Kuznets and Kondratiev periods each a prime number of days, 1,459, 3,271, 6,947 and 18,251 (about 4, 9, 19 and 50 years, the Kuznets moved off 18, where it would be two Juglars), so that no ratio is a whole number; amplitudes 0.1, 0.15, 0.1 and 0.15; the drift 1.0002 a day; the phases the market's draws 1 to 4 of the first day, held. Tier 0, one wave of a year and the drift, the same at every seed, is written beside it as the picker's first rung, and the index is `CHOOSE` on the Difficulty cell from the first day, the cell a value of 1 until `POT.6` makes it a picker. No class row at course 0, stocks being the market itself; no price, the four rungs being `STATION.1`'s, each `=INT(<its start in cents>*<its multiplier>+0.5)`, the idiosyncratic factor tier 2's. A price is a function of the day alone, so a boundary recomputes the market once and the days between recompute nothing (`BD-7`), and the index twenty days back (`Market!C25` beside today's `B25`, the same fill, recomputed and not remembered) gives `FRONT.3` its weather word's slope until `POT.4`. *Oracle* (rewritten 2026-10-14 at the owner's word, as that day's measurement leaves it: `vla-lang` 0.9.3 computes `SIN` and `^` with Rust's std, whose results differ between platforms and from the module's in the last bit): the hand, `oracle/src/buffet.rs`, calls the same std functions as the cells, in each formula's order, so it holds the index and every price exactly natively, here on Windows and in CI on Linux; the tests compare the hand with the cartridge live, and no committed golden holds an unrounded value derived from `SIN` or `^`, so whatever a replay golden holds later is rounded once to what it shows, as money is; the two-peak test, the path not repeating within a hundred years; and `BD-7`, no formula outside the Day sheet reading the frame or the phase and the census keeping `(every-frame 0)`, so the market recomputes on a boundary and nowhere else. *Pays into:* the rungs' prices (`STATION.1`), the weather word (`FRONT.3`), the benchmark (`CRITIC.1`), the picker's first rung (`POT.6`). `~days`

  *Closed 2026-10-14.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: the Market sheet in the source and its cartridge, now 96 rows and 51 formula cells; the hand's market in `oracle/src/buffet.rs`, the same functions as the cells in each formula's order; the Market held inside the test of every frame of two years, three tests of the cartridge more (its settings, a year at difficulty 0, a hundred years at one frame a day) and three of the hand (both tiers opening at 1, the amplitudes, the two-peak test), twenty in all; the describe golden blessed again, moved by the Market alone; and, in the same commit at the owner's word, `Cargo.toml`'s comment naming the pin it holds, `alonzo` 0.1.1. Three of the eight predictions missed in part, each named beside its number: the nearest repeat's lag, seed 777's first year falling on every one of its days, and an ordinary frame 0.1 µs slower. The page held to the hand to the last bit in every one of the 70 cells the hand test showed.

## 🍳 THE STATIONS

- ✅ **`STATION.1`, first slice — stocks: four rungs.** *Scoped and built 2026-10-14, hand-tested 2026-10-15: the scoping, its fifteen decisions approved whole by the owner, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The Stocks sheet, its name plain as the model's are, a rung a row, cheapest first: four fictional rungs at $1, $5, $25 and $100, labeled `rung 1` to `rung 4`, the working labels (a penny stock, a small cap, a mid cap and a blue chip) kept in the docs and never in a cell until `MENU.3` names them, since real index and fund names are others' marks; each rung's start in whole cents, its beta and its dividend yield as values, and its workings one fill a column. A price is `=INT(<start>*Market!$B$25^<beta>+0.5)`, the index the difficulty picks to the rung's beta, whole cents, one rounding, half up: a power, so that a beta holds at every level of the index and no price reaches zero at any seed, none falling below 11, 96, 833 and 4,386 cents, a bound the hand holds; the betas 2, 1.5, 1 and 0.75, the third the market itself. Yesterday's price is the same formula over yesterday's index, `Market!D25`, the Market sheet's fills reaching a column D, recomputed and never remembered. The fourth rung pays a dividend, 2% a year, 50 cents a share fixed from its start, on the year's 91st, 182nd, 273rd and 364th days, so that income exists from the first quarter, the cell holding it all that day for `PLATE.1` to pay at the day's first frame. Written as rows and not yet a ladder, a rung at a time, in the order a walk over a list of rungs writes, since the ladder is the pattern's second appearance and enters the library with the bonds (`STATION.2`). *Oracle, met:* `buffet.rs`, every price today and yesterday, each dividend and the dividend today held at every frame of two years and two days and on every day of a hundred years at one frame a day; the opening exactly the starts; each rung's least at or above the bound; the dividend on exactly four days a year, the days the hand lists; `BD-7`'s scan and the census's `(every-frame 0)` kept; the owner's hand test in Alonzo's page, every cell it showed the hand's. *Pays into:* `PLATE.1`'s rows, one a rung; the Market panel's prices and their change; the ladder hoist at course 1, whose proof is that this slice expands to its bytes unchanged, which a scratch ladder of 24 lines has already met. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: the Stocks sheet and the Market's column D in the source and its cartridge, now 131 rows and 79 formula cells; the hand's rungs, their prices, the dividend's days and the bound in `oracle/src/buffet.rs`; the Stocks sheet held inside the tests of two years, a year at difficulty 0, eight hundred days and a hundred years at one frame a day, two tests of the cartridge more (the rungs' settings, the opening) and three of the hand (the opening at both tiers, the bound, the dividend's days), 27 in all; the describe golden blessed again, moved as predicted; and, in the same commit at the owner's word, `POT.2`'s P2 cell, CI's Linux holding the market bit for bit at the first push. One of the eight predictions missed, and only at first: the suite took 11.6 s while the Stocks sheet's 56 cells were viewed at every frame and every day, and 7.4 s once the window held the four columns that compute, half again the old suite's time on the same machine under the same load. The build found that a cent is a wide step for a $1 share: the first rung's price moves on 25 of the first year's 365 days.

## 📜 THE MENU CARD

- ✅ **`MENU.1` — the strings sheets and the picker.** *Scoped, built and hand-tested 2026-10-15: the scoping, its fourteen decisions approved whole by the owner and amended at the hand test, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The content sheets under `src/content/`, a file a sheet holding words alone, one macro named after the file whose body is one call of the library's `strings`, each sheet one kind of string, hidden after the model's sheets, its rows of text each with the course that added it, its vintage, and the reviewer who approved it; the source's manifest its first form, then every include, then the code, each content sheet's macro called where the sheet belongs (the owner's words at the hand test); the library's `picker`, `CHOOSE(k, ...)` over a sheet's rows, generated from the sheet's extent so that a row added needs no formula edited, one cell to 1,547 rows below the headers, the pages of 500 written by the dish whose sheet first passes 1,000 rows; the first sheet, the fine print as a rotating footer, the Disclosures sheet, eight rows in the owner's words, a disclosure a month from the day; and the one-page *How to add a row* in the folder, for a stranger with five minutes. Every other sheet arrives with the dish that reads it, named for it: `Questions` (`MENU.2`), `Headlines` (`FRONT.3`), `Memos` (the first of `MENU.4` and `GUARD.3`), `Firms` (`BACK.2`), `Awards` (`MENU.7`), `Errors` (`PLATE.6`) and `Exits` (`FRONT.7`). *Oracle, met:* the layout check's content rule and the source's order (`tools/check_layout.ps1`, the content sheets' pin raised from 0 to 1, its control fifteen mutants), their headers held by `check_spdx.ps1`; the page's content blocks (`tools/build_web.ps1`, `tools/check_web_offline.ps1`, its control fifteen mutants); the oracle's tests, the fine print held to the hand at every frame, the rows counted by their extent, every row picked in its turn, a row added expanding with no other edit, the one picker's limit and the tripwire before it, and every sheet written bare with the content hidden; and the owner's hand test in the page. *Pays into:* every string any view shows: the ticker (`FRONT.3`), the exam (`MENU.2`), the names (`MENU.3`), the errors (`PLATE.6`), the endings (`FRONT.7`); and the cheapest pull request there is. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build, the owner's hand test and its amendment. What landed: `src/content/disclosures.vla` and its *How to add a row*, `src/content/README.md`; `strings` and `picker` in `src/kitchen.vla`; the source's include at its top, its call of the sheet and the fine print's index, and the game's notes rewritten for `BD-2`, the back office open and the front office not; the hand's rotation and five tests of the cartridge, 33 in all; the describe golden blessed again; the layout check's content rule; the page's content blocks; `CONTRIBUTING.md`'s fifth rule; and, in the same commit, the Linux cells CI's run 38712895205 answered for `STATION.1` and `HOUSE.4`. One of the nine predictions missed: the page grew 9,587 characters where about 5,000 were predicted, the library's own block forgotten; and P6 held at the median alone, one round of five past it. After the hand test passed, the owner rewrote seven of the eight rows, grouped every include together and set the manifest first, the includes after it and the code after them; the cartridge moved in those seven rows and nowhere else.

- ✅ **`MENU.2`, first slice — the exam bank: twenty questions and their whys.** *Scoped, built and hand-tested 2026-10-15: the scoping, its fifteen decisions approved whole by the owner, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The Questions sheet, `src/content/questions.vla`, hidden after the Disclosures sheet: twenty graded questions, the nineteen drafted in `BUFFET.md` §3, checked against `BD-2` and for accuracy, and a twentieth written, *Shares*; each its concept, the question, four options, the right one's letter, the letters spread five to each over the bank, and a one-line why a concept card can quote alone (`FRONT.5`); a finance educator's read before the release (`BD-5`), the deposit insurance's figure marked to confirm. The draft's twentieth, ungraded, which sets the archetype, is no row of the bank, being asked last and always and never drawn: its options are recommended to the Firms sheet and its prompt to the Exam view (`GUARD.1`, `BACK.2`), and the contract's sentence reads nine scored questions drawn from the bank of twenty, then a tenth, ungraded. The library's `strings` grown to several fields, each a header and a width, the fine print its case of one and expanding to its bytes unchanged, `picker` unchanged, a picker a field; the row shown, `Questions!M2`, a value until `GUARD.1`'s draw drives it, with the finding for that dish that a stride picked among the primes above 1,547 is coprime to every bank a picker holds, the dialect having no `GCD`. Second slices at each license, with `GUARD.2`, toward forty to sixty. *Oracle, met:* each content sheet held to its file, read by the language's own reader; every question whole; every field picked at every row; a question added expanding with no other edit; the first question shown; the tripwire and the tab order over both sheets; the layout check's content pin raised from 1 to 2, its control fifteen mutants over seven files; the describe golden blessed again; five mutants, each caught by the test written for it; and the owner's hand test in the page. *Pays into:* `GUARD.1` and every exam after it; every content sheet of several fields after it. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: `src/content/questions.vla`; `strings` of several fields in `src/kitchen.vla`, the Disclosures file's one line of fields respelled; the source's include, call and row shown, and the game's notes; five tests of the cartridge more and three extended, 38 in all; the describe golden blessed again; the layout check's pin and control; the *How to add a row* for a sheet of several fields; and, in `OMAKASE.md`, the contract's sentence, `GUARD.1`'s and `BACK.2`'s notes and the hoist ledger's row. Two of the nine predictions missed: the pass took 1.7 ms more where under one was predicted, a cost paid at a build and never at a frame, and the first frame some 40 µs more where under 10 was predicted; every frame after it held. After the hand test passed, the owner rewrote eighteen of the twenty questions and title-cased the bank's headers; the cartridge moved in seventy rows of text and nowhere else.

## 🔪 MISE EN PLACE

- ✅ **`MISE.3` — the first measurements: three probes, each a day in scratch, each deciding a spelling.** *Scoped, measured and hand-run 2026-10-14: the scoping, every number beside its prediction, the owner's hand run and what it found are under THE ITEMS, SCOPED.* Run over Alonzo's own page and runner with a Buffet-shaped scratch cartridge, before a row of the game is written, and each number written in `SPIKE.md` beside its prediction so that course 0's scoping cites them. (1) **The window's cost.** An 80 by 50 panel of texts and numbers drawn in Chrome at `(rate 30)`. The host draws from the full record until `ENGINE.8`'s second slice lands, and the record carries every formula's text: by `vla-lang` 0.9.3's release notes, an 80 by 50 window of one short formula costs 10.3 ms natively from the record and 1.7 to 1.9 ms from the `values` view (1.3 to 1.4 in the browser). A panel of many short formulas is Buffet's shape and nobody has measured it. The number decides whether course 0 ships over the record or asks Alonzo for the slice with the number in hand (`MISE.6`), and it is re-measured on the real window at first light (`HOUSE.4`). (2) **A number format with a glyph.** Whether the viewport honors a number format with a literal glyph in it (the record's `format` row carries a number format and `KERNEL.5`'s painter spells numbers by it; a `▲` in a format is (check)), which decides `FRONT.2`'s spelling of gains: a format, or a glyph cell beside the number. (3) **The picker's limit.** `CHOOSE` over references against the 8,192-character formula, about a thousand references, and the text join `&`, which the dialect has (the language's `Concat`, checked 2026-10-14), so that a ledger row's address may be joined from a count and not chosen; the number decides `MENU.1`'s picker and `PLATE.1`'s ledger ring. The fourth question the draft asked, the boundary frame against the ordinary frame, needs a model to measure and is answered by the first model: it is `POT.1`'s oracle. *Needs:* nothing. *Oracle:* the three numbers written beside their predictions. *Pays into:* `MISE.1`'s decisions, `FRONT.2`, `MENU.1`, `PLATE.1`, the ask of `ENGINE.8`. *Measured 2026-10-14, every number beside its prediction in `SPIKE.md`:* course 0's densest panel draws a frame in 1.5 to 1.7 ms in Chrome at full screen, so course 0 ships over the record; the spellings the numbers settle, approved by the owner the same day, stand as dated lines on `FRONT.1`, `FRONT.2`, `MENU.1`, `PLATE.1`, `MISE.1` and `MISE.6`; the owner's hand run in the owner's own Chrome made the browser number real, 1.4 ms a frame at p50. `~days`

  *Closed 2026-10-14.* Predicted `~days`, a day a probe; cost one session, the three probes and the owner's hand run. What landed: no file of the game, since every probe lived in scratch; in this file, the scoping, its predictions with the numbers beside them, what the probes and the hand run found, and the spellings; in `OMAKASE.md`, six spellings approved on their items and three candidates in `MISE.6`'s table. Five of the seventeen predictions missed in part, each named beside its number: the record's bar falls near 1,400 formula cells and not 2,000; a background spelled by cells costs more in Chrome than predicted; a joined text's two decimals take 140 characters; the picker's text costs a view 43 µs; the way out takes five cells and not four. The probes' premise for (2) did not hold, which the scoping found by reading before any probe ran.

- ✅ **`MISE.1` — the kitchen's layout, first slice: one folder, one library, a source a menu of calls, the content as files, and the checks the house keeps.** *Decided 2026-10-14 at the owner's word, at `POT.1`'s build:* the game lives in one folder, `src/`, and not in Alonzo's `cartridges/<name>/`, since this repository is one game's: the source, `src/buffet.source.vla`; the cartridge the pass writes beside it, `src/buffet.vla`, committed so that a stranger loads it without building; Buffet's own library, `src/kitchen.vla`; the kit's copy, `src/common.vla` (`HOUSE.3`); and later the content sheets under `src/content/` and the frozen courses under `src/courses/`. `REUSE.toml` names the game's own files, `src/buffet*.vla` and `src/courses/**`, the content and the kit's copy, and `tools/check_spdx.ps1` holds every text file under `src/` to the header the map gives it. The source includes the kit and the library, each by its own line, and the library includes nothing: the pass resolves every include against the source's own folder, an included file's includes too, so a library that included the kit would name it by a path true at one depth only. The library grows by the ratchet, a verb entering at its second appearance (`draws`, a stream's draws beneath its die, entered with `POT.1`, which writes two streams; `panel` and `tab`, a Screen region and the Camera move that shows it, when the second panel is written; `picker`, `CHOOSE` over a sheet's rows, at `MENU.1`; `timer` at `PLATE.3`; `ladder`, a station's rungs as rows from a list, at `STATION.2`; `station-total`, a roll-up over the stations a source names, at `BACK.3`). A course's source is a short menu of calls naming the dishes it serves and including the content sheets, one file a sheet, the stranger's path under the content's own terms (`MENU.1`, `REUSE.toml`); the cartridge carries the source's header, the game's expression, and is always the latest course, the one the page carries. The pass is the engine crate's, `alonzo::pass::expand`, driven by a runner of the oracle crate in the shape of Alonzo's own, `cargo run --release --example expand [-- --check]`, since a dependency's examples are not the workspace's to run. *Built with `POT.1`, 2026-10-14:* the folder, the library, the source and its cartridge, the runner, and the oracle crate's tests that the cartridge is its source expanded and that `describe` holds its golden, `oracle/testdata/describe/buffet.vla`, which `BUFFET_BLESS=1 cargo test describe` writes again. *Built at the house's close, 2026-10-14, in place of a port of Alonzo's cartridge check (805 lines), since a row is the engine's to judge and the oracle crate's tests load the cartridge through `alonzo` on every push:* the layout check, `tools/check_layout.ps1`: `src/` holds exactly the source, its cartridge and the two libraries, any other file failing by name, each kind counted against its pin (sources 1, cartridges 1, libraries 2, content sheets and courses 0 until they exist); the cartridge's second line is the pass's banner naming its source; the source includes `common.vla`, then `kitchen.vla`, each by its own line, and `kitchen.vla` includes nothing. The rest of the slice went where it belongs: the content sheets' folder rule to `MENU.1`, the replay test to `COURSE.0`'s tasting, the three open decisions to the second slice (`COURSE.1`), and the picker's approved shape to `MENU.1`. *Decided now, built at course 1:* each course served is frozen as a cartridge folder of its own, `src/courses/buffet<k>/` (rec; the names the scoping's), holding its source, its cartridge, its replay and its golden, its includes `../../common.vla` and `../../kitchen.vla`; the folder rule applied there is the second slice's. *Needs:* `HOUSE.3`; `MISE.3` for the picker's spelling. *Oracle, met:* the check green; `-Control`, a clean copy passing and seven mutants each failing by name and proving it applied (a stray file under `src/`, the cartridge missing, a banner naming another source, an include in `kitchen.vla`, the source's includes out of order, a content sheet and a course before their pins are raised); `run_checks.ps1`'s floor raised to 3. *Pays into:* every dish, each a call of the library and a sheet of the source; `HOUSE.4`'s places. `~days`

  *Closed 2026-10-14, at the house's close.* Predicted `~days`; cost parts of two sessions: the folder, the library, the source, its cartridge, the runner and the tests with `POT.1`, and the layout check with the house's close. The plan's port of Alonzo's `check_cartridge.ps1`, 805 lines that read every row because Alonzo's cartridges have no oracle crate beside them, gave way at the owner's word of `MISE.3`'s close to a check of 331 lines that reads no row, since Buffet's oracle crate holds every row through the engine. Its control has seven mutants where the owner named five: a content sheet and a course before their pins are raised were added, so that a pin of 0 is proved like the rest.

# THE ITEMS, SCOPED

## `POT.1` — the calendar and the dice

*Scoped 2026-10-14 at the owner's word, "Begin work on COURSE.0 by
scoping and implementing POT.1", and built the same session, so each
decision below is written as built, for the owner to approve, amend or
strike. Decision 1 is the owner's own, given while it was built.*

**What it stood on.** `POT.1` is the game's first row, so it needed three
items of the mise en place, each built as far as `POT.1` reads it: the
kit's copy and its check (`HOUSE.3`, whole); the layout (`MISE.1`: the
folder, the library, the source, the runner, and the tests that the
cartridge is its source expanded and that `describe` holds its golden);
and the hand (`MISE.2`, founded). `MISE.3`'s three probes decide spellings
`POT.1` never uses, a panel's draw, a glyph in a format and the picker's
limit, so they wait for the dishes that do.

**The decisions.**

1. **The game in one folder, `src/` (the owner's word).** The owner asked
   why a repository that is itself one cartridge keeps Alonzo's
   `cartridges/` and a `kit/` folder for one file; neither had a reason
   beyond Alonzo's shape, and Alonzo's own arcade item, `REPO.5`, puts
   nothing on a game repository's layout. So the source, the cartridge
   the pass writes beside it, Buffet's library and the kit's copy stand in
   `src/`, and the content sheets and the frozen courses will stand in
   `src/content/` and `src/courses/`. The license map names the game's
   files (`src/buffet*.vla`, `src/courses/**`), the content and the kit's
   copy by path, and `check_spdx.ps1` holds every text file under `src/`
   to its header; both controls pass on the new paths. The cartridge is a
   build artifact in a folder called `src`, which is the pass's own
   convention, the cartridge beside its source, and is kept because the
   cartridge is committed, the file a stranger loads.
2. **The source includes the kit and the library, each by its own line,
   and the library includes nothing.** The pass resolves every include
   against the source's own folder, an included file's includes too
   (`alonzo::pass::expand`, as Alonzo's runner documents it), so
   `MISE.1`'s plan, a library that includes the kit at its top, would name
   the kit by a path true at one depth only, and a frozen course stands two
   folders deeper.
3. **The day counts from the frame less one:** `=INT((Clock!B1-1)/B1)`
   and `=MOD(Clock!B1-1,B1)`, where the roadmap wrote the frame itself. A
   load computes no formula and the first step is frame 1, so the
   roadmap's spelling opened the game on the first day's second frame and
   gave the first day 29 frames, its boundary never seen. This way every
   day, the first too, opens on a boundary frame and holds thirty, and the
   first day's dice roll on the first frame.
4. **The frames a day a value, 30, held to the manifest's rate by a
   test,** and never the Clock's rate itself: a day read off the rate
   would jump when the teacher moves the dial (day 100 at frame 3,000
   becomes day 50 at a rate of 60), where a fixed thirty lets the dial
   scale the game and nothing else. The days a year a value too, 365, every
   day a market day, a row for the accuracy sheet (`MENU.5`).
5. **The market's die rolls on a day's first frame: the kit's `hold`, and
   not the kit's `dice`.** The kit's `dice` reads `Clock!B1` and rolls
   every frame, which breaks `BD-7` twice: a reader of the frame outside
   the Day sheet, and a cell that changes every frame, so that every cell
   reading a draw would compute every frame (`SPEC.md` section 2, rule 3).
   It breaks `BD-3` as well: draws rolled by the frame make a day's dice a
   function of the frames a day, and `MISE.4`'s floors fixture, one frame
   a day, would play another market. The die is the kit's `hold` over the
   boundary: `=IF(Day!$B$5=1,MOD(IF(Day!$B$3=0,Clock!$B$3,Dice.last!B1)*$F$2,2147483647),Dice.last!B1)`.
6. **A day jumps 1,017 draws, by one multiplication.** A dish takes the
   next free draw of the day, so a later dish adds a draw and moves none
   an earlier one reads, and a seed's market is the same in every course.
   A jump of `n` draws is one multiplication by 16807^n modulo the prime,
   exact in a double only when that multiplier is below 2^22; 1,017 is the
   first stride past a thousand whose multiplier is, 86,383. A day holds
   up to 1,016 draws, against course 6's two hundred or so.
7. **The exam's die stands half the period away:** the seed jumped
   2^30 + 88 draws, the first such offset with a multiplier below 2^22,
   1,164,196, a function of the seed alone, so the exam rolls once a game.
   The market's draws reach the exam's place after more than 1,055,000
   days, about 2,890 years of a game, so the two never share a die.
8. **The draws by the library's first verb, `draws`,** each a step of
   Park and Miller's minimal standard on from the cell above, and each
   draw's uniform, the draw over the prime, beside it. Two streams write
   the pattern twice in one dish, so it enters `src/kitchen.vla` at once,
   by the ratchet. Four market draws, `POT.2`'s phases the first to read
   them, and two exam draws, `GUARD.1`'s start and stride.
9. **The jumps stated in cells,** each in draws beside its multiplier,
   `Dice!E1:F4`, so that the sheet says what its magic numbers are; a test
   holds them to the hand, which finds each by a search at compile time.
10. **`BD-7`'s oracle restated.** The roadmap held `BD-7` by the step
    row's count, but `(step frame evaluated of done)` counts the plan's
    cells passed, computed or not (`vla-lang` 0.9.3, `Machine::step`), so
    a frame's computed cells are no number any door reads. `BD-7` is held
    instead by a test that no formula outside the Day sheet reads the
    frame or the phase, and by the census's `(every-frame 0)`; with the
    engine's rule that a cell computes only when a cell it reads changed,
    an ordinary frame then computes the calendar's three cells, the day,
    the phase and the boundary. The count itself is a candidate ask, in
    `MISE.6`'s table, not asked until the playtest pass times an ordinary
    frame.
11. **The hand a public module of the oracle crate,** which is never
    published, where Alonzo's references are modules for its tests alone
    because its crate is a package.
12. **The hand test in Alonzo's page,** since Buffet's page is `HOUSE.4`'s:
    the cartridge pasted into the engine panel, the Day and the Dice
    sheets watched in "A sheet, live".

**What it found.**
- The kit's `dice` cannot serve a game whose clock is the day (decision 5).
  Whether the kit wants a die that rolls on an event, a second spelling
  beside `dice`, is a question for Alonzo when a second cartridge writes
  the pattern, by the ratchet, and not before.
- Park and Miller's stream is linear in its seed: the draws of seed `s + 1`
  are those of seed `s` plus the draws of seed 1, modulo the prime, so
  neighboring seeds give related markets, each wave's phase shifted by a
  constant. A host's seed is the browser's random number and never
  neighbors another; `CRITIC.4`'s daily market and `POT.9`'s scenarios
  should spread their seeds, by a hash of the date for instance, and not
  count them.
- The market's die counts the days the game has lived, not the day's
  number: an edit of the frames a day in mid-game renumbers the days at
  once, and the dice go on from where they were, so the stream is a
  function of the day from the first frame on, and not across that edit.
  The teacher's dial is the Clock's rate, which renumbers nothing.
- A small seed's first draws are small, 777 times 16807 being 13,059,039,
  a uniform of 0.006; no draw is the die itself or the seed, and the jump
  scrambles a small seed before the first draw.

**The owner's hand test, 2026-10-14,** in Alonzo's page at `v0.1.0`, the
cartridge pasted into the engine panel: it loads and runs, "a 80 by 50
grid at 30 frames a second, seed 777", 19 formula cells a frame; through
"A sheet, live", the Day sheet at day 64, phase 19, the day of the year 65,
and the Dice sheet's exam stream exactly the hand's, 904,580,292 and
1,244,230,531; `(cell "Clock" "B2" 60)` "changes the game rate to two days
per second". Passed. It found three things of Alonzo's page and none of
the cartridge's, asked of Alonzo as `ENGINE.14` (`MISE.6`): the engine
grid's sheet tabs answer no click, since the host never takes up the
viewport's request for a sheet's record (`KERNEL.5`'s decision 6 promised
it); the wheel is the game's over the whole grid at any rate above zero,
tabs and scrollbars too (`ENGINE.8`'s decision 7 says over the cells); and
a number wider than its column, the Dice sheet's `F3`, 1,073,741,912 in
the default 64 pixels, is drawn cut at the cell's left edge, where Excel
shows `#`s. The Day and Dice sheets carry no column widths, so their
labels and ten-digit draws want a drag to read.

**Numbers.** Seed 777's first frame: the market's die 67,119,591, its
first draw 650,051,262 (0.3027); the exam's die 904,580,292, its first
draw 1,244,230,531 (0.5794). The cartridge: 41 rows, 19 formula cells in
11 shapes, 8 levels, `(every-frame 0)`. The tests: 14, the oracle crate's
whole suite in 2.2 seconds in a debug build, two years and two days of
frames among them; `fmt --check` and `clippy -D warnings` clean. The
mutants, in a scratch copy: the die rolling every frame, the day counted
from the frame itself, the die reading the frame with its values the
same, the verb's multiplier changed, a source edit not expanded, the
frames a day off the rate and the exam rolling the seed itself; each was
caught by the test written for it.

## `MISE.3` — the first measurements: three probes, each deciding a spelling

*Scoped 2026-10-14 at the owner's word, "Scope and implement MISE.3 in
Buffet: the first measurements, three probes in scratch, each deciding a
spelling every later formula repeats", the probes as the owner
recommended them and weighed below. Every prediction here was written
before any number was taken; each number is written beside its own,
marked a measurement or derived. Nothing goes under `src/`: every probe
cartridge, crate and page lives in the session's scratchpad, the crate
pinned as the oracle crate is (`alonzo` =0.1.0 and `vla-lang` =0.9.3
from crates.io) with a target of its own, and Alonzo is read and its
tools run, nothing in it edited.*

**What it stands on, read in the code on the day.** The owner's reading
of what `ENGINE.14` changed holds, each point checked at its line:
- The host views a grid cartridge's Screen through the Camera's whole
  window at every frame, the manifest's 80 by 50 at the Camera's corner,
  and hands the record to its viewport, whose `load` reads the record and
  paints at once (Alonzo `web/host.js`, `_drawGrid`; `web/viewport.js`,
  `load`); another sheet's tab is viewed through the cells in view alone
  (`refreshSheet`). Added by the reading: the record prints a sheet's
  filled cells alone (`vla-lang` 0.9.3, `src/view.rs`), so a window costs
  what it holds and not its 4,000 cells, a formula dearest, a shared
  child's text moved from its master at every view after a walk of the
  whole sheet for the masters, the window's panel and every other.
- A number or a date wider than its column draws as `#`s (`hashesOf`);
  the record's `format` rows carry `general` or `text` and nothing else
  (`src/sheet.rs`, `NumFmt`), and the viewport's reader counts any other
  as malformed. Probe (2)'s premise does not hold, and the probe is turned
  into `FRONT.2`'s choice among the spellings the dialect has.
- The library's fourteen functions (`src/calc/library.rs`, `NAMES`), no
  `TEXT`, `ROUND`, `INDEX` or `INDIRECT`. `&` joins texts, a number joined
  as its fifteen-digit text (`src/calc/value.rs`, `number_to_text`), so
  12.30 joins as `12.3`. A formula longer than 8,192 characters, its `=`
  aside and counted in characters and not bytes, refuses the cartridge at
  load (`src/calc/formula.rs`, `LENGTH_LIMIT`, `calc-formula-too-long`).
- Added: a look matches its value, a number, a text, a truth value or an
  error, in every cell of its sheet (`SPEC.md` §3.1, §7.3), so a look keyed
  on a number colors every cell that happens to hold it.
- Added: the host issues each Write row whose address is not empty as its
  own `derived` write before the next frame's inputs; the replay never logs
  one and refuses a replay that holds one (`_readWrite`, `_issueDerived`,
  `_readReplay`). Alonzo's native runners apply no Write sheet, so the
  scratch runner applies it as the host does, and the host itself is
  driven in the page.

**The probes, as scoped.**

1. **The window's cost.** *The cartridge:* a scratch grid cartridge in
   Buffet's shape at `(rate 30)` and `(screen grid 80 50)`: `POT.1`'s Day
   sheet, so that the market moves on a day's first frame and on no other
   (`BD-7`); a Pot sheet, an index of three sines of the day; a Market
   sheet of fifty vehicles, each its price, yesterday's, the change, the
   change in percent and its sign, every one a function of the day; a
   Plate sheet, holdings and basis as values and the marks as formulas; a
   Camera; and a Screen of course 0's five panels stacked down it, 80 by
   250 (`FRONT.1`), each a tab strip, labels, values and short formulas
   reading the model. *The proportions, and why:* counted from the dishes'
   own text, since no panel is designed yet: the Exam a question, four
   options and a ten-cell progress row (`GUARD.1`); the Overview two
   pockets, the date, the weather word, the ticker and a strategy guide of
   four rows (`FRONT.3`); the Market four rungs, each a name, its price,
   change, percent and glyph, the holding and its value and six buttons
   (`STATION.1`, `PLATE.1`); the Plate a row a rung a pocket, seven
   columns, and a five-order ledger excerpt (`PLATE.1`, `PLATE.2`); Work
   one button and its counts (`BACK.1`). That is 40 to 130 filled cells a
   panel, 20 to 95 of them formulas of 10 to 45 characters, 1 to 3% of the
   window: the *lean* Screen, measured through each panel's window. Later
   courses fill panels (twelve rungs a station, the curve, the lattice,
   the league table), so a *ladder* on the Market panel's window measures
   250, 500, 1,000, 2,000 and 4,000 short formulas in eight shapes reading
   the Market and the Plate, 11 to 45 characters, to find the density at
   which the record crosses the bar; and one *background* variant, the
   lean Market panel with every empty cell holding a space and a look,
   measures a panel's background spelled by cells rather than by column
   formats. *Natively:* a scratch crate's runner, release, opted out of
   Windows' throttling and on the performance cores, three runs of 91
   frames (three market days and a frame), timing at every frame the step,
   the `grid` view of the window, the `values` view of the same window,
   and the host's other views (the Camera and the Clock). *In the
   browser:* Alonzo's player built by its own `tools/build_web.ps1` into
   the scratchpad, the probe pasted into the drawer's box and loaded,
   driven in real time in headless Chrome 155 at 2,560 by 1,440 CSS pixels
   and a ratio of 1.5, the owner's full screen, opted out and pinned, the
   drawer closed; at every frame the tick that runs it timed by a wrapper
   on `host.tick`, and `host.timing`'s view and draw read; and the main
   thread's whole busy time from the DevTools protocol's
   `Performance.getMetrics`; frames 31 to 180 a run. *The rule, written
   before the numbers:* course 0 ships over the record when its densest
   lean panel keeps a frame's tick under 8.3 ms at p95 and under 16.7 ms at
   its longest in Chrome at full screen, the bars `ENGINE.8`'s second
   slice set for the grid's blit case; else `MISE.6` asks Alonzo for that
   slice now, with the number. The ladder's crossing becomes `FRONT.1`'s
   budget for a panel and `MISE.6`'s trigger for the ask.
2. **Gains by glyph.** Answered by reading as far as formats go, and
   turned into `FRONT.2`'s choice. *The cartridge:* a scratch source over
   the kit at widths a Buffet panel would take (a name column of 22
   characters, numbers of 12, a glyph column of 3), a row each spelling for
   a gain, a loss, a flat day and a number too wide for the default 64
   pixels: (a) a glyph cell beside the number, `FRONT.2`'s current
   spelling, its shade from a look keyed on the glyph's own text; (b) a
   text joined with `&`, the glyph and a number rounded to cents by `INT`
   arithmetic, its two decimals kept by hand; (c) a look keyed on a sign
   cell, a fill and no glyph, keyed once on numbers, 1 and -1, and once on
   texts of spaces; (d) number formats in the record, which no cartridge
   can draw today, described as the ask it would be. Each drawn in the
   player at full screen and captured, its record and the viewport's draw
   list read. *The glyph's path:* `▲` and `▼` through the pass (natively
   and the page's own Build), the load, the record, the page's module,
   decoder and viewport, and Buffet's checks and tests over a scratch
   clone whose source carries a glyph. *The rule:* `FRONT.2` takes the
   spelling that shows a glyph and a shade, keeps a number right-aligned
   and a number, costs least, and needs no other repository.
3. **The picker's limit, and the ledger's address.** *The picker:*
   `CHOOSE` over references down column A of a content sheet, its index a
   cell, counted for each of `MENU.1`'s eight sheet names by arithmetic,
   and held by loading the largest picker and the next, one reference past
   the limit; a load and a step timed at that count natively, the index
   moved once a day; the picker in the Screen's window, and read there
   through a reference; the picker on its own sheet and on its content
   sheet, whose references carry no sheet's name; a library macro writing
   it through the pass, timed; and the way out at 2,000 rows, a picker by
   pages. *The ledger:* `PLATE.1`'s ring, a Ledger sheet of a header and a
   hundred rows of six fields (the order, the day, the pocket, the rung,
   the shares, the price), a Write row a field through the kit's `put`, its
   address joined from the order count, `"A"&(MOD(n-1,100)+2)`, an order
   at every frame the left button is held; driven 260 frames with a
   scripted hand through the real host in the page (`runFrameSync`, its
   own module), then its replay played by the same host
   (`play(text, true)`), then natively by the scratch runner applying the
   Write sheet as the host does; the three ledgers compared, and the
   counters of derived writes read.

**The predictions, before any number.** The numbers column was filled as
each was taken, the same day.

| | Probe 1, the window's cost | Predicted | The number |
|---|---|---|---|
| P1.1 | the lean panels natively: the densest one's `grid` view; its `values` view; an ordinary frame's step; a day's first frame's step | 0.1 to 0.4 ms; 0.02 to 0.1 ms; under 0.05 ms; 0.1 to 0.5 ms | *Measurement*, p50s of three runs: 0.20 ms (the Plate; p95 0.21 to 0.30); 0.072 to 0.075 ms; 0.0037 to 0.0041 ms; 0.13 to 0.15 ms. Inside every range. The other panels' `grid` 0.026 (Work) to 0.089 ms (the Market). The host's own calls a frame cost more than an ordinary frame's step: the Input rows written, 0.058 ms; the Camera, File and Clock viewed, 0.025 |
| P1.2 | the full window natively, 4,000 short formulas: `grid`; `values`; the ladder's slope | 7 to 10 ms (the owner's "about 10"); 1.6 to 1.9 ms; linear, about 2 µs a formula cell from the record and 0.4 from `values` | *Measurement*: 8.9 to 9.1 ms; 1.81 to 1.87 ms; linear, 2.2 µs a formula cell from the record (250 cells 0.51 ms, 500 1.09, 1,000 2.24, 2,000 4.56) and 0.46 from `values` (0.12, 0.24, 0.47, 0.93). Inside, `values` a little dearer a cell than predicted. A day's first frame steps 0.14 ms at 250 cells and 0.52 at 4,000 |
| P1.3 | the densest lean panel in Chrome at full screen: a frame's tick at p50 and p95; the Screen's view in it | 1 to 2 ms and under 3 ms; under 0.5 ms | *Measurement*, headless Chrome 155 on the GPU (ANGLE, Direct3D 11), three runs of 150 frames: 1.5 to 1.7 ms and 2.4 to 2.7, at its longest 2.8 to 3.4; the view 0.2 to 0.3. Inside. The frame is the viewport's, its reading and painting 1.0 to 1.1 ms; the other four panels 1.1 to 1.4 ms at p50 and 1.8 to 2.3 at p95; the main thread's whole a frame 2.4 to 2.6. **The rule is met three times over: course 0 ships over the record.** The owner's hand run, in the owner's own Chrome at full screen: 1.4 ms and 2.2, at its longest 2.7; the view 0.2 |
| P1.4 | the full window in Chrome at full screen: a frame's tick | 14 to 16 ms (the owner's "about 14"): the view about 9, the viewport's reading about 1.7, its painting 3 to 4 at a ratio of 1.5 | *Measurement*: 15.5 to 15.7 ms (p95 16.2 to 16.7, at its longest 17.1 to 17.6): the view 8.2, the viewport's load 6.9 to 7.1, with 2,000 of the cells in view. Inside the range; the view lighter and the load heavier than predicted. The main thread's whole a frame, 18.4 to 18.8 ms. The owner's hand run: 14.7 ms and 15.6, at its longest 15.9, the view 7.9 |
| P1.5 | the ladder's crossing of 8.3 ms at p95 in Chrome | about 2,000 formula cells in the window | *Derived*, between the measured rungs: about 1,400 cells. The measured p95s: 250 cells 3.3 to 3.5 ms, 500 4.3 to 4.7, 1,000 6.4 to 6.5, 2,000 10.6 to 11.1. **Missed**: the bar falls lower than predicted, since the viewport's load grows with the cells as the view does, about 4.4 ms a thousand cells in all. The owner's hand run of the 1,000-cell rung: 5.1 ms and 6.0, at its longest 7.3 |
| P1.6 | the background spelled by cells, over the lean Market panel | 1 to 2 ms more natively, 2 to 4 ms more in Chrome | *Measurement*: natively 2.0 to 2.2 ms against the lean Market panel's 0.09, 1.9 to 2.1 more, and `values` saves none of it (1.9 to 2.1 ms: cells are cells); in Chrome 5.8 to 6.0 ms against 1.3 to 1.4, 4.4 to 4.7 more. **Missed** in Chrome. The same background spelled by the columns' format, every column of the window carrying a lavender style (measured beside it): 0.11 ms natively, and 1.7 to 2.0 in Chrome (p95 2.8 to 3.0), the fills' painting alone |

| | Probe 2, gains by glyph | Predicted | What it showed |
|---|---|---|---|
| P2.1 | (a) the glyph cell | `▲` and `▼` legible in the viewport's font, the number right-aligned beside them, the look's shade on the glyph's cell; a cell and a shape more a number | As predicted, drawn at full screen: `▲` and `▼` crisp in a 3-character column, shaded `#D9CCF5` and `#A99BC4` by looks keyed on the glyphs' own texts, `=` on a flat day unshaded. Found: the number beside it is the record's spelling, so 12.30 draws `12.3` and 25.00 draws `25`, here and in every spelling that keeps a number |
| P2.2 | (b) the joined text | `▲12.34` left-aligned; two decimals kept by about 80 to 100 characters of formula; decimal points that do not line up; a text too wide cut or spilled, never `#`s | As predicted but for the length: `▲13.55`, `=25.00`, `▲12.30` left-aligned, no two decimal points in one column; 140 characters of formula (**missed**: 80 to 100), a price and a change read twice each; the plain join beside it losing the zeros, `▲12.3` and `=25`; a joined text too wide spilled into the empty cell beside it, never `#`s |
| P2.3 | (c) the look on a sign cell | a fill and no glyph, so not `BD-6`'s glyph and shade by itself; keyed on numbers, every Screen cell holding 1 takes the gain's fill; texts of spaces collide with nothing | As predicted: keyed on 1 and -1, the sign cell shows its digits, and the two holdings of one share beside it take the gain's shade; keyed on texts of spaces, a fill alone, colliding with nothing on the card |
| P2.4 | (d) number formats in the record | a `SPEC.md` version, a language change (`NumFmt`, the record, the core's writer and reader) and the viewport's painter learning Excel's format grammar: weeks across three repositories, not asked for course 0 | Not drawable, by reading. Yet the only spelling that keeps a number, its right alignment and its zeros together, which (a) and (b) each give half of: written into `MISE.6` as a candidate, not asked |
| P2.5 | the glyph's path | through the pass, the load, the record, the page and Buffet's checks unchanged; one character to the length limit; the host's digest over LF bytes the same as PowerShell's | *Measurement*, as predicted: the native pass and the page's own (`alonzo_expand`, in the module) write the same 3,296 characters, glyphs and all; the load, both records and the draw list carry them (six `▲` and seven `▼` text operations); the host's replay digest, `04AFFF39…FC4A62B5`, is PowerShell's over the same LF bytes; a formula of 8,190 glyphs in quotes, 8,192 characters and 24,572 bytes, loads, and one glyph more is refused. In a scratch clone whose source writes `uniform ▲` on the Dice sheet: Buffet's runner, its fourteen tests and `run_checks.ps1 -WithExtras`, two checks and two controls, all green |

| | Probe 3, the picker and the ledger | Predicted | The number |
|---|---|---|---|
| P3.1 | references a picker holds, its index `B1`, relative references down column A (*derived*, by arithmetic) | Exam 829; Jokes, Memos and Names 753; Errors 690; Endings 637; Headlines 552; Achievements 460; on its own content sheet, references without a sheet's name, 1,548; one reference more refuses the load | *Measurement*, exactly as derived for every name: each count loads, and one reference more refuses the cartridge at load, `calc-formula-too-long` at the picker's line. A picker written on the Screen itself, its index `Pick!B1`, still holds Headlines' 552 |
| P3.2 | a load and a step at the limit, natively | about 1 to 3 ms more a load for each full picker; the step that moves the index under 0.05 ms; the frames that do not, nothing | *Measurement*, two runs: one full picker (Headlines, 552) 2.3 to 2.5 ms more a load (3.1 to 3.3 against 0.8); `MENU.1`'s eight sheets each at its limit with its picker, 2.9 to 3.0 ms a picker (30.4 to 30.9 ms against 6.7 to 7.2, a cartridge of 285 KB); the step that moves the index 0.0030 to 0.0034 ms against 0.0025 to 0.0028 without the picker; an ordinary frame 0.0015 to 0.0016 either way. Inside |
| P3.3 | the picker's text in the Screen's window | 10 to 30 µs more a `grid` view natively, nothing in `values`; read through a reference, nothing | *Measurement*: 0.049 ms against 0.006 through a reference, 43 µs more (**missed**, a little: 10 to 30), the record 8,771 bytes against 591; `values` 0.005 ms either way. Read through a reference, nothing, as predicted |
| P3.4 | the library's macro writing a full picker through the pass | under 50 ms, within the expander's limits (the walk trampolined, the text accumulated under `unquote`) | *Measurement*: a scratch library's `picker`, a walker joining a reference a step under `unquote`, writes Headlines' 552-reference picker byte for byte as the generator writes it; the pass over the whole source, its 552 content rows with it, 4.2 ms |
| P3.5 | the way out at 2,000 rows, a picker by pages | four cells, the right row at every index, a step still under 0.05 ms | *Measurement*: five cells, four pages of 500 and the picker over them (**missed** by the top picker's own cell); every index from 1 to 2,000 picks its own row; a step that moves the index 0.0036 ms |
| P3.6 | the ledger ring | every order lands a frame late, the hundred-and-first on the first row, no refusal; the replay's ledger byte for byte the live run's, and the native runner's the same; no derived row in the replay's text | *Measurement*, as predicted. Through the real host in the page, 260 frames: 246 orders, six derived writes landing on each of 245 frames and none on the other 15, none refused, a hundred rows filled and row 102 empty, the last order's six waiting for a frame the run never steps. The replay, 17,722 bytes, holds no derived row, and played by the same host re-derives the 1,470 writes into a ledger record equal to the live one, character for character. The native runner's 600 cells equal both, and the hand's |

**How the numbers were taken.** Natively, the scratch crate's runner, release
under Buffet's profile, each run its own process, opted out of Windows'
throttling and pinned to the performance cores, a fixed computation timed at
its start and its end and no run's clock moved more than 9%. In the browser,
Alonzo's player built by its own `tools/build_web.ps1` from its tree at
`29ef8ae` with the uncommitted follow-up (a tab strip's CSS, no drawing),
carrying the module built from the registry pin, 749,386 bytes (224 more
than the module in Alonzo's tree: the panic locations name each crate's
source path, the engine's from the registry here and from the tree there),
the host reached by one line added to a copy, `window.__host = host;`;
headless Chrome 155 at 2,560 by 1,440 CSS pixels and a ratio of 1.5, on the
GPU, every process of its own profile opted out and pinned, a Chrome a
cartridge, the frames at 30 a second on 60 Hz ticks; in-page times are
Chrome's, in steps of 0.1 ms. A scratch number is a scoping's, never a
floor. The browser numbers are headless; the owner's hand run, below, made
P1.3 real.

**The owner's hand run, 2026-10-14.** Three pages, each the player with the
host reached and a probe loading itself, read in the owner's own Chrome at
full screen, 2,560 by 1,440 at 1.5, the drawer closed, each printing a
sample every five seconds in a corner; three screenshots. The densest lean
panel: a frame's tick 1.4 ms at p50, 2.2 at p95, 2.7 at its longest, the
view 0.2, the viewport's load 0.9, 36 columns and 68 rows in view. The
1,000-cell rung: 5.1, 6.0 and 7.3, the view 2.0, the load 2.9. The 4,000:
14.7, 15.6 and 15.9, the view 7.9, the load 6.5. Each about a tenth below
the headless run's. In the owner's words: "All three hand tests run
(screenshots attached); recommendations approved." The screenshots showed
what no run of mine had looked at, the panels' money, written below.

**What it found.**
- Course 0's frame is the viewport's and not the record's. The densest lean
  panel's tick is 1.5 to 1.7 ms, of which the Screen's view is 0.2 to 0.3
  and the viewport's reading and painting 1.0 to 1.1; `ENGINE.8`'s second
  slice would take about 0.15 ms of it (*derived*: the panel's `values` view
  natively 0.07 ms against its record's 0.20).
- `BD-7` is seen working: a lean ordinary frame steps in 4 µs and a day's
  first in 0.13 to 0.15 ms, and the host's own calls a frame, the Input rows
  written and the device sheets viewed, cost twenty times an ordinary step.
- A number shows its shortest spelling: money in whole cents draws `12.3`
  for $12.30 and `25` for $25.00 in a number cell, and only a joined text
  keeps two decimals: a row for `MENU.5`'s accuracy sheet until the day
  number formats are asked for.
- Money computed in dollars draws its binary noise, found in the owner's
  screenshots: the Plate panel's unrealized gain, a mark less shares times
  basis, drew `-8.280000000000001` and `6.380000000000003`, a change of two
  rounded prices `0.03000000000000025`, and a mark of 6 shares at 30.19,
  $181.14, drew `########` in its 11-character column, its spelling being
  181.14000000000001. The record spells a number's shortest round trip and
  the viewport never respells it (`SPEC.md` §3.1); every price that was one
  rounding, `INT(x*100+0.5)/100`, drew clean, as whole cents divided by 100
  always do below $70 trillion (*derived*: below 2^46 dollars two doubles
  stand closer than a cent).
- A background spelled by cells costs a panel 4.4 to 4.7 ms a frame in
  Chrome, and the same lavender spelled by the columns' format 0.3 to 0.7.
- At the owner's full screen the player shows 38 to 40 columns and 68 rows
  of the default cells, so a 50-row panel leaves 18 rows blank beneath its
  window; rows of 20 points, 27 pixels, would fill the height (`FRONT.1`'s).
- Alonzo's player, found while measuring: a grid cartridge whose Camera
  starts below row 1, loaded while the grid's box is hidden (its first grid
  load after a plane, the page opening on Life), opens scrolled to rows 1
  to 68, blank rows above its window. The load scrolls to the Camera's
  corner before the page shows the box, and frame 1 sees no Camera move to
  follow; the Screen's tab, or a second load, gives the window back. Not
  Buffet's at first light, whose Exam panel stands at row 1 in a page with
  no plane before it; Alonzo's to mend, written into `MISE.6`.

**The spellings it settles**, all six approved by the owner 2026-10-14,
each written as a dated line on its item in `OMAKASE.md`:
1. **`MISE.6`, `ENGINE.8`'s second slice: not asked for course 0.** The ask's
   trigger: a panel past 1,000 formula cells in its window, or the playtest
   pass's frame over 8.3 ms at p95 (`MISE.5`).
2. **`FRONT.1`: a panel holds at most 1,000 formula cells in its window**
   (p95 6.4 to 6.5 ms, where the bar falls near 1,400); its background is
   its columns' format and never a filled cell; rows of 20 points so that a
   panel fills a full screen's height, the owner's to judge by eye.
3. **`FRONT.2`: the glyph cell beside the number, shaded by a look keyed on
   the glyph's own text**, `▲`, `▼` and `=`, never on a number, which would
   shade every holding of one share; a fill with no glyph keyed on a text of
   spaces where a shade alone is wanted; the joined text only where two
   decimals matter more than a column's alignment (a ticker's sentence, the
   certificate, the report card's remarks). Number formats in the record,
   the spelling that would keep both, written into `MISE.6` as a candidate,
   not asked.
4. **`MENU.1` and `MISE.1`'s library: the picker on its content sheet**, its
   references bare, 1,548 rows a picker; past that a picker by pages of 500;
   its cell off the Screen and read there through a reference; the library's
   `picker` the scratch walker's shape, a reference joined a step under
   `unquote`. On a sheet of its own it would hold Headlines' 552 and
   Achievements' 460.
5. **`PLATE.1`: the ledger's row computed once, `=MOD(n-1,100)+2`, and each
   field's address joined from it with `&`**, a Write row a field through the
   kit's `put`. It lands a frame late, every frame an order stands, and the
   replay re-derives it, so the order's fields are its own frame's values.
6. **`FRONT.2`, found at the hand run and approved the same day as
   sufficient for Buffet: money is whole cents as integers in every cell of
   the model, and reaches a panel by one division by 100**, never as a
   difference or a product of dollar amounts, so that no money cell draws
   binary noise or `#`s for a number that fits. The owner's word with it:
   decimal precision for real-world spreadsheets, Excel's or better and of
   arbitrary length, is an important item for Frazaro once it expands on
   Buffet's lessons, Julia's handling of number precision a reference;
   written into `MISE.6` as the owner's candidate for Frazaro.

## `POT.2` — tier 1: four waves and a drift, prices a function of the day

*Scoped 2026-10-14 at the owner's word, "Scope and build Buffet's POT.2,
tier 1 of the market: four waves and a drift, prices a function of the
day", from `OMAKASE.md`'s line, `BD-3`, `BD-7`, `BD-10` and `FRONT.2`'s
money rule. Each decision carries its recommendation, for the owner to
approve, amend or strike before the build. The predictions were written
before any number was taken; each number goes beside its own at the build,
marked a measurement or derived. Approved whole by the owner the same day,
"Recommendations approved; fix the Cargo.toml comment to 0.1.1 and proceed
with implementing POT.2 as scoped", and built that day.*

**What it stands on, read in the code on the day** (`vla-lang` 0.9.3, the
language `alonzo` 0.1.1 pins).
- `SIN` is Rust's `f64::sin` and `^` is `f64::powf` (`src/calc/library.rs`,
  `sin_of`; `src/calc/eval.rs`, `power`, which answers `0^0` with `#NUM!`
  and 0 to a negative power with `#DIV/0!` before it calls `powf`). A
  number in a formula is read by Rust's own `parse::<f64>`, correctly
  rounded, so `1.0002` in a cell and in Rust are one double. The operators
  associate to the left, a sign binds tighter than `^`, `SUM` adds in the
  walk's order from -0.0, and every result passes `finite`, which turns -0
  into 0 and a number no cell can hold into `#NUM!`.
- One `arithmetic` serves a cell computed alone and a run computed by lanes
  (`KERNEL.26`), and the lanes call the same `sin_of`, so a formula's value
  is one double whichever path computes it.
- std's `sin` and `powf` are the platform's: the Windows runtime's here,
  glibc's in CI, the module's own in the browser. Measured 2026-10-14 on
  146,000 inputs of Buffet's shape (four waves over a hundred years of days,
  four drifts raised to the day): natively on Windows against the module,
  `sin` differs on 1.6% of them, by at most one unit in the last place, and
  `powf` on 9.7%; a price rounded once to whole cents differed on none,
  unrounded on 8.8%. CI's Linux was not measured. A fix in the language
  (`libm` on every target) is deferred and unminted, so this item is built
  over std.
- `describe`'s `(every-frame n)` counts the shapes the engine cannot follow a
  change to (a defined name, a span of sheets, a broken reference:
  `src/calc/census.rs`), not the shapes that read the frame, so it stays 0
  for a market that reads the day by plain references.
- `CHOOSE` reads its index down to a whole number and answers `#VALUE!`
  outside 1 to its count. The kit's `hold` writes a range at once, each cell
  over its own twin, `=IF(when,new,Sheet.last!first)` filled relative.
- Alonzo's player at `v0.1.1` (`ENGINE.14`) answers the grid's sheet tabs,
  so a hand test can watch a new sheet by its tab.

**The decisions.**

1. **The sheet: `Market`, plain.** Course 0's open decision, the kitchen's
   names or plain (`OMAKASE.md`), recommends the kitchen's on the panels and
   plain in the model; the model's sheets so far are Day and Dice, and
   `BUFFET.md` §5 calls the market's sheet `Market`. Approving this settles
   the model's half for every sheet after it, the panels' names staying
   `FRONT.1`'s, and `POT.6`'s "one Pot sheet" becomes the Market sheet. Its
   tab stands after Dice.
2. **The layout: the settings, the waves, then two columns, today and the
   look back.** A label in column A beside each thing, as Day and Dice have
   it. Every formula of column C is column B's own fill, so the look back is
   computed by the very formula that computes today:

   | Row | A, the label | B | C | D | E |
   |---|---|---|---|---|---|
   | 1 to 4 | pi; the drift, a day; the difficulty; the look back, in days | `3.141592653589793`; `1.0002`; `1`; `20` | | | |
   | 6 | the waves | period, in days | amplitude | speed, radians a day | phase, radians |
   | 7 | a year, the Kiddie Pool's one wave | 365 | 0.1 | `=2*$B$1/B7`, one fill to `D11` | 0 |
   | 8 to 11 | Kitchin, inventories; Juglar, investment; Kuznets, building; Kondratiev, the long wave | 1459; 3271; 6947; 18251 | 0.1; 0.15; 0.1; 0.15 | | `=IF(Day!$B$3=0,2*$B$1*Dice!$C2,Market.last!E8)`, one fill |
   | 13 | | today | the look back | | |
   | 14 | the day | `=Day!$B$3` | `=B14-$B$4` | | |
   | 15 to 19 | each wave's term, the year's to Kondratiev's | `=$C7*SIN($D7*B$14+$E7)`, one fill over `B15:C19` | | | |
   | 20 | the trend, the drift to the day | `=$B$2^B14` | the same fill | | |
   | 21 | the cycle: one and the four waves | `=1+B16+B17+B18+B19` | the same fill | | |
   | 22 | the cycle on the first day | `=1+C8*SIN(E8)+C9*SIN(E9)+C10*SIN(E10)+C11*SIN(E11)` | | | |
   | 23 | tier 0, the Kiddie Pool | `=B20*(1+B15)` | the same fill | | |
   | 24 | tier 1, the Day Trader | `=B20*B21/$B$22` | the same fill | | |
   | 25 | the index, as the difficulty picks | `=CHOOSE($B$3+1,B23,B24)` | the same fill | | |

   The index today is `Market!B25` and twenty days back `Market!C25`, the
   two addresses every later dish reads. Pi is a literal, since `PI()`
   parses but is never evaluated, and stands once in `B1` where the speeds
   and the phases read it.
3. **The periods: each a prime number of days, nearest 4, 9, 19 and 50 of
   the game's 365-day years: 1,459, 3,271, 6,947 and 18,251.** A prime is a
   whole multiple of no other period and of no year, so no ratio between two
   waves is a whole number and no wave keeps the calendar's step (a Kitchin
   of 1,460 days would peak on the same day of the year every fourth year).
   The Kuznets is 19 years and not 18: at 18 it is two Juglars to within
   0.8% (6,569 over 3,271 is 2.008, and the probe's Julian periods of the
   same day, 1,461, 3,287, 6,575 and 18,263, had 2.0003), and the two would
   move as one wave for a century, a whole ratio in all but name; at 19 it
   is 2.124, the pair's relative phase turning an eighth of a turn a
   Kuznets. Nineteen years is inside Kuznets's own fifteen to twenty-five.
   The Juglar is the lower of the two primes fourteen days from nine years,
   3,271 and 3,299, the one further from two Juglars a Kuznets. *Derived*,
   by a scratch search: with these four, the Kitchin and the Juglar come
   back within 5% of a whole cycle each after 36 years, the 4:9 of the
   years themselves, whatever the primes; all four do only after 152 years.
   The 18-year Kuznets is the alternative, if the owner prefers the
   classical number and the lock.
4. **The amplitudes: 0.1, 0.15, 0.1 and 0.15 of the trend,** the Juglar's,
   the business cycle's, the largest swing a session lives through, the
   Kitchin's the ripple a year shows, the Kondratiev's the generation's.
   Their sum, 0.5, keeps the cycle between 0.5 and 1.5, never at or below
   zero. At their steepest the waves move the cycle 0.043%, 0.029%, 0.009%
   and 0.005% a day (*derived*, amplitude times speed), together four times
   the drift's 0.02%, so the weather turns and the trend still shows.
5. **The drift: 1.0002 a day,** about 7.6% a year (1.0002^365 is 1.0757), a
   stock market's long-run price return, written as the daily factor the
   formula raises to the day. A hundred years multiplies it about 1,480
   times, a $100 rung then near $148,000, which whole cents hold exactly
   below $90 trillion. The gentler 1.0001, 3.7% a year, is the alternative.
6. **The phases: the market's draws 1 to 4 of the first day, each uniform
   times 2π, held.** The kit's `hold` over `E8:E11`, taking each phase on day
   0 and keeping its twin after, so the phases read the day and never the
   frame (`BD-7`), are a function of the seed alone, and are the first dish
   to read the market's draws, as `POT.1` left them. Every course plays the
   same market on the same seed, and at one frame a day the first day is
   the first frame and nothing else changes.
7. **The index opens at 1.** Tier 1 is the trend times the cycle over the
   cycle on the first day. `B22` computes that cycle from the phases with the
   operations of `B21` at day 0 in the same order (a speed times 0 is 0, and
   0 plus a phase is the phase), so the quotient on day 0 is exactly 1, and a
   rung opens at its contract's price exactly, $1, $5, $25 and $100. The
   roadmap's index, unnormalized, opens at the cycle's own level, anywhere
   from 0.5 to 1.5 by seed. Computed from the parameters and not held, so
   an edit of an amplitude or a period normalizes the whole path again: the
   open book's first lab (`AD-6`).
8. **Tier 0 beside tier 1, and the difficulty a value, 1, read by `CHOOSE`
   now.** Tier 0, the Kiddie Pool: one wave of a year, 365 days, amplitude
   0.1, phase 0, fixed and the same at every seed, over the same drift. It
   rises through the year's first quarter, peaks on day 99 and bottoms on
   day 268 (*measured* at the build, where the scoping wrote the sine's own
   days, 91 and 274: the drift's slope delays the peak and hastens the
   trough), a whole cycle in a session's first six minutes ("markets have
   cycles", `BUFFET.md` §5), and needs no normalizing, its cycle on day 0
   being 1. The roadmap has `CHOOSE` arrive with `POT.6`; it is written
   now because the index is the address every price, the weather and the
   benchmark read, so it stands at its last address from the first day,
   and `POT.6` adds the picker and the names, not a cell every price reads.
   A person's edit of `B3` to 0 already plays the Kiddie Pool; 2 or more is
   `#VALUE!` until `POT.6` refuses it in words.
9. **The look back: twenty days in a cell, recomputed and never
   remembered.** `C14` is the day twenty days back, and column C is column
   B's fill, so the index twenty days back is the index's own formula at
   that day. Before day 20 the look back reads the model's own past, a day
   below zero, which the waves and the drift define. The slope,
   `=Market!B25-Market!C25`, and its word stay `FRONT.3`'s; a history ring
   waits for `POT.5`, the first market whose next day is not a function of
   the day.
10. **The class row: none at course 0.** Stocks are the market at course 0:
    their multiplier is the index itself, a beta of 1 and no wave of their
    own, so the Investment Clock asks nothing of them. A class row of one
    class is the index under another name, and an instrument with no
    specimen waits for the course that has one (`METAMETALISP4.md` §8). The
    row (a class's beta on the index, its own wave at the Kitchin's speed,
    its phase the Kitchin's plus its quarter of the clock: reflation bonds,
    recovery stocks, overheating commodities, stagflation cash) enters with
    the first class priced off the market that is not the market itself.
    The four rungs' betas are `STATION.1`'s, on the index. *The
    alternative:* the row now, stocks at a beta of 1 and an amplitude of 0,
    so the rungs read a class cell from the first day; six cells and a
    formula.
11. **The price: whole cents, one rounding, half up; the rows `STATION.1`'s.**
    A vehicle's price is `=INT(<its start in cents>*<its multiplier>+0.5)`,
    an integer from which every mark and gain is integer arithmetic
    (`FRONT.2`'s rule). At tier 1 the multiplier is the index under the
    rung's beta, and how a beta applies is `STATION.1`'s choice to make
    knowing this: a beta on the index's level, `1+b*(index-1)`, goes negative
    once the index falls below `1-1/b`, which a power, `index^b`, never does.
    The idiosyncratic factor, `(1+idio)`, is written by `POT.5` when it
    exists, never as `*(1+0)` now. `POT.2` writes no price: the four rungs
    are `STATION.1`'s dish, and first light takes both (`OMAKASE.md`'s
    critical path).
12. **No wave verb yet.** The pattern, a wave's term at a day, is written
    once, one fill of ten cells over five waves and both columns. The
    ratchet brings a `wave` into `kitchen.vla` at its second writing:
    `POT.3`'s rate cycle (course 1) or `STATION.4`'s harvests, whichever is
    built first.
13. **The hand calls the same std functions as the cells, in each formula's
    order.** `oracle/src/buffet.rs` gains the market: the settings as
    constants whose literals are the cells' (pi as `std::f64::consts::PI`,
    whose shortest spelling is `B1`'s); each speed `2.0 * PI / period`; each
    phase from the hand's own dice, the draw found by its place in the
    stream (`market_draw`), and never the cartridge's chain; each term
    `amplitude * (speed * day + phase).sin()`; the trend `DRIFT.powf(day)`;
    the cycle `1.0 + t1 + t2 + t3 + t4`; the index as the cells divide it.
    The calendar and the dice share no predicate with the cartridge; the
    waves cannot, since a double's last bit is the order of its operations,
    so the market's hand is the formulas written a second time in Rust, its
    independence the dice beneath it and the day it is called on. The tests
    compare bits after adding 0.0 to the hand's value, which turns its rare
    -0 into the cell's 0 (`finite`) and moves nothing else.
14. **The oracle, as the measurement leaves it** (`OMAKASE.md`'s line,
    rewritten at the owner's word). The hand holds the index, and from
    `STATION.1` every price, exactly natively, here on Windows and in CI on
    Linux, both writings calling one platform's functions. The tests compare
    the hand with the cartridge live and commit no unrounded value derived
    from `SIN` or `^` (the describe golden holds formulas, never values), so
    whatever a replay golden holds later is rounded once to what it shows,
    as money is. The two-peak test: the path does not repeat within a
    hundred years. `BD-7`: no formula outside the Day sheet reads the frame
    or the phase, and the census keeps `(every-frame 0)`, so the market
    recomputes on a boundary and nowhere else.
15. **The tests.** Every frame of two years and two days, now with every
    formula cell of the Market sheet held to the hand bit for bit; the
    Market's settings held to the hand's constants; the difficulty written 0
    before the first frame, playing the Kiddie Pool for a year; a hundred
    years at one frame a day, every day's index, today and twenty days back,
    held to the hand; and the two-peak test over the hand: for every lag
    from two years to a hundred, the cycle that lag later differs from the
    cycle somewhere in the first Kitchin by at least 0.02, where a path that
    repeated would differ by nothing. The scans that stand (the frame read by
    the Day sheet alone, the census) cover the new sheet unchanged.
16. **The hand test in Alonzo's page, the player of `v0.1.1`**, since
    Buffet's own page is `HOUSE.4`'s: the cartridge pasted into the drawer's
    box and loaded, the Market tab watched as the days turn, the index read
    at a paused day against the hand's, the difficulty written 0 and back,
    the Clock's rate written 60.

**The predictions, before any number.** The numbers column was filled at
the build, the same day.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 96 rows (41), 51 formula cells (19) in 22 shapes (11) of 20 texts (9), 13 levels (8), `(every-frame 0)`; the describe golden moves by the Market's readers, the eleven new shapes and the budget, `(budget 30 51 1530 2000000)` | *Measurement*, as predicted in every count; the golden's diff the notes, the budget, the plan, the readers (Day 9 to 11, Dice 6 to 7, the Market's 31, its twin's 1) and the eleven shapes, `POT.1`'s eleven unmoved. Not predicted: 2 runs of 9 lanes (the five speeds, and four of the ten terms), 42 cells computed alone |
| P2 | the hand at every frame of two years and two days | every formula cell of the Market sheet equal bit for bit at all 21,960 frames, natively here and in CI on Linux | *Measurement*: equal at all 21,960 frames here, on Windows, at the first run; and CI's Linux held the market bit for bit at the first push, run 38689162382, its 22 tests green, the two years and the hundred years among them |
| P3 | a hundred years at one frame a day | the index today and twenty days back equal at all 36,500 days; the test under 10 s in a debug build | *Measurement*: equal at all 36,500 days; the test 3.0 s alone in a debug build. Inside. The two years' test grew from 2.2 s to 5.1 s, the Market viewed at every frame |
| P4 | the two-peak test | the smallest difference over the lags between 0.04 and 0.12 of the trend, at a lag within days of a whole number of Kitchins, near 36 or 100 years | *Measurement*, seed 777: 0.089 at a lag of 36,296 days, 99.4 years. Inside for the difference; **missed** for the lag, 24.88 Kitchins, not within days of a whole number of them: the nearest repeat is where the four waves' misses sum least, not where the Kitchin's alone is nothing. Five other seeds, 1, 2, 42, 1,000 and 123,456,789: 0.039 to 0.104, each above the test's 0.02 |
| P5 | seed 777's first year | the index ends it between 0.85 and 1.05: the Kitchin opens past its peak, its phase 1.90 from the first draw's 0.3027 (`POT.1`'s published number), and falls 0.127 of the trend in the year (*derived*), against the drift's 7.6%; the look back's slope below zero on 50 to 90% of its days | *Measurement*: 0.904 on day 365, inside; the slope below zero on every one of the year's 365 days, **missed**. Seed 777 opens near a cyclical high, its first cycle 1.297, the Kitchin and the Juglar both past their peaks (phases 1.90 and 3.40), so the index falls without a turn to 0.866 on day 586, and the look back's slope first turns on day 597 |
| P6 | seed 777's hundred years | the index ends between 900 and 2,200 (the drift's 1,480 times the cycle's ratio to its first day); its largest fall from a peak 20 to 45%; days up 55 to 70% | *Measurement*: 1,295 on day 36,500; the largest fall 30.0%, from day 2,784 to day 3,626 (years 7.6 to 9.9); days up 69.4%. Inside, each |
| P7 | the step natively, a scratch runner in release | an ordinary frame no slower than `POT.1`'s cartridge's, the market computing nothing between boundaries; a day's first frame under 0.05 ms slower | *Measurement*, a scratch runner at opt-level 3, three rounds of a year of frames each cartridge, alternated, each run under a tenth of a second: a day's first frame 7.8 to 7.9 µs at the median against `POT.1`'s 3.4, 4.5 µs more, inside; an ordinary frame 1.5 µs against 1.4, **missed** by 0.1 µs, most likely the plan's walk, since the step passes every cell of the plan, computed or not (`POT.1`'s decision 10), 51 where it passed 19 |
| P8 | the page against the hand, at the owner's paused day | the index equal to the hand's in its first 15 significant digits, its last digit differing on about one day in ten (`powf`'s 9.7%, `sin`'s 1.6%) | *Measurement*, the owner's three screenshots, read from crops at full resolution: equal to the hand to the last bit in all 70 cells they show, days 60, 84 and 100 with their look backs (30 sines, 6 powers and every cell built on them), the phases, the speeds and the cycle on the first day. Inside: the one difference the measured rates made likely in these 36 results did not come, a chance near one in three |

**What the build found.**
- Seed 777's first game opens on a fall. The index declines from 1 to 0.904
  at the first year's report card (day 365) and to 0.866 on day 586, and the
  look back's slope turns only on day 597, ten minutes into a game at a day a
  second; it then rises to 1.339 by day 1,672. So every first game's first
  year is a bear market of 9.6%, in which buy-and-hold loses and cash wins,
  and a weather word read off the slope's sign alone would not change in it;
  a word read off the slope's size changes as the fall steepens. The seed is
  the contract's, 777, and its market is what its dice make it: the owner's
  to keep, or to change by the contract's one number, which every course
  would then play (`BD-3`).
- Two peaks do not give the third, mostly. Over seed 777's hundred years the
  index peaks 19 times, the gaps between peaks running from 1,112 to 3,055
  days; a player who takes the last gap for the next comes within six weeks
  of the peak twice in seventeen (18 and 35 days off) and misses by more than
  a year ten times.
- The first market costs a day's first frame 4.5 µs natively, and an
  ordinary frame 0.1 µs, the plan's walk.

**The owner's hand test, 2026-10-14,** in Alonzo's player as built in
Alonzo's tree that day, its picker opening on an empty sheet (`ENGINE.15`'s,
unreleased), over the module of `alonzo` 0.1.1 and `vla-lang` 0.9.3, as its
log says; the cartridge pasted into the drawer's box and loaded. The log's
budget line read 51 formula cells a frame at 30 frames a second, 1,530 a
second; the Market tab showed the settings, the waves and both columns, the
index moving once a second. Three screenshots, at days 60, 84 and 100: the
speeds and the phases, which no `SIN` or `^` reaches, equal to the hand's in
every digit, and every other cell they show too (P8);
`(cell "Market" "B3" 0)`, written at frame 2,389, made the index the Kiddie
Pool's from the next frame (on day 84, `B25` equal to `B23` and `C25` to
`C23`), and `(cell "Market" "B3" 1)` at frame 2,891 gave tier 1 back (on
day 100, `B25` equal to `B24`); and `(cell "Clock" "B2" 60)`, in the owner's
words, "successfully doubles the pace of play". Passed.

**What the hand test raised.** The owner, on seed 777's falling first year:
"should the first year be a slight bull market to encourage new players with
a hidden handicap? ... we do want to encourage new players to continue
playing." The facts for the decision, measured by the hand the same day over
the 3,700 seeds below a million that hold 777: 977 end their first year 5 to
15% up; 580 of those climb without a turn of the slope, and 177 dip early
and then climb; none climbs, dips and climbs again within the year, since
tier 1's fastest wave is four years long and its slope turns at most once
in a year. Among the early dips, seed 777395 falls 1.4% to day 90, its
weather turns on day 101 and its report card reads +10.1%, and a steady
climb, seed 252777, reads +10.0% with no dip. A kind first year chosen this
way is the contract's own seed chosen in the open, never a hidden term in the
model (`BD-3`, `BD-5`), and leaves the score untouched, since the score is
alpha over buy-and-hold on the same seed (rec). The decision is the opening
contract's, the owner's (`COURSE.0`).

*Settled the same day.* The owner: "Why does the seed need to contain 777?"
It need not: the scoping had kept the contract's 777 for the Series 777
pun, a restriction it should have named. Over every seed below a million,
613 dip early and then climb (a dip of 1 to 3% before day 120, the weather
turning once by day 130, the report card 7 to 13% up), 2 of them holding
777; every one of the 613 then climbs for five years with no fall past 4%,
since an early dip means the short waves bottom together. A gentle first
year followed by a fall of 10% or more from a peak in years 2 or 3 is
common, 81,702 seeds; an early dip and then such a storm is rarer, 184 with
the report card 4 to 16% up. The owner, on the dip: "a subtle teaching
moment about dollar cost averaging, at the risk of deterring new players
with 'losing' money in the first few minutes of play. C'est la vie,
friends", and chose seed 791886 from four profiled: the index dips 1.1% to
day 91, the weather turns on day 102, the report card reads +10.2%, then a
climb to 1.696 by day 1,094 and a fall of 10.4% to day 1,670, the weather
turning on days 102, 1,105 and 1,681 in the first five years. The others
profiled: 507887, a dip to day 81 and five years without a fall of 5%;
10, no dip and a fall of 31.8% from day 627 to day 1,407; 777 itself. The
manifest's seed moved alone, the hand naming it (`SEED`), a test holding
the manifest to it, another holding the first year kind; the two-peak test
reads the first game's market, its least difference 0.089 at 52.2 years.

## `STATION.1`, first slice — stocks: four rungs

*Scoped 2026-10-14 at the owner's word, "Scope and build Buffet's
STATION.1, first slice: stocks, four rungs", from `OMAKASE.md`'s line
(`COURSE.0`'s item 4), `BD-1`, `BD-3`, `BD-7`, `BD-10`, `FRONT.2`'s money
rule and `POT.2`'s decision 11. Each decision carries its recommendation,
for the owner to approve, amend or strike before the build. The predictions
were written before any number was taken; each number goes beside its own
at the build, marked a measurement or derived. Approved whole by the owner
the same day, "Recommendations approved; proceed with implementing STATION.1
as scoped", and built that day.*

**What it stands on, read in the code on the day** (`vla-lang` 0.9.3, the
language `alonzo` 0.1.1 pins).
- `^` is `power`, which answers `0^0` with `#NUM!` and 0 to a negative power
  with `#DIV/0!` and else calls `powf`; `INT` is `floor`, and `MOD(n,d)` is
  `n-d*INT(n/d)`; every result passes `finite` (`src/calc/eval.rs`,
  `src/calc/library.rs`). So `=INT(<start>*<index>^<beta>+0.5)` is `powf`, a
  product, a sum and a floor, in that order, and the hand writes them in it.
- The index is above zero at every seed, on every day a price reads and at
  both tiers (*derived*). Tier 1 is the trend times the cycle over the cycle
  on the first day, each cycle between 0.5 and 1.5 since the amplitudes sum
  to a half, so the quotient is at least a third; the trend is at least
  1.0002^-1, on day -1, yesterday's on day 0 and the earliest day a price
  reads; tier 0 is at least nine tenths of the trend. So the index to any
  beta is a positive number, and never `0^b`'s error.
- The index is exactly 1 on day 0 (`POT.2`'s decision 7), and `pow(1, y)` is
  1 for every `y` by Annex F of the C standard, which the platforms'
  libraries keep (check: the opening test holds it here and on Linux), so a
  rung opens at its start exactly.
- The record prints a number as its shortest round trip, so a whole number
  of cents prints as its digits, `10000`, and the tests compare it as text,
  as the dice's draws are compared.
- The kit's `cells` writes its entries in the order given, `headers` along a
  row and `labels` down a column (`src/common.vla`). The pass carries no
  comment of the source into the cartridge, its header line and its banner
  aside.
- The Market sheet's column D is empty below row 11, and every fill of its
  workings reaches columns B and C (`POT.2`'s decision 2).
- Alonzo's player at `v0.1.2` (`ENGINE.15`), deployed by its pages run
  38687332704 to https://spreadsheet-company.github.io/Alonzo/, loads a
  cartridge pasted into the drawer's box, pauses, and takes a person's edit,
  a row at a time, in the drawer.

**The decisions.**

1. **The sheet: `Stocks`, plain, its tab after Market.** The class's own
   word (`BUFFET.md` §6), as Day, Dice and Market are plain (`POT.2`'s
   decision 1 settled the model's half); a sheet a station, so `STATION.2`'s
   Bonds and `STATION.3`'s Cash will stand beside it; the panel's name is
   `FRONT.1`'s.
2. **The labels: `rung 1` to `rung 4`, and no name in a cell.** The working
   labels, a penny stock, a small cap, a mid cap and a blue chip, stay in the
   docs (`BUFFET.md` §6, `OMAKASE.md`'s line, this file) until `MENU.3`'s
   first slice, with `BACK.2`, names the four, fictional, as rows of the
   Names sheet. The source's comments say what each rung is without them. A
   rung's number is its place on the ladder, cheapest first, the word every
   station's ladder shares.
3. **The layout: two settings, then a table, a rung a row.** A label beside
   each setting, as on the Day, Dice and Market sheets; a rung's settings
   left of its workings; every formula of a column one fill over the four
   rows:

   | Row | A | B | C | D | E | F | G | H |
   |---|---|---|---|---|---|---|---|---|
   | 1 | dividends a year | `4` | | | | | | |
   | 2 | the days between two | `=INT(Day!$B$2/B1)`, 91 | | | | | | |
   | 4 | the rungs | start, in cents | beta | dividend yield, a year | price today, in cents | price yesterday, in cents | a dividend, in cents a share | the dividend today, in cents a share |
   | 5 to 8 | rung 1 to rung 4 | `100`; `500`; `2500`; `10000` | `2`; `1.5`; `1`; `0.75` | `0`; `0`; `0`; `0.02` | `=INT(B5*Market!$B$25^C5+0.5)` | `=INT(B5*Market!$D$25^C5+0.5)` | `=INT(B5*D5/$B$1+0.5)` | `=IF(MOD(Day!$B$7,$B$2)=0,G5,0)` |

   The prices today are `Stocks!E5:E8`, the cells the Market panel and
   `PLATE.1` read; every cell that holds money holds whole cents (`FRONT.2`).
4. **The starts: 100, 500, 2,500 and 10,000 cents,** the contract's $1, $5,
   $25 and $100 a share, in cents, so that the price's one product is cents
   and its one rounding makes them whole.
5. **How a beta applies: a power of the index, `index^b` (rec).** A rung's
   price is its start times the index to its beta, so its beta holds for all
   time in the logarithm: a day's move of the index by 1% moves the rung by
   about b%, at any level, and a fall of the index by a fraction f between
   two days is a fall of exactly `1-(1-f)^b` for the rung between the same
   days. A power of a positive index is positive, and it is one `^`, written
   by the cell and the hand in the same order. The level form,
   `1+b*(index-1)`, goes negative once the index falls below `1-1/b`, a half
   at a beta of 2, which the index's bound, a third, allows; and its beta
   fades toward 1 as the index grows: at an index of 10, a beta of 2 moves
   the price 1.05% for the index's 1%. What the power also does, said
   plainly: it raises the drift to the beta, so a rung's trend is the drift
   to b times the day, 15.7%, 11.6%, 7.6% and 5.6% a year at decision 6's
   betas (*derived*), a hundred years multiplying the first rung 2.2 million
   times and the fourth 239. A higher risk earns a higher long-run return,
   the security market line, and at tier 1 no rung has a risk of its own,
   `POT.5`'s idiosyncratic factor: a row for `MENU.5`, at tier 1 no stock
   goes bust.
6. **The betas: 2, 1.5, 1 and 0.75 (rec), the cheapest the wildest.**
   `BUFFET.md` §6's barbell, price and risk separate axes: the rung is the
   price scale and the beta the risk. The third rung, a beta of 1, is the
   market itself, the broad rung whose buy-and-hold `CRITIC.1` scores
   against. On the first game's first year (*derived* from `POT.2`'s index,
   1.1016 on day 365 and near 0.989 at the low of day 91), the four rungs end
   it up 21.4%, 15.6%, 10.2% and 7.5%, their dips 2.2%, 1.7%, 1.1% and 0.8%,
   and the fourth's dividends add 2% more. *The alternative:* 1.5, 1.25, 1
   and 0.75, gentler, the first rung's swing half again the index's and not
   twice it.
7. **The price: `=INT(<start>*Market!$B$25^<beta>+0.5)`, over the index the
   difficulty picks (rec).** `POT.2`'s decision 11: whole cents, one
   rounding, half up; the idiosyncratic factor `POT.5`'s, never `*(1+0)`
   now. `B25`, so that at difficulty 0 the Kiddie Pool prices the rungs and
   the picker changes the market a diner trades; `B24`, tier 1 always, would
   let the picker change the Market sheet and not one price.
8. **No price at or below zero, at any seed, ever: a bound, and a test.**
   From the index's bound, a third of 1.0002^-1 (*derived*, above), no
   seed's price falls below 11, 96, 833 and 4,386 cents on any day a price
   reads, at either tier built (*derived*, and the hand's to hold). A whole
   cent stays exact in a double below 2^53 cents, $90 trillion, which the
   first rung reaches no sooner than two centuries in, at the cycle's top
   (*derived*, 205 years): `PLATE.6`'s limit, not this item's. The hand
   states the bound, a test holds every rung's price at it to a cent or
   more, and the first game's hundred years are held on every day.
9. **Yesterday's price: recomputed, the Market sheet's column D (rec).**
   `POT.2`'s decision 9's rule: a function of the day is recomputed and never
   remembered. The Market's six fills over `B:C` reach `B:D`, its header row
   gains "yesterday", and `D14` is `=B14-1`, so the index yesterday,
   `Market!D25`, is the index's own formula at the day before, as the look
   back, `C25`, is at twenty days back; a rung's yesterday is its price's
   formula over `D25`. The change, today less yesterday in whole cents,
   exact, is the Market panel's one subtraction before its one division by
   100 (`FRONT.2`), its glyph `FRONT.2`'s; on day 0 yesterday is the model's
   own day -1, as the look back's days below zero are. *The alternatives:*
   remembered, the kit's `hold` of today's price at a day's first frame,
   reads a twin, opens day 0 on a yesterday of 0, a change of the whole
   price, and carries a person's edit of the difficulty in mid-day into the
   next day's change; left to the panels, it puts a market's formula into a
   view, or brings a panel's dish back to the Market sheet after more dishes
   read it. `PLATE.1`'s fill at the price shown is the price's twin, the last
   frame's quote, a cell of its own and not this one. *The cost:* `POT.2`'s
   six fills change their ranges, inside course 0, which `BD-1` leaves free
   until the course is served.
10. **The dividend: the fourth rung's, 2% a year, 50 cents a share on the
    91st, 182nd, 273rd and 364th days of each year (rec).** A yield for every
    rung, 0 for three, since a rung that pays none says so in a cell and the
    table's rows stay alike, the column a later station's income fills (the
    bonds' coupons, course 1). Its amount fixed in whole cents from the
    start, `=INT(<start>*<yield>/$B$1+0.5)`, 50, the dividends a year its
    divisor; and not a yield on the day's price: a dividend is a sum a board
    declares, so its yield rises as the price falls, which is the income
    lesson, and it grows only with `STATION.1`'s second slice (dividend
    growth, course 6). Its days: each day of the year that is a multiple of
    the days between two, `INT(365/4)`, 91, four fixed days of the calendar,
    game days 90, 181, 272 and 363 in the first year, the first on the eve of
    the first game's low (`POT.2`: 1.1% to day 91). With a beta of 0.75, the
    fourth rung's trend is 5.6% a year and its income 2%, the market's 7.6%
    with three quarters of its swing (*derived*). *The alternatives:* 3%, 75
    cents, a total of 8.6%, more than the market at less risk; a yield on the
    day's price, a dividend that falls in a storm.
11. **What holds it: the dividend today, a share, a function of the day
    (rec).** `=IF(MOD(Day!$B$7,$B$2)=0,<a dividend>,0)`: the amount on its
    day, all thirty frames of it, and 0 on every other day, so nothing
    accumulates. `PLATE.1` pays it once, at that day's first frame, to the
    shares a pocket held as the day before closed (the twin, the record date,
    `PLATE.1`'s to scope), into that pocket's cash with a ledger row; until
    `PLATE.1` no share is held and nothing is paid. The report card's income
    line (`CRITIC.1`, course 1) sums the pockets' dividend rows, never this
    cell.
12. **The rows in a ladder's order, and no verb (rec).** The table written by
    the kit's verbs alone, `headers`, `cells` and `formulas`, a rung at a
    time: each rung's four values across its row, then the next rung's
    (where `labels` would write a column first), then one fill a computed
    column, E to H. That is the order a walk over a list of rungs writes, so
    `STATION.2`'s `ladder` (course 1), a station's rungs as rows from a list,
    can rewrite course 0's rows byte for byte, `BD-1`'s first catch. A
    scratch `ladder` measures that at the build and is never committed:
    `kitchen.vla` gains nothing before the second writing.
13. **The hand.** `oracle/src/buffet.rs` gains `Rung`, its start in cents,
    its beta, its yield and its dividend in cents, and the four as `RUNGS`,
    the contract's numbers; a price, `(start * index.powf(beta) +
    0.5).floor()` over `Market::index`, the power, the product, the sum and
    the floor in the cell's order (`POT.2`'s decision 13: a double's last bit
    is its operations'); and the dividend's days as a list, 91, 182, 273 and
    364, where the cell divides by `MOD`, and each dividend the contract's
    whole cents, 50, where the cell computes it from the yield, so the
    dividend shares no predicate with the cell. The bound of decision 8 is a
    function of the hand, with its test.
14. **The tests.** Every frame of two years and two days (the walk that
    stands): every rung's price today and yesterday, its dividend and the
    dividend today, and the Market's column D beside B and C, held to the
    hand; the Stocks sheet's settings, starts, betas and yields held to the
    hand's constants; on day 0 every price exactly its start; a year at
    difficulty 0, the prices the Kiddie Pool's; a hundred years at one frame
    a day, every price today and yesterday and the dividend today held on
    every one of the 36,500 days, the least of each rung at or above the
    bound, and the dividend paid on exactly 400 days, each a day the hand
    lists; the bound's own test. `BD-7`'s scan and the census's
    `(every-frame 0)` stand unchanged and cover the new sheet: the rungs read
    the index and the day of the year, never the frame or the phase.
15. **The hand test in Alonzo's player at `v0.1.2` (rec),** the deployed
    page, since Buffet's own page is `HOUSE.4`'s: the cartridge pasted into
    the drawer's box and loaded, the Stocks tab read on day 0 (the starts),
    at a day paused against the hand's prices, across the first dividend's
    day, at difficulty 0 and back, and at the Clock's rate written 60.

*With it, in its commit:* `POT.2`'s P2 cell filled, CI's Linux holding the
market bit for bit at the first push, run 38689162382, its 22 tests green,
the two years and the hundred years among them.

**The predictions, before any number.** The numbers column was filled at
the build, the same day.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 131 rows (96), 79 formula cells (51) in 28 shapes (22) of 26 texts (20), 14 levels (13), `(every-frame 0)`; the describe golden moved by the notes, the budget, `(budget 30 79 2370 2000000)`, the plan, the readers, the Market's six fills that reach D, and six new shapes | *Measurement*, as predicted in every count; the golden's diff the notes, the budget, the plan, the readers (Day 11 to 13, the Market's 31 to 34, the Stocks' 8), the Market's six fills, each a cell more and its shape's number one on since `D14` stands before them, and the six new shapes, `POT.1`'s and the rest of `POT.2`'s unmoved. Not predicted: 6 runs of 27 lanes (2 of 9), each of the Stocks' four fills one run of its four rows, and 52 cells computed alone (42) |
| P2 | the hand at every frame of two years and two days | every price today and yesterday, each dividend, the dividend today and the Market's column D equal at all 21,960 frames, natively here and in CI on Linux | *Measurement*: equal at all 21,960 frames here, on Windows, at the first run; and on Linux at the push, run 38712895205, the cartridge's 14 tests green, this one among them |
| P3 | a hundred years at one frame a day | every price and the dividend today equal on all 36,500 days, the dividend paid on exactly 400 of them; the whole suite under 10 s in a debug build (about 5 now), 6 to 9 | *Measurement*: equal on all 36,500 days, the dividend paid on exactly 400. The suite first took 11.6 s, its two long tests 11.3 and 12.1 s alone, **missed**: the Stocks sheet's settings and workings, 56 cells, were viewed at every frame and every day, and a debug build's view is the cost, the hand's arithmetic microseconds. With the window narrowed to the four columns that compute and the settings read once by a test of their own, 7.4 s, the two long tests 7.4 and 6.6 s: inside. A suite's time is the machine's load as much as the code's: the next day, the processor half busy with another program, the cartridge's tests took 10.9 s against the old suite's 7.3 to 7.5 s, alternated in a worktree of the commit before, half again its time either way |
| P4 | the opening | every price on day 0 exactly its start, 100, 500, 2,500 and 10,000, here and on Linux | *Measurement*: exactly the starts at every frame of day 0 here, and the hand's own at both tiers; and on Linux at the push, run 38712895205 |
| P5 | each rung's least price in the first game's hundred years | at the first year's low, near day 91: 97 to 99, 489 to 494, 2,465 to 2,480 and 9,905 to 9,925 cents; every price far above the bound's 11, 96, 833 and 4,386 | *Measurement*, by a scratch probe over the hand: 98, 492, 2,474 and 9,921 cents, inside each. The index's least is 0.98943 on day 91, and each rung reaches its least cent sooner, on days 44, 72, 75 and 82, and holds it through the low, a cent being a wide step at these prices. The bound as derived, 11, 96, 833 and 4,386 |
| P6 | the largest fall from a peak in the first game's hundred years | the index's between 20 and 35% (seed 777's was 30.0%); each rung's `1-(1-f)^b` of it to a cent, so the first rung's 36 to 58%, the second's 28 to 48% and the fourth's 15 to 28% | *Measurement*: the index's 34.7%, from day 21,325 to day 22,159 (years 58.4 to 60.7), inside near the top of the range; the rungs' 57.3%, 47.2%, 34.7% and 27.3%, each `1-(1-f)^b` to four places, inside, three of their peaks a day earlier, where the rounding to a cent flattens the top |
| P7 | the ladder | a scratch `ladder` of under 25 lines, a walk over a list of rungs, writes the table's 20 rows below its headers byte for byte from a call of a line a rung | *Measurement*: four template macros, 24 lines, a walk over the rungs handing its list on whole and the fills after it, wrote the 20 rows byte for byte through the engine's pass over the kit, from a call of a line a rung and a line a fill; deleted with the probe. Inside, by a line |
| P8 | the page against the hand at the owner's paused day | every price, yesterday's and the dividend equal to the hand's, since money is rounded once and a last bit moves a price only within a few units in the last place of a half cent, a chance below one in 10^10 a price; the Market's column D equal to the hand's in its first 15 digits at least | *Measurement*, the owner's five screenshots, read from crops at full resolution: equal to the hand to the last digit in every cell they show. The Stocks sheet on days 46 and 90, 34 formula cells, 16 prices each a power among them, and the dividend paid on day 90 alone; the Market sheet on day 90 at difficulty 1 and day 111 at difficulty 0, 43 formula cells each (the speeds, the phases, the days and 31 workings, column D among them), 34 sines and 6 powers and every cell built on them. Inside: the prices as predicted, and the index to the last bit, where the prediction allowed the 16th digit |

**What the build found.**
- A cent is a wide step for a cheap share. In the first year the four rungs'
  prices move on 25, 93, 231 and 316 of its 365 days (*measured* by the same
  probe), since a day's move of the index is a few hundredths of a percent
  and a cent is 1% of the first rung: the $1 rung shows no change on
  fourteen days in fifteen of the first year, and ends it 21 cents up. Over
  the hundred years, as the prices grow, the rungs move on 32,327 to 35,853
  of the 36,500 days. Whole cents are `FRONT.2`'s rule and the starts the contract's;
  the step is a fact for the Market panel's glyph (`=` most days on the
  first rung) and a row for `MENU.5`, a penny stock moving a penny at a time.
- Yesterday on day 0 is the model's day -1, and the market fell into day 0:
  the first frame shows the third and fourth rungs a cent and two cents
  down, their yesterdays 2,501 and 10,002.
- A view of a sheet in a debug build costs tests more than anything they
  compute (P3).

**The owner's hand test, 2026-10-15,** in Alonzo's player at `v0.1.2`,
deployed, its log naming `alonzo` 0.1.2 over `vla-lang` 0.9.3; the cartridge
pasted into the drawer's box and loaded. The budget line read 79 formula
cells a frame at 30 frames a second, 2,370 a second; the Stocks tab stood
after Market. At frame 1,389, day 46, the four rungs at 98, 494, 2,480 and
9,941 cents and their yesterdays at 98, 494, 2,481 and 9,942.
`(cell "Clock" "B2" 3)`, written at frame 2,463, made a day ten seconds; paused
at frame 2,711, day 90, the day of the year 91, the fourth rung's dividend
today read 50 and the others' nothing, the prices 98, 492, 2,474 and 9,921
today and yesterday alike, and the Market sheet showed its new column,
yesterday, day 89. `(cell "Clock" "B2" 30)` at frame 2,712 gave back a day a
second, and `(cell "Market" "B3" 0)` at frame 3,084 made the index the Kiddie
Pool's, at frame 3,353, day 111, in all three columns. Five screenshots, every
cell they show the hand's (P8). Passed. The last two steps asked, the
difficulty written back to 1 and the rate written 60, are not among the
screenshots; neither is new here, `POT.2`'s hand test having passed both, and
the tests hold the stocks at both difficulties and at one frame a day.

## `HOUSE.4`, first slice — the page

*Scoped 2026-10-15 at the owner's word, "scope and build HOUSE.4, first
slice: the page", from `OMAKASE.md`'s line (`COURSE.0`'s item 6) and
Alonzo's `web/README.md`, "A game's page" (`ENGINE.15`). The owner's prompt
was written before `alonzo` 0.1.3 was published and named `v0.1.2`; the
owner said so at the session's start, and decision 1 reads the newer tag.
Each decision carries its recommendation, for the owner to approve, amend
or strike before the build. The predictions were written before any number
was taken; each number goes beside its own at the build. Approved whole by
the owner the same day, "HOUSE.4 recommendations approved; proceed with
implementing as scoped", and built that day, over `00eb51c`, the dev
profile's optimization.*

**What it stands on, read on the day** (Alonzo's tags `v0.1.2`, `47066cf`,
and `v0.1.3`, `78aa82e`).
- `alonzo` 0.1.3 was released on 2026-10-15: tagged, published to crates.io
  by Trusted Publishing, its page deployed. It pins `vla-lang` =0.9.3, as
  0.1.1 and 0.1.2 do. Its engine is 0.1.1's code: `git diff v0.1.1 v0.1.3
  -- engine/src` is one line, the version a test of `abi.rs` expects, so the
  oracle's 27 tests run over the same engine.
- What 0.1.3 adds is in the page alone, `ENGINE.8`'s second slice
  (`5b72e68`): from frame 1 the host draws the Screen from the `values`
  view, the grid record less its formula rows, and at frame 0 from the grid
  record (`host.js` 12 lines in and 8 out, `fake.js` 6 and 4, `viewport.js`
  15 and 1, the template 79 and 31, its oracle among them). A sheet shown by
  its tab, Stocks or Market, is still viewed as the grid record.
  `build_web.ps1` and `check_web_offline.ps1` are the same file at both tags.
- The kit is one file at `v0.1.1`, `v0.1.2` and `v0.1.3`: SHA-256
  `323AAB61…D8B30A2` over 23,897 LF bytes. Buffet's copy is `v0.1.0`'s,
  `315D3611…BFFC16C9` over 23,787. `HOUSE.2` moved the pin to 0.1.1 and did
  not copy the kit again, and `check_kit.ps1` could not see it, its baseline
  naming the tag only in a comment. The 22 lines that differ are comments
  and docstrings, "SPEC.md" become "docs/SPEC.md" (Alonzo's `2e687f2`); no
  macro's body moved.
- The template names no game's file (`ENGINE.15`). Every text is a block
  its builder writes at `{{BLOCKS}}`, `<script type="text/plain"
  id="KIND-NAME" data-path="PATH">`, beside four places more, the three
  scripts and the module. The page always opens on the empty sheet, its
  picker set to it and no query or hash naming another, with its cartridges
  after it by their manifests' titles. Every bench mode refuses on a page
  without Alonzo's eight cartridges, its library and its seven records, and
  says what it lacks.
- Build in the drawer runs the module's pass over the source's box with the
  source block's path and every library block as a path and a text
  (`host.expand`). The command line's runner passes `src/buffet.source.vla`
  and the source's includes resolved against its folder, `src/common.vla`
  and `src/kitchen.vla` (`oracle/src/lib.rs`, `gather`). Blocks at those
  paths make Build write the same bytes.
- The double, `web/fake.js`, reads a cartridge's manifest and never a row
  after it, evaluates nothing and holds no pass: a page without the
  engine's module cannot play the game or build it.
- The module is the step CI has run since `HOUSE.2`, `cargo build --release
  --locked -p alonzo --target wasm32-unknown-unknown`, written to
  `target/wasm32-unknown-unknown/release/alonzo.wasm`. Built here from 0.1.0
  on 2026-10-14 it was 749,386 bytes; Alonzo's own build of 0.1.2 and 0.1.3
  is 749,218, its own sources' paths shorter than the registry's.
- `.gitattributes` checks out `*.js` and `*.html` LF, `REUSE.toml` already
  gives `web/**` Apache-2.0, and `.gitignore` already ignores
  `web/index.html`.

**The decisions.**

1. **The pin: `alonzo` =0.1.3 (rec), `vla-lang` =0.9.3 kept.** The newest
   release, its engine the code Buffet has run since 0.1.1, so no frame can
   move; its page draws the grid from the `values` view, the slice
   `MISE.6` would have asked for at 1,000 formula cells, met before it was
   asked. The lock moves in `alonzo`'s version and checksum. *The
   alternative:* =0.1.2, the prompt's: the same engine and the page drawing
   from the full record, with a second pin move to come for no change of
   Buffet's.
2. **Every copy at that one tag, `v0.1.3`.** The kit again, its 22 lines;
   the template and the three scripts unchanged, each with its Apache-2.0
   header, at the paths Alonzo gives them, `web/index.template.html`,
   `web/host.js`, `web/fake.js` and `web/viewport.js`, so each diffs against
   Alonzo's by name. Not the bench's records, which no mode of a game's page
   reads, and not `web/README.md`, whose section "A game's page" this
   scoping cites. The template is never edited here. `NOTICE` and
   `REUSE.toml` say "vendored" in the present tense, and `README.md`'s
   Building names the pin and the page's two commands.
3. **One check holds every copy: `tools/check_kit.ps1` grown and renamed
   `tools/check_vendored.ps1` (rec).** One table, a row a file, the kit and
   the page's four, each its SHA-256 over its LF text and its length in LF
   bytes, and the tag written once, `v0.1.3`, and held equal to
   `Cargo.toml`'s pin of `alonzo`, so that a pin moved without the copies
   fails, the drift `HOUSE.2` left and no check saw. And `web/` holding
   nothing but the four files and the page the builder writes there:
   `REUSE.toml` gives everything under `web/` Apache-2.0, and a file of
   Buffet's own put there would take the wrong terms. `-Control`: the clean
   copies pass; a byte changed in the kit, a byte changed in `host.js`, a
   copy missing, the pin moved and not the tag, and a stray file under
   `web/` each fail by name, five mutants. *The alternatives:* the name
   kept, the page's files held by a check named for the kit; or a check of
   the page's own beside it, two baselines each naming the tag, free to
   disagree.
4. **The builder: `tools/build_web.ps1`, Buffet's own (rec).** Alonzo's at
   `v0.1.3`, adapted by the same copyright holder under this repository's
   terms, as `oracle/examples/expand.rs` was. It fills the same five
   places, each once, and refuses a template with a place naming a file.
   It inlines `{{VIEWPORT_JS}}`, `{{FAKE_JS}}` and `{{HOST_JS}}` whole, and
   writes `{{BLOCKS}}` in this order:
   - `page-name`, Buffet;
   - for each `src/<name>.source.vla` (one, today), `cartridge-<name>` at
     `src/<name>.vla` with `data-file` `<name>.vla`, then `source-<name>` at
     its own path;
   - every other `.vla` of `src/` a library at its own path,
     `library-common` at `src/common.vla` and `library-kitchen` at
     `src/kitchen.vla`, the paths the source's includes resolve to.

   Each text is read with CRLF as LF, and a text holding `</script` is
   refused. The page is written UTF-8 without a byte-order mark to
   `web/index.html`. There is no order list, the page holding one
   cartridge, and no records. The replay and golden places come with
   `COURSE.0`'s tasting, with their files; `MENU.1`'s content sheets, under
   `src/content/`, will want library blocks at their paths, and the
   builder's rule widens then.
5. **The module required (rec).** `{{ALONZO_WASM}}` is the base64 of
   `target/wasm32-unknown-unknown/release/alonzo.wasm`. Alonzo's builder
   writes a page with no module when none is built, so that a machine
   without Rust still builds its bench, whose cases run over the double. A
   game's page without the module plays nothing, so Buffet's builder
   refuses without it and names the cargo command that builds it; `-NoWasm`
   builds a page anyway, for the ratchets job and the check's control. *The
   alternative:* Alonzo's way, a page whose bar says it carries no module.
6. **The offline check: `tools/check_web_offline.ps1`, Alonzo's adapted
   (rec).** It holds the template and the three scripts to no reference
   that reaches out: a script's `src`, a link, an image, a frame, a form, an
   anchor, `@import`, `url(`, `http://`, `https://`, `fetch`,
   `XMLHttpRequest`, `WebSocket`, `EventSource`, `sendBeacon`, a dynamic
   `import(` and `importScripts`. It also holds a charset, the five places
   once each, and no place naming a file. That keeps `SD-13` in Buffet's
   own CI whatever a later tag brings. When a built page stands beside the
   template, the check holds it too:
   - the same markup;
   - no place left;
   - its blocks are `src/`'s files both ways, each block at its file's path
     with its file's LF text, so that a page built before an edit of the
     source fails here and not at a hand test.

   Alonzo's list of Web Audio calls is left out. It is Alonzo's charter on
   its own host (its `CHARTER.md` §8), held at the tag by Alonzo's check
   and here by the digest. `-Control` passes the real template and scripts,
   and a page the builder writes in a scratch folder with `-NoWasm`.
   Fourteen mutants each fail by name:
   - in the template: a script's `src`, a `fetch`, the viewport's place
     removed, an anchor, an image, a place naming a file, and the blocks'
     place removed;
   - in the scripts: a `fetch` in the viewport and one in the host;
   - in a built page: the cartridge's block cut, a library block for no
     file, the template standing as a page, a library block at another
     path, and a block whose text is not its file's.

   `run_checks.ps1`'s floor rises from 3 to 4, and the control joins
   `-WithExtras`.
7. **CI (rec).** The crate job, after the module: the page built under
   `pwsh`, then the offline check over it under `pwsh`, so the page with the
   module in it is held on Linux. The ratchets job, under Windows
   PowerShell 5.1: the page built with `-NoWasm` before `run_checks.ps1
   -WithExtras`, so that the builder and the check run under the house's
   floor too and the blocks are held there. No artifact is kept: the deploy
   is `HOUSE.5`'s. *The alternative for the ratchets job:* no page, the
   check holding the template alone and saying it found no built page.
8. **The opening: the empty sheet, Buffet in the picker after it (rec, for
   now).** `ENGINE.15`'s page opens on the empty sheet whatever it holds,
   the owner's word of 2026-10-14, and the copied template is never edited.
   A candidate row in `MISE.6`: a block naming the cartridge a page opens
   on, asked of Alonzo before `HOUSE.5` serves the page, since a diner
   opening the deployed game should be seated at it; not asked now. *The
   alternative:* ask now, an `ENGINE` item there.
9. **The stale places, rewritten at the build.** `OMAKASE.md`'s `HOUSE.4`
   line went stale in three places:
   - its places, now `{{BLOCKS}}`;
   - its *Needs*: `FRONT.1` struck, since the page shows the model's sheets
     live by their tabs before any panel exists, and "Alonzo's next tag"
     met by `v0.1.3`;
   - first light, which moves to `FRONT.1`, the dish that completes it, and
     `MISE.3`'s re-measure of the window with it.

   `MISE.6`'s `ENGINE.14` row is met in 0.1.1 and vendored here. Its
   `ENGINE.8` row is met unasked in 0.1.3, so the re-measure at first light
   is of the `values` view. The critical path's fifth step and the sentence
   on `ENGINE.8` in "What Buffet does not wait for" follow. The deploy,
   `pages.yml`, stays `HOUSE.5`'s.
10. **The hand test, the owner's, on the page built here:**
    - the page opened from disk in Chrome, at full screen;
    - its bar named Buffet, its log naming `alonzo` 0.1.3 over `vla-lang`
      0.9.3;
    - Buffet picked and run at `(rate 30)`, the Stocks tab's four prices
      moving on a day's first frame;
    - Build in the drawer writing the cartridge byte for byte, compared with
      `src/buffet.vla` by a command given at the handover.

**The predictions, before any number.** The numbers column was filled at
the build, the same day, P6 at the push and P7 at the owner's hand test.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the pin | `alonzo` 0.1.3 from crates.io, the lock moving in its version and checksum alone | *Measurement*, as predicted: `cargo update -p alonzo` moved the lock from 0.1.1 to 0.1.3 and its checksum to `90c377d0…77610`, `vla-lang`'s entry unmoved |
| P2 | the kit copied again | the cartridge the same bytes (`expand --check`), the describe golden unmoved, the 27 tests green here and in CI on Linux | *Measurement*, here: `expand --check` same, the describe golden unmoved, the 27 tests green, the suite 4.1 s. The kit copied CRLF, 409 lines, its LF text the tag's digest. On Linux at the push, run 38712895205: the 27 green, 13 of the crate and 14 of the cartridge |
| P3 | the module | 749,386 bytes, the size of the 0.1.0 build here, differing from it only where the version and the registry folder's name are spelled | *Measurement*: 749,434 bytes, **missed** by 48. The prediction read 0.1.1 to 0.1.3 alone and set them against a build of 0.1.0, and since 0.1.1 the engine's catalog cites `docs/SPEC.md` where it cited `SPEC.md`, in ten references, 50 bytes of text more (Alonzo's `2e687f2`). The version and the registry folder's name are spelled at the same lengths, as predicted. 216 bytes more than Alonzo's own build of the same code, the registry's paths in the engine's three panic locations |
| P4 | the page | 1.58 to 1.59 million characters: the template and the scripts 538,000, the module's base64 999,000, the five blocks about 47,000; written in under 2 s | *Measurement*: 1,585,014 characters, every one ASCII: the template and the scripts 538,044 once their five places are filled, the module's base64 999,248, the five blocks 47,722; written in 0.33 s under Windows PowerShell 5.1. Inside |
| P5 | the checks | 4 checks and 4 controls green, the vendored check's 5 mutants and the offline check's 14 each failing by name | *Measurement*: ALL GREEN, 4 checks and 4 controls in 4.2 s, every mutant failing for its own reason. `check_spdx` reads 37 tracked files, the page's four Apache-2.0 |
| P6 | CI | the crate job longer by the page and its check, under 10 s; the ratchets job by under 5 s | *Measurement*, run 38712895205: the page built and held offline in a step of 3 s in the crate job, and built in a step of 4 s in the ratchets job, which took 22 s in all where it had taken 24. Inside. The crate job as a whole grew from 40 s to 74 s, but by the dev profile's optimization (`00eb51c`, pushed with this item), its build, lint and test step 51 s where it had been 19, the engine and the language compiled optimized on every run |
| P7 | the hand test | the four prices moving on a day's first frame alone, as the hand has them; Build's text equal to the cartridge's LF bytes | *Measurement*, the owner's two screenshots and a clipboard comparison: paused at frame 1,394, day 46, the four prices today 98, 494, 2,480 and 9,941 cents and yesterday 98, 494, 2,481 and 9,942, the hand's to the cent, as `STATION.1`'s hand test had them at frame 1,389; Build's text in the drawer the cartridge's 5,607 LF characters, the comparison printing "same bytes". Inside. The screenshots hold one paused frame, so the moving itself was seen live and is not in them |

**What the build found.** The engine's refusal texts are bytes of its
module: Alonzo's move of its documents into `docs/` (`2e687f2`, in 0.1.1)
changed ten of the catalog's sentences, and with them the module's size,
without a line of code (P3). A module's size witnesses everything the
engine embeds, its catalog as much as its code, so a prediction of it
reads every embedded file between the two versions it compares.

The pass writes CRLF whatever the line endings it reads: from the page's
LF texts at their `src/` paths it writes the checked-out cartridge's own
5,749 bytes, the CRLF file, whose LF text, 5,607 bytes, is the block the
page holds (a scratch test of the oracle crate, run natively before the
hand test and deleted). So Build in the drawer is compared with the file as
the runner's `--check` compares it, line endings read as LF.

**The owner's hand test, 2026-10-15,** on the page built here, opened from
disk in Chrome. It opened on the empty sheet, the bar named Buffet and the
picker set to "An empty sheet", Frame 0 and the budget of 0 formula cells a
step: decision 8's opening, which the owner noted, the candidate row in
`MISE.6` asking before service for a block that names the opening
cartridge. Buffet picked and run at full screen, paused at frame 1,394, day
46, the Stocks tab read the hand's prices today and yesterday (P7), the
dividends a year 4 and the days between two 91, the fourth rung's dividend
50 cents and the dividend today 0, day 46 being no dividend day; the tabs
Screen, Day, Dice, Market, Stocks, Clock, File and Input. Build in the
drawer wrote the cartridge's bytes, "same bytes: 5607 LF characters". The
log's line naming `alonzo` 0.1.3 over `vla-lang` 0.9.3 is in no
screenshot; a headless load of the same page read it before the handover,
and Build, which only the engine's module can run, says the module was
there. Passed.

## `FRONT.1` — the workbook: the game's views, each a sheet of its own

*Scoped 2026-10-15 at the owner's word, "Scope and implement FRONT.1 from
Buffet's OMAKASE.md": first as panels on the Screen, from `OMAKASE.md`'s
line (`COURSE.0`'s item 5), the three spellings the owner approved for it at
`MISE.3`'s close and the code of the page `HOUSE.4` vendored at `v0.1.3`;
then, at the owner's two readings the same day, as the game's views on
sheets of their own, which waits on Alonzo. Nothing was built.*

*The owner's first reading. "No kitchen names anywhere. The kitchen analogy
is for the dev documents only. The title, "Buffet", is the extent of the
kitchen terminology in the game, because the game should look and feel like
a fund manager's workbook with the humor limited to financial and economic
winks and jokes throughout": `BD-2`'s amendment, striking the kitchen's
titles the first scoping put on its panels. "The tabs should not be
numbered, because they're not necessarily serial": its labels, `1 Exam` to
`5 Work`, and the number keys with them. "The game should open on the
player's portfolio until the Exam is created", where it opened on the
Market. And why the panels shared their columns' widths: "That's not how
spreadsheets work."*

**What it stands on, read on the day** (`web/` as vendored at `v0.1.3`;
`vla-lang` 0.9.3 and `alonzo` 0.1.3 as the registry holds them).
- The page opens a grid cartridge on its Screen, drawn through the Camera's
  window (`host.js`, `_drawGrid`), and the player loads a cartridge picked
  from its list paused at frame 0, where a formula shows its text until its
  first value (`index.template.html`).
- The pointer. The host writes `Input!B11` and `B12`, the Screen's own
  column and row under the pointer through the Camera, 0 off the window's
  cells, and `B13`, the left button held (`_inputRows`, `_windowCellAt`).
  While another sheet shows, the pointer is 0, so a click there is never
  the game's (`ENGINE.14`).
- A sheet shown by its tab is viewed at every frame as the `grid` record of
  the region in view, every formula's text with it (`refreshSheet`,
  `viewRecord`), where the Screen has been drawn from the `values` view,
  the grid record less its formula rows, after frame 0 since `alonzo` 0.1.3
  (`ENGINE.8`'s second slice).
- The keys. The host writes `Input!B17` and `B18`, the last key pressed
  during a frame as `KeyboardEvent.key` and `.code`, for that frame alone,
  for every key, bound or not (`keyDown`, `_inputRows`). A key the Keys
  sheet binds, one key to each of ten buttons a player, is taken before the
  viewport sees it; the viewport moves by the arrows, Page Up and Down,
  Home, End and Enter (`viewport.js`, `_keyDown`).
- The step follows a formula cell to its readers only when it computes a
  value whose bits differ from its last (`vla-lang` 0.9.3,
  `machine/mod.rs`), so a cell that reads the pointer and holds 0 between
  presses shields everything that reads it from the pointer's moves.
- A row's height is one `(row ...)` row a sheet row, in points, with no
  range form (`machine/load.rs`), and a column's width one `(column ...)`
  row a column, in Excel's characters, about 7 CSS pixels a character, the
  default 64 pixels (`viewport.js`, `columnPixels`). Frazaro's `.xlsx`
  writer already writes Excel's sheet-wide default row height,
  `<sheetFormatPr defaultRowHeight="15"/>` (`core/src/sheet/ooxml.rs`).
- `describe`'s `every-frame` counts only a shape the reach cannot follow, a
  defined name's or a span's, and none that reads the Input sheet
  (`calc/census.rs`).
- At the owner's full screen, 2,560 by 1,440 CSS pixels at a ratio of 1.5,
  the player shows 38 to 40 columns and 68 rows of the default cells
  (`MISE.3`).

**The first scoping, struck.** It laid the line's five panels down the one
Screen, fifty rows each, the Camera moving the window from one to another;
a tab strip on each panel's first row, the panel's own tab marked; a Window
sheet that read a click and the number keys and kept the tab shown; and the
game opening on the Market. Stacked on one sheet, the panels shared every
column's width, so the widths had to be a grid every panel lay on, and a
panel's 20-point rows cost 250 `row` rows. It followed the line, and the
line followed the page, which opens on the Screen and hears a click there
alone. Offered, after the first reading, the panels side by side on the
Screen, each its own widths and only the strip's columns shared, or the
game's views as sheets of their own, which waits on Alonzo, the owner chose
the sheets: "Real sheets are the path forward ("the UI is a workbook"),
because Buffet is a mini, blossoming Frazaro, so each Buffet tab should
have its own properties as if each were an independent view." Of the keys,
which the first scoping had read as the tabs' shortcuts: "Keyboard
shortcuts are crucial to a spreadsheet, but I expect key chords [Alt +
sequence of keys] to be the main approach to mouseless interaction." And a
horizon past this item, named and set aside in the same breath: "A nifty
pipe dream of mine would be to allow arbitrary cartridges to be loaded
alongside spreadsheet sheets, since every cartridge is a grid of cells, so
Kairos could be a tab beside any other sheet... but I don't need to get
ahead of myself here."

**What Buffet needs of Alonzo**, asked 2026-10-15 at the owner's choice,
its ID that repository's to mint (`ENGINE.18` its next free), and written
into `MISE.6`. Each need names the dish that needs it first; each spelling
is Alonzo's.
1. **The sheet a grid cartridge opens on** (`FRONT.1`). Buffet opens on its
   Portfolio sheet, where the page opens every grid cartridge on its
   Screen.
2. **A sheet shown drawn as the Screen is** (`FRONT.1`). Buffet's views are
   sheets that change once a market day, and a sheet shown moves every
   formula's text at every frame; asked, the `values` view after frame 0.
   The still frame for a sheet shown, its load and paint skipped when its
   record is the one last drawn, is `ENGINE.17`'s decision 8, built the
   same day at the owner's word (below).
3. **A click on the sheet shown, the game's** (`BACK.1`, `PLATE.1`,
   `GUARD.1`). Asked: the cell under the pointer on whatever sheet the
   viewport shows, and that sheet's name, in the Input sheet, so that a
   button compares the sheet, the row and the column, and a button on the
   Portfolio is ever pressed.
4. **The game showing a sheet** (`GUARD.1`, whose exam ends on the
   Portfolio). An effect the host reads after each step, the person's own
   tab followed between its changes, as the Camera's move is.
5. **The Screen of a cartridge whose views are sheets** (with 1). Buffet's
   Screen holds nothing, yet stands first among the tabs and is what the
   page draws: hidden, or none.
6. **Key chords** (a sitting of its own, before the first dish that needs
   one). Alt and a sequence of keys, the main way about the workbook
   without a mouse: which chords the workbook keeps for itself, moving
   between sheets the first; how a cartridge declares its own as data, the
   Keys sheet being "Emacs's keymap as a cell edit" (`SPEC.md` §3.4) for
   one key a button today; and which chords the browser keeps for itself
   (Chrome's Alt+Left goes back a page).

**What `FRONT.1` becomes**, rescoped when Alonzo releases the item, its
decisions then the owner's to approve:
- The game's views are sheets of their own, each with its own widths,
  heights, looks and gridlines, reached by the workbook's own tabs: no
  strip drawn in cells, no Camera, no Window sheet, no `panel` in the
  library.
- The Portfolio first, the sheet the game opens on until the Exam exists
  (`GUARD.1`). A view enters with the dish that fills it, the Exam with
  `GUARD.1`, the Overview with `FRONT.3`, Work with `BACK.1`, the market's
  view and its buttons with `PLATE.1`, so no sheet stands empty waiting.
- First light: the Portfolio showing the day and the four stocks, each its
  price today and its change since yesterday in dollars, whole cents over
  100 once (`FRONT.2`'s money rule), moving a day a second; the shares, the
  cost, the value and the gain are `PLATE.1`'s columns beside them.
- The model's sheets after the views, visible: the spreadsheet a player may
  open (`AD-6`).
- The names a fund manager's, plain, the tabs unnumbered (`BD-2`'s
  amendment). A market view named Market and the model's Market sheet,
  `POT.2`'s index, cannot share a name, and one is renamed at `PLATE.1`.
- The rows the default height, unless the owner asks for taller ones by
  eye. A sheet's default row height, Excel's `<sheetFormatPr
  defaultRowHeight>`, is a candidate ask of Frazaro (`MISE.6`); a `row` row
  over a range of rows is the other spelling.
- The notes: the kitchen's words out (`BD-2`), the tagline kept, "because
  it's the game's tagline and punchline" (the owner, 2026-10-15), spelled
  as the README's title spells it, "All You Can Sheet".
- Kept from the first scoping: a view holds at most 1,000 formula cells in
  view (`MISE.3`); a background by the columns' format; a button reading a
  click kept once in cells that hold 0 between presses, never the raw
  pointer; `MISE.3`'s first number re-measured on the real view by a probe
  page; the owner's hand test; the hand and the tests holding every frame.

**`ENGINE.17`, built beside this in Alonzo.** A peer session built
Alonzo's `ENGINE.17` the same day, minted for Buffet, its client: a grid
frame whose Screen record is the one the viewport last drew is neither
loaded nor painted. Its decision 8, the owner's word on its first build,
gives a sheet shown in the Screen's place the same skip, so Buffet's views
on sheets take it, each still between market days so long as it shows
nothing that changes every frame (`BD-7`: only the Day sheet reads the
frame, and the Day sheet, its phase moving every frame, is never still).
Uncommitted there on 2026-10-15, awaiting the owner's `?blit=1` run and
hand test; the views' item builds on its tree.

## `MENU.1` — the strings sheets and the picker

*Scoped 2026-10-15 at the owner's word, "Scope and implement MENU.1 in
Buffet (Delta/Buffet): the strings sheets and the picker", to stop for the
owner's approval before building: from `OMAKASE.md`'s line (`COURSE.0`'s
item 11), the numbers behind it (`MISE.3`'s probe 3, P3.1 to P3.5) and the
spelling the owner approved from them, and from what the same day changed,
the workbook as the game's interface (`FRONT.1`) and `BD-2`'s amendment, no
kitchen word in the game. Each decision carries its recommendation, for the
owner to approve, amend or strike before the build. The predictions were
written before any number was taken; each number goes beside its own at the
build. Approved whole by the owner the same day, "Recommendations approved;
proceed with implementing MENU.1 as scoped", and built that day. Alonzo's
and Frazaro's trees, where peers built `ENGINE.18` and `KERNEL.42`, were
read and never touched.*

**What it stands on, read on the day** (`vla-lang` 0.9.3 and `alonzo`
0.1.3 as the registry holds them; `web/` as vendored at `v0.1.3`).
- The pass splices each top-level include at its own line, resolved
  against the source's folder, so `(include "content/disclosures.vla")` in
  `src/buffet.source.vla` is the text at `src/content/disclosures.vla`,
  which the runner's `gather` reads and the page's Build finds by a block
  at that path. The pass defines every `defmacro` first and copies the
  first other form, unexpanded, as the manifest (`alonzo::pass`), so a file
  that writes rows is included after the manifest: the kit and the library
  before it, the content after.
- A sheet stands among the tabs where a row first names it, and the
  devices after every sheet the cartridge names (`ensure_sheet`; `HOUSE.4`'s
  hand test read Screen, Day, Dice, Market, Stocks, Clock, File and Input);
  `(sheet "X" hidden)` hides one. The machine makes a twin, `X.last`, for
  every sheet, hidden, and the record lists every sheet in tab order with
  its state, the twins among them. The vendored viewport leaves a hidden
  sheet out of the strip and lists it under a box, "hidden sheets (n)", a
  click away (`viewport.js`, `_renderTabs`); today the box holds Buffet's
  eight twins.
- A name a formula writes bare, by `vla-lang`'s quote rule (`refers.rs`,
  `sheet_needs_quotes`, Excel's): letters, digits, `_` and `.`, no leading
  digit, and not itself a cell, as `FY2026` is, or an R1C1 reference, as
  `R2C3` is. Excel caps a sheet's name at 31 characters.
- The expander has what a walk needs: `quote-if`, `null?`, `car`, `cdr`,
  `cddr`, the comparisons, `+expand`, `-expand`, `quotient-expand`,
  `mod-expand`, `&expand` and `address-expand`. A chain stops at 5,000
  links, an application and an unwrapped `begin` each (the kit's header),
  so a walk that writes a row's cells in a `begin` and recurses takes two
  links a row and stops near 2,500 rows.
- `MISE.3`'s walker, still in that session's scratchpad: `picker` and
  `picker-walk`, `=CHOOSE(` and the index, then a reference a row joined
  onto the text so far under `unquote`, rows `first` to `last`, the formula
  written once the row passes `last`. On its content sheet, its references
  bare and its index `B1`, it held 1,548 rows, `A1` to `A1548`, 8,191
  characters past the `=`.
- The page keys a library by its block's id less `library-`, and at Build
  hands the pass every library as its path and its text
  (`index.template.html`, `libraryPairs`); `check_web_offline.ps1` reads a
  block's id as `[a-z0-9_-]`. Both writers of `src/`'s blocks,
  `build_web.ps1` and the check's `Get-OwedBlocks`, read `src/`'s top level
  alone.
- `check_layout.ps1` pins content sheets at 0, wants the source's includes
  to be exactly `common.vla` then `kitchen.vla`, and counts a content sheet
  before its pin among its control's seven mutants. `check_spdx.ps1`
  already holds every text file under `src/`, a `.md` among them, to the
  map's header, `CC-BY-SA-4.0` for `src/content/**`.
- The cartridge is committed, and `the_cartridge_is_its_source_expanded`
  fails after any edit of what the source includes until the pass runs
  again: a stranger who adds a row in GitHub's editor, without Rust, leaves
  that test red.
- The game's notes still read "Buffet: all you can sheet. The kitchen is
  open and the dining room is not", against `BD-2`'s amendment and the
  tagline's settled spelling, "All You Can Sheet"; `FRONT.1`'s rescoping
  claims their rewrite, and waits on Alonzo.
- `ENGINE.18`, 🟡 in Alonzo's tree: a grid cartridge opens on the sheet
  `Camera!B3` names, else on the first visible sheet in tab order, and
  every sheet shown is drawn from the `values` view after frame 0. So a
  hidden content sheet never opens the game, and one a player opens moves
  no formula's text.

**The decisions.**

1. **One sheet now, the fine print's, and every other with the dish that
   reads it (rec).** `FRONT.1`'s rule, that no sheet stands empty waiting
   for its dish, holds as well for a sheet whose rows nothing reads, and
   the one string course 0 reads through this item is the fine print. So
   `MENU.1` builds the verbs, the folder's rules, the page's blocks, the
   tests, the *How to add a row* and one content sheet; the question bank
   comes with `MENU.2`, the headlines with `FRONT.3`, the memos with the
   first of `MENU.4` and `GUARD.3`, the firms' names with `BACK.2`, the
   error values' jokes with `PLATE.6`, the awards with `MENU.7` and the
   exits with `FRONT.7`, each a file, an include and an index, named by
   decision 2's rule. *The alternative:* all eight now, seeded from
   `BUFFET.md`'s appendix, their rows read by no cell until their dishes.

2. **The names: a fund manager's word for the one kind of string a sheet
   holds, written bare, colliding with no view and no model's sheet
   (rec).** One kind a sheet, since a sheet's rows are read by one picker
   and a row added moves no other: a sheet of every kind of joke would want
   a picker a kind, each over a stretch that a row added in its middle
   moves. The line's eight, checked:

   | The line's name | What it holds, and what reads it | The verdict | The sheet | With |
   |---|---|---|---|---|
   | Jokes | the fine print, `BUFFET.md`'s rotating footer, which its appendix files under Jokes; read at every view's foot (`FRONT.1`) | renamed: a fund's workbook keeps no sheet of jokes, and keeps its fine print as its disclosures, where these jokes sit deadpan | `Disclosures` | `MENU.1` |
   | Exam | the bank: a question, four options, the answer and the why; read by the Exam view (`GUARD.1`) | renamed: it cannot sit beside the Exam view | `Questions` | `MENU.2` |
   | Headlines | the ticker's headline of the day; read by the Overview (`FRONT.3`) | kept: a terminal's own word | `Headlines` | `FRONT.3` |
   | Memos | Mildred's memos; read by the audit and the events (`GUARD.3`, `MENU.4`) | kept: an office's own word | `Memos` | the first of `MENU.4` and `GUARD.3` |
   | Names | the firms' four, by archetype (`BACK.2`); later the rivals and every vehicle (`MENU.3`) | renamed: "Names" is Excel's word for its defined names, which is what a spreadsheet's user reads in it first, and it would hold several kinds; a rung's name belongs on the rung's own row of the Stocks sheet | `Firms` | `BACK.2` |
   | Achievements | the achievements and their triggers; read by the Overview (`MENU.7`) | renamed: a game's word, where a fund wins awards | `Awards` | `MENU.7` |
   | Errors | an error value's joke, where the error lands (`PLATE.6`) | kept: plain, and what they are | `Errors` | `PLATE.6` |
   | Endings | the endings (`FRONT.7`) | renamed: a fund ends by its exit | `Exits` | `FRONT.7` |

   Each is bare by `refers.rs`'s rule and under 31 characters, and none is
   a view's name as `FRONT.1` lists them (Portfolio, Market, Overview, Exam,
   Work), a model's sheet's (Day, Dice, Market, Stocks) or a device's.
   `Disclosures` alone is built; the others are this scoping's
   recommendations to their dishes, each dish's scoping free to amend its
   own. The headers over a sheet's columns are a fund's words too
   (decision 4).

3. **Hidden, after the model's sheets (rec).** A content sheet is the
   workbook's reference data, which a fund's workbook keeps hidden at the
   back: the strip shows the views and the model, and the strings are a
   click away under the strip's box, so nothing is locked (`AD-6`), and the
   exam's answers, later, are not the first thing a player stumbles on.
   Hidden, a content sheet never opens the game under `ENGINE.18`'s rule.
   The library's `strings` writes the sheet's `(sheet "<name>" hidden)`
   row, so the rule is written once, and the sheet stands among the tabs
   where the source's include of it stands: after the `(sheets ...)` row,
   which names every view and model's sheet, so after all of them and
   before the devices. *The alternative:* visible, after the model's sheets
   as the model's own are, `FRONT.1`'s reading of `AD-6`.

4. **A content file holds words alone, one call of the library's `strings`
   (rec).** `src/content/disclosures.vla`, `CC-BY-SA-4.0` by its header,
   holds one form, `(strings "Disclosures" "disclosure" ...)`: the sheet,
   the word for a row of it, which heads its texts' column, then the rows,
   a row a line, each `(<vintage> "<approved by>" "<text>")`, under a
   comment naming the three columns, the two short fields first so that
   they stand in columns and the text runs to the line's end. No
   `defmacro`, no include and no formula stands in a content file (decision
   9 holds it): the code that writes the rows is the library's,
   `AGPL-3.0-or-later`, and what a sheet's rows mean to the game is the
   source's, so the line between the words and the code is the line between
   the two licenses. The library gains two verbs at their first writing,
   where the ratchet waits for a second, since a content file can hold no
   code; the second sheet is their second writing:
   - `strings`, the sheet: its `(sheet ... hidden)` row; column A 100
     characters wide and C 20; the headers in row 1, the word for a row,
     `vintage` and `approved by`; a row of the call a row of the sheet from
     row 2, the text in A, the vintage in B and the approver in C; and beside
     the table, labels in E and values in F: `rows` in `F1`, the count the
     walk made, a value; `the row shown` in `F2`, which the dish that reads
     the sheet writes; and `shown` in `F3`, the picker. A walk over the
     call's rows, its list handed on whole, two links of the chain a row.
   - `picker`, `MISE.3`'s walker with the sheet's name taken out of its
     references: `=CHOOSE(F2,A2,A3,...)` in one cell, a reference a row
     joined a step under `unquote`. `strings` calls it over `A2` to the last
     row, and a dish may call it over another column of the same rows, as
     `MENU.2`'s question bank will, a picker a field.

   So a row added is a line of the call: the row's three cells, the count
   and the picker move, and no formula is edited (the oracle holds it,
   decision 11). A view reads `Disclosures!F3` and never holds the text, as
   `MISE.3`'s approved spelling has it. A quote inside a text is a single
   quote, since a double one is written `\"`. *The alternatives:* the text
   first and its two fields after it, where a stranger's eye finds them at
   a ragged line's end; or a picker the source writes over a count the
   source states, an edit beside every row added.

5. **The picker: one cell to 1,547 rows, and the pages of 500 when a sheet
   nears it (rec).** On this layout one picker holds 1,547 rows, one fewer
   than `MISE.3`'s 1,548, since row 1 holds the headers (*derived*:
   `CHOOSE(F2,A2,...,A1548)` is 8,188 characters past its `=`, and one row
   more 8,194, past the 8,192). No sheet nears it for courses: the question
   bank aims at 40 to 60 rows, the fine print at dozens. So the walk writes
   the one cell, a test holds the limit, and the pages of 500, the spelling
   the owner approved, are written by the dish whose sheet first passes
   1,000 rows, which a tripwire in the oracle names with this decision
   (`OMAKASE.md`'s rule, that an instrument with no specimen waits for the
   course that has one). *The alternative:* the pages now, one cell to
   1,547 rows and pages of 500 past it, a walk that counts by `mod-expand`
   and a test of 2,000 rows as `MISE.3`'s was: some twenty lines of the
   library more, which no row of course 0 runs.

6. **The fine print: a disclosure a month, from the day (rec).** The source
   writes the Disclosures sheet's index beside its include: `days a
   disclosure stands` in `E5` and 30 in `F5`, a value, the teacher's dial as
   `Day!B1` is; and `F2`, `=MOD(INT(Day!$B$3/F5),F1)+1`, the month of the
   day round the rows, row 1 on the first day. It reads the day and never
   the frame or the phase (`BD-7`), so it computes on a day's first frame
   alone, and the picker only when its index moves, once in thirty days. A
   month is thirty seconds at `(rate 30)`: long enough to read, and twelve a
   year, so that each of eight is seen once or twice in the first year. The
   cell stays on its sheet until `FRONT.1`'s views read it at their foot.
   *The alternatives:* a week, 7, every disclosure in the first minute and
   again in every minute after; a day, 1, a new one a second, too fast to
   read.

7. **The rows: the appendix's eight fine prints, two rewritten (rec).**
   Checked against `BD-2`'s amendment and the policy's four rules:

   | The seed (`BUFFET.md`, Appendix A) | The verdict | The row |
   |---|---|---|
   | "Past performance is not indicative of future results, but it is all we have." | kept | as written |
   | "Buffet is not a financial advisor. Buffet is barely a game." | kept: the title is the one name allowed, and the joke is on ourselves | as written |
   | "All investments carry risk. Some carry a lunch tray." | rewritten: a lunch tray is the buffet's | "All investments carry risk. Some also carry a load." (a load, a fund's sales charge) |
   | "Securities are not FDIC insured. Neither are your feelings." | kept | as written |
   | "Diversification does not ensure a profit; it ensures you will be wrong in several places at once." | kept | as written |
   | "Forward-looking statements are statements of hope wearing a tie." | kept | as written |
   | "This is not a recommendation to buy or sell. It is barely a recommendation to breathe." | kept | as written |
   | "Your capital is at risk. So is the office microwave." | rewritten: the break room's appliance, a kitchen's, and no financial joke | "Your capital is at risk. Our fees are not." |

   The rest of the appendix was read the same day, for the dishes that will
   seed from it: the headlines, the renames, the error values, the rivals,
   the endings and all the memos but one hold no kitchen word (a soybean is
   a commodity there and ice cream a business; the rivals and the renames
   keep their `(check)` for a living mark). Two more fall: Mildred's
   "Reminder: the break-room refrigerator is not an asset class.", the
   kitchen's flavor again, for which "Reminder: petty cash is not a
   liquidity facility." is offered to `MENU.4`; and the achievement "Greedy
   When Others Were Fearful", no kitchen's but the investor's own
   quotation, which `BUFFET.md` §1 keeps out of the game, for `MENU.7` to
   rename. `BUFFET.md` stays the brief: one dated line under its appendix's
   heading names what fell and where the rows stand. The eight rows'
   approver is the owner, who approves them by approving this scoping,
   under the GitHub name that commits here, `prestoncolejohnson` (rec; the
   owner's to name).

8. **The stranger's five minutes (rec).** `src/content/README.md`, "How to
   add a row", one page, where GitHub shows it beneath the folder's files:
   open the sheet's file and press edit; copy the row above yours onto a
   line below it; write your text, keep the vintage, and leave the approver
   `""`; keep the voice (a fund's compliance department, deadpan; the humor
   financial and economic; punch up; fictional names; PG-13; no kitchen
   word); a single quote inside a text; commit signed off, and open the
   pull request. Then, plainly: one check stays red until the cartridge is
   expanded again, which the reviewer does with the approval, writing their
   name in the row's `approved by`; and, for a person with Rust, the two
   commands that do both at home. GitHub signs off a browser's commit by
   itself once the repository requires it, a setting of the owner's
   (`MENU.6`'s path). The vintage is the course the row enters, 0 until
   course 0 is served, and a number, so that `MISE.1`'s second slice can
   keep a frozen course's bytes when a later course adds rows to a live
   content file, either by keeping each course's rows to its vintage or by
   copying the content into the frozen folder, which is that slice's
   decision. No check holds the approver filled, so a stranger's checks go
   green once the cartridge is expanded. *The alternatives:* a check that
   every row has its approver, red until approval, a sign-off a newcomer
   reads as a failure; a bot that expands a content pull request, an action
   writing to strangers' branches.

9. **The layout check's content rule (rec).** `tools/check_layout.ps1`
   holds:
   - `src/content/` holds content sheets, each `<name>.vla` named in
     lower-case letters, digits and hyphens, the block id the page needs
     (decision 10), and its `README.md`; anything else is stray.
   - The content sheets' pin raised from 0 to 1.
   - The source includes `common.vla`, then `kitchen.vla`, then each
     content sheet once as `content/<name>.vla`, each by a line of its own;
     an include of a content sheet that is not there fails, and so does a
     content sheet the source does not include.
   - A content sheet includes nothing and defines no macro (decision 4).

   `-Control`: the clean copy, six files now, and eleven mutants: the
   seven, the content sheet's become a second sheet past the pin, and four
   more, a content sheet the source does not include, one that includes,
   one that defines a macro, and one with a capital letter in its name.

10. **The page: a content sheet a library block of its own (rec).**
    `tools/build_web.ps1` writes, after the top level's libraries, each
    `.vla` of `src/content/` as `library-content-<name>` at
    `src/content/<name>.vla`, so that Build in the page's drawer hands the
    pass the text the source's include resolves to and still writes the
    cartridge byte for byte, as `HOUSE.4`'s decision 4 foresaw; the id's
    `content-` keeps a sheet from colliding with a library of its name.
    `tools/check_web_offline.ps1`'s `Get-OwedBlocks` owes the same blocks,
    both ways, and its control's fourteen mutants become fifteen, a page
    whose content block is cut. The drawer's library box then lists
    `src/content/disclosures.vla`, where a row can be tried and built in the
    page. *The alternative:* `library-<name>`, a content sheet free to
    collide with a library by name.

11. **The oracle (rec).** In `oracle/`:
    - The hand, `buffet.rs`: the days a disclosure stands, 30, and the row
      shown on a day, the day over 30 round the rows, from 1; never the
      texts, which are the content's, written once.
    - At every frame of two years and two days and of 800 days at one
      frame a day: the Disclosures sheet's `F2` the hand's row and `F3`
      that row's text, read off the sheet's own column A.
    - The count: `F1` the number of rows the sheet holds, by its extent.
    - Every index: the days a disclosure stands written 1, at one frame a
      day, eight days pick the eight rows in turn.
    - A row added: the source expanded with one row more in the content
      file's call, handed to the pass as that file's text, differs from the
      cartridge in the new row's three cells, `F1` and `F3` alone, loads,
      and picks the new row at its index.
    - The limit: a scratch sheet of `strings` at 1,547 rows loads and at
      1,548 is refused, `calc-formula-too-long`; and the tripwire, every
      content sheet under 1,000 rows, failing with decision 5's name.
    - Every sheet bare: each sheet of the cartridge its own spelling under
      `refers.rs`'s quote rule, no two the same without case, and every
      content sheet hidden.
    - Kept: the cartridge its source expanded; `BD-7`'s scan and the
      census's `(every-frame 0)`, which hold the two new formulas; the
      describe golden, blessed again and read in its diff.

12. **The words (rec).** `CONTRIBUTING.md`'s content policy gains the
    amendment as its fifth rule, *A fund manager's words*: the game looks
    and feels like a fund manager's workbook, its humor financial and
    economic, and no kitchen word reaches it, the kitchen naming the work in
    the developers' documents alone; the title, "Buffet", and the tagline,
    "All You Can Sheet", are the only exceptions. Its voice paragraph names
    a fund's compliance department as the voice, its promise of the *How to
    add a row* becomes a link to it, and its names' paragraph spells the
    tagline as the README's title does. And the game's notes are rewritten
    now, a string of the game being this family's and `BD-2` in force:
    "Buffet: All You Can Sheet. The back office is open and the front office
    is not: so far ...", the rest as it stands, the fine print added, "and
    the fine print, a disclosure a month", and the Disclosures sheet named
    among the hidden ones. `FRONT.1` then finds its notes done. *The
    alternative:* the notes left to `FRONT.1`, which waits on Alonzo.
    `NOTICE` and `TRADEMARK.md` keep the tagline in lower case: legal texts,
    the owner's and counsel's (`HOUSE.6`).

13. **The hand test, the owner's, on the page built here:**
    - Buffet run at full screen; the strip's box ticked, "hidden sheets";
      the Disclosures sheet opened, its eight rows with their vintage and
      approver, and `F3`, `shown`, the first disclosure;
    - `F3` turning to the second on day 30, at frame 901, thirty seconds
      in;
    - the drawer's library box listing `src/content/disclosures.vla`, and
      Build writing the cartridge byte for byte, compared with
      `src/buffet.vla` by a command given at the handover.

14. **The stale places, rewritten at the build.** `OMAKASE.md`'s `MENU.1`
    line (the one sheet, the names, the pages' trigger), its open decision
    on the sheet names, and `MISE.6`'s hoist table (`strings` and `picker`
    into `src/kitchen.vla`, no cartridge's bytes yet to stand); the source's
    opening comment, which lists the dishes served. `MISE.1`'s line in this
    file names the verb `picker` at `MENU.1`, and stands. And in the same
    commit, as `POT.2`'s P2 rode in `STATION.1`'s, the Linux cells CI's run
    38712895205 answered: `STATION.1`'s P2 and P4, and `HOUSE.4`'s P2 and
    P6 with its closing note.

**The predictions, before any number.** The numbers column was filled at
the build, the same day, P9 at the owner's hand test.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 169 rows where 131 stood: the sheet's row, two widths, three headers, three labels, eight rows of three cells, the count, the picker, and the source's label, setting and index; 81 formula cells where 79 | *Measurement*, as predicted: the runner printed `(rows 169) (loads yes)`, the rows in the order listed, 81 formula cells |
| P2 | the describe golden | moved in the notes; the budget's formulas, 79 to 81 and 2,370 to 2,430 a second; the plan's shapes, texts, formulas and alone, 2 more each, its levels, runs, lanes, every-frame and cycles unmoved; the Day sheet's readers one more; a readers row and two shapes for the Disclosures sheet; nothing else | *Measurement*, as predicted: the notes; `(budget 30 81 2430 2000000)`; shapes 30, texts 28, formulas 81 and alone 54, the levels 14, runs 6, lanes 27, every-frame 0 and cycles 0 unmoved; `(readers "Day" 14)`; `(readers "Disclosures" 11)` and two shapes, the row shown at level 1 and the picker at level 2; nothing else |
| P3 | the pass and the load | the pass under a millisecond longer; the load under 0.1 ms longer, a picker of eight references where `MISE.3`'s 552 cost 2.3 to 2.5 ms | *Measurement*, a scratch runner of the pass and the load, three runs of 200 each, opted out of Windows' throttling and pinned to the performance cores: the pass 1.45 to 1.51 ms at p50 where it took 1.41 to 1.45, 0.02 to 0.09 more; the load 0.58 to 0.59 ms where it took 0.48 to 0.51, 0.08 to 0.10 more. Inside, the load by a hair |
| P4 | the step | an ordinary frame unmoved; a day's first frame computing the index and, once in thirty days, the picker, under a microsecond more | *Measurement*, the same runs, two years at thirty frames a day: an ordinary frame 1.8 µs at p50 before and after; a day's first frame on which the fine print stays 11.8 µs where it took 11.6 to 11.8; a day's first frame on which it moves, day 30 and every thirtieth after, 12.5 to 12.6 µs where the same frames took 11.6 to 11.9, 0.6 to 0.9 µs more. Inside |
| P5 | the limit | 1,547 rows load and 1,548 are refused, `calc-formula-too-long` at the picker's line; the pass over 1,548 rows under 50 ms | *Measurement*, as derived: 1,547 rows load, the picker 8,188 characters past its `=`; 1,548 are refused, `calc-formula-too-long` at the picker's line, its 8,194 characters; the pass over 1,548 rows 20.7 to 21.0 ms at p50. Inside, and the test holds the limit on every run |
| P6 | the suite | 33 tests green here, five of the cartridge and one of the hand more, and in CI on Linux; the suite under a second longer than `bea77bc`'s, alternated against it in a worktree | *Measurement*, here: 33 green, 14 of the crate and 19 of the cartridge. The cartridge's binary run alternately with `bea77bc`'s from a worktree, five rounds, each opted out and pinned, on eight threads: 3.93 to 4.76 s where it took 3.63 to 4.37, the medians 4.27 and 3.85 s; the rounds 0.13, 0.17, 0.25 and 0.30 s apart, and the first 1.09. Inside at the median, one round of five past it. Linux at the push |
| P7 | the page | one block more, `library-content-disclosures`, and the page about 5,000 characters longer, near 1,590,000; Build in the drawer writing the cartridge byte for byte | *Measurement*: 1,594,601 characters, 9,587 more, **missed** by nearly double: the prediction forgot the library's own block, which grew 4,217 characters with its two verbs and their comments. The cartridge's block grew 2,031, the source's 1,652, and the new block holds 1,578 in a tag of 109. Written in 0.34 s. Build in the drawer at the owner's hand test, P9 |
| P8 | the checks | 4 checks and 4 controls green; the layout check's control eleven mutants and the offline check's fifteen, each failing by name | *Measurement*, as predicted: ALL GREEN, 4 checks and 4 controls; the layout check's eleven mutants and the offline check's fifteen each failing for its own reason. `check_spdx` reads the tracked tree, and the two new files were held through a scratch index that tracked them: 39 files resolved, 6 headers read under `src/` |
| P9 | the hand test | the box counting ten hidden sheets, the nine twins and Disclosures; `F3` the first disclosure from frame 1 and the second from frame 901 | *Measurement*, the owner's screenshot and comparison, in the owner's Chrome at full screen: the box read "hidden sheets (10)", and the Disclosures sheet stood between Stocks and Clock, the twins after Input; its eight rows with their vintage and approver, `F1` 8 and `F5` 30; at frame 2,845, day 94, `F2` 4 and `F3` "Securities are not FDIC insured. Neither are your feelings.", the hand's row for the day, the round three months on; and Build in the drawer writing the cartridge's 7,638 LF characters, "same bytes". Inside. The first turn, at frame 901, is in no screenshot, and the fourth row on day 94 says the round turned three times; Build, which refuses a source whose include it cannot find, says the content's block was in the page |

**What the build found.** The new tests catch what they stand for, and not
only through the cartridge's expansion test: three mutants of the cartridge,
the days a disclosure stands written 7, the picker one reference short, and
the row shown read off the frame, failed five, six and seven tests, among
them the fine print held at every frame, every index in its turn, and
`BD-7`'s scan. A row whose approver is left empty, `""`, as a stranger
leaves it, expands and loads, so the *How to add a row* holds. The tab order
the record prints is the one decision 3 predicted: Screen, Day, Dice,
Market and Stocks, visible; Disclosures, hidden; Clock, File and Input; then
the nine twins, hidden, each in its sheet's order. And `check_spdx.ps1`
reads the tracked tree alone, so a content file is held to its header from
its first `git add` and not before: a stranger's file is tracked in their
pull request, and only a working copy goes unread.

**Amended at the owner's hand test, 2026-10-15.** Three changes, each the
owner's. First, the rows: the owner rewrote seven of the eight fine prints
in their own words, the second alone kept, so that the content file holds
the owner's rows and not the appendix's. Second, the includes: "I would
argue that all (include ...) forms should be grouped together at the start
of the file like most other programming languages so a missed/failed import
throws an error at the top of the file." The pass already refuses a missing
include while it splices, before anything expands
(`vla-include-cannot-read`, wherever the include stands), so what the
grouping moves is the refusal's line, to the top, and every dependency into
one place. It met one fact of the pass: an include is spliced where it
stands, and the first form that is no include and no macro is the manifest,
so a file included among the source's first forms may define macros alone.
So a content file defines its rows and the source uses them, as an import
defines and a body uses in most languages: each file is one macro named
after it, `(defmacro (disclosures) ...)`, whose body is the one call of
`strings`, and the source calls `(disclosures)` where the include had
stood, so that the sheet keeps its place among the tabs. Third, the order:
"I think the (cartridge ...) form should always be the first form, since
it's the cartridge's metaform, followed by (include ...) forms, then
whatever cartridge code comes after includes." The pass reads that order
as it reads the other, so the source opens on its manifest, its three
includes follow it, and its code follows them, as do the two sources the
oracle's tests write; Alonzo's eight sources, which open on their include,
take the same order in a lint pass of Alonzo's own, at the owner's word.
Decision 4's "no `defmacro`" became "one macro named after the file, and
nothing else", and decision 9's rule followed: the manifest the source's
first form, then every include, then the code; a content sheet one form, a
macro of its file's name, its body one call of `strings`; and the source
calling it once. The layout check's control grew from eleven mutants to
fifteen, a second macro, a macro bearing another file's name, the source's
call gone, an include among the code and an include before the manifest,
each failing for its own reason; a first cut of two of them refused to
apply until a pipeline that handed `Get-LfLines`' array on whole was split.
The cartridge expanded from the amended files differs from the one the
owner tested in the seven rows the owner wrote and nowhere else, 169 rows
still, the describe golden unmoved and the 33 tests green; the page, built
again, holds 1,595,679 characters.

## `MENU.2`, first slice — the exam bank: twenty questions and their whys

*Scoped 2026-10-15 at the owner's word, "Scope and implement MENU.2's first
slice in Buffet (Delta/Buffet): the exam bank, twenty questions and their
whys, the game's second content sheet", to stop for the owner's approval
before building: from `OMAKASE.md`'s line (`COURSE.0`'s item 13), the draft
bank in `BUFFET.md` §3, `COURSE.0`'s opening contract, and `MENU.1`'s
scoping, build record and amendments, which set the shape a content sheet
takes. It comes now because Alonzo is building `ENGINE.19` and `ENGINE.20`,
so `ENGINE.18`'s views, which `FRONT.1` and every dish with a button wait
on, are unreleased, and this item needs nothing of Alonzo's; and because it
is `strings`' second writing, the ratchet's moment to shape a sheet of
several fields before another sheet copies the one-text shape. Each
decision carries its recommendation, for the owner to approve, amend or
strike; the rows and the whys are drafts, for the owner to approve or
rewrite, as the owner rewrote `MENU.1`'s. The predictions were written
before any number was taken. Approved whole by the owner the same day,
"Recommendation approved; proceed with implementing MENU.2 as scoped", and
built that day. Alonzo's tree, where a peer builds `ENGINE.19`, was read and
never touched.*

**What it stands on, read on the day** (`vla-lang` 0.9.3 and `alonzo`
0.1.3 as the registry holds them; `web/` as vendored at `v0.1.3`; Buffet at
`3dd3eeb`).
- `MENU.1`'s shape, as amended at its hand test: a content file is one
  macro named after it, whose body is one call of `strings`; the source
  includes it after the manifest, with every include, and calls the macro
  where the sheet belongs. `check_layout.ps1` holds both, its content pin
  at 1, its control fifteen mutants over a clean copy of six files.
- `strings` writes one field: the text in A, the vintage in B, the approver
  in C; A 100 characters wide and C 20; the headers in row 1; labels in E
  beside values in F: `rows` and the count, `F1`; `the row shown` and the
  index the source writes, `F2`; `shown` and the picker, `F3`. Its rows
  come in this order: the sheet's, A's width, C's, the three headers, the
  three labels, each row's A, B and C, `F1`, `F3`. `picker` takes the
  column it picks, so it serves a field each unchanged, as `MENU.1`'s
  decision 4 foresaw.
- The expander's list primitives are `car`, `cdr`, `cddr`, `cons`, `list`,
  `null?`, `eq?`, `equal?`, `quote-if` and `cond`, beside the arithmetic,
  the comparisons, `&expand` and the address primitives (`expand.rs`). None
  counts a list and none tells a list from a word, so one parameter cannot
  take either; a count is a walk that hands its number on to the macro that
  uses it, as the kit's `first-cell` hands on a text.
- The dialect computes fourteen functions, `IF`, `AND`, `OR`, `NOT`, `SUM`,
  `MIN`, `MAX`, `ABS`, `INT`, `MOD`, `ROW`, `COLUMN`, `CHOOSE` and `SIN`
  (`calc/library.rs`): no `GCD`, `MATCH`, `INDEX`, `CODE` or `LEN`. It
  compares texts with their case folded, as Excel does (`calc/eval.rs`), so
  "c" equals "C".
- The language's reader is public, `vla_lang::reader::read_forms`, a number
  read as a symbol and a text as a string, so a test can read a content
  file's rows without the library's walk.
- `describe` counts a sheet's readers as its shapes' references to it,
  Disclosures' 11 being `F2`'s two and `F3`'s nine, and from sixteen it
  indexes them by where a change stands, `(most k)` beside the count
  (`calc/census.rs`, `calc/reach.rs`).
- The page writes each `.vla` of `src/content/` as a block,
  `library-content-<name>`, and the offline check owes each by the same
  rule (`build_web.ps1`; `check_web_offline.ps1`, `Get-OwedBlocks`), so a
  second sheet asks nothing of either. The player's drawer holds "An edit,
  one row:" and its Write, where a hand test writes a cell.
- The exam's die and its two draws stand on the Dice sheet, `B7:C9`, a
  function of the seed alone, read by nothing: `POT.1`'s decision 8 kept
  them for `GUARD.1`'s start and stride.
- The draft bank holds nineteen graded questions and a twentieth,
  ungraded, which sets the archetype. The nineteen answer B twelve times, A
  six, C once and D never. Four of its texts hold a quotation in double
  quotes, which a VLA string writes `\"`, where `MENU.1`'s *How to add a
  row* asks for single ones; and it writes "pavement" and "films" where the
  house writes American English (the owner's word of 2026-10-14).
- `ENGINE.18`, committed in Alonzo's tree (`199789c`) and unreleased: a
  grid cartridge opens on the sheet `Camera!B3` names, else on its first
  visible sheet, so a hidden content sheet never opens the game.

**The decisions.**

1. **The fields: the concept, the question, four options, the answer and
   the why (rec).** In the order the draft writes a question: its concept
   in bold, the question, the options, the answer marked. The concept is
   the idea the question tests, *Opportunity cost*: the title of the
   concept card that later quotes the why (`BUFFET.md` §10, `FRONT.5`) and
   the glossary's term, written now since a field added later is an edit
   of every row. The Exam view never shows it before the question is
   answered, since a concept can name its answer: Q14's is its right
   option's own words. The headers are a reference sheet's plain words,
   `concept`, `question`, `option A` to `option D`, `answer` and `why`, and
   not `A` to `D`, which would stand over the columns C to F and read as
   their letters. Then the vintage and the approver, as on every content
   sheet.

2. **The answer a letter, A to D (rec).** The question reads that way: its
   options lettered in the headers and on the Exam view, its answer marked
   by letter in the draft, its proctor speaking in letters ("Also B"); so a
   reviewer checks a row at a glance, and a stranger writes the letter they
   would circle. It costs `GUARD.1` a comparison where a click becomes a
   letter, `CHOOSE(k,"A","B","C","D")`, or each option cell held to its own
   letter; and four `IF`s where a formula wants the right option's text,
   the dialect holding no `MATCH` or `CODE`. The case is folded, so "c"
   would score, and the oracle holds every answer a capital (decision 12).
   *The alternative:* the option's place, 1 to 4, which a click's row gives
   by arithmetic and `CHOOSE(answer,...)` turns into the right option's
   text, where a reviewer counts.

3. **The answers spread over the letters, five each (rec).** A player who
   always answered B would score two in three of the draft's nineteen and
   learn the pattern before the concepts. So the right option moves among
   the four, each question's other options keeping the order in which
   their jokes build, and the twenty answer A, B, C and D five times each
   (decision 7). The draw takes nine, so the letters a player meets are the
   draw's; the bank's spread is the reviewer's to keep as rows arrive, and
   the oracle holds only that no letter answers more than a third of the
   bank, which the draft's twelve B would fail and one stranger's row will
   not (decision 12).

4. **The twentieth: off the bank, a question of its own kind, and a
   twentieth graded question in its place (rec).** "Which best describes
   you?" is not the bank's kind of string. It has no right answer and no
   why; it is asked last and always, never drawn; it belongs to the Series
   777 alone, where every license's exam after it draws from a bank
   (`GUARD.2`); and its four options are the four archetypes `BACK.2` names
   the firm by. So it goes where the archetypes are, recommended to the
   dishes that read it: its options a column of the Firms sheet (`MENU.1`'s
   name for `BACK.2`'s), each beside the firm it picks, and its prompt the
   Exam view's, arriving with the first dish that asks it, `GUARD.1`, ahead
   of `BACK.2`, which names the firm; their scopings settle it, and the
   draft's text waits in `BUFFET.md` §3. The bank then holds graded
   questions alone: every row the draw's, a row added joining the draw, the
   count the bank's size. The contract squares in one sentence: nine scored
   questions drawn from a bank of twenty by a seeded start and a stride
   coprime to the bank's size, so no question repeats, then a tenth,
   ungraded, which sets the archetype. The twentieth graded question is
   written new, *Shares*: what a share of stock is, the first thing course
   0's player buys and the bond question's missing pair. *The
   alternatives:* the archetype a row of this sheet, its answer empty, kept
   at row 2 so that rows added after it extend the drawn range, the draw
   over the rows below it and the count less one: two kinds on one sheet,
   and a stranger's row put above it drawn silently as the archetype; or
   the draft's nineteen alone, the contract's twenty amended to nineteen.

5. **What reads the sheet: a row shown the source writes, 1, until
   `GUARD.1`'s draw drives it (rec).** The draw is `GUARD.1`'s line and its
   oracle, no repeats over every seed of a thousand; and where the exam's
   state stands, the draw, the question asked, the score and the
   archetype, is `GUARD.1`'s to lay out with its view. Written now, the
   draw would be half a mechanism waiting on a shape its other half
   decides, and nothing the player sees would read it either way. So the
   source writes `M2`, the row shown, a value, 1: the pickers show the
   first question, and the oracle writes the cell to walk every row. What
   this slice hands `GUARD.1` instead is a finding: with no `GCD` in the
   dialect, a stride coprime to the bank's size has no test, but a prime
   larger than the bank's size is coprime to it, so a stride picked by the
   exam's second draw among the primes above 1,547, the most rows one
   picker holds (1,549, 1,553, 1,559 and on), is coprime to every bank a
   picker can hold, with nothing to test; the start, the first draw modulo
   the bank's size. *The alternative:* the draw now, beside the table, the
   start, the stride and the nine rows drawn, the row shown the first of
   them, the hand holding the draw over a thousand seeds: some fifteen rows
   and a test, `GUARD.1`'s oracle moved forward and no view reading it.

6. **`strings` of several fields, Disclosures its case of one (rec).** The
   call names the sheet, then its fields as a list, each a header and a
   width in characters, then the rows: `(strings "Questions" ("concept" 20
   "question" 130 ...) rows ...)`. Disclosures' file changes in one line,
   `"disclosure"` becoming `("disclosure" 100)`, its rows untouched, and
   its cartridge rows stand byte for byte, which is this decision's proof.
   The verb counts the fields, a walk that hands the count on, then writes
   in the order Disclosures' rows already take: the sheet's hidden row;
   each field's width at its column, then the approver's 20, the vintage's
   column left at the default; the headers, the fields' then `vintage` and
   `approved by`; beside the table, past one empty column, labels in the
   column the fields' count plus four and values in the next: `rows`, `the
   row shown` and `shown` as today, then each later field's header down
   the column; the rows, each its fields from column A, then its vintage
   and its approver; the count; and a picker a field, the first beside
   `shown`, each later one beside its header. The first field's picker
   keeps Disclosures' label and each later one is labeled by its field: one
   rule with a first step, the price of Disclosures' bytes. For
   Disclosures, one field: the same rows in the same order, `E:F` and
   `F3`. For Questions, eight: the table A to J, labels in L and values in
   M, `M1` the count, `M2` the row shown, `M3` to `M10` the pickers,
   `shown` beside the concept's. `picker` stands unchanged. A row added is
   still a row of the call: its ten cells, the count and the eight pickers
   move, and no formula is edited. Each field is as wide as its longest
   drafted text, the concept 20, the question 130, each option 62, the
   answer 7 and the why 140, so the sheet runs some 4,000 pixels across and
   every row reads whole at the hand test. *The alternatives:* a second
   verb for several fields beside `strings`, two shapes for a stranger to
   learn and the generalization the ratchet asks for not made; or the
   pickers along row 3, `M3` to `T3`, in the table's own order beside the
   one label `shown`, no first step, and no telling one field's picker from
   the next.

7. **The rows: the draft's nineteen, checked, and the twentieth (rec).** A
   row is a block in the file: its vintage and its approver on its first
   line, then a field a line in the order the fields name them, since a
   question's row runs near 400 characters and a field a line diffs a
   changed option as one line. The fields' list above the rows is written a
   field a line with its comment, the owner's rule for a call of many
   arguments (2026-10-08), and the rows bare beneath it. Each row was
   checked against `BD-2` (no kitchen word, the humor financial or
   economic, punch up, fictional names, PG-13, clarity first), for accuracy
   (`BD-5`, **(check)** where a finance educator confirms before the
   release), and for the house's American English; a quote inside a text
   is a single quote. Every row's approver is the owner,
   `prestoncolejohnson`, by approving this scoping, and its vintage 0.

   | | The concept and the question | The options, the right one ✔ | The verdict |
   |---|---|---|---|
   | 1 | *Opportunity cost.* You can read the prospectus or nap. The opportunity cost of the nap is: | (A) nothing, naps are free; (B) the prospectus, and probably a lawsuit; (C) the value of the best thing you gave up ✔; (D) $4.99 a month, billed annually | kept as written; the proctor's "Correct. Also B." waits with the remarks (decision 9) |
   | 2 | *Supply and demand.* It rains and umbrella prices rise. This is: | (A) a conspiracy by Big Umbrella; (B) demand rising against a fixed supply ✔; (C) a weather-adjusted vibe; (D) illegal in at least one state | kept as written |
   | 3 | *Inflation.* Your cash earns 0% while prices rise 3% a year. After a year it buys: | (A) the same, a dollar is a dollar; (B) nothing, it has gone to a better place; (C) about 3% less ✔; (D) more, if you believe in it | the right one moved to C; accurate, a dollar buying 1/1.03 of what it did, 2.9% less |
   | 4 | *The rule of 72.* At 8% a year, money roughly doubles in: | (A) 9 years ✔; (B) 8 years, because the number is 8; (C) never, money only multiplies in movies; (D) one quarter, according to the brochure | "films" made "movies"; D rewritten, since "until the next quarterly report" does not complete "doubles in"; accurate, the exact doubling 9.006 years |
   | 5 | *Risk and return.* On average, the higher the expected return, the: | (A) higher the risk, which is what you are being paid for ✔; (B) lower the risk, obviously; (C) better the logo; (D) closer your brother-in-law is to being right | the right one moved to A |
   | 6 | *Diversification.* 'Don't put all your eggs in one basket' is advice about: | (A) poultry; (B) baskets, mostly; (C) spreading risk without necessarily lowering expected return ✔; (D) eggs, specifically | the right one moved to C; for the owner's eye: the eggs are the proverb's, investing's own phrase for diversification, and no kitchen's, but they are food |
   | 7 | *Bonds.* A bond is: | (A) a lasting emotional connection; (B) a spy; (C) a stock with a sad backstory; (D) a loan you make, which pays interest and returns the principal ✔ | the right one moved to D |
   | 8 | *Bonds and rates.* When interest rates rise, existing bonds: | (A) fall in price, because new bonds pay more ✔; (B) rise, out of solidarity; (C) stay put, bonds are loyal; (D) are not discussed at parties | the right one moved to A |
   | 9 | *Liquidity.* Which is the most liquid? | (A) a private island; (B) a 1952 baseball card; (C) your cousin's startup; (D) cash ✔ | the right one moved to D, after the three that are not |
   | 10 | *Bear market.* A bear market is a fall of at least: | (A) 2%, and a deep sigh; (B) 20% from the peak ✔; (C) 100%, a figure usually reserved for your cousin's startup; (D) a bear's weight in honey | kept as written; 20% is a convention, and the why says so |
   | 11 | *Price to earnings.* A stock with a price-to-earnings ratio (P/E) of 20 costs, per dollar of yearly earnings: | (A) $0.05; (B) 20 hugs; (C) a bargain, according to the company; (D) $20 ✔ | the ratio spelled out for a first player; "a type of coffee" rewritten as C, a beverage's joke, the break room's, where `BD-2` wants a financial one; the right one moved to D |
   | 12 | *Moral hazard.* 'Too big to fail' encourages banks to: | (A) shrink; (B) take more risk, because someone else pays for the downside ✔; (C) take up jogging; (D) fail bigger, for the exposure | kept as written |
   | 13 | *Leverage.* You put in $1, borrow $9 and buy a $10 asset that falls 10%. Your equity is now: | (A) $9; (B) $10; (C) $0 ✔; (D) a learning experience | the right one moved to C; accurate, interest aside |
   | 14 | *Sunk cost.* You paid $50 for a terrible movie and stay 'because you paid'. This is: | (A) the sunk-cost fallacy ✔; (B) loyalty; (C) cinema; (D) a deductible | "film" made "movie"; the right one moved to A |
   | 15 | *Deposit insurance.* A US bank deposit is insured, per depositor, per insured bank, up to: | (A) $2.50; (B) $250,000 ✔; (C) $250 million; (D) a pinky promise | **(check)** $250,000, the FDIC's standard maximum per depositor, per insured bank, for each ownership category: a figure of law, to confirm at the release; "per insured bank" for "per bank", the FDIC's own words; the right one moved to B |
   | 16 | *Central banks.* A central bank raises rates to: | (A) make borrowing cheaper; (B) impress other central banks; (C) give the bond market something to talk about; (D) cool inflation by making borrowing more expensive ✔ | "dearer" made "more expensive", for an American classroom; "fund the pens" rewritten as C, a joke whose target no reader finds; the right one moved to D |
   | 17 | *Efficient markets.* Two economists see a $20 bill on the sidewalk. One says, 'If it were real, someone would have picked it up.' This is a joke about: | (A) the efficient-market hypothesis ✔; (B) littering; (C) sidewalks; (D) twenties | "pavement" made "sidewalk" in both places |
   | 18 | *Time value.* $100 today or $100 in a year? | (A) in a year, I like surprises; (B) both, with a lawyer; (C) today, it can earn something meanwhile ✔; (D) neither, cash is dead | the right one moved to C |
   | 19 | *Short selling.* You short a stock at $10 and it goes to $20. Your loss is: | (A) $0, you never owned it; (B) $10 a share, and it can grow ✔; (C) capped at $10; (D) spiritual | the right one moved to B; accurate, the borrowing's fee aside |
   | 20 | *Shares.* A share of stock is: | (A) a loan you make to the company; (B) a voucher for the company's products; (C) a stock photo, usually of a handshake; (D) a small piece of the company, with a claim on its profits ✔ | new (decision 4): A the bond's answer, the pair of Q7; C the word's own pun |

   No row names a real company, person or mark; the FDIC is a fact and no
   parody. A finding for the owner, and no decision: in most questions the
   right option is the one written straight among three jokes, so a careful
   reader answers all nine. Six hold a wrong answer a reader could believe,
   Q1's free nap, Q3's dollar that stays a dollar, Q11's earnings yield,
   Q13's asset taken for the equity, Q16's cheaper borrowing and Q19's
   capped loss; the rest teach by contrast. At $5,000 and $500 a correct
   answer, most first firms may open near $9,500. The playtest pass would
   show the spread, and the exam cannot be failed, by design.

8. **The whys: twenty lines, each standing alone (rec).** A concept card
   quotes a why apart from its question (`BUFFET.md` §10), so each reads
   whole without it and names no option or letter. Each is one sentence of
   at most 140 characters, the line the oracle holds, and says the concept
   plainly, its deadpan clause after it where one fits. Where none fits,
   `BD-2`'s reason is written here once: a why is the lesson, and clarity
   comes first.

   | | The concept | The why | Characters |
   |---|---|---|---|
   | 1 | Opportunity cost | Every choice costs the value of the best alternative you gave up, whether or not anyone sends an invoice. | 105 |
   | 2 | Supply and demand | When more people want a thing than there is of it, its price rises until enough of them stop wanting it at that price. | 118 |
   | 3 | Inflation | Rising prices shrink what idle cash buys: at 3% inflation, a dollar earning nothing buys about 3% less each year. | 113 |
   | 4 | The rule of 72 | Divide 72 by the yearly rate in percent for the years money takes to double: 72 over 8 is 9, give or take a few days. | 117 |
   | 5 | Risk and return | Investors demand a higher expected return for bearing more risk, so an offer of high return and low risk is usually missing one of the two. | 139 |
   | 6 | Diversification | Owning things that do not all fall at once can lower the risk of the whole without lowering its expected return, a rare bargain. | 128 |
   | 7 | Bonds | A bond is a loan: the borrower pays you interest on a schedule and repays the principal when it matures, barring the unforeseen. | 128 |
   | 8 | Bonds and rates | When new bonds pay more, an old bond's fixed interest is worth less, so its price falls until it yields what the new ones do. | 125 |
   | 9 | Liquidity | Liquidity is how quickly something turns into cash without a discount, and cash, conveniently, already is. | 106 |
   | 10 | Bear market | By convention, a fall of 20% from a peak is a bear market, a fall of 10% a correction, and a fall of 2% a Tuesday. | 114 |
   | 11 | Price to earnings | A price-to-earnings ratio of 20 means a share costs 20 times its yearly earnings: $20 for each dollar it earns in a year. | 121 |
   | 12 | Moral hazard | When someone else will cover the losses, risk-taking grows; economists call this moral hazard, and banks call it a business model. | 130 |
   | 13 | Leverage | Borrowing magnifies gains and losses alike: at ten times leverage, a 10% fall takes everything you put in. | 106 |
   | 14 | Sunk cost | Money already spent is gone whatever you do next, so it should have no say in what you do next. | 95 |
   | 15 | Deposit insurance **(check)**, as its question | The FDIC insures deposits up to $250,000 per depositor, per insured bank, for each ownership category; the cautious use more than one bank. | 139 |
   | 16 | Central banks | Higher rates make borrowing cost more, so people and firms borrow and spend less, and prices rise more slowly, eventually. | 122 |
   | 17 | Efficient markets | If prices already reflect what is known, beating the market takes knowing what others do not, which is rarer than it feels. | 123 |
   | 18 | Time value | A dollar today can be invested to become more than a dollar by next year, so a dollar promised later is worth less than one now. | 128 |
   | 19 | Short selling | A short seller sells borrowed shares and must buy them back at whatever the price becomes, and a price has no ceiling. | 118 |
   | 20 | Shares | A share is a small piece of a company: a claim on its profits and usually a vote at its meetings, though seldom a parking space. | 128 |

9. **The proctor's remarks: later, with the exam's flow (rec).** The draft
   has one, Q1's "Correct. Also B." The exam as `GUARD.1`'s line has it
   shows no answer on screen: a click asks the next question, and the whys
   wait for the concept cards (`FRONT.5`). A remark is feedback for a
   moment the exam does not have yet. The day `GUARD.1` decides the exam
   answers a question on screen, the proctor's lines by moment, the start,
   a right answer, a wrong one and the certificate, are a kind of their
   own, Gary's (`MENU.4`'s proctor who is never promoted), and a question's
   own remark, which names its own option as Q1's names B, a column of this
   sheet then. The cost is the same either way, a field added to twenty
   rows then, or nineteen empty ones carried from now and struck if never
   shown, so the lean way wins. Q1's remark waits in `BUFFET.md`'s draft,
   and decision 3 leaves Q1's options in their order, so its B is still the
   prospectus. *The alternatives:* a column now, Q1's remark and nineteen
   empty; a sheet of the proctor's lines now, read by nothing until
   `GUARD.1`.

10. **The source: the include, the call, the row shown and the notes
    (rec).** `(include "content/questions.vla")` after Disclosures'
    include; `(questions)` called after the Disclosures block, so the sheet
    stands after it among the tabs, both hidden, before the devices; and
    beside the call, the source's own, `M2`, the row shown, 1, with its
    comment. The game's notes, a player's string under `BD-2`, gain the
    bank: "...; the fine print, a disclosure a month; and the Series 777's
    question bank, twenty questions awaiting an exam the regulator has yet
    to schedule. Pick the Day, the Dice, the Market or the Stocks sheet to
    watch, or the Disclosures or the Questions sheet among the hidden
    sheets." The opening comment lists `MENU.2` among the dishes served.

11. **The layout check (rec).** The rule holds as it stands, since
    `questions.vla` is one macro named after its file whose body is one
    call of `strings`; the content pin rises from 1 to 2; the control's
    clean copy holds seven files, its mutant "a second content sheet past
    its pin" becomes a third, and its fifteen mutants stand.

12. **The oracle (rec).** In `oracle/tests/cartridge.rs`, the content
    sheets two, `Disclosures` and `Questions`:
    - Each content sheet is its file: the file read by the language's
      reader, its call's fields and rows taken from it, and every row a row
      of the sheet from row 2, its fields from column A and its vintage and
      approver after them; the headers its fields' then `vintage` and
      `approved by`; the count beside its label `rows`. A second reading of
      the words that shares no walk with the library, for both sheets.
    - The Questions sheet states its labels, down column L, and its row
      shown, 1.
    - Every question whole: each field a text and none empty; the four
      options distinct; the answer `A`, `B`, `C` or `D`; the why at most
      140 characters; the vintage a whole number; no letter the answer of
      more than a third of the bank.
    - Every field's picker at every row: the row shown written 1 to 20 as a
      person's edit, and at each step the eight pickers that row's fields,
      read off the sheet's own columns.
    - A question added: the source expanded with a twenty-first question in
      the content file's call differs from the cartridge in its ten cells,
      the count and the eight pickers alone, loads, and shows the new
      question at its row.
    - Extended: the tripwire over both sheets, each count read beside its
      label; every sheet bare, and Questions hidden after Disclosures; the
      limit's scratch source written in the call's new shape.
    - Disclosures' bytes: proved at the build by the cartridge's diff, in
      which no Disclosures row moves, and held after by `MENU.1`'s five
      tests.
    - Kept: the cartridge its source expanded; `BD-7`'s scan and the
      census's `(every-frame 0)`; the describe golden, blessed again and
      read in its diff.
    - The hand unchanged: the row shown is a constant until `GUARD.1`, and
      the texts are the content's, written once.

13. **The *How to add a row* (rec).** `src/content/README.md` names both
    sheets, and says that a row is its vintage and its approver, then the
    sheet's fields in the order its file names them at its top; it shows a
    question's row a field a line beside the disclosure's one line; and it
    says what a question needs: four options and one right; the answer its
    letter; a why of one line, at most 140 characters, naming no option,
    since a concept card quotes it alone; and the concept, the idea it
    tests, which the exam never shows before the answer. The reviewer keeps
    the letters spread. Still one page.

14. **The hand test, the owner's, on the page built here:**
    - Buffet at full screen; the strip's box reading twelve hidden sheets,
      the ten twins, Disclosures and Questions; the Questions sheet opened,
      its twenty rows whole with their vintage and approver, and `M3` to
      `M10` the first question, *Opportunity cost*;
    - `(cell "Questions" "M2" 17)` written in the drawer's edit row, and at
      the next frame the card *Efficient markets*;
    - the drawer's library box listing `src/content/questions.vla`, and
      Build writing the cartridge byte for byte, compared with
      `src/buffet.vla` by a command given at the handover.

15. **The stale places, rewritten at the build.** In `OMAKASE.md`: the
    contract's sentence (decision 4); `GUARD.1`'s line and `BACK.2`'s, a
    dated sentence each, the twentieth's home and, on `GUARD.1`'s, the
    stride's finding; `MISE.6`'s hoist row for `strings` and `picker`,
    their second writing made and Disclosures' bytes standing. In
    `BUFFET.md` §3, a dated line under the bank's heading: where the rows
    stand, what was rewritten, and the twentieth's home. `kitchen.vla`'s
    header and `strings`' comment; `check_layout.ps1`'s header; the
    cartridge's notes; `src/content/README.md`. `MENU.2`'s first slice
    leaves `OMAKASE.md` for this file with its commit, and its second
    slices stay open there with `GUARD.2`.

**The predictions, before any number.** The numbers column is filled at
the build, P9 at the owner's hand test.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 409 rows where 169 stood, the Questions sheet's 240: its row, nine widths, ten headers, ten labels, twenty rows of ten cells, the count, eight pickers and the source's row shown; 89 formula cells where 81; Disclosures' rows byte for byte, and the notes moved | *Measurement*, as predicted: the runner printed `(rows 409) (loads yes)`, the rows in the order listed, 89 formula cells; the cartridge's diff 241 lines in and one out, the notes, and no Disclosures row among them |
| P2 | the describe golden | moved in the notes; the budget's formulas, 81 to 89 and 2,430 to 2,670 a second; the plan's shapes 30 to 38, texts 28 to 36, formulas 81 to 89 and alone 54 to 62, its levels, runs, lanes, every-frame and cycles unmoved; `(readers "Questions" 168)`, each picker's row shown and its twenty cells, indexed past sixteen and so with `(most 8)`, the row shown's eight readers, the least sure number here; eight shapes, each a picker alone at level 0; nothing else | *Measurement*, as predicted, `(most 8)` with the rest: the notes; `(budget 30 89 2670 2000000)`; shapes 38, texts 36, formulas 89 and alone 62, the levels 14, runs 6, lanes 27, every-frame 0 and cycles 0 unmoved; `(readers "Questions" 168 (most 8))`; shapes 30 to 37, `M3` to `M10`, each `(cells 1) (levels 0 0 1 1)`; nothing else |
| P3 | the pass and the load | the pass under a millisecond longer; the load under a millisecond longer, 240 rows mostly of text where `MENU.1`'s 38 cost 0.08 to 0.10 ms | *Measurement*, a scratch runner over each tree, `3dd3eeb`'s and this one's, three runs each alternated, 200 passes and 200 loads a run, opted out of Windows' throttling and pinned to the performance cores, a fixed computation before and after steady: the pass 3.34 to 3.44 ms at p50 where it took 1.61 to 1.64, 1.7 to 1.8 ms more, **missed** by nearly double. The prediction scaled `MENU.1`'s 38 rows, whose difference stood inside the runs' noise; the limit's pass prices a row of one field at 13 µs through the old walk and 18 through the new, and the bank brings 240 rows, the walk a field at a time, and eight pickers' walks of twenty steps. The load 1.25 to 1.29 ms where it took 0.52 to 0.57, 0.7 ms more: inside |
| P4 | the step | an ordinary frame and a day's first frame unmoved, the pickers reading nothing that moves; the first frame computing the eight pickers once, under 10 µs more | *Measurement*, the same runs, two years and two days at thirty frames a day: an ordinary frame 1.70 µs at p50 before and after; a day's first frame on which the fine print stays 11.3 to 11.4 µs where it took 11.2 to 11.3, and one on which it turns 12.0 to 12.1 where 11.8 to 11.9, inside; the first frame, which computes every formula once, 122 to 134 µs where it took 73 to 95, **missed**: some 40 µs where under 10 was predicted. No frame after the first computes a picker |
| P5 | the limit | unmoved: a sheet of one field holds 1,547 rows and 1,548 are refused at the picker's line; the pass over 1,548 rows within the expander's 5,000 links and under 50 ms | *Measurement*: 1,547 rows load and 1,548 are refused at the picker's line, the test holding both through the new walk on every run; the pass over 1,548 rows 28.3 to 28.9 ms at p50 where it took 20.3 to 20.8, two fifths more and inside |
| P6 | the suite | 38 tests green here, five of the cartridge more, and in CI on Linux; the suite under a quarter of a second longer than `3dd3eeb`'s at the median, nothing of the bank held at every frame | *Measurement*, here: 38 green, 14 of the crate and 24 of the cartridge. The cartridge's binary run alternately with `3dd3eeb`'s, built from that commit's tree in a target of its own, five rounds, the order turned each round, each run opted out and pinned, on eight threads: 4.02 to 4.75 s where it took 4.00 to 4.73, the medians 4.30 and 4.61 s. Inside: no difference five rounds can tell. Linux at the push |
| P7 | the page | one block more, `library-content-questions`, about 9,000 characters; the page about 27,000 longer, the cartridge's block some 13,900, the content's 9,000, the library's 2,700 and the source's 1,200, near 1,623,000; Build writing the cartridge byte for byte | *Measurement*: 1,623,410 characters, 27,731 more, inside; the new block 9,094 characters, the file's 9,025 in its tag; the cartridge's block grew 13,655, the library's 3,411 where 2,700 was predicted, the source's 1,512 and Disclosures' 23. Build in the drawer at the owner's hand test, P9 |
| P8 | the checks | 4 checks and 4 controls green; the layout control's clean copy seven files and its fifteen mutants each failing for its own reason; the offline control's fifteen unchanged | *Measurement*, as predicted: ALL GREEN, 4 checks and 4 controls; the layout control's clean copy seven files and its fifteen mutants, the third content sheet's among them, each failing for its own reason; the offline control's fifteen unchanged. `check_spdx` reads the tracked tree, so `questions.vla`, untracked, was held through a scratch index that tracked it: 40 files resolved, 7 headers read under `src/` |
| P9 | the hand test | the box reading twelve hidden sheets; the card the first question from frame 1, and *Efficient markets* the frame after `M2` is written 17; Build "same bytes" | *Measurement*, the owner's screenshots and comparison, in the owner's Chrome at full screen: the box read "hidden sheets (12)", and the Questions sheet stood between Disclosures and Clock, its twin among the others after Input; its twenty rows whole, each its vintage 0 and its approver; at frame 2,068 the edit `(cell "Questions" "M2" 17)` written, and at frame 2,238 `M2` 17 and `M3` to `M10` *Efficient markets*'s eight fields, its answer A; the budget 89 formula cells a frame, 2,670 a second; and Build in the drawer, "same bytes". Inside. A third screenshot, the sheet as first opened at frame 1,395, before the edit, showed the card the first question, `M2` 1, *Opportunity cost* and its answer C, and every column at its width. The comparison, as the handover gave it, lost its backticks to the message's inline code, so it replaced "rn" with "n" in both texts where it meant CRLF with LF; both carried CRLF, the clipboard as the file does, and it printed 21,801, the file's 21,820 characters less its nineteen pairs of "rn": the two texts equal all the same |

**What the build found.** The new walk expands the fine print to its bytes
unchanged, the cartridge's diff its proof: 241 lines in and one out, the
notes, and no Disclosures row moved. The new tests catch what they stand
for, and not only through the cartridge's expansion: five mutants, each
expanded into the cartridge before the suite ran, failed the tests written
for them. Every picker over the next field's column failed eight, among them
every field at every row, the first question shown, the question added and
the fine print at every frame; an answer in lower case failed the whole
question's test alone; a question of seven fields failed it, the file's
reading and every field picked; the row shown written 0 failed the first
question's test alone; and the vintage and the approver swapped failed the
file's reading and both rows added, a placement no test of `MENU.1`'s held,
since they read the texts' column and the count. The page's blocks followed
by themselves, `build_web.ps1` and `check_web_offline.ps1` untouched, as
`MENU.1`'s decision 10 foresaw, and the layout check wanted its pin and its
control's copy and no new rule. The one cost that moved past its prediction
is the pass's (P3), some 85 µs a question, so a bank of sixty would add about
3.5 ms more to a pass that runs at a build and never at a frame.

**Amended after the owner's hand test, 2026-10-15.** Before the commit the
owner rewrote eighteen of the twenty questions in their own words, Liquidity
and Leverage alone kept, and title-cased the bank's eight headers, `Concept`
to `Why`. No answer's letter moved, so the letters stand five to each, and
the rewrite keeps every rule the oracle holds, each why within 140
characters and every option its own; the oracle's constant of the bank's
fields followed the headers. Three texts now run past their columns on the
hidden sheet, a matter of the widths, left as the owner left them: Q7's and
Q20's last options, 75 and 92 characters in 62, and Q11's concept, 23 in
20. The library's own words, `vintage`, `approved by` and the labels `rows`,
`the row shown` and `shown`, keep their lower case, as the Disclosures
sheet's `disclosure` does. The cartridge expanded from the amended file
differs from the one the owner tested in seventy rows and nowhere else, the
eight headers, the seven labels that repeat them and fifty-five cells of the
eighteen questions, 409 rows still; the describe golden stands unmoved and
the 38 tests green; the page, built again, holds 1,624,320 characters.
