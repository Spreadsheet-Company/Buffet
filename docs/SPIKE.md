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

- ✅ **`HOUSE.4`, second slice — the loop oracle over Buffet's page: the owner's session, its golden and the hand.** *Scoped and approved 2026-11-19 (`e187eec`); the session recorded by the owner 2026-11-21 on `FRONT.8`'s page; built the same day at the owner's word, "Recommendations approved; mint everything, then proceed with the second slice of HOUSE.4". The build record is under THE ITEMS, SCOPED, at the end of the scoping.* The owner's own session from the exam to the first quarter's card, `src/buffet.replay.vla`, 132,945 frames, beside the cartridge with its golden, `src/buffet.golden.vla`, the Report's card, and the hand, `src/buffet.hand.js`, 63 cells at 685 frames; `oracle/src/replay.rs`, the oracle's own SHA-256, the replay read and played as the page's host plays it, and the two writers; `cargo run --example replay [-- --check]`; the native test holding every frame of the session to `buffet.rs`; the builder and the offline check taking the three blocks; and `tools/check_host_loop.ps1`, the page's loop oracle judged by Buffet's own floors, the keep case withheld (the build's Found). *Passed at the owner's hand test, 2026-11-21: "Hand test passes; commit it and ask Alonzo for the fix".*

  *Closed 2026-11-21.* Predicted `~days`; cost the scoping of 2026-11-19 and the build of 2026-11-21, the day the owner recorded the session, each prediction beside its number under THE ITEMS, SCOPED, all but CI's (P8), which comes with the push. The playtest's notes went into `OMAKASE.md` first, a commit of their own: `POT.5`'s first slice, `PLATE.8` and `FRONT.9`. The keep case's fix was asked of Alonzo the same day at the owner's word (`OMAKASE.md`, `MISE.6`): `ENGINE.30`'s decision 13, the derived writes a save's frame left waiting made at its load, the case withheld here until the pin that carries it.

- ✅ **`HOUSE.2`, the pin to `alonzo` 0.4.0 — the engine at 0.4.0 and the language at `vla-lang` 0.14.0, the kit and the page copied again at `v0.4.0`, and the eight calls the new kit spells anew.** *Built 2026-11-18 at the owner's word, the day Alonzo released 0.4.0, as the pin to 0.3.0 was (`2213bf1`): what moved, what held, the loop oracle over the page and what arrived with it unasked are under THE ITEMS, SCOPED.* `alonzo = "=0.4.0"` and `vla-lang = "=0.14.0"`, the version that release pins, and the lock with them under crates.io's checksums; the kit and the page's template and three scripts copied again at `v0.4.0` (`6e7b659`), `check_vendored.ps1`'s tag and five digests read again off the tag; the seven Ledger `put`s written with their pieces named, and the content sheets' `labels-from` called by its verb, `labels`, the cartridge its source expanded byte for byte; and `check_web_offline.ps1` carrying Alonzo's three stores at the tag, a person's keymap's, a person's file's and a run's unsaved work's, so that the page's Save, which names `createObjectURL`, passes and a store a later tag adds fails by its name. The manifest stays `(spec 6)`. *Oracle, met:* `expand --check` `same` against the committed cartridge; 107 tests, every frame held and the describe golden unmoved; fmt and clippy clean; the checks and their controls ALL GREEN, the offline check's control twenty-two mutants; Alonzo's loop oracle at `v0.4.0` over the page built here, every case a game's page runs OK, eighteen at the first ratio and fifteen at the second, `grid` among them (`MISE.6`); the owner's hand test. `~hours`

- ✅ **`HOUSE.2`, the pin to `alonzo` 0.5.0 — the engine at 0.5.0 and the language at `vla-lang` 0.15.0, the kit and the page copied again at `v0.5.0`, and the keep case back.** *Built 2026-11-24 at the owner's word, the day Alonzo released 0.5.0: "Alonzo 0.5.0 is live on GitHub and crates.io, so pin Buffet to the new version before recommending next steps for Buffet". What moved, what held, the loop check over the page and what arrived with it unasked are under THE ITEMS, SCOPED.* `alonzo = "=0.5.0"` and `vla-lang = "=0.15.0"`, the lock with them under crates.io's checksums; the kit, the template and the host copied again at `v0.5.0` (`5aab4c6`), the fake module and the viewport unchanged, `check_vendored.ps1`'s tag and three digests read again off the tag; no call spelled anew, the cartridge its source expanded byte for byte and the session's three files as they were; `check_web_offline.ps1` carrying Alonzo's open picker beside the save picker (`ENGINE.49`); and `tools/check_host_loop.ps1` at twenty cases, `clipboard` and `file` among them, with the keep case back in a run of its own, Alonzo's fix of the restore that lost an order's rows (`ENGINE.30`'s decision 13, Buffet's ask) its floors. The manifest stays `(spec 6)`. *Oracle:* `expand --check` `same`; 112 tests, every frame held and the describe golden unmoved; the replay's `--check` `same`; fmt and clippy clean; the checks and their controls ALL GREEN; every case of Buffet's page OK at both ratios, every restore of the session held; the owner's hand test. *Passed at the owner's hand test, 2026-11-24, but for two steps, a save opened on the Exam and no restore after it, each measured after and Alonzo's, the second the hand test's own step: "commit and push the pin".* `~hours`

  *Built 2026-11-18.* Predicted `~hours`; cost one sitting, after `PLATE.6`'s two commits were pushed at the owner's word: the pin, its checks, the offline check's stores, and the loop oracle run twice, over the page before the source's notes and after. What landed: `Cargo.toml`, `Cargo.lock`, `src/common.vla` and the page's four files; the eight calls, in `src/buffet.source.vla` and `src/kitchen.vla`; `tools/check_vendored.ps1`, `tools/check_web_offline.ps1` and `tools/run_checks.ps1`'s line for the latter's control; the source's notes; `README.md`, `OMAKASE.md` and this file. The module came out 2,258,410 bytes, 0.3.0's 1,866,609, and the page 4,670,699 characters, `PLATE.6`'s 3,897,066. *The owner's hand test, 2026-11-19:* passed, the notice of Autosave from a file excepted, which the owner's browser no longer shows: it answered Autosave at `ENGINE.38`'s hand test, and every page from a file shares the answer.

## 🍽 THE COURSES

## 🍲 THE STOCK POT

- ✅ **`POT.1` — the calendar and the dice.** *Scoped, built and hand-tested 2026-10-14: the scoping, the owner's hand test and what they found are under THE ITEMS, SCOPED.* A Day sheet, the one sheet that reads `Clock!B1`, a label in column A beside each value: the frames a day, 30, a value the manifest's `(rate 30)` and a test hold equal, so that the teacher's dial, the Clock's rate, scales the game without touching a formula; the days a year, 365, every day a market day; the day from 0, `=INT((Clock!B1-1)/B1)`, and the phase, `=MOD(Clock!B1-1,B1)`, the frame less one so that the first frame opens the first day and every day holds its thirty; the boundary, `=IF(B4=0,1,0)`; the year and the day of the year, each from 1. A Dice sheet of two streams, each a die and its draws, each draw's uniform beside it. The market's die is the kit's `hold` over the boundary, and not the kit's `dice`, which reads the frame and rolls every frame (`BD-7`, `BD-3`): the seed on the first day, and else yesterday's die jumped 1,017 draws by one multiplication exact in a double, so that the market is a function of the seed and the day alone, and a day's draws are taken by the dishes one by one, a later draw moving no earlier one. The exam's die is the seed jumped 2^30 + 88 draws, half the period away, so that the market and the exam never share a die. Four market draws and two exam draws so far, `POT.2`'s phases and `GUARD.1`'s start and stride the first to read them, by the library's first verb, `draws`; a near-normal draw later as twelve uniforms less six (`POT.5`), each uniform a draw's cell. *Oracle, met:* the owner's hand test in Alonzo's page; every frame of two years and two days held to the hand (`MISE.2`), the calendar at every frame and the dice moving on a day's first frame alone; the same dice on the same days at one frame a day; no formula outside the Day sheet reading the frame or the phase, and the census's `(every-frame 0)` (`BD-7`, whose oracle the scoping corrected); the library's ten-thousandth draw from 1 equal to Park and Miller's published 1,043,618,065; seven mutants of the cartridge, the library and the source, each caught. *Pays into:* every dish after it reads the day and never the frame, so each costs a boundary and nothing between; the exam's die (`GUARD.1`); the phases of `POT.2`; the timers of `PLATE.3`. `~days`

  *Closed 2026-10-14.* Predicted `~days`; cost one session, which also copied the kit (`HOUSE.3`), laid the layout's first pieces (`MISE.1`) and founded the game by hand (`MISE.2`), each of those its own line. What landed: the source, `src/buffet.source.vla`, and its cartridge, `src/buffet.vla`, 41 rows; the library, `src/kitchen.vla`, and its first verb, `draws`; the hand, `oracle/src/buffet.rs`; eight tests of the cartridge in `oracle/tests/cartridge.rs` and four of the hand; the runner, `oracle/examples/expand.rs`; and the describe golden.

- ✅ **`POT.2` — tier 1: four waves and a drift, prices a function of the day.** *Scoped, built and hand-tested 2026-10-14: the scoping, its sixteen decisions approved whole by the owner, every prediction beside its number, the owner's hand test and what it raised are under THE ITEMS, SCOPED.* The Market sheet, its name plain as the model's are: the index `=g^day * (1 + A1*SIN(w1*day+p1) + ... + A4*SIN(w4*day+p4))` over its own value on the first day, so that it opens at exactly 1 and a rung at its price, with Kitchin, Juglar, Kuznets and Kondratiev periods each a prime number of days, 1,459, 3,271, 6,947 and 18,251 (about 4, 9, 19 and 50 years, the Kuznets moved off 18, where it would be two Juglars), so that no ratio is a whole number; amplitudes 0.1, 0.15, 0.1 and 0.15; the drift 1.0002 a day; the phases the market's draws 1 to 4 of the first day, held. Tier 0, one wave of a year and the drift, the same at every seed, is written beside it as the picker's first rung, and the index is `CHOOSE` on the Difficulty cell from the first day, the cell a value of 1 until `POT.6` makes it a picker. No class row at course 0, stocks being the market itself; no price, the four rungs being `STATION.1`'s, each `=INT(<its start in cents>*<its multiplier>+0.5)`, the idiosyncratic factor tier 2's. A price is a function of the day alone, so a boundary recomputes the market once and the days between recompute nothing (`BD-7`), and the index twenty days back (`Market!C25` beside today's `B25`, the same fill, recomputed and not remembered) gives `FRONT.3` its weather word's slope until `POT.4`. *Oracle* (rewritten 2026-10-14 at the owner's word, as that day's measurement leaves it: `vla-lang` 0.9.3 computes `SIN` and `^` with Rust's std, whose results differ between platforms and from the module's in the last bit): the hand, `oracle/src/buffet.rs`, calls the same std functions as the cells, in each formula's order, so it holds the index and every price exactly natively, here on Windows and in CI on Linux; the tests compare the hand with the cartridge live, and no committed golden holds an unrounded value derived from `SIN` or `^`, so whatever a replay golden holds later is rounded once to what it shows, as money is; the two-peak test, the path not repeating within a hundred years; and `BD-7`, no formula outside the Day sheet reading the frame or the phase and the census keeping `(every-frame 0)`, so the market recomputes on a boundary and nowhere else. *Pays into:* the rungs' prices (`STATION.1`), the weather word (`FRONT.3`), the benchmark (`CRITIC.1`), the picker's first rung (`POT.6`). `~days`

  *Closed 2026-10-14.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: the Market sheet in the source and its cartridge, now 96 rows and 51 formula cells; the hand's market in `oracle/src/buffet.rs`, the same functions as the cells in each formula's order; the Market held inside the test of every frame of two years, three tests of the cartridge more (its settings, a year at difficulty 0, a hundred years at one frame a day) and three of the hand (both tiers opening at 1, the amplitudes, the two-peak test), twenty in all; the describe golden blessed again, moved by the Market alone; and, in the same commit at the owner's word, `Cargo.toml`'s comment naming the pin it holds, `alonzo` 0.1.1. Three of the eight predictions missed in part, each named beside its number: the nearest repeat's lag, seed 777's first year falling on every one of its days, and an ordinary frame 0.1 µs slower. The page held to the hand to the last bit in every one of the 70 cells the hand test showed.

- ✅ **`POT.10` — the market's hours: an hour a second, the opening and closing bells, and a session's prices each half hour.** *Scoped and approved 2026-10-16, the report cards amended to quarterly and annually; built the same day; amended at the owner's hand test, a price each half hour, a prospectus a week and a day's work, and every dollar a view shows in two decimals, and at the second, 2026-10-17, those dollars flush right; passed at the third: the scoping, the approval, every prediction beside its number, what the build found and the owner's three hand tests are under THE ITEMS, SCOPED.* Minted 2026-10-16 at the owner's word, who had played the game at `BACK.1`'s hand test: "one second per market day is too fast to conceptualize. The game should run at one real-world second per hour of in-game market play. I also think the market should have trading hours and non-trading hours, so the player has time between trading days to focus on other game tasks. ... these are necessary changes before anything else gets built"; so it stands before every other dish of this course. *Scoped the same day, its nineteen decisions the owner's to approve (`SPIKE.md`), as recommended there and amended at the hand test:* two minutes of the market's time a frame, thirty frames an hour at `(rate 30)`, so an hour a second, a day 24 seconds and a year about two hours and a half, every hour alike, the night and the weekend too; the market open from the opening bell, 9:30, to the closing bell, 16:00, Monday to Friday, with no holidays, 261 sessions in the first year, and the game opening on Monday's bell; the Day sheet a calendar of minutes, the day, the time to the half hour, the session and its mark, and still the one sheet that reads the frame (`BD-7`, restated); every day's close tier 1's index of that day to the bit, so that the first year stays `POT.2`'s, and between two closes the session's prices moving each half hour along a path from the last close to the day's close, drawn from the day's next thirteen draws, the exam's stream moved to columns of its own; outside the session the last close and its change, day 0 opening and closing at the starts; the views' date with its weekday and its time, a line under it saying when the market opens or closes, and their dollars in two decimals; the hustle a prospectus a week, each a day's work; the fine print a month and the dividend on its days, as they stood; an order while the market is closed refused, `PLATE.1`'s to build. *Needs:* nothing. *Oracle:* every frame of ten days, every hour of a year and two days at a frame an hour, and eight hundred days and a hundred years at a frame a day, each held to the hand, its calendar counted in minutes; `BD-7`'s scan and the census's `(every-frame 0)`; the describe golden; mutants; a frame's cost natively and in Chrome at the owner's full screen; and the owner's hand test. *Pays into:* every dish after it, each reading the calendar's day, session or mark and never the frame: `PLATE.1`'s orders, filled only while the market is open, `PLATE.3`'s timers, `CRITIC.1`'s report card, `RECIPE.1`'s cadence and `FRONT.3`'s ticker. `~days`

  *Closed 2026-10-17.* Predicted `~days`; cost one session over two days, the scoping, its approval, the build, its measurements and three hand tests, the first two amending it. What landed: the Day sheet a calendar of minutes, the Dice's seventeen market draws and the exam's stream moved to `H1:J3`, the Market's session `B27:B45` and its swing, the Stocks' price now over the index now and the change in cents, the Hustle's week and day's work, and the views' date, bell and dollars in the source and its cartridge, 569 rows and 201 formula cells; `bell` and `dollars` in `src/kitchen.vla`, and `as-of` with the weekday and the time; the caveats' rows 2, 11 and 12 rewritten and 13 and 14 added, drafts for the owner; the hand's calendar counted in minutes, its session's path, its bells, its hustle by the day and the half hour, and its spelling of dollars; 56 tests, the long runs each at the pace it needs; the describe golden blessed again; and, in `OMAKASE.md`, the contract, `BD-7` restated, and the notes on `PLATE.1`, `GUARD.1`, `MISE.4`, `MISE.5`, `PLATE.3`, `RECIPE.1`, `CRITIC.1`, `FRONT.2` and `MISE.6`, whose number format was asked of Frazaro and Alonzo at the hand test. Five of the nine predictions missed in part, each named beside its number: the describe golden's order and the waves' run (P2); a half hour's turn over its range (P4); the longest test, the hundred years' and not the hour's year (P5); the page's blocks (P6); and the dial's 120, half a frame's cost where the same was predicted, with the bell in the owner's Chrome (P8). The build found that a harness attaching after the game is picked misses its first frames; the hand tests asked a price each half hour, a prospectus a week and a day's work, two decimals on every dollar, and those dollars flush right, the record holding no number format; and a column dragged wider or narrower moves its cell's edge and not a padded text.

- ✅ **`POT.11` — the pace and the volatility factor: ten minutes a second while the market trades, an hour a second between, a price every five minutes.** *Minted, scoped and built 2026-10-19 in one sitting at the owner's word, its sixteen decisions built as recommended; passed at the owner's hand test the same day, which kept the ticks and the caveats' drafts and set the factor at 16 and the clock between sessions to the half hour: the scoping, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* Minted at the owner's word from `PLATE.2`'s hand test ("the gameplay feels too fast. The trading windows is too narrow") and the owner's adjudications of the same day: twenty seconds of the market's time a frame in the regular session, 9:30 to 16:00, and two minutes outside it, the Clock's rate staying 30, the time a sum over its twin in seconds; the pace keyed on the regular session's bells and never on the cell that says orders fill, so that the extended hours a license unlocks later, 4:00 to 20:00, run at an hour a second and every player plays a seed at one pace; a price every five minutes, 78 a session, each step scaled by 1/√6 so that the day's spread holds at a factor of 1; the clock on every view to the five minutes in session and to the half hour between; and a volatility factor within the day, a setting of the Market sheet, 16, each close staying the waves' (`BD-3`), across days `POT.5`'s. No player control of time, and no license that changes the clock. *Oracle:* the hand's calendar in seconds at both paces and its path of 78 ticks with the factor; every frame of a weekday and its night at the game's pace, the long runs at their own paces; the factor written; `B19` written open at every hour; mutants. *Pays into:* every dish that reads the calendar; the license's extended hours, unminted; `POT.5`'s walk, which takes the factor across days. `~days`

  *Closed 2026-10-19.* Predicted `~days`; cost one sitting, the scoping, the build, its measurements and mutants, and the owner's hand test, which amended it. What landed: the Day sheet's time in seconds, a sum over its twin at two paces, its pace now, in session and the time shown, and the tick each five minutes; the Dice's 82 draws; the Market's factor, its waves and workings a row down, its session of 78 five minutes and the index now at `B112`; the Stocks over it; the Hustle's due a day from the time shown, spelled to the minute; the Ledger's header; the source's notes and comments; 1,198 rows and 790 formula cells; the caveats' rows 13 and 14 rewritten and 19 and 20 added; the hand's calendar in seconds at a `Pace`, its path with the factor, the bound and the kind year at 16; 76 tests, the every-frame run a weekday and its night where ten days stood, the long runs holding the dice and the session's table whole at each day's turn and byte for byte between; eighteen mutants caught; the describe golden blessed again; and, in `OMAKASE.md`, the contract, `BD-7` restated, `BD-5`'s rows, `POT.5`'s line rescoped to take the factor across days, and the notes on `CRITIC.1`, `MISE.4`, `MISE.5` and `MISE.6`. Four of the nine predictions missed in part, each named beside its number: the plan's depth (P2), the load (P3), midnight and the bell (P4), and the page's blocks (P6). The build found the long runs' parse of the day's draws dearer than the market's frames, and the plan 98 levels deep by the draws' chain; the hand test set the factor at 16, found the sure gain of a dip bought below a close known at the bell, a caveat until `POT.5`, kept the clock between sessions to the half hour, and asked again for a dragged column's width kept a sheet, Alonzo's, unminted there.

## 🍳 THE STATIONS

- ✅ **`STATION.1`, first slice — stocks: four rungs.** *Scoped and built 2026-10-14, hand-tested 2026-10-15: the scoping, its fifteen decisions approved whole by the owner, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The Stocks sheet, its name plain as the model's are, a rung a row, cheapest first: four fictional rungs at $1, $5, $25 and $100, labeled `rung 1` to `rung 4`, the working labels (a penny stock, a small cap, a mid cap and a blue chip) kept in the docs and never in a cell until `MENU.3` names them, since real index and fund names are others' marks; each rung's start in whole cents, its beta and its dividend yield as values, and its workings one fill a column. A price is `=INT(<start>*Market!$B$25^<beta>+0.5)`, the index the difficulty picks to the rung's beta, whole cents, one rounding, half up: a power, so that a beta holds at every level of the index and no price reaches zero at any seed, none falling below 11, 96, 833 and 4,386 cents, a bound the hand holds; the betas 2, 1.5, 1 and 0.75, the third the market itself. Yesterday's price is the same formula over yesterday's index, `Market!D25`, the Market sheet's fills reaching a column D, recomputed and never remembered. The fourth rung pays a dividend, 2% a year, 50 cents a share fixed from its start, on the year's 91st, 182nd, 273rd and 364th days, so that income exists from the first quarter, the cell holding it all that day for `PLATE.1` to pay at the day's first frame. Written as rows and not yet a ladder, a rung at a time, in the order a walk over a list of rungs writes, since the ladder is the pattern's second appearance and enters the library with the bonds (`STATION.2`). *Oracle, met:* `buffet.rs`, every price today and yesterday, each dividend and the dividend today held at every frame of two years and two days and on every day of a hundred years at one frame a day; the opening exactly the starts; each rung's least at or above the bound; the dividend on exactly four days a year, the days the hand lists; `BD-7`'s scan and the census's `(every-frame 0)` kept; the owner's hand test in Alonzo's page, every cell it showed the hand's. *Pays into:* `PLATE.1`'s rows, one a rung; the Market panel's prices and their change; the ladder hoist at course 1, whose proof is that this slice expands to its bytes unchanged, which a scratch ladder of 24 lines has already met. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: the Stocks sheet and the Market's column D in the source and its cartridge, now 131 rows and 79 formula cells; the hand's rungs, their prices, the dividend's days and the bound in `oracle/src/buffet.rs`; the Stocks sheet held inside the tests of two years, a year at difficulty 0, eight hundred days and a hundred years at one frame a day, two tests of the cartridge more (the rungs' settings, the opening) and three of the hand (the opening at both tiers, the bound, the dividend's days), 27 in all; the describe golden blessed again, moved as predicted; and, in the same commit at the owner's word, `POT.2`'s P2 cell, CI's Linux holding the market bit for bit at the first push. One of the eight predictions missed, and only at first: the suite took 11.6 s while the Stocks sheet's 56 cells were viewed at every frame and every day, and 7.4 s once the window held the four columns that compute, half again the old suite's time on the same machine under the same load. The build found that a cent is a wide step for a $1 share: the first rung's price moves on 25 of the first year's 365 days.

## 🍴 THE PLATE

- ✅ **`PLATE.1` — the plate and the ledger: orders by click and by key, fills at the price shown.** *Scoped and built 2026-10-18 in one sitting at the owner's word, "Scope and implement PLATE.1", its fifteen decisions built as recommended, and passed at the owner's hand test the same day with no change asked: the scoping, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* Trading, a view of its own beside the Portfolio, a stock a row with its price now and the pocket's shares of it, and six buttons a stock, Buy 1, Buy 10, Buy max, Sell 1, Sell 10 and Sell all, each drawn in Lovelace Lavender by a look keyed on its text; the presses the library's `clicked` at its third writing, one fill over the Orders sheet's grid, an act a column and a stock a row, its places values. The cartridge at the page's version 5 (`ENGINE.25`): ten commands of its own, held while Trading shows and listed in the palette as `buffet.rung-1` to `buffet.sell-all`, the digits 1 to 4 picking a stock, marked ▸, and B, N and M buying a share, ten and the most of it, S, D and F selling a share, ten and every one; a click picks its stock too. An order is one a frame, a click's before a command's, judged by what the last frame showed: filled at the price shown, `Stocks.last!E5:E8`, only while the market showed open (`POT.10`'s decision 13), a share or ten whole, as many as the cash covers, or every share held, and each refusal said by its reason. The Orders sheet keeps the desk's books and holds the last order for the view to say; the Positions sheet both pockets' books, a row a pocket and a stock, the shares, the cost basis at the average cost, the cents spent and received and the dividends as state over their twins, the value and the gains taken and not taken as functions of them; the Cash sheet each pocket's cash as the sum of its flows, its holdings and its net worth; and the Ledger the last hundred fills, a ring written by the Write sheet's seven rows a frame late (`MISE.3`'s decision 5). Orders fund the firm, `Orders!B1`, until `PLATE.2`'s switch; a person's edit of 2 funds your own. The dividend is paid at a day's first frame to the shares the last frame held (`STATION.1`'s decision 11), into the pocket's dividends and cash, with no ledger row. The Portfolio gains the holding's columns, Shares, Cost ($), Value ($) and Gain ($), and each pocket's net worth; the caveats their row 17, the order filled whole at the price shown with no spread, no commission and no day to settle (`BD-5`). *Oracle, met:* every frame of a scripted session of orders by clicks, keys and the palette, in both pockets, through every refusal, the click beside a command, a mark's frame and the closing bell's, held to the hand, the desk's books, the positions, the cash, the commands, both views and the ledger a frame late, the host's derived writes made as the host makes them; the ledger's ring past a hundred fills; nine dividends at a frame a day, the ninth on a frame an order fills; the desk's settings, buttons and commands; the views' new cells at rest in every long run; `BD-7`'s scan and the census's `(every-frame 0)`; the describe golden blessed again; 20 mutants, each caught by the test written for it; the cost natively; and the owner's hand test. *Pays into:* the pockets' switch (`PLATE.2`), every order a later course places, the timers (`PLATE.3`), the cost model (`PLATE.5`), the report card (`CRITIC.1`) and the rules' orders (`RECIPE.1`). `~days`

  *Closed 2026-10-18.* Predicted `~days`; cost one sitting, the scoping, the build, its measurements and mutants, and the owner's hand test. What landed: Trading, the Orders, the Positions and the Ledger sheets, the Commands and the Write sheets, and the Cash sheet's and the Portfolio's new rows in the source and its cartridge, 1,028 rows and 520 formula cells; the caveats' row 17; the hand's order desk, its books, its cash, its ledger and its views' texts, and the oracle's `derived`, the host's Write sheet read; 71 tests, the crate's 27 and the cartridge's 44; the describe golden blessed again; and, in `OMAKASE.md`, the contract's version, `BD-5`'s caveat taken, `PLATE.2`'s line and `MISE.6`'s rows and hoist; and in the same commit the three Linux cells run 39088078269 answered, `BACK.1`'s, `POT.10`'s and `GUARD.1`'s. The predictions missed in part, each named beside its number. The build found a write into a cell that held nothing dear for the next two steps, while the ledger fills its hundred rows; a mutant of the price now living through the $1 stock's mark, until a sale at a dearer stock's mark was added; and Trading held at every half hour making the hour's year half again as long. The hand test found a column's width a person drags kept by its letters on every sheet, Alonzo's.

- ✅ **`PLATE.2`, first slice — the two pockets' switch: a button and a key, and no money moved.** *Scoped and built 2026-10-18 in one sitting at the owner's word, "Scope and implement PLATE.2", its eleven decisions built as recommended, and passed at the owner's hand test on 2026-10-19 with no change asked of it: the scoping, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The pocket an order funds, `Orders!B1`, formula state over its twin: the firm's at the first frame and the other pocket at each switch, read by every formula that follows it through `B19` as before, so that Trading's line, its shares and its cash available, the Portfolio's line and holding, and the ledger's pocket and cash after follow the switch at its own frame with no formula of theirs moved. The switch: `[ Switch ]` beside Trading's cash available, `D11`, "or press C" beside it, drawn in Lovelace Lavender by a style row, its place values of the Orders sheet, `B43:B44`, its press the library's `clicked` at its fourth writing; and the cartridge's eleventh command, `switch-cash`, on C, held while Trading shows, its words naming the pocket it turns to. One act a frame, a click's before a command's, an order, a pick or a switch, so that no order fills on a switch's frame and an order funds the pocket its person saw. The caveat `BD-5` owed this dish, its row 18: cash that earns no interest and loses nothing to inflation, the 3% of its real world marked for the educator's read. *Oracle, met:* every frame of a scripted session of switches, by the click, by C and by the palette, beside an order both ways and beside a pick, at a mark and with the market closed, and C and a click at the switch's place while the Portfolio shows, held to the hand; at every switch's frame and the frame after it, every balance the last frame's (`BD-9`), compared from the cells before the hand holds the frame; `PLATE.1`'s two sessions switching by the switch where a person's edit had stood; the desk's settings grown by the switch; a hand test of the rule; `BD-7`'s scan and the census's `(every-frame 0)`; the describe golden blessed again; 16 mutants, each caught by the test written for it; the cost natively; and the owner's hand test. *Pays into:* each pocket as the report card's (`CRITIC.1`), the draw and every transfer (`BACK.6`, this dish's second slice), and every order after it. `~days`

  *Closed 2026-10-19.* Predicted `~days` for the dish; the first slice cost one sitting, the scoping, the build, its measurements and mutants, and the owner's hand test the next day. What landed: `Orders!B1` a switch's state, the switch's block `B43:B47`, the gates of `B16`, `B20` and `B21`, Trading's switch and its key, the eleventh command and the notes in the source and its cartridge, 1,048 rows and 526 formula cells; caveat 18; the hand's switch and its rule; 73 tests, the crate's 28 and the cartridge's 45; the describe golden blessed again; in `OMAKASE.md`, `BD-5`'s caveat taken; and `Delta/CLAUDE.md`'s map. The predictions missed in part, each named beside its number: the describe golden's readers, a switch's step under its range, and the cartridge's block in the page. The build found `BD-9`'s comparison first placed after the hand's, which caught the money-moving mutants first, and a harness trap. The hand test asked two things of the game the same day, a minute a second where an hour a second stood and a volatility factor, taken up as `POT.11`.

- ✅ **`PLATE.6`, first slice — the limit, as a test.** A sandbox edit pushing a balance past what a double holds, the error value flowing through every sum and recovering when the edit is undone, held cell by cell, and the error's own joke from the Errors sheet, which arrives with this dish (`MENU.1`), shown where it lands. The second slice, the ending as a designed thing, is course 7's. *Oracle:* the test, and a mutant that lets the error vanish. *Pays into:* every sum written after it is written knowing an error value passes through. *Scoped and built 2026-11-15 in one sitting at the owner's word, its eight decisions built as recommended for the owner's hand test to amend (`SPIKE.md`, under THE ITEMS, SCOPED):* the edit a correct answer's worth, `Proctor!B2`, written 1E+308 after the begin, the capital 5E+308 and `#NUM!`, undone by writing 50,000 back, `alonzo` 0.3.0 having no undo; `vla-lang` 0.12.0's machine turning every result past the end into `#NUM!`, so that no NaN and no infinity reaches a cell; the error landing on the firm's capital, cash, net worth and the card's worth now, the views' figures of them and the order desk's cash, held cell by cell against a twin with no edit, itself held to the hand, and every cell the twin's again once undone; the Errors sheet, `src/content/errors.vla`, the five jokes of `BUFFET.md`'s appendix, `#NUM!`'s the row shown; beside every balance a view shows, its error's joke, the library's `error-joke`; an order against cash shown as an error refused, a sale too, so that no book takes one; and caveat 26, money that ends where a double does. *Found there and unminted:* a card's turn or the begin while the error stands latches it into the card's holds, measured; an order in the edit's own frame fills by the cash it was shown, rightly, and its ledger row keeps six of its seven fields, the host writing no error; the books' counts pass the end by play, not by edit, which the second slice's ending decides; the jokes are `#NUM!`'s alone until `ERROR.TYPE` (`vla-lang` 0.13.0) reaches a pin, on the asks table with the record's spelling of 1E+308. *Passed at the owner's hand test the same day with one change, `#NUM!`'s joke in the owner's words: "Net worth exceeds the present value of the known universe. Please purchase another universe to continue playing."* `~days`

  *Closed 2026-11-15.* Predicted `~days` for the dish; the first slice cost one sitting, the scoping, the build, its measurements and mutants, and the owner's hand test the same day, which passed it with one change, the joke's words. What landed: the Errors sheet, `src/content/errors.vla`, five jokes, the layout check's content pin raised to 7; `kitchen.vla`'s `error-joke`; the eight joke cells of four views; the order desk's guard, `ISERROR(Orders!B24)`, its sixth outcome and Trading's seventh sentence; caveat 26; the source's notes and comments; 1,478 rows and 871 formula cells; the hand's end of the numbers, counted by a typed number's digits, the capital at a worth, the landing and the sixth outcome; 103 tests, the crate's 42 and the cartridge's 61, the edited game held cell by cell against a twin held to the hand through the edit, refused orders, a switch, a Delete and the undo; twenty mutants caught; the describe golden blessed again; in `OMAKASE.md`, `BD-5`'s row and two rows of the asks table; `BUFFET.md`'s appendix in the owner's words; and `Delta/CLAUDE.md`'s map. Three of the eight predictions missed in part, each named beside its number: the pass 0.5 ms dearer where 0.4 at most, the limit's test just under its range, and the page 9,398 characters longer where 8,000 at most. The build found the hand's guard against the end drawn too wide, a score of none raising ten to the 308th, and a joke reading its view's own figure living until a test of the library's rule caught it. The hand test saw `Proctor!B2` drawn as `#`s, the record's spelling of 1E+308, on the asks table.

- ✅ **`PLATE.6`, second slice — the ceiling: no book counts past the end of the numbers.** Taken into course 0 from `FRONT.7`'s line at the owner's word, 2026-11-15, "Ceiling now, island later": an order whose fill would take a book past the end is refused, so that no count, book or ledger row keeps `#NUM!` once an edit is undone; the island at a stated net worth stays `FRONT.7`'s, the third slice. *Oracle:* a session of a person's ceilings held to the hand cell by cell, each rule of the guard refusing alone and each boundary filling; a session near the end where no book, cash or ledger row holds an error; a mutant for each rule. *Pays into:* every book a later course adds counts under the ceiling. *Scoped and built 2026-11-15 in one sitting, its decisions built as recommended for the owner's hand test to amend (`SPIKE.md`, under THE ITEMS, SCOPED):* the ceiling 1E+307 cents, `Orders!B49`, a setting below the end by a factor of nearly eighteen, so that four books summed and ten times a basis stay numbers; an order refused, the seventh outcome, whose cents take its stock's cents bought past the ceiling, or a sale's its cents sold or the pocket's cash after it, or whose cents no number holds; each book compared with the ceiling less the cents, so that the guard sums nothing that could pass the end; Trading's sentence, "Refused: the order would take the books past their ceiling."; and caveat 27. *Passed at the owner's hand test the same day with no change asked.* `~days`

  *Closed 2026-11-15.* Predicted `~days` for the slice; it cost one sitting, the question of how to take it, the scoping, the build, its measurements and mutants, and the owner's hand test the same day, which passed it whole. What landed: the Orders sheet's ceiling and guard, `B49:B54`, `B26` the order asked unless refused, the seventh outcome in `B27` and Trading's eighth sentence; caveat 27; the source's notes and comments; 1,494 rows and 876 formula cells; the hand's ceiling, a person's judged exactly, the seventh outcome and the order's asked and books; 107 tests, the crate's 44 and the cartridge's 63, the desk's six new cells held at every frame of every session, a session of a person's ceilings held cell by cell, and a session near the end of the numbers scanning the books, with witnesses for each refusal; thirteen mutants caught; the describe golden blessed again; in `OMAKASE.md`, `FRONT.7`'s line amended, its slice the third, and `BD-5`'s row; and `Delta/CLAUDE.md`'s map. Three of the eight predictions missed in part, each named beside its number: the pass 0.38 ms longer where 0.3 at most, the exact session under its range, and the page 5,037 characters longer where 5,000 at most. The build found the guard as first written summing a book and the order's cents, which near the end is `#NUM!` and the fill's, and compared them by a difference instead. The hand test found a paste by Control+V into a cell missing from Alonzo's page.

## 🧽 THE BACK OF HOUSE

- ✅ **`BACK.1` — the dish pit: the side hustle, the first button.** *Scoped 2026-10-16, approved the same day with the view named by the owner, built and hand-tested that day, and amended at the hand test, the key bound on its view alone: the scoping, the approval, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* Gigs, the game's second view, the owner's name for "the sheet where various clicker jobs can produce cash at varying speeds, cooldowns, and pay rates", and its one job at course 0, proofreading a prospectus: a button cell, `[ Proofread a prospectus ]` in a column as wide as its text, pressed by a click on it while Gigs shows, the pointer on the cell and the left button's rising edge, or by P while Gigs shows, the Keys sheet binding P on that view alone by a formula of the sheet shown, so that every view binds keys of its own (the owner's word at the hand test: "keybindings should be scoped to each tab to avoid limiting the entire application to 26 key commands"); each press a cell that holds 0 between presses; $25 a claim, once a market day, at most four claims a run, the owner's cap over a claim a day for good, four claims being $100, about twice a perfect trader's first year on the personal pocket's $100. A claim is judged by what the last frame showed and counts against the day shown, as an order fills at the price shown; its books stand on the Hustle sheet, the fee, the cap and the button's place as values, so that a layout of the view moves no formula; the fees are the claims times the fee, which `PLATE.2`'s personal pocket reads when it comes; the day moving reopens the hustle, and nothing reads the boundary. The views' date and fine print entered `kitchen.vla` at their second writing, `as-of` and `fine-print`, the Portfolio expanding over them to its bytes unchanged. *Oracle, met:* every frame of a scripted session of clicks and keys, written as the host writes them and the Keys sheet read after every step as the host reads it, held to the hand, the cap, the day a press counts against, the reset and the key's binding among them; Gigs at rest on every day of two years and two days and of eight hundred at one frame a day; the settings, the button at their place and the one key bound on its view; nothing reads a view, `BD-7`'s scan and the census's `(every-frame 0)` kept; the describe golden blessed again; nine mutants, each caught by the test written for it; a click's cost natively and in Chrome at full screen, a claim shown 48 to 50 ms after the click at p95, the contract's 100 ms met by half; and the owner's hand test in the page, twice. *Pays into:* `PLATE.1`'s six buttons a rung and `GUARD.1`'s four options a question, each a press's cell, its place values of the model and its keys its view's; the gigs to come, each its own speed, cooldown and pay. `~days`

  *Closed 2026-10-16.* Predicted `~days`; cost one session, the scoping, its approval, the build, its measurements, the owner's hand test and its amendment. What landed: Gigs, the Hustle sheet and the Keys sheet's one key in the source and its cartridge, 474 rows and 125 formula cells; `as-of` and `fine-print` in `src/kitchen.vla`; the hand's hustle and its test; the scripted session and the binding's test, 49 tests in all; the describe golden blessed again; and, in the same commit, the five Linux cells run 38939138517 answered, `MENU.1`'s, `MENU.2`'s, `MENU.5`'s, `FRONT.1`'s and `FRONT.2`'s. Five of the nine predictions missed in part, each named beside its number: the describe golden's shapes, the views' date and fine print being one shape over two sheets; the step after a write of the Input rows; the cartridge's block in the page; P beside a click in Chrome; and the key, bound on every sheet as decision 9 had it until the owner's word at the hand test. The build found that every record lists every sheet, so that each sheet a dish adds makes the oracle's long tests dearer; and the owner, playing the game, asked before any other dish for an hour a second and the market's trading hours, recorded with the hand test for the item that takes it.

- ✅ **`BACK.2` — the firm's name and flavor, and `MENU.3`'s first slice, four names.** *Scoped and built 2026-10-19 in one sitting at the owner's word, its eight decisions built as recommended; passed at the owner's hand test the same day with no change asked: the scoping, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The last exam answer picks one of four archetypes, the archetype names the firm from the Firms sheet (`MENU.1`'s name for the line's Names) and sets its flavor, never its power; the certificate panel prints the name, the date and the score; the firm's capital is the exam's. With it `MENU.3`'s first slice: four rung names, fictional, and the firms' four. The second slice, every vehicle's name with the lawyer's glance, is course 6's. *Pays into:* the overview's name (`FRONT.3`), the report card's header (`CRITIC.1`). *Recommended at `MENU.2`'s first slice, 2026-10-15 (`SPIKE.md`):* the exam's last question, the one that picks the archetype, reads its four options off the Firms sheet, a column beside the firm each picks, so the sheet arrives with `GUARD.1`, which asks the question, and this dish names the firms beside them. *Arrived with `GUARD.1`, 2026-10-17 (`SPIKE.md`):* the Firms sheet, hidden, `src/content/firms.vla`, its column of four styles the last question's options, the style answered `Proctor!B40`, 0 until then, the row shown `Firms!D2` and its text `D3`; the Certificate prints the date, the score, the capital and the style, its style line where this dish's name will stand. *Found at `FRONT.3`'s build, 2026-10-19 (`SPIKE.md`):* the Overview's `A3` holds the firm's line, a value and a placeholder, "Your firm: name pending registration.", which this dish makes a formula of the Firms sheet's name. *Scoped and built 2026-10-19 in one sitting, each decision the owner's to approve, amend or strike at the hand test (`SPIKE.md`):* the Firms sheet two fields, each style beside the name it gives the firm, Prudent & Sons, Above the Fold Capital (the draft's Headline Capital being a living venture firm's name), Read-the-Room Partners and Vibes LLC, its row shown `E2` and its pickers `E3` and `E4`; the eleventh answer, or a waiver's style, picks both; the Certificate prints the name where the style stood, "pending registration" until then, the Overview's `A3` the name and the style, and the Exam's eleventh line says the answer names the firm; the flavor words alone at course 0, read by no number of the game; and `MENU.3`'s first slice, the four stocks named on their own rows of the Stocks sheet, Longshot Mining, Disruptr, Average Co. and General Widget, every column that shows them wide enough; caveat 23, the name the exam picks. `~days`

  *Closed 2026-10-19.* Predicted `~days`; cost one sitting, the scoping, the build, its measurements and mutants, and the owner's hand test, which passed it whole. What landed: the Firms sheet's second field, the name, `src/content/firms.vla` four firms, its values moved from `D` to `E`; the Certificate's name line and the Overview's firm line, each pending registration until the eleventh answer, and the Exam's eleventh line; the four stocks named on `Stocks!A5:A8`, the Stocks sheet's column `A` given its width, the Positions' `C` and the Ledger's `D` widened, and the Commands' words widened for a line of `PLATE.1`'s the new test found cut; caveat 23; the source's notes and comments; 1,271 rows and 807 formula cells; the hand's names on its rungs, its firm line, its order's sentence and its commands' words by them; 84 tests, every reader of a name held to the hand's names, each name inside every column that shows it, and the style moving no number by a scan and by the four styles' games compared cell for cell; twenty-two mutants caught; the describe golden blessed again; and, in `OMAKASE.md`, `BD-5`'s row and the notes on `CRITIC.1` and `STATION.1`'s second slice. Five of the ten predictions missed in part, each named beside its number: the Firms sheet's readers 14 where 9 to 11; the pass and the load cheaper than priced, inside the noise; the caveats' block eight characters under; three columns given widths where two were priced; and twenty-two mutants where fifteen to twenty.

## 🛡 THE SNEEZE GUARD

- ✅ **`GUARD.1` — the Series 777: the opening screen and its controls tutorial.** *Scoped 2026-10-17, its twenty decisions approved whole by the owner the same day, built that day, and amended at the owner's hand test, eleven questions, the dollars grouped by thousands, the buttons in lavender, the caveat's heading, the cash on hand and $250 a prospectus; passed at the second the same day: the scoping, the approval, every prediction beside its number, what the build found and the owner's two hand tests are under THE ITEMS, SCOPED.* The game opens on the Exam, a view of its own first among the tabs: ten questions drawn from the bank by the exam's own die (`POT.1`), a seeded start and a stride among the first ten primes above 1,547, coprime to every bank a picker holds, so that none repeats; then an eleventh, ungraded, "Which best describes you?", its options the four styles of the Firms sheet, `src/content/firms.vla`, the answer the firm's style (eleven at the owner's word at the hand test, where the scoping had nine and a tenth). Each question shows with its four options, each a button in Lovelace Lavender, answered by a click, the press the library's `clicked` at its second writing, or by its key, A to D, bound on the Exam alone while a question is open (the Keys sheet's free buttons, until `ENGINE.25`'s commands are released and pinned). An answer is judged by the question the last frame showed, as an order fills at the price shown; the score is a count, the firm's capital $5,000 and $500 a right answer; the exam cannot be failed. The Proctor sheet keeps its books: the settings, the draw, the buttons' table and the workings. The Camera hands the game over: the Exam, the Certificate at the eleventh answer, then where the certificate leads, a value, the Portfolio until the Overview (`FRONT.3`), by the certificate's button or O. The Certificate: the score of ten, the capital in dollars, the style, its button, and the plain statement that the game is not advice under its heading, "Caveat:", the proctor's signature beneath. The market's first bell waits for the certificate's button, the Day sheet counting the frames it waited and taking them out of the time (`BD-7`'s clause), so that no market day passes under an exam. A second run may skip the exam by a waiver, a person's edit of `Proctor!B3`, at five correct answers, $7,500, and the first style. And at the hand test, at the owner's words: every view's dollars grouped by thousands, `dollars` and the new `signed-dollars`; the Cash sheet, the personal cash at the start, $100, the firm's cash and the personal, shown on the Portfolio as its cash on hand; and the prospectus's fee $250 where $25 (`BACK.1`). *Oracle, met:* the draw at every seed of a thousand and the contract's, the hand's and never a question twice; every frame of a scripted exam answered by clicks and by keys, the proctor's books, both views, the keys each view binds, the Camera's hand-over, the bell's wait and the cash held to the hand, then the market's first hour after the bell; the score funding the firm as the contract says at every score of the ten and at a waiver's; a waiver at the first frame and in the middle; the settings, the buttons at their places, every option whole inside its column and the buttons' looks; every amount spelled with its thousands grouped, beside every power of ten to 2^53 cents; `BD-7`'s scan and the census's `(every-frame 0)`; the describe golden blessed again; 28 mutants, each caught by the test written for it; the cost natively and in Chrome at full screen, an answer shown 49 ms after its click at p95, the contract's 100 ms met by half; and the owner's two hand tests. *Pays into:* the firm's capital and name (`BACK.2`), the pockets (`PLATE.2`), and the license ladder (`GUARD.2`), whose every exam is this one over another bank. `~days`

  *Closed 2026-10-17.* Predicted `~days`; cost one session, the scoping, its approval, the build, its measurements, the owner's hand test and its seven changes, and the second hand test. What landed: the Exam, the Certificate, the Proctor, the Cash and the Firms sheets and the Camera in the source and its cartridge, 748 rows and 271 formula cells; `clicked`, `signed-dollars`, the grouped `dollars` and the bell's fifth line in `src/kitchen.vla`; `src/content/firms.vla`, the fourth content sheet; the caveats' rows 15 and 16; the hand's exam, its draw, its keys, the calendar's wait, its grouped dollars and the cash; 67 tests, the crate's 27 and the cartridge's 40; the describe golden blessed again; the layout check's pin at four content sheets; and, in `OMAKASE.md`, the contract, `BD-5`, `BD-7`'s clause and the notes on `PLATE.1`, `PLATE.2`, `FRONT.2`, `FRONT.3`, `BACK.2` and `MISE.6`, whose asks gained the white text and the pointer over a button. Every prediction measured missed in part, each named beside its number. The build found the first frame's press refused, which the hand had taken; the long runs' waiver; a held click that a mutant first passed; and each view's date about 2 µs at every half hour. The hand test asked seven changes, made the same day, the grouping written three ways to cost the load 1.3 ms and nothing at a mark; found and unminted: the white text's deep shade, a dollar's text costing its load until the record holds a number format, and the pointer over a button, Alonzo's.

## 🛎 THE FRONT OF HOUSE

- ✅ **`FRONT.1`, first slice — the workbook: the game's views, each a sheet of its own.** *Scoped 2026-10-15, first as panels on the Screen and then, at the owner's word, as the game's views on sheets of their own, gated on Alonzo; rescoped at the page's version 4 the same day, once `ENGINE.18` was released, its seventeen decisions approved whole by the owner on 2026-10-16, and built and hand-tested that day: both scopings, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* Buffet is played full-screen as a workbook: each of the game's views a sheet of its own, with its own widths, heights, looks and gridlines, reached by the workbook's own tabs and standing first among them, the model's sheets after them, visible (`AD-6`), every name a fund manager's and no tab numbered (`BD-2`'s amendment). The cartridge is written to the page's version 4, `(spec 4)`: the game opens on its first visible sheet with no Camera, which enters with `GUARD.1`; a click on any cell of the sheet shown is the game's, `Input!B19`, `B11` and `B12`, which the first button reads (`BACK.1`); and its Screen, holding nothing, is hidden, the manifest's screen line kept for a Screen a game may show (`FRONT.7`). The first view, the Portfolio: its date, "As of year 1, day 1 (unaudited)"; a row a stock on the Stocks sheet's own rows, each its name read from that sheet, its price today and its change since yesterday in dollars, whole cents over 100 once and the change a difference of cents; and the fine print at its foot, `Disclosures!D3`. Fourteen formula cells, reading the model and the day and never the frame (`BD-7`), and read by nothing, a view being the workbook's output; widths 22, 12 and 12 characters, the default row height, 15 points, kept by the owner's eye, gridlines on, and no background until `FRONT.2`. The caveat on number formats rewritten in the same commit, the first view to show dollars taking it (`BD-5`). Each later view enters with its dish: the Exam with `GUARD.1`, the Overview with `FRONT.3`, Work with `BACK.1`, the market's view with `PLATE.1`, the curve's at course 1. *Oracle, met:* the Portfolio held to the hand at every frame of two years and two days, whole on a day's first frame and byte for byte still through the day, and on every one of eight hundred days at one frame a day, its dollars the Stocks sheet's cents spelled by text, which a test of the hand holds to the record's own spelling from −200,000 cents to 2^52; its headers, and its names the model's; the game opening on it, written to version 4 with no Camera and the Screen hidden; no formula reading a view, each at most 1,000 formula cells; `BD-7`'s scan and the census's `(every-frame 0)`; the describe golden blessed again; eight mutants, each caught by the test written for it; `MISE.3`'s first number re-measured on the real view, in headless Chrome and in the owner's; and the owner's hand test in the page. *Pays into:* every view after it, each a sheet; every button (`BACK.1`, `PLATE.1`, `GUARD.1`), each a click on the sheet shown; the looks (`FRONT.2`), whose glyph column stands beside the change. `~days`

  *Closed 2026-10-16.* Predicted `~days`; cost two sessions, the first scoping and the rescoping, then the build, its measurements and the owner's hand test. What landed: the source's version, notes, hidden Screen, tab order and the Portfolio's section, and its cartridge, 429 rows and 107 formula cells; caveat 12 rewritten; the hand's `dollars` and `as_of` and a test of them; three tests of the cartridge more and three extended, the two years', the eight hundred days' and the tab order's, 45 in all; the describe golden blessed again. Four of the nine predictions missed in part, each named beside its number: an ordinary frame 0.1 µs dearer, five shapes more for the step's own flags; the suite 0.77 s longer at the median where half a second was predicted, a view held at every frame; the source's block 3,243 characters longer where about 2,600; and in the browser the still frame a step of the clock above its prediction and the main thread past it, a frame now about a seventh of `MISE.3`'s.

- ✅ **`FRONT.2`, first slice — the glyph: which way each change went, beside it.** *Scoped 2026-10-16 as the looks, Lovelace Lavender on the views and the glyphs in the owner's colors, a text color to be asked of Frazaro and Alonzo; approved the same day with every color deferred to an aesthetic pass at the owner's word, and so built as the glyph alone, and hand-tested that day: the scoping, the approval, every prediction beside its number and the owner's hand test are under THE ITEMS, SCOPED.* On the Portfolio, a 3-character column `D` beside each stock's change since yesterday, `▲`, `▼` or `=` by `=IF(C5>0,"▲",IF(C5<0,"▼","="))`: the direction of the number beside it, read off it, so that the two never disagree, a whole number of cents over 100 keeping its sign; a view reading its own cells and read by nothing; no header, no look and no color. *Oracle, met:* the hand's glyph, comparing whole cents and sharing no comparison with the cell, held at every frame of two years and two days and on every one of eight hundred days at one frame a day, each of the three shown on some day; `BD-7`'s scan and nothing reads a view; the describe golden blessed again, a view's first `readers` row among its changes; five mutants, each caught by the test written for it; the cost re-measured natively, in headless Chrome and in the owner's; and the owner's hand test in the page. *Pays into:* every figure a later view shows with its direction, the market's view (`PLATE.1`), the overview (`FRONT.3`), the report card (`CRITIC.1`); and the second slice, the looks, which stays in `OMAKASE.md`. `~hours`

  *Closed 2026-10-16.* Predicted `~days` for the whole dish; the first slice cost one session, the scoping, its approval, the build, its measurements and the owner's hand test, beside the pin's move to `alonzo` 0.1.5, a `HOUSE.2` commit of its own. What landed: the Portfolio's column `D` and the glyph's fill in the source and its cartridge, 431 rows and 111 formula cells; the hand's `glyph` and its test; the Portfolio held over `A1:D10`; 46 tests; the describe golden blessed again. Five of the nine predictions missed in part, each named beside its number: the cartridge's characters by three, each escaped glyph counted short; the load under its range and a day's first frame where the fine print stays a tenth under it, both inside the noise; the source's block under its range, its comments shorter than priced; and the main thread's least a hundredth under. The colors stand as the second slice, an aesthetic pass, the scoping's findings with them.

- ✅ **`FRONT.2`, second slice — the looks: Lovelace Lavender on the views, and the glyphs' shades.** *Scoped and built 2026-10-30 at the owner's word, over the pin to `alonzo` 0.3.0 and `vla-lang` 0.12.0, a `HOUSE.2` commit of its own; amended at the owner's two hand tests the same day and passed at the third, 2026-11-15: its decisions, the oracle, the measurements and what the hand tests found are under THE ITEMS, SCOPED.* The views' figures numbers under a format's code, the library's `dollars`, `signed-dollars` and `percent` written over `rule`, the padded texts of nested IFs gone; a button white on Lovelace Lavender's deep `#6B4FC8`, bold but for Trading's grid, and one standing alone bordered in the view's ink; a band of the same shade and ink under each table's headers, and down Trading's keys; a gain's glyph on the owner's mint and a loss's in the burgundy; a bold label over what it heads; and Trading's keys, over each column of buttons and beside each stock not picked. *Oracle, met:* every format a view draws reading at 4.5:1 or more by the viewport's rule; each code held to the hand's spelling; the looks where a person meets them; every frame of every run held; 69 mutants, each caught by a test other than the describe golden. `~days`

  *Closed 2026-11-15.* Predicted `~days`; cost one session, the scoping and the build beside the pin, two rounds of the owner's changes at the hand test, and the third, which passed it. What landed: fifteen formats named by `define` in one section of the source; the kitchen's three verbs of a view's figures over `rule`; the views' looks; Trading's keys; the caveat on number formats rewritten; 96 tests, the describe golden blessed again; the cartridge 84,227 bytes where 99,451, a tick of the session 20.0 µs where 37.9 and the load 8.1 ms where 12.0, and a window viewed whole dearer, 82 µs where 51. Found, each a finding for Alonzo on the asks table and the owner's to pass on: a format's font lost on a patch's second and later rows, which leaves the Exam's options plain from the second question until a pin carries the one-line fix; the loop oracle's `grid` case, which gates `HOUSE.4`'s second slice; a text piece unrefused at the pass; the selection's wash over a button and the date; and a stray key answered in an author's words.

- ✅ **`FRONT.3` — the overview, the ticker, the weather word.** *Scoped and built 2026-10-19 in one sitting at the owner's word, its ten decisions built as recommended; passed at the owner's hand test the same day with no change asked: the scoping, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The dashboard: both pockets' net worth, the day and the year, the firm's name, the weather word from the index's slope, the ticker's headline of the day by `CHOOSE` over the Headlines sheet, course 0's ten headlines flavor alone and moving nothing until `SPECIAL.1`; the alerts that link to their panel (a margin call, a maturity, a redemption) as later courses add them; and the live sheet's strategy guide beside it, a column of payback times anyone can read (`AD-6`). *Oracle:* `buffet.rs`; the headline of the day held to the picker. *Pays into:* every alert a later course raises. *Found at `GUARD.1`'s build, 2026-10-17 (`SPIKE.md`):* the certificate leads to the sheet named at `Proctor!B6`, a value, the Portfolio until this dish writes the Overview's name there; the Camera shows it at the certificate's button, so one value moves the game's first view after the exam. *Scoped and built 2026-10-19 in one sitting, each decision the owner's to approve, amend or strike at the hand test (`SPIKE.md`):* the Overview after the Certificate, where the certificate now leads; the firm's line a placeholder until `BACK.2`; each pocket's net worth; the weather, five words by the market's change in tenths of a percent over the twenty days to the last close made, the session before's in session and the session's own from its close, `Market!B114:B124`, the look back moved with it, so that the word changes at most once a session, at its close; the Headlines sheet, ten rows of `BUFFET.md`'s fifteen, a headline a calendar day by the day's 83rd market draw; the alerts and the guide of payback times left to the dishes that give them something to hold, the Overview binding no key; caveats 21 and 22. `~days`

  *Closed 2026-10-19.* Predicted `~days`; cost one sitting, the scoping, the build, its measurements and mutants, and the owner's hand test, which passed it whole. What landed: the Overview, the sixth view, after the Certificate, `Proctor!B6` naming it; the Market's weather at its foot, `B114:B124`, and its look back moved to the last close made; the Dice's 83rd draw; the Headlines sheet, `src/content/headlines.vla`, ten rows; caveats 21 and 22; the source's notes and comments; 1,258 rows and 805 formula cells; the hand's last close made, its change, its weather and its headline; 81 tests, the Overview and the weather held at every bell and every day's turn of the long runs and at every frame of the order desk's sessions, the run that writes the open reaching Tuesday's close; eighteen mutants caught; the describe golden blessed again; `check_layout`'s content pin at 5; and, in `OMAKASE.md`, `BD-5`'s rows and the notes on `BACK.2`, `MENU.7`, `SPECIAL.1` and `POT.4`. Three of the nine predictions missed in part, each named beside its number: Cash's readers 8 where 9; the bell no dearer where 1 to 6 µs were priced, and the first frame 19.4 µs dearer, under the 20; and the suite cheaper than priced, the hour's year 3% longer, the weekday 1% and the whole 0.15 s.

- ✅ **`FRONT.8` — the night desk: every view a card of midnight plum, as Seiyaku's board and Brickwork's wall are.** *Minted, scoped and built 2026-11-19 in one sitting at the owner's word, "Buffet is boring. It looks boring, for starters, which should be an easy fix. Kairos, Seiyaku, and Brickwork all got aesthetic passes which made them much more playable. Can we do that to Buffet first so my eyes aren't bleeding during my playtest?", before `HOUSE.4`'s second slice records its session; at the owner's look the same evening, "Wow, everything looks much better", two notes built as decisions 9 and 10, the zoom and the back office hidden; at the second, 2026-11-21, the zoom brought to 135% so that a smaller screen never scrolls a card sideways, and the Input tab left for Alonzo's fix first, at the owner's word; at the third, the same day, the hand tests passed and the certificate's lead given back to the Portfolio, where the prices move: the decisions, the comparison and what the build found are under THE ITEMS, SCOPED.* Each of the seven views bound to its extent with its gridlines off, a card 126 characters wide at 100% on the page's gray, so that a tab turned moves no edge but the viewport's rounding to whole pixels, and drawn at the game's own zoom, 1.35, the owner's 135%, its type, columns and rows' heights said once at 100% and handed the zoom by the source's `the-zoom`; the strip the seven cards, every other sheet behind the box of hidden sheets but the engine's Input, an ask of Alonzo; the certificate leading to the Portfolio, its prices moving from the first bell; the whisper on midnight plum, lilac for a label, the date, a hint and the fine print, a view's headline larger in lilac, and the bell's line bold and, by a rule, in the mint while the market is open; a table's band a shade lighter; every button the deep shade lightened until white reads at 4.53:1, so that the lavender is what a click does; the glyphs chips of the owner's mint and burgundy, the stock picked a chip of the whisper; the certificate's title in Georgia; and an error's joke italic and small enough to end inside its card. No formula, address or button's place moved, and one value, the certificate's lead. *Passed at the owner's hand test, 2026-11-21, after three looks: "Passes, commit it".*

## 📜 THE MENU CARD

- ✅ **`MENU.1` — the strings sheets and the picker.** *Scoped, built and hand-tested 2026-10-15: the scoping, its fourteen decisions approved whole by the owner and amended at the hand test, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The content sheets under `src/content/`, a file a sheet holding words alone, one macro named after the file whose body is one call of the library's `strings`, each sheet one kind of string, hidden after the model's sheets, its rows of text each with the course that added it, its vintage, and the reviewer who approved it (*the vintage and the reviewer struck 2026-10-15 at the owner's word, a row's review its pull request's and no course a freeze, `BD-1` amended: `MENU.2`'s amendments after its close*); the source's manifest its first form, then every include, then the code, each content sheet's macro called where the sheet belongs (the owner's words at the hand test); the library's `picker`, `CHOOSE(k, ...)` over a sheet's rows, generated from the sheet's extent so that a row added needs no formula edited, one cell to 1,547 rows below the headers, the pages of 500 written by the dish whose sheet first passes 1,000 rows; the first sheet, the fine print as a rotating footer, the Disclosures sheet, eight rows in the owner's words, a disclosure a month from the day; and the one-page *How to add a row* in the folder, for a stranger with five minutes. Every other sheet arrives with the dish that reads it, named for it: `Questions` (`MENU.2`), `Headlines` (`FRONT.3`), `Memos` (the first of `MENU.4` and `GUARD.3`), `Firms` (`BACK.2`), `Awards` (`MENU.7`), `Errors` (`PLATE.6`) and `Exits` (`FRONT.7`). *Oracle, met:* the layout check's content rule and the source's order (`tools/check_layout.ps1`, the content sheets' pin raised from 0 to 1, its control fifteen mutants), their headers held by `check_spdx.ps1`; the page's content blocks (`tools/build_web.ps1`, `tools/check_web_offline.ps1`, its control fifteen mutants); the oracle's tests, the fine print held to the hand at every frame, the rows counted by their extent, every row picked in its turn, a row added expanding with no other edit, the one picker's limit and the tripwire before it, and every sheet written bare with the content hidden; and the owner's hand test in the page. *Pays into:* every string any view shows: the ticker (`FRONT.3`), the exam (`MENU.2`), the names (`MENU.3`), the errors (`PLATE.6`), the endings (`FRONT.7`); and the cheapest pull request there is. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build, the owner's hand test and its amendment. What landed: `src/content/disclosures.vla` and its *How to add a row*, `src/content/README.md`; `strings` and `picker` in `src/kitchen.vla`; the source's include at its top, its call of the sheet and the fine print's index, and the game's notes rewritten for `BD-2`, the back office open and the front office not; the hand's rotation and five tests of the cartridge, 33 in all; the describe golden blessed again; the layout check's content rule; the page's content blocks; `CONTRIBUTING.md`'s fifth rule; and, in the same commit, the Linux cells CI's run 38712895205 answered for `STATION.1` and `HOUSE.4`. One of the nine predictions missed: the page grew 9,587 characters where about 5,000 were predicted, the library's own block forgotten; and P6 held at the median alone, one round of five past it. After the hand test passed, the owner rewrote seven of the eight rows, grouped every include together and set the manifest first, the includes after it and the code after them; the cartridge moved in those seven rows and nowhere else.

- ✅ **`MENU.2`, first slice — the exam bank: twenty questions and their whys.** *Scoped, built and hand-tested 2026-10-15: the scoping, its fifteen decisions approved whole by the owner, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The Questions sheet, `src/content/questions.vla`, hidden after the Disclosures sheet: twenty graded questions, the nineteen drafted in `BUFFET.md` §3, checked against `BD-2` and for accuracy, and a twentieth written, *Shares*; each its concept, the question, four options, the right one's letter, the letters spread five to each over the bank, and a one-line why a concept card can quote alone (`FRONT.5`); a finance educator's read before the release (`BD-5`), the deposit insurance's figure marked to confirm. The draft's twentieth, ungraded, which sets the archetype, is no row of the bank, being asked last and always and never drawn: its options are recommended to the Firms sheet and its prompt to the Exam view (`GUARD.1`, `BACK.2`), and the contract's sentence reads nine scored questions drawn from the bank of twenty, then a tenth, ungraded. The library's `strings` grown to several fields, each a header and a width, the fine print its case of one and expanding to its bytes unchanged, `picker` unchanged, a picker a field; the row shown, `Questions!M2`, a value until `GUARD.1`'s draw drives it, with the finding for that dish that a stride picked among the primes above 1,547 is coprime to every bank a picker holds, the dialect having no `GCD`. Second slices at each license, with `GUARD.2`, toward forty to sixty. *Oracle, met:* each content sheet held to its file, read by the language's own reader; every question whole; every field picked at every row; a question added expanding with no other edit; the first question shown; the tripwire and the tab order over both sheets; the layout check's content pin raised from 1 to 2, its control fifteen mutants over seven files; the describe golden blessed again; five mutants, each caught by the test written for it; and the owner's hand test in the page. *Pays into:* `GUARD.1` and every exam after it; every content sheet of several fields after it. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: `src/content/questions.vla`; `strings` of several fields in `src/kitchen.vla`, the Disclosures file's one line of fields respelled; the source's include, call and row shown, and the game's notes; five tests of the cartridge more and three extended, 38 in all; the describe golden blessed again; the layout check's pin and control; the *How to add a row* for a sheet of several fields; and, in `OMAKASE.md`, the contract's sentence, `GUARD.1`'s and `BACK.2`'s notes and the hoist ledger's row. Two of the nine predictions missed: the pass took 1.7 ms more where under one was predicted, a cost paid at a build and never at a frame, and the first frame some 40 µs more where under 10 was predicted; every frame after it held. After the hand test passed, the owner rewrote eighteen of the twenty questions and title-cased the bank's headers; the cartridge moved in seventy rows of text and nowhere else. After the close, the same day and at the owner's word, the approver and then the vintage were struck from every row of both sheets, a row's review its pull request's and no course a freeze of the rows before it (`BD-1`, amended); the cartridge lost 62 rows, 347 where 409 stood (THE ITEMS, SCOPED).

- ✅ **`MENU.5` — the accuracy sheet: *Things this game gets wrong*, and *not advice*.** *Scoped, built and hand-tested 2026-10-15: the scoping, its fifteen decisions approved whole by the owner with the name the owner preferred, the amendment the approval and the moving base made, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The Caveats sheet, `src/content/caveats.vla`, the one content sheet shown among the tabs, after the model's sheets, since a player reads it whole and no view can show a sheet's rows whole: a caveat a row, its fields what the game gets wrong, what the real world does, Management's response, the joke in the voice of a model's owner answering a finding, and where, the cells it qualifies as a formula writes them, held to a range that holds a value. Its first row is the plain statement that the game is not advice, beside its joke, *Any resemblance to financial advice, living or dead, is purely coincidental*, which the certificate reads, `Caveats!G4:G5`, at the row shown the source writes, 1. Course 0's twelve rows, three of them correcting lessons tier 1 would teach by accident, that the future can be read, that holding the four stocks spreads risk and that risk always pays; four claims of the world marked for a finance educator's read before the release (`BD-5`). The library's `strings` writes no sheet's state, the engine taking one a sheet: the source writes each content sheet's beside its call, hidden but for this one. The README and `SECURITY.md` say it plainly, in American English, and `BD-5` names the sheet and the nine caveats later dishes owe. *Oracle, met:* each content sheet held to its file and in the state its source writes; every caveat whole, the first the plain statement; every caveat's cells holding a value; the first row shown; every field picked at every row; the layout check's content pin raised from 2 to 3, its control fourteen mutants over eight files; the describe golden blessed again; seven mutants, each caught by the test written for it; and the owner's hand test in the page. *Pays into:* every later simplification has a row to go to; the certificate's plain statement (`GUARD.1`). `~hours`

  *Closed 2026-10-15.* Predicted `~hours`; cost one session, the scoping, the build and the owner's hand test, the content rows' strike committed beneath the build (`adc1bc7`), which was built on a snapshot of it in a scratch worktree and moved onto the commit unchanged. What landed: `src/content/caveats.vla`; `strings` without a sheet's state, in `src/kitchen.vla`; the source's include, the three states, the call, the row shown, the labels' width and the game's notes; three tests of the cartridge more and two extended, 41 in all; the describe golden blessed again; the layout check's pin and control; the *How to add a row* for the third sheet; the README's and `SECURITY.md`'s statements, their British spellings respelled; and, in `OMAKASE.md`, `BD-5`'s line, `GUARD.1`'s note and the hoist ledger's row, and `BUFFET.md` §10's line. Two of the nine predictions missed in part: the describe golden's `most`, 8 where 4 was predicted, the census indexing a sheet's readers by row; and the source's block in the page, 2,062 characters where 1,400. A mutant found the test of the plain statement too weak at its first writing, the word *advice* standing in the caveat's explanation too, and it was strengthened before the hand test.

## ⭐ THE CRITIC

- ✅ **`CRITIC.1` — the report card: alpha over the same seed.** *Scoped and built 2026-10-19 in one sitting at the owner's word, its eleven decisions built as recommended; passed at the owner's hand test the same day with no change asked: the scoping, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* At every quarter's end and every year's end (*the quarters the owner's word of 2026-10-16 at `POT.10`'s approval: "report cards should come quarterly and annually instead of just annually"; the first card comes about 36 minutes in at the manifest's rate, the quarter already the model's in the dividend's 91 days, and at this seed it reads the market near the year's low; at 1:11:03 of play since `POT.11`'s pace, 2026-10-19, and at 1:11:27 since the scoping, the quarter's turn at Monday's midnight*) a Report sheet and panel: the pocket's return against buy-and-hold of the broad rung, on the same seed, and against a 60/40 once bonds exist; Sharpe and maximum drawdown beside (*at course 3 since the scoping, its risk line*); the tier and the seed printed; a grade and the proctor's remark; the course's own line, added a course at a time (income at 1, the firm at 2, drawdown at 3, the option book at 4, the bot at 5, every class at 6). The score the open-book problem cannot spoil (`BD-3`). *Oracle:* the Report sheet is the replay golden's window; the benchmark held to `buffet.rs`. *Pays into:* every course's ending reads here; the league table (`CRITIC.2`), the leaderboard (`CRITIC.3`), the classroom's rubric (`CRITIC.5`). *Found at `BACK.2`, 2026-10-19 (`SPIKE.md`):* the header's firm name is the Firms sheet's `E4`, read beside the Proctor's style, `B40`, which is 0 and the name pending registration until the exam's last answer; and the broad rung the card scores against is named, Average Co., on its own row of the Stocks sheet, `A7`. *Scoped and built 2026-10-19 in one sitting (`SPIKE.md`), each decision built as recommended, the owner's to approve, amend or strike at the hand test, every text a draft:* the firm's book graded, its capital taking no flow at course 0, so its plain return is its time-weighted one, your own pocket ungraded until the firm line; against Average Co. bought with the capital at the begin's price, $25.00 at every seed, in whole shares, the cents left over held, and held; a card at each quarter's turn, quarters of 91, 91, 91 and 92 days on the Day sheet (`B33:B35`), the year's card the fourth's second column, each a snapshot until the next; the returns over the quarter and the year so far in whole hundredths of a percent, alpha their difference, a grade each by a table of five, A from two points ahead to F past two behind, and the proctor's remark from a content sheet of its own, Remarks; the view the Report, after Gigs, which the Camera shows at every card, naming nothing between; its books the Performance sheet, after the Ledger; `kitchen.vla`'s `percent`; caveats 24 and 25; Sharpe and the maximum drawdown moved to course 3, measured meaningless at tier 1 (a quarter's Sharpe ratio -12.5 to 28.2 at the seed, every drawdown after the first quarter 0.000%). `~days`

  *Closed 2026-10-19.* Predicted `~days`; cost one sitting, the scoping, the build, its measurements and mutants, and the owner's hand test, which passed it whole. What landed: the Report, the seventh view, after Gigs, which the Camera names at every card and at nothing between; the Performance sheet after the Ledger, the benchmark's shares and cents left `B3:B5`, the card's turn and the year's `B6:B10`, the worths `A12:G14`, the card `B17:C22` and the grades `A25:B29`; the Day sheet's quarter, its count and its turn, `B33:B35`; the Remarks sheet, `src/content/remarks.vla`, five remarks, the layout check's pin raised to 6; `kitchen.vla`'s `percent`; caveats 24 and 25; the source's notes and comments; 1,389 rows and 861 formula cells; the hand's quarters, returns, grades, percents and cards, and the exam's begin by its frame; 92 tests, the Performance sheet held at every card and every day's turn of the long runs and the Report at every card, bell and day's turn, a year at a day a frame with every card shown and the tabs between, and a scripted quarter at an hour a frame and a day a frame; thirty mutants caught; the describe golden blessed again; in `OMAKASE.md`, the income and the firm lines' notes, the risk line renamed from the drawdown line, `BD-5`'s rows, `POT.6`'s note and the hoists' `percent`; and `Delta/CLAUDE.md`'s map. Four of the ten predictions missed in part, each named beside its number: the step's half hour between sessions 1.8 µs dearer for the Report's own date line, the bell and the close inside their noise, and the first days' midnights cheaper than priced; the eight hundred days 14% longer where 3 to 10%, every frame there a day's turn; the caveats' block 623 characters, under the 700 priced; and thirty mutants where twenty to twenty-six.

## 🔪 MISE EN PLACE

- ✅ **`MISE.3` — the first measurements: three probes, each a day in scratch, each deciding a spelling.** *Scoped, measured and hand-run 2026-10-14: the scoping, every number beside its prediction, the owner's hand run and what it found are under THE ITEMS, SCOPED.* Run over Alonzo's own page and runner with a Buffet-shaped scratch cartridge, before a row of the game is written, and each number written in `SPIKE.md` beside its prediction so that course 0's scoping cites them. (1) **The window's cost.** An 80 by 50 panel of texts and numbers drawn in Chrome at `(rate 30)`. The host draws from the full record until `ENGINE.8`'s second slice lands, and the record carries every formula's text: by `vla-lang` 0.9.3's release notes, an 80 by 50 window of one short formula costs 10.3 ms natively from the record and 1.7 to 1.9 ms from the `values` view (1.3 to 1.4 in the browser). A panel of many short formulas is Buffet's shape and nobody has measured it. The number decides whether course 0 ships over the record or asks Alonzo for the slice with the number in hand (`MISE.6`), and it is re-measured on the real window at first light (`HOUSE.4`). (2) **A number format with a glyph.** Whether the viewport honors a number format with a literal glyph in it (the record's `format` row carries a number format and `KERNEL.5`'s painter spells numbers by it; a `▲` in a format is (check)), which decides `FRONT.2`'s spelling of gains: a format, or a glyph cell beside the number. (3) **The picker's limit.** `CHOOSE` over references against the 8,192-character formula, about a thousand references, and the text join `&`, which the dialect has (the language's `Concat`, checked 2026-10-14), so that a ledger row's address may be joined from a count and not chosen; the number decides `MENU.1`'s picker and `PLATE.1`'s ledger ring. The fourth question the draft asked, the boundary frame against the ordinary frame, needs a model to measure and is answered by the first model: it is `POT.1`'s oracle. *Needs:* nothing. *Oracle:* the three numbers written beside their predictions. *Pays into:* `MISE.1`'s decisions, `FRONT.2`, `MENU.1`, `PLATE.1`, the ask of `ENGINE.8`. *Measured 2026-10-14, every number beside its prediction in `SPIKE.md`:* course 0's densest panel draws a frame in 1.5 to 1.7 ms in Chrome at full screen, so course 0 ships over the record; the spellings the numbers settle, approved by the owner the same day, stand as dated lines on `FRONT.1`, `FRONT.2`, `MENU.1`, `PLATE.1`, `MISE.1` and `MISE.6`; the owner's hand run in the owner's own Chrome made the browser number real, 1.4 ms a frame at p50. `~days`

  *Closed 2026-10-14.* Predicted `~days`, a day a probe; cost one session, the three probes and the owner's hand run. What landed: no file of the game, since every probe lived in scratch; in this file, the scoping, its predictions with the numbers beside them, what the probes and the hand run found, and the spellings; in `OMAKASE.md`, six spellings approved on their items and three candidates in `MISE.6`'s table. Five of the seventeen predictions missed in part, each named beside its number: the record's bar falls near 1,400 formula cells and not 2,000; a background spelled by cells costs more in Chrome than predicted; a joined text's two decimals take 140 characters; the picker's text costs a view 43 µs; the way out takes five cells and not four. The probes' premise for (2) did not hold, which the scoping found by reading before any probe ran.

- ✅ **`MISE.1` — the kitchen's layout, first slice: one folder, one library, a source a menu of calls, the content as files, and the checks the house keeps.** *Decided 2026-10-14 at the owner's word, at `POT.1`'s build:* the game lives in one folder, `src/`, and not in Alonzo's `cartridges/<name>/`, since this repository is one game's: the source, `src/buffet.source.vla`; the cartridge the pass writes beside it, `src/buffet.vla`, committed so that a stranger loads it without building; Buffet's own library, `src/kitchen.vla`; the kit's copy, `src/common.vla` (`HOUSE.3`); and later the content sheets under `src/content/` and the frozen courses under `src/courses/`. `REUSE.toml` names the game's own files, `src/buffet*.vla` and `src/courses/**`, the content and the kit's copy, and `tools/check_spdx.ps1` holds every text file under `src/` to the header the map gives it. The source includes the kit and the library, each by its own line, and the library includes nothing: the pass resolves every include against the source's own folder, an included file's includes too, so a library that included the kit would name it by a path true at one depth only. The library grows by the ratchet, a verb entering at its second appearance (`draws`, a stream's draws beneath its die, entered with `POT.1`, which writes two streams; `panel` and `tab`, a Screen region and the Camera move that shows it, when the second panel is written; `picker`, `CHOOSE` over a sheet's rows, at `MENU.1`; `timer` at `PLATE.3`; `ladder`, a station's rungs as rows from a list, at `STATION.2`; `station-total`, a roll-up over the stations a source names, at `BACK.3`). A course's source is a short menu of calls naming the dishes it serves and including the content sheets, one file a sheet, the stranger's path under the content's own terms (`MENU.1`, `REUSE.toml`); the cartridge carries the source's header, the game's expression, and is always the latest course, the one the page carries. The pass is the engine crate's, `alonzo::pass::expand`, driven by a runner of the oracle crate in the shape of Alonzo's own, `cargo run --release --example expand [-- --check]`, since a dependency's examples are not the workspace's to run. *Built with `POT.1`, 2026-10-14:* the folder, the library, the source and its cartridge, the runner, and the oracle crate's tests that the cartridge is its source expanded and that `describe` holds its golden, `oracle/testdata/describe/buffet.vla`, which `BUFFET_BLESS=1 cargo test describe` writes again. *Built at the house's close, 2026-10-14, in place of a port of Alonzo's cartridge check (805 lines), since a row is the engine's to judge and the oracle crate's tests load the cartridge through `alonzo` on every push:* the layout check, `tools/check_layout.ps1`: `src/` holds exactly the source, its cartridge and the two libraries, any other file failing by name, each kind counted against its pin (sources 1, cartridges 1, libraries 2, content sheets and courses 0 until they exist); the cartridge's second line is the pass's banner naming its source; the source includes `common.vla`, then `kitchen.vla`, each by its own line, and `kitchen.vla` includes nothing. The rest of the slice went where it belongs: the content sheets' folder rule to `MENU.1`, the replay test to `COURSE.0`'s tasting, the three open decisions to the second slice (`COURSE.1`), and the picker's approved shape to `MENU.1`. *Decided now, built at course 1:* each course served is frozen as a cartridge folder of its own, `src/courses/buffet<k>/` (rec; the names the scoping's), holding its source, its cartridge, its replay and its golden, its includes `../../common.vla` and `../../kitchen.vla`; the folder rule applied there is the second slice's. (*Struck 2026-10-15 with that second slice, at the owner's word: no course is frozen, `BD-1` amended, and its line left `OMAKASE.md` unbuilt; the layout check's courses kind, pinned at 0, and its mutant, and the license map's `src/courses/**`, struck with it the same day.*) *Needs:* `HOUSE.3`; `MISE.3` for the picker's spelling. *Oracle, met:* the check green; `-Control`, a clean copy passing and seven mutants each failing by name and proving it applied (a stray file under `src/`, the cartridge missing, a banner naming another source, an include in `kitchen.vla`, the source's includes out of order, a content sheet and a course before their pins are raised); `run_checks.ps1`'s floor raised to 3. *Pays into:* every dish, each a call of the library and a sheet of the source; `HOUSE.4`'s places. `~days`

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

## `FRONT.1`, rescoped — the Portfolio and first light: the game's first view, a sheet of its own, at version 4

*Rescoped 2026-10-15 at the owner's word, "Scope and implement FRONT.1 in
Buffet (Delta/Buffet): the workbook, the game's views each a sheet of its
own, and first light", to stop for the owner's approval before building:
from `OMAKASE.md`'s line (`COURSE.0`'s item 5) with its note of the release
and the numbers the owner approved from `MISE.3`; the first scoping above,
the stacked panels struck, the owner's two readings and "What `FRONT.1`
becomes"; `BD-2` with both its dated notes, `BD-5`, `BD-6`, `BD-7` and
`BD-8`; `MISE.3`'s numbers and spellings; and Alonzo's `SPEC.md` at
version 4 (§3.1, §3.3, §3.8 and decision 36), with `ENGINE.18`'s scoping
and record in its `REARVIEW.md`. It comes now because `alonzo` 0.1.4
released `ENGINE.18`, and with it `ENGINE.19` and `ENGINE.20`, a frame read
and painted by its changes, and Buffet's pins moved there the same day, the
page copied again at `v0.1.4` (`HOUSE.2`, `bd93434`): the gate is open, and
this rescoping is the dish's first act. Every course-0 dish after it needs a
view or a click on one (`BACK.1`, `PLATE.1`, `PLATE.2`, `FRONT.3`,
`GUARD.1`). Each decision carries its recommendation, for the owner to
approve, amend or strike; the texts a player reads are drafts, for the owner
to approve or rewrite. The predictions were written before any number was
taken. Alonzo's tree was read and never touched.*

**What it stands on, read on the day** (`alonzo` 0.1.4 and `vla-lang`
0.9.4 as the registry holds them; `web/` as vendored at `v0.1.4`; Buffet at
`b11bcd7`, its tree clean, its main eight commits ahead of its remote).
- *The version.* Version 4's rows and behaviors are a `(spec 4)`
  cartridge's alone (`SPEC.md` decision 36). Buffet says `(spec 3)`, so
  today the page opens it on its Screen, empty and first among the tabs, a
  click off the Screen is 0, and its Input sheet holds eighteen rows. At
  `(spec 4)` the engine takes the Input sheet to `A1:E19` and the Camera to
  `A1:B3`, makes the Input's nineteenth label, `sheet`, and `describe`
  prints the Input's extent as `A1:E19` (`cartridge.rs`, `Device::layout`,
  `INPUT_SHEET_LABEL`).
- *The opening* (`host.js`, `_open`): the sheet the Camera's third row
  names at the load, else the first visible sheet of the cartridge's own
  order. Buffet has no Camera, so a Portfolio listed first among the visible
  sheets opens the game with none, `ENGINE.18`'s own reading of Buffet (its
  decision 2). A hidden Screen is never viewed until it is shown (§3.1).
- *The sheet shown and the click* (§3.3). At `(spec 4)` the host writes
  `Input!B19`, the name of the sheet its viewport shows, among the Input
  rows that changed (`ENGINE.19`); `B11` and `B12` are the pointer's column
  and row on that sheet, any cell in view, and 0 off the cells; a button is
  `B19`, `B11`, `B12` and the rising edge of `B13`. The wheel scrolls a
  view. The drawer's viewer shows any sheet beside the player's, the Input
  sheet among them, so `B19` can be read while the Portfolio shows.
- *The draw* (§3.1, §5). Every sheet drawn is the values record after
  frame 0 (`ENGINE.18`); a frame whose window holds no change is neither
  viewed nor painted, a still frame (`ENGINE.17`, `ENGINE.19`); and a patch
  naming at most a quarter of the cells in view, 2,720 at the owner's full
  screen, is repainted by its cells alone (`ENGINE.20`; `viewport.js`,
  `REPAINT_SHARE`). The drawer counts them, "S of N frames still and P
  patched". Measured in Alonzo's record over Buffet's Stocks shown in the
  Screen's place: a still frame's main thread 0.42 to 0.46 ms, a market
  day's 0.6 to 0.8.
- *What a view declares* (`vla-lang`'s `machine/load.rs`; the viewport's
  `readRecord`): a column's width in characters and its format, `(column
  ...)`; a row's height in points, one `(row ...)` a row; `(gridlines
  "<sheet>" on|off)`; a format's fill, `general` or `text`, wrap or not; a
  cell's style and a value's look. No font, weight, text color or
  alignment: a number stands right and a text left, and a text spills only
  into the empty cells on its right. The default cell is 64 by 20 pixels,
  Excel's 8.43 characters and 15 points, about 7 pixels a character.
- *The cells a view reads.* The year and the day of the year, `Day!B6` and
  `B7`, each from 1 and each moving on a day's first frame alone; the
  stocks' names, `Stocks!A5:A8`, `rung 1` to `rung 4`, the working labels
  until `MENU.3` names them; the prices today and yesterday in whole cents,
  `Stocks!E5:F8`, the change between them a view's to compute (`STATION.1`'s
  source: "the change between them, and its glyph, are a panel's"), and
  yesterday's on the first day being day −1's, so that the first day's
  change is the start less day −1's price; and the fine print,
  `Disclosures!D3`, a disclosure a month.
- *Money* (`FRONT.2`'s rule, approved at `MISE.3`): whole cents in every
  model cell, reaching a view by one division by 100, which draws its own
  decimals at their shortest, 12.3 for $12.30 and 25 for $25.00, below $70
  trillion (*derived*). The caveats' row 12 says money reads in cents, "9921
  for $99.21", its `Where` `Stocks!D5:F8` in a column 13 characters wide;
  `BD-5`'s owed list gives its rewrite to `FRONT.2`, and the owner's prompt
  for this item to whichever dish first shows dollars.
- *The census* (`vla-lang`'s `calc/shape.rs`, `Shapes::of`, and
  `calc/reach.rs`): shapes are numbered walking the sheets in tab order and
  each sheet's cells by row, so a view standing before the model renumbers
  every shape after it; a sheet's readers are each shape's references to it,
  single cells side by side in one row read as one; and from sixteen readers
  a sheet keeps them indexed by where a changed cell stands, `(most k)`.
- *The names taken.* The devices; the model's sheets, Day, Dice, Market and
  Stocks; the content sheets built, Disclosures, Questions and Caveats, and
  those recommended to their dishes. The market's view and the model's
  Market sheet cannot share a name, `PLATE.1`'s to settle (the first
  scoping).
- *The page.* The player opens on its empty sheet, Buffet in the picker
  after it (`ENGINE.15`), and runs a cartridge the moment it is picked; its
  strip shows the visible sheets in tab order and lists the hidden in its
  box, "hidden sheets (13)" today. Build in the drawer writes the cartridge
  from the source's block.
- *The number this re-measures*, `MISE.3`'s probe 1: course 0's densest lean
  panel, 95 formula cells drawn from the full record at every frame, a
  frame's tick 1.5 to 1.7 ms at p50 and 2.4 to 2.7 at p95 in headless
  Chrome at full screen; in the owner's own Chrome 1.4, 2.2 and 2.7.
- *No replay is committed*: the tasting records the first.

**The decisions.**

1. **One view, the Portfolio (rec).** The line's plan: the Portfolio first,
   the sheet the game opens on until the Exam exists, and each later view
   entering with the dish that fills it, the Exam with `GUARD.1`, the
   Overview with `FRONT.3`, Work with `BACK.1` and the market's view with
   `PLATE.1`, so that no sheet stands empty waiting and each view's scoping
   lays out its own. No verb enters the library: the first view is written
   by hand, and a pattern two views write alike enters `kitchen.vla` at the
   second, where `MISE.1` had a `panel` verb waiting for the second panel.
   *The alternative:* every course-0 view now, an empty Market, Overview,
   Exam and Work beside the Portfolio, each a title read by nothing until
   its dish.

2. **The page's version 4, `(spec 4)` (rec).** The views need it: at
   version 3 the page opens Buffet on its Screen and takes no click off it.
   What moves with it: the manifest's line; `describe`'s Input extent; the
   Input sheet the engine makes, its nineteenth row written by the host
   with the sheet shown; the pointer, the game's over any cell of the sheet
   shown, though no formula reads it until `BACK.1`'s button; and a
   replay's header, `(spec 4)`, with forty-nine Input rows at its first
   frame. No replay is committed, so none moves; the tasting records the
   first at version 4. The contract's `(spec 3)` in `OMAKASE.md` becomes
   `(spec 4)`, dated. The oracle's scratch cartridges, the smoke test's, the
   picker's limit's and the draws', keep `(spec 3)`: they hold the pin, the
   library and the kit, which version 4 does not touch. *The alternative:*
   none that builds a view.

3. **The opening: the Portfolio, the first visible sheet, and no Camera yet
   (rec).** Listed first among the visible sheets, the Portfolio opens the
   game under `ENGINE.18`'s rule with no Camera at all. The Camera enters
   with `GUARD.1`, the first dish whose game shows a sheet, its exam ending
   on the Portfolio by its third row's change,
   `=IF(<the exam done>,"Portfolio","Exam")`. *The alternative:* a Camera
   now, the kit's
   `camera` at row 1 and column 1 and `B3` "Portfolio", a value: the
   opening stated in a cell a player can read, and a save opening where its
   game stood (`ENGINE.5`, course 7), at six rows and a device's tab in the
   strip that nothing reads until `GUARD.1`.

4. **The Screen hidden, first, where it stands (rec).** `(sheet "Screen"
   hidden)`, a row of its own before the sheets row, version 3's own
   spelling (`SPEC.md` decision 41): the strip keeps it in its box, nothing
   views it until it is shown, it holds no cell, and the manifest's `(screen
   grid 80 50)` keeps its words, the window a Screen shown is drawn
   through, so that an ending or a crash can be a Screen the game shows
   (`FRONT.7`). *The alternatives:* no Screen, which loads (`ENGINE.18`'s
   substrate) but leaves the manifest's line describing nothing and
   `describe` without its row; or hidden after the content sheets, beside
   the devices it is one of.

5. **The tab order: the views, the model, the content, the devices (rec).**
   `(sheets "Portfolio" "Day" "Dice" "Market" "Stocks")`: the views first,
   then the model's sheets, visible, the spreadsheet a player may open
   (`AD-6`); then the content sheets where the source calls them, each in
   the state its source writes, the Disclosures and the Questions hidden
   and the Caveats shown; then the engine's Clock, File and Input. The strip
   reads Portfolio, Day, Dice, Market, Stocks, Caveats, Clock, File and
   Input, and its box "hidden sheets (15)": the Screen, the Disclosures,
   the Questions and twelve twins. A later view stands among the views, its
   place its dish's, and no tab is numbered (`BD-2`'s amendment). *The
   alternative:* the model's sheets hidden behind the views, a back office
   out of the strip, open by `AD-6` but no longer in plain sight.

6. **First light: the date, the four stocks, each its price and its change
   since yesterday, and the fine print at the foot (rec).** The Portfolio
   laid out as a fund's holdings report, its stocks on the Stocks sheet's
   own rows, so that a stock's row in the view is its row in the model:

   | | A | B | C |
   |---|---|---|---|
   | 1 | the date, `As of year 1, day 1 (unaudited)` | | |
   | 4 | `Security` | `Price ($)` | `Change ($)` |
   | 5 to 8 | each stock's name, `rung 1` to `rung 4` today | its price today, in dollars | its change since yesterday, in dollars |
   | 10 | the fine print, a disclosure a month | | |

   The date and the fine print spill over the empty cells beside them. The
   shares, the cost, the value and the gain are `PLATE.1`'s columns beside
   the change, and the pockets' block is `PLATE.2`'s, laid out by that
   dish. The source writes the view last, a section of its own after the
   caveats', since it reads the sheets before it; its sheet stands first
   because the sheets row names it first:

   ```text
   (on "Portfolio"
     (column "A" 22 shown none)                 ; the names; the date and the fine print spill past it
     (column "B" 12 shown none)                 ; the price, in dollars
     (column "C" 12 shown none)                 ; the change since yesterday, in dollars
     (formulas
       ("A1" "=\"As of year \"&Day!$B$6&\", day \"&Day!$B$7&\" (unaudited)\""))   ; the date, from the day
     (headers "A4" "Security" "Price ($)" "Change ($)")
     (formulas
       ("A5:A8" "=Stocks!A5")                   ; each stock's name, the model's
       ("B5:B8" "=Stocks!E5/100")               ; its price today: whole cents over 100, once
       ("C5:C8" "=(Stocks!E5-Stocks!F5)/100")   ; its change since yesterday: a difference of cents, over 100 once
       ("A10" "=Disclosures!$D$3")))            ; the fine print, a disclosure a month
   ```

   Fourteen formula cells, where a view may hold 1,000 in view. The date's
   *unaudited* is the sheet's one joke, an interim statement's own word and
   true of every number on it until the examiner calls (`GUARD.3`); the
   headers name their columns and carry none, which is their reason
   (`BD-2`). *The alternatives:* the date as two labels and two values,
   `year` and `day`, as the model's sheets write a setting; `Day 1` alone,
   the line's "the day"; the table from row 3, its rows not the model's.

7. **Dollars: whole cents over 100, once (rec).** A price is
   `=Stocks!E5/100` and a change `=(Stocks!E5-Stocks!F5)/100`, the
   difference taken in whole cents, exactly, and divided once: `FRONT.2`'s
   rule as approved, no money cell a difference or a product of dollar
   amounts, so none draws binary noise or `#`s. A number cell draws its
   shortest spelling, right-aligned, its decimal points out of line; two
   decimals kept would take a joined text of 140 characters a cell, which
   the owner kept for a sentence and not a column (`MISE.3`). *The
   alternative:* cents, as the Stocks sheet shows them, `Price (cents)`, no
   caveat moved, a fund's report in pennies.

8. **The change a number; the glyph and its shade `FRONT.2`'s (rec).** A
   loss reads its minus sign and a gain none, as a plain cell does.
   `FRONT.2`, the next dish, brings the glyph cell, `▲`, `▼` or `=` in a
   3-character column, and the shade keyed on its text together, a glyph
   without its shade being half of one look; nothing reads a view (decision
   10), so it inserts its column where it likes. *The alternatives:* both
   now, the glyph's cell unshaded and `FRONT.2` bringing the shades alone;
   or the change in percent beside it, `=INT(10000*(E5-F5)/F5+0.5)/100`,
   hundredths of a percent rounded once, which reads a $1 share's cent, 1%,
   more truly beside a $100 share's 37 cents, 0.37%: a column the line does
   not name.

9. **The names the model's (rec).** `=Stocks!A5`, filled down: the view
   shows the Stocks sheet's names, `rung 1` to `rung 4` until `MENU.3` names
   them on their own rows ("a rung's name belongs on the rung's own row of
   the Stocks sheet", `MENU.1`'s decision 2), and holds none of its own, so
   the day they are named the view follows with no edit. The header over
   them is `Security`, a holdings report's word, under which a bond's row
   also stands at course 1. Four cells computed once, their inputs values.
   *The alternative:* the names as the view's own texts, two places to
   rename.

10. **A view reads the model and the day, and nothing reads a view (rec).**
    Every formula of the Portfolio reads the Day, the Stocks or the
    Disclosures sheet, and none the frame, `Clock!B1`, or the phase,
    `Day!B4`: `BD-7`'s scan holds it as it stands, the census keeps
    `(every-frame 0)`, and the view moves on a day's first frame alone, a
    still frame on every other. And no formula outside a view reads a view:
    the views are the workbook's outputs, the model its workings and the
    Input sheet its inputs, ICAEW's tenth principle as `SPEC.md` §3 cites
    it, so that any dish may lay a view out again without moving a
    formula, and a view costs what it shows. A view may read its own cells,
    a total of its own column. `describe` shows the rule: no `readers` row
    names the Portfolio. Held by a test, with the approved cap beside it, a
    view holding at most 1,000 formula cells, a tripwire as the picker's
    1,000 rows are. *The alternative:* views free to read one another, the
    Overview reading the Portfolio's totals; a view then a model too, its
    layout dear to change.

11. **The view's own widths; the default height; gridlines on; no
    background (rec).** Widths A 22, B 12 and C 12 characters, `MISE.3`'s
    probe's name and number columns, so that a name, a price and a change
    each read whole and a name of twenty characters fits; set by hand,
    `ENGINE.21`'s fit being unbuilt. The height the default, 15 points, no
    `row` row written: the 20 points approved at `MISE.3` were for a
    fifty-row panel to fill a full screen, a premise a sheet that scrolls no
    longer holds, and the owner judges by eye at the hand test, 20 points
    being then a `row` row for each of the view's ten rows. Gridlines the
    default, on, a workbook's; no background, the looks being `FRONT.2`'s,
    whose approved spelling is the columns' format and never a filled cell.
    *The alternatives:* 20 points now; gridlines off, a report's page,
    which leaves the table floating until `FRONT.2`'s fills.

12. **The caveat on number formats rewritten here (rec).** The first view
    to show dollars takes `BD-5`'s owed rewrite: row 12 in its own file and
    in this commit, its `Where` the Portfolio's money, the field's width
    raised from 13 to 15 to hold it.

    | The field | Now | Rewritten |
    |---|---|---|
    | Things this game gets wrong | Numbers have no formats: a price reads in cents, 9921 for $99.21, and a yield of 2% reads 0.02. | Numbers have no formats: $12.30 reads 1230 in the model's cents, 12.3 on the Portfolio; 2% reads 0.02. |
    | In the real world | A workbook formats its numbers for people: $99.21, 2.00%, 1,000,000. | A workbook formats its numbers for people: $12.30, 2.00%, 1,000,000. |
    | Management's response | The decimal point has been furloughed pending a review of number formats. | kept |
    | Where | `Stocks!D5:F8` | `Portfolio!B5:C8` |

    `BD-5`'s owed list gains a dated line: the dollars' caveat taken by
    `FRONT.1`. No other simplification enters with this dish. *The
    alternative:* the row left to `FRONT.2`, as `BD-5` has it, the Portfolio
    showing dollars one dish before its caveat says so.

13. **The notes: the front office open, though not yet for business
    (rec).** The manifest's notes, a player's string (`BD-2`) the drawer
    shows, rewritten: "Buffet: All You Can Sheet. The front office is open,
    though not yet for business: the game opens on the Portfolio, the day
    and four stocks on it, each its price and its change since yesterday, a
    market day a second, with the fine print at its foot. Behind it, the back
    office: the market's calendar; its dice, rolled once a day where anyone
    can watch them roll; the market itself, four waves and a drift; four
    stocks priced off it in whole cents, the dearest paying a dividend four
    days a year; the fine print, a disclosure a month; the Series 777's
    question bank, twenty questions awaiting an exam the regulator has yet
    to schedule; and the model's caveats, the things this game gets wrong,
    each with Management's response. Pick the Day, the Dice, the Market or
    the Stocks sheet to watch the model work, or the Caveats sheet to read,
    or the Disclosures or the Questions sheet among the hidden sheets." *Open
    for business* is the course's own name, and the front office opens a
    dish before the first order can be placed (`PLATE.1`). The source's
    opening comment lists `FRONT.1` among the dishes served.

14. **The oracle (rec).** In `oracle/src/buffet.rs` and
    `oracle/tests/cartridge.rs`:
    - *The hand's dollars, spelled by text*: `dollars(cents)`, whole cents
      written as their dollars, a point and the cents less their trailing
      zeros, a minus before a loss, sharing nothing with the record's
      formatter; and its test, that it is Rust's own `{}` of the cents over
      100, the record's spelling (`SPEC.md` §3.1), for every count of cents
      from −200,000 to 2,000,000 and at each power of two to 2^46, so that
      `MISE.3`'s derived claim is held on the platform. And `as_of`, the
      date's text from the calendar.
    - *The Portfolio at every frame*, inside `held`, so at every frame of
      two years and two days and on every one of eight hundred days at one
      frame a day: `A1` the hand's date; `B5:B8` and `C5:C8`, each rung's
      price and change, `dollars` of the hand's cents; `A10` the fine print
      the hand picks.
    - *Its columns*: the headers in `A4:C4`; the names in `A5:A8` the Stocks
      sheet's, after the first frame.
    - *The opening and the tabs*: the manifest's `(spec 4)`; every sheet in
      its order and its state, the Screen hidden first and the Portfolio the
      first visible, so that the game opens on it under `ENGINE.18`'s rule;
      and no Camera, which would open another (the tab order's test, its
      list rewritten).
    - *Nothing reads a view, and the cap*: no formula of a sheet that is no
      view names a view, read without case; every view at most 1,000
      formula cells, counted from the cartridge's rows.
    - *Kept*: the cartridge its source expanded; `BD-7`'s scan and the
      census's `(every-frame 0)`; every caveat whole and its `Where` holding
      a value, row 12's now on the Portfolio; the describe golden, blessed
      again and read in its diff, where every shape's number moves by five,
      the Portfolio's five first.
    - *Mutants*, each expanded into the cartridge before the suite runs and
      failing the test written for it: a price in cents, `=Stocks!E5`; a
      change of dollar amounts, `=Stocks!E5/100-Stocks!F5/100`, the noise
      `MISE.3` found; the date reading the frame, `Clock!B1`; a model's
      sheet reading the Portfolio; the Screen visible; the Portfolio after
      the Day sheet; and a Camera naming the Day sheet.

15. **`MISE.3`'s first number, re-measured on the real view (rec).**
    `MISE.3`'s instrument over the real page: Buffet's own page built here,
    a scratch copy reaching the host by one line, `window.__host = host;`,
    Buffet picked and run in real time in headless Chrome over the DevTools
    protocol at the owner's full screen, 2,560 by 1,440 CSS pixels at a
    ratio of 1.5, on the GPU, every process of its profile opted out of
    Windows' throttling and pinned to the performance cores; each tick timed
    by a wrapper on `host.tick` and filed by what its frame drew, a market
    day's patch or a still frame, by the drawer's counters; the main
    thread's whole from `Performance.getMetrics`; three runs of 600 frames,
    twenty market days each. In-page times are Chrome's, in steps of 0.1
    ms. And the owner's own Chrome at the hand test: a probe copy printing
    the same numbers in a corner every five seconds, read by one
    screenshot, as `MISE.3`'s hand run made its browser number real. The
    number goes into `MISE.6`'s row for `ENGINE.8`, and decides nothing
    course 0 waits on. *The alternatives:* headless alone; or the Stocks
    sheet's numbers in Alonzo's record standing for the view's.

16. **The hand test, the owner's, on the page built here:**
    - the page built by the commands given at the handover in a fenced
      block, opened in Chrome at full screen, Buffet picked: the game opens
      on the Portfolio, not the Screen;
    - the strip reading Portfolio, Day, Dice, Market, Stocks, Caveats,
      Clock, File and Input, its box "hidden sheets (15)", the Screen
      among them;
    - the Portfolio: the date moving a day a second; the four names, prices
      and changes in dollars, each whole in its column; the fine print in
      row 10, turning every thirty seconds; the rows' height judged by eye,
      the default or 20 points;
    - the drawer: its line's still frames rising twenty-nine a second and
      its patched one; its viewer on the Input sheet reading `B19`
      "Portfolio", "Stocks" once the Stocks tab is clicked, and `B11` and
      `B12` the Portfolio's column and row under the pointer;
    - the Caveats sheet's row 12 rewritten, its texts whole;
    - Build in the drawer, "same bytes";
    - the probe copy's corner, one screenshot after a minute.

17. **The stale places, rewritten at the build.** In `OMAKASE.md`:
    `FRONT.1`'s line, 🟡 from the approval; the contract's `(spec 3)`;
    `BD-5`'s owed list (decision 12); `FRONT.2`'s line, a dated sentence,
    its glyph column beside the Portfolio's change and its caveat taken;
    `COURSE.0`'s critical path, step 5's first light, dated; and `MISE.6`'s
    row for `ENGINE.8`, the number re-measured, and the views' row, its
    first view built. The source's opening comment, the notes, the Screen's
    row, the sheets row and the Portfolio's section;
    `src/content/caveats.vla`'s row 12 and its `Where`'s width; `oracle/src/buffet.rs`'s
    module comment, its two functions, and `DAYS_A_DISCLOSURE`'s comment,
    which names `F5` where the days have stood in `D5` since the vintage was
    struck; the module comments of `oracle/src/lib.rs` and
    `oracle/tests/cartridge.rs`; the describe golden; `Delta/CLAUDE.md`'s
    map, a view's cells among the edits that trip a test; the memory.
    `FRONT.1` leaves `OMAKASE.md` for this file with its commit, in a room
    of its own, 🛎 THE FRONT OF HOUSE. Not this item: the other views, the
    Camera (`GUARD.1`), the glyph and the looks (`FRONT.2`), a click the
    game reads (`BACK.1`), the market view's name (`PLATE.1`), key chords
    and the fit (Alonzo's, `ENGINE.21`), a sheet's default row height
    (`MISE.6`'s candidate), the replay golden (the tasting) and the loop
    oracle over Buffet's page (`HOUSE.4`'s second slice).

**The predictions, before any number.** The numbers column is filled at the
build, P9 at the owner's hand test. P3, P4 and P5 are measured as
`MENU.2`'s and `MENU.5`'s were: a scratch runner opted out of Windows' power
throttling and pinned to the performance cores, alternated against a `git
archive` of the commit before this one's, `b11bcd7`, built in a target of
its own; P8 as decision 15 says.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 429 rows where 417 stand, the Portfolio's twelve: its sheet's row, three widths, five formula rows and three headers; 107 formula cells where 93; the diff the manifest's version and notes, the Screen's row made hidden in place and the Portfolio's after it, caveat 12's three cells and the `Where`'s width in place, and the Portfolio's eleven rows at the end; no other row moved | *Measurement*, as predicted: the runner printed `(rows 429) (loads yes)`; 107 formula cells in 47 formula rows; the diff seven lines out and nineteen in, those listed, no other row moved |
| P2 | the describe golden | moved in the version, the notes, `(device "Input" "A1:E19" input)` and `(budget 30 107 3210 2000000)`; the plan `(shapes 47) (texts 45) (formulas 107) (levels 15) (runs 9) (lanes 39) (alone 68)`, every-frame and cycles 0; `(readers "Day" 16 (most 6))`, the Day sheet indexed at its sixteenth reader, the day's row 3 the most read; `(readers "Stocks" 11)`, the change's two cells side by side one reader; `(readers "Disclosures" 12)`; the rest unmoved, and no row for the Portfolio; five shapes first, the Portfolio's: `A1` alone at level 2, `A5`, `B5` and `C5` each a run of four, at levels 0, 14 and 14, and `A10` alone at level 3; every other shape's number five more, its `same` with it; nothing else | *Measurement*, as predicted in every count: the version, the notes, `(device "Input" "A1:E19" input)` and `(budget 30 107 3210 2000000)`; `(plan (shapes 47) (texts 45) (formulas 107) (levels 15) (runs 9) (lanes 39) (alone 68) (every-frame 0) (cycles 0))`; `(readers "Day" 16 (most 6))`, `(readers "Stocks" 11)` and `(readers "Disclosures" 12)`, no row for the Portfolio; its five shapes first, `A1` and `A10` alone at levels 2 and 3, `A5`, `B5` and `C5` each `(runs 1 4 4 4)` at levels 0, 14 and 14; the 42 shapes after them their own rows but for their numbers, `(same 11)` and `(same 12)` where 6 and 7 stood |
| P3 | the pass and the load | the pass 0.1 to 0.3 ms longer, twelve rows that call no picker; the load 0.05 to 0.15 ms longer | *Measurement*, a scratch runner over each tree, `b11bcd7`'s and this one's, each in a target of its own under the profile the tests use, three runs each alternated, 200 passes and 200 loads a run, every run pinning itself to the performance cores and opting itself out of Windows' throttling, a fixed computation before and after within 1.1%: the pass 4.52 to 4.63 ms at p50 where it took 4.42 to 4.44, 0.09 to 0.21 more, inside; the load 1.58 to 1.62 ms where it took 1.46 to 1.50, 0.08 to 0.16 more, the medians 0.11 apart, inside at the median |
| P4 | the step | an ordinary frame unmoved, nothing new reading the frame; a day's first frame 1 to 4 µs more, the date, four prices and four changes, on which the fine print stays and on which it turns; the first frame 5 to 15 µs more, fourteen cells computed once | *Measurement*, the same runs, two years and two days at thirty frames a day: a day's first frame on which the fine print stays 15.0 to 15.8 µs where it took 12.3 to 12.5, 2.5 to 3.5 more, inside; one on which it turns 16.3 to 17.6 where 13.2 to 13.4, the medians 3.1 apart, inside at the median; the first frame 85 to 91 µs where 76 to 79, 6 to 15 more, inside. **Missed** in one: an ordinary frame 2.0 to 2.1 µs where it took 1.9 at every run, the step's own work by shape, two flags a shape cleared at every frame's end (`vla-lang`'s `machine/mod.rs`, `whole` and `recheck`), for five shapes more (*by reading*) |
| P5 | the suite | 45 tests green here, the crate's 15 and the cartridge's 30, and in CI on Linux; within half a second of the commit before's at the median, the Portfolio viewed at 22,760 frames | *Measurement*, here: 45 green, the crate's 15 and the cartridge's 30. **Missed**: each tree's binary run alternately, five rounds, the order turned each round, each run pinned and opted out, on eight threads, 4.81 to 5.68 s where it took 4.17 to 4.96, the medians 5.18 and 4.41, 0.77 apart; the suite's longest test alone, alternated, 4.70 to 4.79 s where it took 4.15 to 4.21, about 0.55 more. The view parsed and spelled at every frame by the oracle's unoptimized helpers cost that test 0.85 s at its first writing; held whole at a day's first frame and byte for byte still through the day, the same cells held, it costs the 0.55 measured. Linux at the push, run 38939138517, which tested its head, `ec069e8`, two commits on: the 46 green, 16 of the crate and 30 of the cartridge, the cartridge's in 8.25 s, this item's tests among them as `FRONT.2` grew them |
| P6 | the page | the cartridge's block about 900 characters longer, the source's about 2,600 and the caveats' a few dozen: the page near 1,787,300 where 1,783,841 stands; Build writing the cartridge byte for byte | *Measurement*: 1,787,808 characters, 3,967 more, the cartridge's block 714 longer, the source's 3,243 and the caveats' 10, the three adding to the page's growth exactly; **missed** in the source's block, its comments longer than priced, and so the page 508 past its "near"; Build writing the cartridge byte for byte, 25,701 characters (P9) |
| P7 | the checks | 4 checks and 4 controls green, no check edited, no file added under `src/` | *Measurement*, as predicted: ALL GREEN, 4 checks and 4 controls, the layout control's fourteen mutants and the offline control's fifteen as they stood |
| P8 | `MISE.3`'s first number, on the real view | in headless Chrome at full screen, the Portfolio shown: a still frame's tick under the clock's step at p50, 0 or 0.1 ms, and at most 0.3 at p95; a market day's 0.3 to 0.9 ms; the main thread's whole a frame 0.35 to 0.6 ms over a run; the drawer, 580 of 600 frames still and 19 patched; against `MISE.3`'s densest lean panel, 1.5 to 1.7 ms at p50 and 2.4 to 2.7 at p95; the owner's Chrome a tenth faster or so | *Measurement*, three runs of 600 frames in headless Chrome 155 at 2,560 by 1,440 CSS pixels at 1.5, its eleven processes opted out and pinned, the clock within 1.7%: a still frame's tick 0.2 ms at p50 and 0.3 at p95, at its longest 0.6 to 0.8; a market day's patch 0.5 at p50, at its longest 0.6 to 1.0; 580 or 581 of the 600 frames still and 20 patched; the main thread's whole 0.74 to 0.78 ms a frame. **Missed** in two: the still tick one step of the clock above, and the main thread past 0.6, a frame at thirty a second holding two of the display's sixty ticks of the browser's own work (*derived*). Against `MISE.3`'s densest lean panel a frame is now about a seventh. The owner's own Chrome, the probe copy at full screen for 23,265 frames: still 0.2 ms at p50 and 0.3 at p95, at its longest 0.7; patched 0.4 and 0.6, at its longest 1.3; 22,488 still and 775 patched, as headless within a tenth |
| P9 | the hand test | the game opening on the Portfolio; the strip, and the box "hidden sheets (15)"; every name, price and change whole in its column; `B19` "Portfolio"; Build "same bytes" | *Measurement*, the owner's three screenshots, in the owner's Chrome, the page windowed with the drawer open and then the probe copy at full screen: the Portfolio shown, the strip Portfolio, Day, Dice, Market, Stocks, Caveats, Clock, File and Input, its box "hidden sheets (15)"; the date, the four names, prices and changes in dollars and the fine print at row 10, each whole in its column; the drawer's "2,102 of 2,176 frames still and 72 patched"; the Caveats sheet's rewritten caveat whole; and the rows' height, the owner's word: "15px row height looks like Excel's standard, so it's fine for now." Not in the screenshots, checked over the probe copy in headless Chrome at the owner's full screen: Build in the drawer, the same bytes as `src/buffet.vla`, 25,701 characters; `B19` "Portfolio", "Stocks" once the Stocks tab is clicked, "Portfolio" again; and a real pointer over the Portfolio's `B5` writing `B11` 2 and `B12` 5, and 0 and 0 over its column's header. The handover had said the page reports "same bytes", which it does not: `MENU.5`'s came from a command that compared the box with the file |

*Scoped 2026-10-15. Approved whole by the owner on 2026-10-16,
"Recommendations approved; proceed with implementing FRONT.1 as scoped.",
and built the same day.*

**What the build found.** A view asks nothing of the library: one `on`
block of the kit's `formulas` and `headers` and three `column` rows, so the
ratchet waits for the second view, as decision 1 had it. The census numbers
shapes walking the sheets in tab order, so a view standing before the model
renumbers every shape after it: the describe golden's diff is five rows new
and forty-two renumbered, read by stripping the numbers, each moved shape
its own row but for them. The view held at every frame through the
oracle's unoptimized helpers, its record parsed and nine values spelled
again on frames that cannot change, cost the suite's longest test 0.85 s;
held whole on a day's first frame and byte for byte the same record through
the day, the same cells held as `BD-7` keeps them, it costs about 0.55 s
(P5). The names' test at its first writing asked only that each name be a
formula, which a name the view computes for itself, `="rung "&(ROW()-4)`,
would pass; found by reading before the mutants ran, it now asks for a
formula reading the Stocks sheet. Eight mutants, each written into the
source by one anchor and expanded into the cartridge before the suite ran,
each failed the test written for it: a price in cents and a change of
dollar amounts, the two years' frames; the date reading the frame, `BD-7`'s
scan; a model's sheet reading the Portfolio, nothing reads a view; the
Screen visible, the Portfolio after the Day sheet and a Camera naming the
Day sheet, the opening; and the name the view computes, the names' test.
And in Alonzo's player, found in the owner's first screenshot: the frame's
line names "the window A1:CB50", the Screen's, while the Portfolio shows,
a version 4 cartridge's Screen being hidden; a finding for Alonzo, the
owner's to pass on.

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
| P6 | the suite | 33 tests green here, five of the cartridge and one of the hand more, and in CI on Linux; the suite under a second longer than `bea77bc`'s, alternated against it in a worktree | *Measurement*, here: 33 green, 14 of the crate and 19 of the cartridge. The cartridge's binary run alternately with `bea77bc`'s from a worktree, five rounds, each opted out and pinned, on eight threads: 3.93 to 4.76 s where it took 3.63 to 4.37, the medians 4.27 and 3.85 s; the rounds 0.13, 0.17, 0.25 and 0.30 s apart, and the first 1.09. Inside at the median, one round of five past it. Linux at the push, run 38939138517, which tested its head, `ec069e8`, nine commits on: the 46 green, 16 of the crate and 30 of the cartridge, the cartridge's in 8.25 s, this item's tests among them as later dishes grew them |
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
| P6 | the suite | 38 tests green here, five of the cartridge more, and in CI on Linux; the suite under a quarter of a second longer than `3dd3eeb`'s at the median, nothing of the bank held at every frame | *Measurement*, here: 38 green, 14 of the crate and 24 of the cartridge. The cartridge's binary run alternately with `3dd3eeb`'s, built from that commit's tree in a target of its own, five rounds, the order turned each round, each run opted out and pinned, on eight threads: 4.02 to 4.75 s where it took 4.00 to 4.73, the medians 4.30 and 4.61 s. Inside: no difference five rounds can tell. Linux at the push, run 38939138517, which tested its head, `ec069e8`, eight commits on: the 46 green, 16 of the crate and 30 of the cartridge, the cartridge's in 8.25 s, this item's tests among them as later dishes grew them |
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

**Amended after the close, 2026-10-15, at the owner's word,** "drop the
approver from the strings macro and from disclosures.vla and questions.vla",
the owner having asked what a username beside every row was for besides
attribution. It had no job of its own: a commit's sign-off names a row's
author and the pull request's approval its reviewer, each timestamped, where
the column was typed by hand; nothing in the game read it, and no check held
it filled (`MENU.1`'s decision 8); every row held the same name; and it
shipped a real person's handle inside the game. A row is now its vintage and
its fields, and its review the pull request's that adds it, by the
repository's named reviewer (`MENU.6`). The library writes one column fewer,
so the block beside each table stands one column left: the fine print's
count, row shown and picker in `E1:E3`, the source's days a disclosure stands
in `D5:E5`; the bank's in `L1:L10`, its row shown `L2`. The cartridge lost 32
rows, 377 where 409 stood: the 28 approvals, their two headers and their two
widths. The describe golden moved in its ten shapes' addresses alone, the
tests followed, the 38 green; the page, built again, holds 1,622,074
characters; and the *How to add a row* and `CONTRIBUTING.md` lost the step.

**Amended again the same day, at the owner's word,** "Remove the leading
zeroes. Courses don't need to be versioned inside the code", the owner
having asked what the vintage was for and answered that `BD-1` "was
intended to be a composability rule in which later version of the game added
features to already-playable games", never a freeze: "Courses represent
tranches of independently-playable features, not code-freeze points." The
vintage's one job was the freeze, `MISE.1`'s second slice keeping a frozen
course's bytes when a later course added rows (`MENU.1`'s decision 8), so it
went with the freeze: `BD-1` is amended in `OMAKASE.md`, with the owner's
words, and `MISE.1`'s second slice struck, with the layout check's courses
kind and its mutant, fourteen mutants where fifteen stood, and the license
map's `src/courses/**`. A row is now its fields alone,
`("Your text here.")`. The library writes no vintage column, so the block
beside each table stands one column further left: the fine print's count,
row shown and picker in `D1:D3`, the source's days a disclosure stands in
`C5:D5`; the bank's in `K1:K10`, its row shown `K2`. The cartridge lost 30
rows more, 347: the 28 vintages and their two headers. The describe golden
moved in its ten shapes' addresses alone; the 38 tests green, the checks and
their controls green; the page, built again, holds 1,620,721 characters.

## `MENU.5` — the accuracy sheet: *Things this game gets wrong*, and *not advice*

*Scoped 2026-10-15 at the owner's word, "Scope and implement MENU.5 in
Buffet (Delta/Buffet): the accuracy sheet, "Things this game gets wrong",
and "not advice". It is the game's third content sheet.", to stop for the
owner's approval before building: from `OMAKASE.md`'s line (`COURSE.0`'s
item 18), `BD-5`, `BUFFET.md` §10's accuracy policy and §5's fifth
principle, a toy labeled as one, and `MENU.1`'s and `MENU.2`'s scopings,
build records and amendments, which set the shape a content sheet takes.
It comes now because Alonzo's `ENGINE.18` is committed and unreleased,
`ENGINE.19` closed and `ENGINE.20` next, so `FRONT.1` and every dish with a
button still wait, and this item needs nothing of Alonzo's; and because it
is the first content sheet written since `strings` learned several fields,
the ratchet's check that the shape holds for a sheet that is not a
question bank. It is written over the shape `MENU.2`'s amendment after its
close left in the working tree, uncommitted, the approver struck from
every row at the owner's word, and its build starts from that amendment's
commit. Each decision carries its recommendation, for the owner to
approve, amend or strike; the rows are drafts, for the owner to approve or
rewrite, as the owner rewrote seven of `MENU.1`'s eight and eighteen of
`MENU.2`'s twenty. The predictions were written before any number was
taken. Approved whole by the owner the same day, "Decisions approved;
proceed.", with the name the owner preferred, "I'd prefer the title
Caveats, but Limitations is fine if Caveats is too vague": `Caveats`, the
first field's header leaving it no vaguer than a sheet's name may be; and
built that day, the amendment before the predictions recording what the
approval and the moving base changed. Alonzo's tree, where `ENGINE.20` is
scoped, was read and never touched.*

**What it stands on, read on the day** (`vla-lang` 0.9.3 and `alonzo`
0.1.3 as the registry holds them; `web/` as vendored at `v0.1.3`; Buffet at
`8775818` with the approver's amendment in the working tree).
- The content shape: a file is one macro named after it, whose body is one
  call, `(strings "Sheet" ("Header" width ...) rows...)`, each row its
  vintage, then its fields in the list's order; the source includes the
  file after its manifest and calls the macro where the sheet belongs. The
  table runs from column A, the vintage after the fields at the default
  width; past one empty column stand labels in the column the fields'
  count plus three and values in the next: the count, the row shown, then
  a picker a field, the first labeled `shown` and each later one by its
  header. `check_layout.ps1`'s content pin is at 2, its control fifteen
  mutants over a clean copy of seven files.
- `strings` writes its sheet's state first, `(sheet "<name>" hidden)`
  (`kitchen.vla`, `strings-sheet`). The engine refuses a second `sheet`
  row that gives a sheet another state, as a row given twice (`vla-lang`'s
  `machine/load.rs`), and a sheet first named by any other row stands
  visible (`Sheet::new`). So no source can show a sheet that `strings`
  hid: a visible content sheet needs its state written some other way.
  The kit's header says how a hidden sheet is written: "as its own row,
  (sheet "X" hidden)" (`common.vla`).
- The names taken. The devices: Screen, Palette, Input, Keys, Clock,
  Audio, File, Camera and Write (`alonzo` 0.1.3, `DEVICES`). The views
  `FRONT.1` lists: Portfolio, Market, Overview, Exam and Work. The model's
  sheets: Day, Dice, Market and Stocks. The content sheets built or
  recommended to their dishes (`MENU.1`'s decision 2): Disclosures,
  Questions, Headlines, Memos, Firms, Awards, Errors and Exits, `Errors`,
  the nearest word to this sheet's, being `PLATE.6`'s. A name is bare by
  `refers.rs`'s rule: letters, digits, `_` and `.`, no leading digit,
  neither a cell nor R1C1's shape.
- The dialect has no `INDEX`, `OFFSET` or `MATCH`, so a view reads a
  content sheet through a picker one row at a time (`MISE.3`'s spelling),
  and no view can show a sheet's rows whole: only the sheet can, where its
  rows stand.
- The page's strip shows a visible sheet in tab order and lists a hidden
  one in the box beneath it, "hidden sheets (12)" today. A text spills
  only into empty cells on its right (`viewport.js`, `_paintCell`), so a
  label longer than its column is clipped by the value beside it, as the
  Disclosures sheet's `the row shown` is. `ENGINE.18`, committed and
  unreleased, opens a grid cartridge on the sheet `Camera!B3` names, else
  on its first visible sheet: while the Screen stands first, no sheet
  after it opens the game.
- What the record set aside for this sheet: the four waves, the drift and
  the whole-cent world (`OMAKASE.md`'s line); every day a market day, 365
  a year (`POT.1`'s decision 4); at tier 1 no stock has a risk of its own,
  so none goes bust (`STATION.1`'s decision 5); a $1 share moving a penny
  at a time, on 25 of the first year's 365 days (`STATION.1`'s build);
  money drawn at its shortest spelling, 12.3 for $12.30, until number
  formats exist (`MISE.3`); and `BUFFET.md` §10's examples, one of them
  course 0's, the Kondratiev wave, which tier 1 carries, and three of
  later courses: volatility's clustering (`POT.5`), the curve's six points
  (`POT.3`) and real banks (`STATION.3`). `POT.9`'s scenario packs promise
  a row each. The record holds no other row for `MENU.5`.
- The game says it is not advice already, twice, in the owner's fine
  print: "Buffet is not a financial advisor. Buffet is barely a game." and
  "This is not a recommendation to buy or sell. 'Ursi et tauri, cavete!'."
  Both are rows of a hidden sheet, turning at every view's foot once
  `FRONT.1`'s views read it. `README.md`'s "Not advice" is plain and has no
  joke, and it writes "labelled", "Licences" and "licence" twice where the
  house writes American English; `SECURITY.md`'s "What this game is not"
  writes "labelled" too, and points at "the sheet the game keeps for that
  purpose" without naming it. The certificate panel is `GUARD.1`'s, gated
  on Alonzo.
- The describe golden carries the manifest's notes and every formula's
  shape, and `strings` writes a picker a field, so a content sheet brings
  formulas whatever its source writes beside it: the golden moves with
  this item, blessed again and read in its diff.
- The first game's market, measured for the rows by a scratch script over
  the source's formulas and held to the record where the two meet (the
  index 1.1016 on day 365; the $1 rung moving on 25 days of the first
  year): the index's daily move 0.031% at the median over a hundred years
  and 0.113% at its largest, 0.027% and 0.074% in the first year; the $1
  rung's largest day one cent, 1.02%; the drift 7.57% a year, and the
  drift raised to the rungs' betas 15.72%, 11.57%, 7.57% and 5.63%. A
  rung's price is its start times the index to its beta, to the cent, so
  the four move together exactly but for the rounding.
- The world, read on the day for the rows: the S&P 500's total return was
  −0.95% a year from 2000 to 2009 (Dimensional Fund Advisors), and about
  10% a year with dividends since 1926; the SEC's Rule 612 lets a quote
  below $1.00 go to $0.0001; low-volatility and low-beta portfolios earned
  high average returns with small drawdowns from 1968 to 2008 (Baker,
  Bradley and Wurgler, *Financial Analysts Journal*, 2011).

**The decisions.**

1. **The name: `Limitations` (rec).** A model's documentation keeps
   exactly this list under this word: what the model gets wrong, written
   down, each limitation answered by the model's owner, and the first of
   them the limits on its use. One word, bare, a fund's workbook's, and it
   collides with no view, model sheet, device or content sheet:

   | The candidate | The verdict |
   |---|---|
   | `Limitations` | recommended: the model's own list, whose first entry, on its use, is *not advice* (decision 4) |
   | `Caveats` | the alternative: shorter, a research note's word, and the same verb as the fine print's *cavete*; but a caveat warns, and nobody answers it (decision 3's third field) |
   | `Assumptions` | struck: a financial model's inputs tab, where a spreadsheet person looks for the drift and the betas, which stand on the Market and the Stocks sheets |
   | `Errors` | taken: `PLATE.6`'s error values |
   | `Errata` | struck: a publisher's corrections, made after printing |
   | `Disclaimers` | struck: beside `Disclosures`, two names for the fine print |
   | `Footnotes` | struck: the running gag's footnote, which grows (`BUFFET.md` §2), and the fine print's place |
   | `Findings` | struck: the audit's (`GUARD.3`) |
   | `Risks` | struck: a prospectus's risk factors, which a player would read as the game's risk |

   *Things this game gets wrong* cannot be a sheet's name, and stays in
   the game as the header over the sheet's first field (decision 3).

2. **Visible, after the model's sheets (rec).** The sheet is meant to be
   read, and read whole, which no view can do (above): hidden, it is one of
   thirteen sheets in the box beneath the strip, and `BD-5`'s own reason
   names that, *a simplification hidden is a liability*. Visible, it stands
   in the strip beside the model it qualifies, Screen, Day, Dice, Market,
   Stocks, Limitations, Clock, File, Input, and behind `FRONT.1`'s views
   when they lead the strip; never first, so never the sheet the game
   opens on. The way, since the engine refuses a second state: `strings`
   writes none (decision 6), and the source writes each content sheet's
   state as a row of its own before the sheet's call, as it writes the row
   shown after it, in the kit's own spelling: `(sheet "Disclosures"
   hidden)` before `(disclosures)` and `(sheet "Questions" hidden)` before
   `(questions)`, each the row `strings` wrote, in the place it stood, so
   both sheets stand byte for byte; and `(sheet "Limitations" visible)`
   before `(limitations)`. Whether a sheet shows is the game's to say,
   which is the source's (`MENU.1`'s decision 4: "what a sheet's rows mean
   to the game is the source's"), and not the words'. A state forgotten
   leaves its sheet visible, and the oracle's tab order names it (decision
   10). This amends `MENU.1`'s decision 3, which had `strings` write every
   content sheet's hidden row, once: the rule stands, a content sheet
   hidden as a workbook keeps its reference data at the back, with its one
   exception, a sheet meant to be read, written where each sheet is
   called. *The alternatives:* hidden, as `MENU.1`'s decision 3 has it, the
   library unmoved and the sheet found in the box; or the state a word of
   the call, `(strings "Limitations" visible ...)`, each content file
   stating its own, a switch of the game's presentation one word from a
   stranger's row.

3. **The fields: what the game gets wrong, what the world does,
   Management's response, and where (rec).**
   - `Things this game gets wrong`: the simplification, plainly; the
     header carries `BD-5`'s name for the sheet into the game.
   - `In the real world`: what the world does, plainly; the field a
     finance educator reads before a release (`BD-5`).
   - `Management's response`: the joke, in the voice of a model's owner
     answering a finding, deadpan (`BD-2`): "noted", "under review", "not
     our department". The header tells a stranger the voice before they
     read a row.
   - `Where`: the cell or range the row qualifies, as a formula writes a
     reference, `Market!B2`; empty when a row qualifies the game as a
     whole. `AD-6`'s open book, from the limitation to its cell; and held
     by the oracle to a range of the cartridge that holds a value (decision
     10), so that a limitation cannot outlive its cell when a later course
     moves a model's sheet.

   Every text is one line of at most 140 characters, as a why is, since a
   later view may quote any field alone; and each field is as wide as its
   longest drafted text, 104, 115, 103 and 13 characters, so that every
   text reads whole on a sheet meant to be read: the four columns and the
   vintage's about 2,430 pixels at the page's 7 a character, inside the
   owner's full screen. The table runs from A to E, the labels stand in G
   and the values in H: the count `H1`, the row shown `H2`, the pickers
   `H3` to `H6`. *The alternatives:* three fields, no `Where`, lighter for
   a stranger, but a view that later shows a figure's limitation beside the
   figure would want the field added to every row, which is `MENU.2`'s
   decision 1's reason to write a field once; or the joke under
   `Footnote`, the running gag's word, which answers nobody.

4. **Not advice: the sheet's first row, and the README (rec).** The plain
   statement is the model's first limitation, the limit on its use, where a
   model's documentation states it; and the row says it in the sheet's own
   kind, what the game gets wrong, which here is the person playing it:
   *You. It knows nothing of your goals, your means or your timeline.*,
   then *Advice starts from those three, from someone licensed to give it.
   This game is not financial advice.*, then *Any resemblance to financial
   advice, living or dead, is purely coincidental.*, and no cell. So the
   statement is in the game from course 0's first frame, on a sheet a
   player opens from the strip, its joke beside it, as `BD-5` and the line
   ask. With the row shown at 1 (decision 5) the pickers hold it, so the
   certificate (`GUARD.1`) shows the plain statement and its joke by a
   reference, `Limitations!H4:H5`, with nothing more written. A row is
   added above the file's last line (the *How to add a row*'s step 2), so
   the first stays first, and the oracle holds it there. The README says
   the same in prose, the plain statement and the same joke (decision 11).
   *The alternatives:* a kind of its own, a sheet of one row that no
   stranger adds to, read by nothing until the certificate, whose strings
   `GUARD.1` lays out (`MENU.2`'s decision 4 sent the archetype's question
   to its dish so); or the README alone, the game then saying so only in
   two fine prints, hidden until `FRONT.1`'s views read them, and plainly
   nowhere until `GUARD.1`.

5. **What reads the sheet: the row shown, a value, 1 (rec).** The source
   writes `H2`, 1, as it writes `Questions!L2`: the pickers show the first
   row, the plain statement and its joke, from the first frame, for the
   certificate, and a person reads the rest on the sheet itself. No
   rotation: the fine print already turns at every view's foot, and a
   second footer turning beside it would teach nothing the sheet does not.
   *The alternatives:* a rotation, a limitation a month, `H2` reading the
   day as `Disclosures!E2` does, a formula more and the Day sheet's readers
   one more, turning where nothing reads it; or nothing written, every
   picker `CHOOSE` of an empty index, an error value on a sheet meant to be
   read.

6. **The library: `strings` writes no state (rec).** `strings-sheet` loses
   its first row, `(sheet content hidden)`, and its comment and the
   library's header say that the source writes each sheet's state beside
   its call (decision 2). `picker` and every walk stand. The proof is the
   cartridge's diff, in which no row of the Disclosures or the Questions
   sheet moves, as `MENU.2`'s decision 6's was.

7. **The rows: course 0's twelve (rec).** A row a block, its vintage
   beside its first field and then a field a line, as the bank's rows are
   laid out; every vintage 0. Each was checked against `BD-2` (no kitchen
   word, the humor financial or economic, punch up, fictional names,
   PG-13, clarity first), for accuracy against the game (the measurements
   above) and the world (`BD-5`; **(check)** where a finance educator
   confirms before the release), and for American English; a quote inside
   a text is a single quote. The order walks the game: its player, the
   calendar, the market, the stocks, the money, the page.

   | | Things this game gets wrong | In the real world | Management's response | Where | The verdict |
   |---|---|---|---|---|---|
   | 1 | You. It knows nothing of your goals, your means or your timeline. | Advice starts from those three, from someone licensed to give it. This game is not financial advice. | Any resemblance to financial advice, living or dead, is purely coincidental. | | decision 4; the joke a film's credits' disclaimer, turned on advice |
   | 2 | Every day is a market day: 365 a year, weekends and holidays included. | US exchanges close on weekends and about ten holidays a year, leaving about 252 trading days. | Weekends are available from the rank of Partner. | `Day!B2` | `POT.1`'s decision 4; 252 is the convention for a US trading year, the NYSE closing for ten holidays since Juneteenth joined them in 2022 |
   | 3 | Tomorrow's price is a formula of the seed and the day, and anyone can read it. | No one knows tomorrow's price, and anyone who says otherwise is selling something. | Reading ahead is not insider trading, since everyone can. Compliance finds this unsettling. | `Market!B25` | new: `BD-3`'s open book, `BUFFET.md` §5's design fact, said to the player; the joke accurate, insider trading being trade on what is not public |
   | 4 | The market is four waves and a drift, and each wave keeps its period to the day. | Real business cycles vary in length and depth, and economists still argue whether the longest ones exist at all. | The Kondratiev wave is fitted to a handful of 50-year cycles. Statistics has requested a larger sample. | `Market!A8:E11` | the line's; §10's "a crime against statistics" rewritten, since the wave bears the name of an economist tried and shot in 1938, and a crime with its counsel stands too near a named man (`BD-2`'s first rule) |
   | 5 | Day to day, the market barely moves: no news, no noise, no crash. | A broad index can move 1% on an ordinary day, and on October 19, 1987, US stocks fell more than 20%. | Trading in news has been halted, pending an announcement. | `Market!B25` | new: the index's median day 0.03% and its largest in a century 0.11% (measured); the S&P 500 fell 20.5% that day; noise and shocks are tier 2's (`POT.5`) |
   | 6 | Beneath the waves, the market grows 7.6% a year, every year, forever. | US stocks have returned about 10% a year over a century, dividends included, but unevenly: 2000 to 2009 lost money. | Here, past performance is indicative of future results. Management asks that this not get around. | `Market!B2` | the line's; 1.0002 a day is 7.57% a year; **(check)** about 10% a year with dividends since 1926, and −0.95% a year from 2000 to 2009 |
   | 7 | All four stocks follow the one index: none can go bust, and owning all four spreads no risk. | Each company has news of its own; owning many cancels much of it, and some companies do go to zero. | Management is pleased to report a 0% default rate, and is investigating why. | `Stocks!E5:E8` | `STATION.1`'s decision 5 and what it implies: the four move together exactly, so the exam's *Diversification* question teaches what tier 1 cannot show; a stock's own risk is `POT.5`'s |
   | 8 | The riskier the stock, the faster it grows: a beta of 2 trends 15.7% a year, a beta of 0.75 trends 5.6%. | Riskier stocks have not reliably earned more: over long stretches, low-volatility stocks did as well or better. | Management has decided that risk will be rewarded, and has asked the data to comply. | `Stocks!C5:C8` | new: `STATION.1`'s decision 5, "a higher risk earns a higher long-run return", measured 15.72% and 5.63%; **(check)** the low-volatility anomaly, as Baker, Bradley and Wurgler found it from 1968 to 2008 |
   | 9 | The dividend is free: a share's price does not fall on the day it pays. | A share's price usually falls by about the dividend on the day it goes ex, since that cash has left the company. | Accounting has been asked not to look into it. | `Stocks!H8` | new: no price formula holds the dividend; the fall is textbook, "about" since taxes and the day's news blur it |
   | 10 | Money comes in whole cents: every price is rounded once, to the nearest cent. | Ledgers keep fractions of a cent: interest accrues in them daily, and the rounding comes last. | Fractions of a cent are swept into an account named 'Rounding (do not open)'. | `Stocks!E5:F8` | the line's: `FRONT.2`'s rule and the contract's; a sweep is a bank's own word |
   | 11 | A $1 share moves a penny at a time, and on most days not at all. | A real stock near $1 can swing several percent a day, and under $1 it may be quoted to a hundredth of a cent. | Analysts rate it a Hold, which it does. | `Stocks!E5` | `STATION.1`'s build, 25 of 365 days, measured again here; **(check)** Rule 612's $0.0001 below $1.00 |
   | 12 | Numbers have no formats: a price reads in cents, 9921 for $99.21, and a yield of 2% reads 0.02. | A workbook formats its numbers for people: $99.21, 2.00%, 1,000,000. | The decimal point has been furloughed pending a review of number formats. | `Stocks!D5:F8` | `MISE.3`'s, as the game shows money today, in cents on the Stocks sheet; the dish that first shows dollars, 12.3 for $12.30, rewrites it (decision 13) |

   No row names a real company, person or mark but the economist the wave
   is named for, whom the Market sheet's label names already, and the SEC's
   rule, a fact and no parody. Four claims of the world are marked for the
   educator; the game's claims are measured. And three rows teach against
   lessons tier 1 would teach by accident: that the future can be read
   (3), that spreading money among the four stocks spreads risk (7), and
   that risk always pays (8).

8. **The source: the include, the states, the call, the row shown, the
   labels' width and the notes (rec).** `(include
   "content/limitations.vla")` after the bank's; the two hidden states
   before their calls (decision 2); and a block after `MENU.2`'s, as
   `MENU.1`'s and `MENU.2`'s are: its comment, `(sheet "Limitations"
   visible)`, `(limitations)`, then `H2`, 1, with its comment, and the
   labels' column, G, 21 characters wide, `Management's response` being the
   longest label, so that the labels read whole beside their values on a
   sheet meant to be read, where the hidden sheets' are clipped and nobody
   reads them. The game's notes, a player's string under `BD-2`, gain the
   sheet: "...; the Series 777's question bank, twenty questions awaiting
   an exam the regulator has yet to schedule; and the model's
   limitations, the things this game gets wrong, each with Management's
   response. Pick the Day, the Dice, the Market or the Stocks sheet to
   watch, or the Limitations sheet to read, or the Disclosures or the
   Questions sheet among the hidden sheets." The opening comment lists
   `MENU.5` among the dishes served.

9. **The layout check (rec).** The rule holds as it stands, since
   `limitations.vla` is one macro named after its file whose body is one
   call of `strings`, and a state row in the source is code after the
   includes; the content pin rises from 2 to 3; the control's clean copy
   holds eight files, its mutant "a third content sheet past its pin"
   becomes a fourth, and its fifteen mutants stand.

10. **The oracle (rec).** In `oracle/tests/cartridge.rs`, the content
    sheets three:
    - Held by the list already: each content sheet is its file, read by
      the language's reader (`MENU.2`); the tripwire under 1,000 rows.
    - The tab order: Limitations visible after Questions, and each content
      sheet in the state the source writes, Disclosures and Questions
      hidden and Limitations visible; the test that has every content sheet
      hidden says so instead.
    - Every limitation whole: the four fields in their order; the first
      three texts of every row not empty; every text one line of at most
      140 characters and no wider than its field; the vintage a whole
      number; and the first row's texts naming advice, so that the
      statement the certificate reads stays first.
    - Every limitation's cells: after the first frame, each `Where` that is
      not empty a sheet of the cartridge, `!`, and a cell or a range of it
      that holds a value.
    - The first row shown: `H2` 1, and from the first frame `H3` to `H6`
      the first row's fields.
    - Every field picked at every row: `MENU.2`'s test over the Limitations
      sheet too, its row shown written 1 to 12.
    - A row added: no third test of it. The walk and the call are
      `MENU.2`'s, held by two such tests; this sheet's own rows are seen in
      the cartridge's diff at the build.
    - Kept: the cartridge its source expanded; `BD-7`'s scan and the
      census's `(every-frame 0)`; the describe golden, blessed again and
      read in its diff. The hand unchanged: no rotation, and the texts are
      the content's, written once.

11. **The README and `SECURITY.md` (rec).** `README.md`'s "Not advice"
    rewritten: Buffet is a game, and nothing in it is financial advice,
    since advice starts from a person's goals, means and timeline and the
    game knows none of them; it is no brokerage and holds no real money;
    its market is a formula of a seed and a day, fed by no real market's
    data; and every model in it is a toy, labeled as one on its Limitations
    sheet, *Things this game gets wrong*; then the joke, the row's own,
    *Any resemblance to financial advice, living or dead, is purely
    coincidental.* The README's "Licences", "licence" (twice) and
    "labelled", and `SECURITY.md`'s "labelled", respelled American;
    `SECURITY.md`'s line names the sheet.

12. **The *How to add a row* (rec).** `src/content/README.md` names the
    third sheet, `limitations.vla`, a limitation a row, laid out as a
    question is: what the game gets wrong, what the real world does,
    Management's response, and where, a reference as a formula writes one,
    or empty. What a limitation needs: the world's half accurate, since a
    finance educator reads it before a release (`BD-5`); each text one
    line, within its column's width at the top of the file; and the voice,
    a model's owner answering a finding. Still one page.

13. **Every later simplification has a row to go to (rec).** `BD-5`
    gains a dated line: the sheet is `Limitations`, its first row the plain
    statement, and a dish that brings a simplification brings its row in
    its own commit, its scoping naming the row. Owed already, each to the
    dish that makes it true: an order filled at the price shown, with no
    spread and no commission (`PLATE.1`, until `PLATE.5`); cash that earns
    nothing and loses nothing to inflation (`PLATE.2`, until `POT.3`'s
    rates); money in dollars at its shortest spelling, row 12 rewritten
    (`FRONT.2`); an exam of ten questions that cannot be failed
    (`GUARD.1`); no taxes (`PLATE.7`); volatility that clusters neatly
    (`POT.5`); a curve of six points (`POT.3`); real banks, which are worse
    (`STATION.3`); and each scenario pack (`POT.9`).

14. **The hand test, the owner's, on the page built here:**
    - Buffet at full screen: the strip reading Screen, Day, Dice, Market,
      Stocks, Limitations, Clock, File and Input, and its box "hidden
      sheets (13)", the eleven twins, Disclosures and Questions;
    - the Limitations sheet opened from the strip: its twelve rows whole,
      each text inside its column, the vintage beside them; `H1` 12, `H2`
      1, and `H3` to `H6` the first row, the plain statement and its joke;
      the labels in G whole;
    - Build in the drawer writing the cartridge byte for byte, compared
      with `src/buffet.vla` by a command given at the handover in a fenced
      block, since `MENU.2`'s, given inline, lost its backticks in the
      copy.

15. **The stale places, rewritten at the build.** In `OMAKASE.md`:
    `MENU.5`'s line (the sheet's name; the certificate's reference; "the
    folder's README", written when the game was a folder of Alonzo's, now
    the repository's); `BD-5`'s dated line (decision 13); `GUARD.1`'s line,
    a dated sentence, the certificate reading `Limitations!H4:H5`; and
    `MISE.6`'s hoist row for `strings`, its state moved to the source. In
    `BUFFET.md` §10, a dated line under the accuracy policy: where the
    sheet stands, its first row, and the Kondratiev joke rewritten.
    `kitchen.vla`'s header and `strings`' comment; `check_layout.ps1`'s
    header; the source's opening comment and the cartridge's notes;
    `src/content/README.md`. `MENU.5` leaves `OMAKASE.md` for this file
    with its commit. Past the files the owner's prompt foresaw, the build
    touches the library (decision 6), `SECURITY.md` (decision 11) and
    `BD-5`; the page's blocks follow by themselves, as `MENU.2`'s did.

**Amended at the approval, 2026-10-15, before the build.** Three changes,
none of a decision's substance. First, the name: `Caveats`, the owner's
preference, read for `Limitations` wherever the decisions name the sheet;
the first field's header keeps *Things this game gets wrong* in the game,
so the name needs to say no more. Second, the base: between the scoping and
the build the vintage was struck from every row too, at the owner's word,
and both strikes were committed as `adc1bc7`. A row is now its fields
alone, so the table runs from A to D, the labels stand in F and the values
in G, the labels' column the source widens is F, and the certificate reads
`Caveats!G4:G5`. The same commit struck the layout check's courses kind and
its mutant, so its control holds fourteen mutants where P8 counted fifteen;
P1's base is 347 rows where 377 stood with the vintage, the sheet bringing
70 rows where 83 were counted, its twelve vintage cells and their header
gone; and P3, P4 and P6 are measured against a `git archive` of `adc1bc7`.
Third, the owner's question at the approval, what the item is for beyond
excessive humility, the sheet reading "like talking to the player as though
they were a toddler", unless it is a legal hedge, answered in chat: the
first row is the legal hedge and earns its place either way; rows 3, 7 and
8 correct three lessons tier 1 would teach by accident; and rows 2, 10 and
12, the calendar, the cents and the formats, are the trivia, the first to
cut when the owner rewrites rows. The build went in a scratch worktree on a
snapshot of the uncommitted strike, and moved onto `adc1bc7` once it was
committed, every game file of the snapshot equal to the commit's.

**The predictions, before any number.** The numbers column is filled at
the build, P9 at the owner's hand test. P3, P4 and P6 are measured as
`MENU.2`'s were: a scratch runner opted out of Windows' power throttling
and pinned to the performance cores, alternated against a `git archive` of
the commit before this one's, `8775818` with the approver's amendment once
it is committed, built in a target of its own.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 460 rows where 377 stood, the Limitations sheet's 83: its state, four widths, five headers, six labels, twelve rows of five cells, the count, four pickers, and the source's row shown and labels' width; 93 formula cells where 89; the Disclosures and the Questions sheets' rows byte for byte, their states now the source's in the same places; and the notes moved | *Measurement*, on the base the approval restated: the runner printed `(rows 417) (loads yes)`, 347 and the sheet's 70, in the order listed; 93 formula cells; the cartridge's diff one line out, the notes, and 71 in, the notes and the sheet's 70, no row of the Disclosures or the Questions sheet among them |
| P2 | the describe golden | moved in the notes; the budget's formulas, 89 to 93 and 2,670 to 2,790 a second; the plan's shapes 38 to 42, texts 36 to 40, formulas 89 to 93 and alone 62 to 66, its levels, runs, lanes, every-frame and cycles unmoved; `(readers "Limitations" 52 (most 4))`, each picker's row shown and its twelve cells; four shapes, each a picker alone at level 0; nothing else | *Measurement*, as predicted but for one count, **missed** in it: the notes; `(budget 30 93 2790 2000000)`; shapes 42, texts 40, formulas 93 and alone 66, the levels 14, runs 6, lanes 27, every-frame 0 and cycles 0 unmoved; shapes 38 to 41, `G3` to `G6`, each `(cells 1) (levels 0 0 1 1)`; nothing else; but `(readers "Caveats" 52 (most 8))` where `(most 4)` was predicted. The census indexes a sheet's readers by row, and a row again by column only once it holds sixteen (`vla-lang`'s `calc/reach.rs`, `most`): row 2 holds the row shown's four readers and the first row's four cells, eight. The Questions sheet's row 2 held sixteen, the row shown's eight and the first question's eight, indexed again by column, where the row shown's eight are the most: why `MENU.2`'s reading of its own 8 held there |
| P3 | the pass and the load | the pass 0.3 to 0.8 ms longer, at `MENU.2`'s price of a cell and of a picker's step; the load 0.2 to 0.4 ms longer | *Measurement*, a scratch runner over each tree, `adc1bc7`'s and this one's, three runs each alternated, 200 passes and 200 loads a run, the runner pinning itself to the performance cores and opting itself out of Windows' throttling, a fixed computation before and after steady: the pass 3.81 to 3.84 ms at p50 where it took 3.24 to 3.27, 0.54 to 0.60 ms more, inside; the load 1.36 to 1.38 ms where it took 1.13 to 1.17, 0.19 to 0.24 more, inside at its lower edge |
| P4 | the step | an ordinary frame and a day's first frame unmoved, the pickers reading nothing that moves; the first frame computing four pickers once, 5 to 20 µs more | *Measurement*, the same runs, two years and two days at thirty frames a day: an ordinary frame 1.70 µs at p50 before and after; a day's first frame on which the fine print stays 11.4 to 11.5 µs where it took 11.3 to 11.6, and one on which it turns 12.2 to 12.6 where 11.9 to 12.6: unmoved; the first frame 131 to 149 µs where it took 95 to 125, the medians 131 and 122, about 10 µs more: inside |
| P5 | the limit | unmoved: a sheet of one field holds 1,547 rows and 1,548 are refused at the picker's line, the scratch sheet now visible and nothing else of it moved; the pass over 1,548 rows under 50 ms | *Measurement*: 1,547 rows load and 1,548 are refused at the picker's line, the test holding both on every run; the pass over 1,548 rows 18.5 to 18.8 ms at p50 on both trees, inside, and lighter than `MENU.2`'s 28.3 to 28.9 since `adc1bc7`'s rows carry one cell where they carried three |
| P6 | the suite | 41 tests green here, three of the cartridge more and two extended, and in CI on Linux; within a quarter of a second of the commit before's at the median | *Measurement*, here: 41 green, 14 of the crate and 27 of the cartridge. The cartridge's binary run alternately with `adc1bc7`'s, built from that commit's tree in a target of its own, five rounds, the order turned each round, each run pinned and opted out, on eight threads: 4.18 to 5.07 s where it took 4.11 to 4.84, the medians 4.41 and 4.23 s, 0.18 s apart: inside. Linux at the push, run 38939138517, which tested its head, `ec069e8`, six commits on: the 46 green, 16 of the crate and 30 of the cartridge, the cartridge's in 8.25 s, this item's tests among them as later dishes grew them |
| P7 | the page | one block more, `library-content-limitations`, about 4,900 characters; the cartridge's block some 6,500 longer, the source's 1,400 and the library's a few hundred; the page about 13,000 longer, near 1,635,000 where the approver's amendment left 1,622,074; Build writing the cartridge byte for byte | *Measurement*: 1,634,034 characters, 13,313 more than `adc1bc7`'s 1,620,721, inside; the new block, `library-content-caveats`, 5,239 characters; the cartridge's block 5,403 longer where some 6,500 was predicted with the vintage, the library's 508, and the source's 2,062 where 1,400 was predicted, **missed**: its comments ran longer than priced. Build in the drawer at the owner's hand test, P9 |
| P8 | the checks | 4 checks and 4 controls green; the layout control's clean copy eight files and its fifteen mutants each failing for its own reason, the fourth content sheet's among them; the offline control's fifteen unchanged | *Measurement*: ALL GREEN, 4 checks and 4 controls; the layout control's clean copy eight files and its fourteen mutants, the fourth content sheet's among them, each failing for its own reason, fifteen having been counted before `adc1bc7` struck the courses mutant; the offline control's fifteen unchanged. `check_spdx` reads the tracked tree, so `caveats.vla`, untracked, was held through a scratch index that tracked it: 41 files resolved, 8 headers read under `src/` |
| P9 | the hand test | the strip showing Limitations between Stocks and Clock and the box "hidden sheets (13)"; the twelve rows whole within their columns; `H3` to `H6` the first row from the first frame; Build "same bytes" | *Measurement*, the owner's screenshot and comparison, in the owner's Chrome at full screen, on the page built in the scratch worktree, byte for byte the shared tree's: the strip read Screen, Day, Dice, Market, Stocks, Caveats, Clock, File and Input, the box "hidden sheets (13)"; the Caveats sheet's twelve rows whole, every text inside its column and every reference inside D; at frame 950, `G1` 12, `G2` 1, `G3` to `G5` the first row, its texts spilling over the empty columns to their right, and `G6` empty; every label in F whole, `Management's response` among them; the budget line 93 formula cells a frame, 2,790 a second; and Build in the drawer, "same bytes (24985 characters)". Inside, the addresses the approval's |

**What the build found.** The content shape holds for a sheet that is not
a question bank: `caveats.vla` is the bank's shape in four fields, and the
library changed only by the state it no longer writes, the cartridge's diff
decision 6's proof, one line out, the notes, and 71 in, no row of the
Disclosures or the Questions sheet among them. The engine takes one state a
sheet, refusing a second `sheet` row that gives another, so the state is
the source's row, and a sheet whose state the source forgot stands visible,
which the tab order names. A text spills only into empty cells on its
right, so the labels' column at the default width clips a long label beside
its value; the source widens the Caveats sheet's F to 21, its longest
label's, and the hidden sheets' labels stay clipped where nobody reads
them. The new tests catch what they stand for: seven mutants, each expanded
into the cartridge before the suite ran, failed the test written for them.
The Questions sheet's state forgotten and the Caveats sheet hidden failed
the tab order; a caveat pointing at an empty cell, and one at no sheet,
the cells' test; a caveat past its column, the whole test; the row shown
written 2, the first row's test; and the first caveat stripped of its plain
statement, the whole test, at its second writing only. The first writing
looked for the word, which passed the mutant, since *advice* stands in the
caveat's explanation too; the test now holds a sentence that denies advice,
a `not`, `no` or `never` before the word. And the census's `most` is the
most readers one changed cell is checked against, in an index by row that a
row of sixteen splits again by column (P2).

## `FRONT.2` — the looks: Lovelace Lavender on the views, and a glyph beside each change

*Scoped 2026-10-16 at the owner's word, "Scope and implement FRONT.2 in
Buffet (Delta/Buffet): the looks, Lovelace Lavender on the views and a
glyph beside each change, upward in a minty green and downward in a
burgundy", to stop for the owner's approval before building, and to draft
any other repository's item as a `MISE.6` row and a starting prompt, both
in chat: from `OMAKASE.md`'s line (`COURSE.0`'s item 7) with `MISE.3`'s
approved spelling and `FRONT.1`'s note on it; `FRONT.1`'s rescoping and
record; `MISE.3`'s probe 2; `BD-2`, `BD-5`, `BD-6` with its amendment and
`BD-7`; Alonzo's `SPEC.md` §3.1, §4.3 and §7.3 and the kit's looks; and the
owner's words of the same day: "upward glyphs should be a minty green like
#ADEBB3, while downward glyphs should be a burgundy like #800020, but
obviously any hex code can be used by authors to customize their color."
It comes now because `FRONT.1` closed (`965d141`): the Portfolio shows each
stock's change since yesterday as a bare number, a loss by its minus sign,
and every number a view shows from here takes these looks, `PLATE.1`'s
market view and `FRONT.3`'s overview among them. While it was being scoped
`alonzo` 0.1.5 reached crates.io, and the pin moved in the tree at the
owner's word, a change of its own awaiting its commit (below). Each
decision carries its recommendation, for the owner to approve, amend or
strike. The predictions were written before any number was taken. Alonzo's
and Frazaro's trees were read and never touched; a peer's uncommitted work
stands in Frazaro's `vla-lang`.*

**What it stands on, read on the day** (`vla-lang` 0.9.4 and `alonzo` 0.1.4
and 0.1.5 as the registry holds them; `web/` as vendored; Buffet at
`965d141`, its main nine commits ahead of its remote).
- *The pin at `alonzo` 0.1.5.* The release is `ENGINE.7`, edit and
  continue; its crate's code is 0.1.4's and its pin `vla-lang` =0.9.4; its
  page alone changed, the template and `host.js`, while `viewport.js`,
  `fake.js` and the kit stand as at `v0.1.4`. Moved in the tree as
  `bd93434` moved it to 0.1.4: the pin and its comment, the lock, the
  README's line, the two files copied at `v0.1.5` (`fb3b2a4`),
  `check_vendored.ps1`'s tag and two digests, and `MISE.6`'s row for
  `ENGINE.7`, met. The 45 tests green, the cartridge its source expanded
  and the describe golden standing; ALL GREEN, 4 checks and 4 controls; the
  page 1,821,787 characters where 1,787,808 stood, its module 766,267
  bytes, unchanged. What it means here: the drawer's Apply the source
  writes the cell and formula rows a rebuilt source changed into the running
  game, and for any other row, a format, a look or a column's width among
  them, it offers Load (its `RELEASES.md`).
- *A format* (`vla-lang`'s `machine/load.rs`, `format`; `sheet.rs`,
  `Style`): `(format <index> "<RRGGBB>"|none general|text wrap|nowrap)`, a
  fill, a number format and wrap, and nothing else: no text color, no font,
  no border. Formats are the workbook's and not a sheet's, up to 65,536 of
  them, and a cell, a column or a look names one by its index. A `style`
  row gives one cell its format, a single cell an earlier row wrote, never a
  range; a `look` row maps a value on one sheet to a format, a text by its
  characters, and a second look for the same value naming another format
  is refused at load; a `column` row gives a column its format; a sheet's
  `gridlines` are on or off.
- *The draw* (`web/viewport.js`, `formatOf` and `_paintCell`): a cell draws
  with its look's format when its value has one, else with its own
  style's, else, an empty cell alone, with its column's, else with format
  0. The fills are painted first, the gridlines over them in the page's own
  `#D9D9D2`, and every value's text last in the page's `#1D1D1B`, a
  formula before its first value in `#0A6B4F`. Found by reading, and not
  seen at `MISE.3`, whose background probe was timed and never looked at: a
  cell that holds anything never takes its column's format, so the approved
  spelling of a view's background, "its columns' format and never a filled
  cell", colors the empty cells alone, and every text and number of the
  view would stand in a white hole unless it carried a `style` row of the
  same fill.
- *Every frame* (`vla-lang`'s `view.rs`): the values record is the grid's
  without its `formula` rows, its `format`, `look` and `style` rows kept,
  and a window's changes print each changed cell's rows as the values do,
  its `style` row with it (`KERNEL.42`), which the viewport lays on its
  model by the same reading; a look or a style holds at every frame.
- *What `describe` prints* (`alonzo`'s `cartridge.rs`): the manifest, the
  devices, the budget and the census, and no `format`, `look` or `style`
  row. The line's oracle, "the record's `look` and `format` rows held by
  the describe golden", cannot hold them; a test over the view's own record
  can.
- *The kit's looks* (`src/common.vla`, `looks`): a look map from the numbers
  0 to n, each to a format of its own fill, the Palette's twin in a grid, as
  Kairos keys its blocks; no use to a glyph keyed on its own text.
- *The Portfolio* (`FRONT.1`): `A1` the date; `A4:C4` the headers; rows 5 to
  8 a stock each, its name, its price in `B` and its change in dollars in
  `C`, `=(Stocks!E5-Stocks!F5)/100`; `A10` the fine print; widths 22, 12
  and 12; the default row height; gridlines on; no format; fourteen formula
  cells, read by no formula. Caveat 12's `Where` is `Portfolio!B5:C8`.
- *The dialect*: fourteen functions, no `SIGN`; `IF` and the comparisons; a
  text result runs as a number does, as the names' `=Stocks!A5` runs.
  `MISE.3`'s probe 2 carried `▲` and `▼` through the pass, the load, the
  record, the page's own Build and Buffet's checks, and drew them crisp in a
  3-character column, shaded by looks keyed on their texts.
- *The owner's colors, by WCAG 2's formula* (*derived*, by a scratch script;
  the bars 4.5:1 for text and 3:1 for a graphic, WCAG's 1.4.3 and 1.4.11).
  The mint, `#ADEBB3`, is light, its luminance 0.716; the burgundy,
  `#800020`, is dark, 0.047, near the view's own ink's 0.012:

  | | on white | on the whisper, `#F7F4FC` | on `#E2D8F6` | under the view's ink | under a white ink |
  |---|---|---|---|---|---|
  | the mint as the glyph's own color | 1.37 | 1.26 | 1.00 | | |
  | the burgundy as the glyph's own color | 10.83 | 9.96 | 7.94 | | |
  | the mint as the glyph's fill | 1.37 against the sheet | | | 12.31 | 1.37 |
  | the burgundy as the glyph's fill | 10.83 against the sheet | | | 1.56 | 10.83 |

  No one background lets both read at 3:1 as glyphs: the mint wants one of
  luminance 0.205 or less and the burgundy one of 0.241 or more, and the
  best between them, the gray `#828282`, reads each at 2.81:1. A light color
  is a fill and a dark one an ink, as Excel's own presets pair them, "Green
  Fill with Dark Green Text", `#C6EFCE` under `#006100`, and "Light Red
  Fill with Dark Red Text", `#FFC7CE` under `#9C0006` (*exhibit*, the
  defaults as XlsxWriter's documentation copies them): the owner's mint
  stands near the first fill, and the burgundy near the second ink.
- *Excel's ways of coloring a glyph* (*exhibit*). A number format's color
  code names one of eight colors or `[Color1]` to `[Color56]`, a slot of the
  workbook's palette, which a workbook may redefine (`Workbook.Colors`, the
  styles part's `indexedColors`), a route Excel's dialogs no longer offer.
  A cell style's font color takes any RGB, and so does a conditional
  format's, which is the look's twin, a format a cell takes by its value.
  Only a font color takes a hex code as its own spelling. A sheet's gridline
  color is a palette index, `colorId`, never an RGB.
- *The convention* (*exhibit*): mainland China and Taiwan show a gain in red
  and a loss in green; Korea a gain in red and a loss in blue; Japan's sites
  differ; Hong Kong shows a gain in green, as the US and Europe do.
- *The frame now* (`FRONT.1`'s P8, over the page at `v0.1.4`): on the
  Portfolio at the owner's full screen, a still frame's tick 0.2 ms at p50
  and 0.3 at p95, a market day's patch 0.5, 580 of 600 frames still, the
  main thread 0.74 to 0.78 ms a frame.
- *What a text color would touch* (read in Frazaro's tree, untouched): the
  language's `Style`, whose three fields are public and which
  `frazaro-core` builds by struct literal (`build.rs`, `sheet/merge.rs`,
  `sheet/ooxml.rs`), so a fourth field is a break in a 0.9 release (the
  owner's rule: an API break is rare, and the owner's call); the loader's
  `format`, which takes exactly four items; the record's `format_row`; the
  core's writer, whose styles part holds one font, Calibri 11, every
  format's `fontId` 0. The core's reader prints no format row. And in
  Alonzo, the viewport's reader, which reads a format's first four items
  and would pass over a fifth, its painter, `SPEC.md` §7.3 and a version.

**The decisions.**

1. **The glyph's color: each of the owner's two in the role its lightness
   can play (rec).** A gain's `▲` drawn in the view's own dark ink on a
   fill of the mint, 12.31:1, and a loss's `▼` drawn in the burgundy on the
   sheet's own white, 10.83:1, no fill; a flat day's `=` in the view's ink
   on white, 16.88:1. Each color is the owner's hex and each is legible: the
   mint is a light green as Excel uses one, a fill behind dark text, and the
   burgundy a dark red as Excel uses one, the text itself. The record says
   the first today and cannot say the second, which is the ask (decision
   2). The direction never rests on a color: the glyph's shape says it
   (WCAG's 1.4.1), and the pair's lightness, 0.716 against 0.047, tells
   them apart to a person who cannot tell red from green.
   *The alternatives, each with its contrast:*
   - (a) *The owner's words to the letter, each glyph in its color:* the
     burgundy at 10.83:1 on white and the mint at 1.37:1, a gain nearly
     invisible, and 1.00:1 on a light lavender; a dark sheet turns it about,
     the mint at 12.65:1 on the night `#1C1828` and the burgundy at 1.60:1.
     It needs the ask too.
   - (b) *Both as fills, today, no ask:* the `▲` at 12.31:1 on the mint and
     the `▼` at 1.56:1 on the burgundy, a dark block whose glyph cannot be
     read.
   - (c) *Both as fills, each under the ink that reads on it:* the mint
     under the view's ink, 12.31:1, and the burgundy under a white ink,
     10.83:1, two cells of one shape; the ask, as (rec) is. The loss's block
     outweighs the gain's, the burgundy at 10.83:1 against the sheet and the
     mint at 1.37:1.
   - (d) *No ask, the shades moved to fit fills:* the burgundy given up for
     a light red the view's ink reads on, Excel's own `#FFC7CE` at 11.51:1.
   - (e) *An ask of Alonzo alone:* the painter drawing a value white over a
     dark fill by its luminance, (c) with no change to the record; but no
     ink an author names by its hex, and an ink nobody declared (`SD-32`).

2. **The ask, and the dish split by it (rec).** A format's text color, any
   hex, beside its fill: Excel's font color, in the language's format, the
   record and the core's writer (Frazaro), then in the viewport's painter,
   a `SPEC.md` version and the pin (Alonzo), each repository minting its
   own item; drafted as a `MISE.6` row and Frazaro's starting prompt, both
   in chat, for the owner to ask. `FRONT.2` is then split by what the record
   can say:
   - *the first slice, now:* the glyph column and its formula, the gain's
     mint, the flat day's `=`, the band of Lovelace Lavender and the house's
     formats in one place, the loss's `▼` in the view's ink, 16.88:1, until
   - *the second slice, at the pin that brings the text color:* the loss's
     burgundy, one format and one look, with a short scoping of its own
     beside this one, its predictions and the owner's hand test; and the
     band deepened under a white ink, if the owner's eye asks for it
     (decision 6).

   The first slice holds nothing the second takes back, the mint being a
   fill under every legible choice of decision 1. *The alternatives:*
   `FRONT.2` gated whole on the ask (🔒), the glyph, its colors and the
   lavender landing together after two releases, as the owner has chosen
   for an engine's item before ("the whole slice waits"); or split as the
   owner's prompt framed it, the glyph and the lavender now and both colors
   at the second slice, so that the pair is first judged together; or no
   ask, decision 1's (d).

3. **The colors live in the source's format rows, in one place (rec).** The
   house's formats in a section of their own before the views, each a row
   with its hex and a comment naming its role, in the record's own
   spelling, so that a person reading the cartridge sees the hex an author
   edits:

   ```text
   ; ---- FRONT.2: the looks -----------------------------------------------------------
   ; The house's formats, the workbook's own: each a fill as six hex digits, the
   ; record's spelling, named by its index wherever a view's style or look draws
   ; with it, so that an author recolors every view here and nowhere else.

   (format 1 "A080E0" general nowrap)   ; the band under a view's headers: Lovelace Lavender, the logo's own
   (format 2 "ADEBB3" general nowrap)   ; a gain's glyph: a minty green behind the view's dark ink
   ```

   A view's styles and looks name them by index. No verb enters the
   library: a view writes its glyph's look by hand, and the second view that
   writes one, `PLATE.1`'s market view, hoists the pattern into
   `kitchen.vla`, as the ratchet has it; the kit's numeric `looks` stays
   unused. A player cannot change a color: a format is a row the load reads,
   and a person's act writes a cell or a formula and nothing else (`SPEC.md`
   §4.3). A player who wants another color becomes its author: the page's
   drawer holds the source, and its Build and a Load play the colors from
   the first frame, since Apply the source carries no format and no look
   (0.1.5). *The alternative:* a library verb taking the colors as its
   arguments now, at their first writing.

4. **The glyph reads the change beside it (rec).**
   `=IF(C5>0,"▲",IF(C5<0,"▼","="))`, filled over `D5:D8`: the glyph says
   what the number beside it says, exactly, since a whole number of cents
   over 100 keeps its sign, and the pattern is any figure's direction,
   which every later figure takes, a gain, a return, alpha. A view may read
   its own cells (`FRONT.1`'s decision 10), so nothing reads a view still
   holds; `describe` gains `(readers "Portfolio" 1)`, the view's own read,
   where no readers row named a view before, and a level, the glyph's 15.
   The flat day's `=` takes no look. *The alternatives:* the glyph from the
   model's cents, `=IF(Stocks!E5>Stocks!F5,"▲",IF(Stocks!E5<Stocks!F5,"▼","="))`,
   no readers row for a view and no level more, but the comparison the
   change holds written a second time, two cells read a glyph; or a flat
   day blank, `""`.

5. **The glyph column: `D`, right of the change, 3 characters, no header
   (rec).** The figure and then its direction, `0.37 ▲`, the number
   right-aligned at `C`'s edge and the glyph at `D`'s: nothing of `FRONT.1`
   moves, no cell and no test, and caveat 12's `Where`, `Portfolio!B5:C8`,
   stays true. 3 characters, 21 pixels, as `MISE.3` drew it. Its header an
   empty text in `D4`, so that the band spans the table, the change's header
   naming both columns. *The alternatives:* the glyph before the change,
   `▲ 0.37`, as a ticker writes it, the change moved to `D` and its tests
   and the caveat's `Where` with it; or no cell in `D4`, the band stopping
   at `C`.

6. **Lovelace Lavender on the Portfolio: a band under the headers, the sheet
   white, gridlines on (rec, by the owner's eye at the hand test).** The
   band over `A4:D4` in Lovelace Lavender itself, `#A080E0`, the logo's
   (`BD-6`), the view's ink on it at 5.35:1: one format and four `style`
   rows. The sheet stays white, a workbook's: a background behind a view
   takes a `style` row for every cell that holds something and a `column`
   row for every column in view, some 38 at the owner's full screen, and
   stops at the last column written. Gridlines on, in the page's gray: the
   record holds no gridline color, and Excel keeps a sheet's as a palette
   index. The shade is one hex, judged at the hand test: `#E2D8F6`, lighter,
   12.37:1; or, at the second slice, `#6B4FC8` under a white ink, 5.87:1,
   the dark header band of Excel's tables. *The alternatives:* the table's
   body in the whisper, `#F7F4FC`, sixteen `style` rows more, 15.52:1 under
   the ink; gridlines off, a report's page. With it, `MISE.3`'s approved
   spelling amended, dated: a view's background is its columns' format for
   its empty cells and a `style` row of the same fill for each cell that
   holds something.

7. **What of the line's rest goes where (rec).** Struck from `FRONT.2` when
   its line is rewritten at the build, each with its reason: "the look map
   (`looks`)", the kit's numeric map, the house's formats in its place;
   "the panels' backgrounds", since a panel is a view (`FRONT.1`) and a
   view's lavender is its band, each later view taking the house's formats
   at its dish; "the chosen tab", the page's own strip (Alonzo's
   `viewport.js`, its `tabActive`), which no cartridge colors, a candidate
   for Alonzo's page if the owner wants the strip in lavender; "the chosen
   pocket", `PLATE.2`'s selector, its look with it; "a number format with a
   glyph instead", answered at `MISE.3`; and the oracle's "held by the
   describe golden", which prints no look (decision 9).

8. **No caveat owed (rec).** A caveat is a simplification of the world, and
   a color is a convention of one's own market, which the glyph's shape
   states whatever its color. Offered for the owner's taste, a thirteenth
   row in the house's voice, its `Where` the glyphs:

   | The field | The draft |
   |---|---|
   | Things this game gets wrong | A gain is green and a loss is burgundy. |
   | In the real world | In mainland China and Taiwan a gain is red, the color of good fortune, and a loss is green. |
   | Management's response | Management, advised that red is good news in Shanghai, has asked to report from there. |
   | Where | `Portfolio!D5:D8` |

9. **The oracle (rec).** In `oracle/src/buffet.rs` and
   `oracle/tests/cartridge.rs`:
   - *The hand's glyph*: `glyph(now, was)`, which way whole cents went,
     sharing nothing with the cell's comparison of the dollars' change with
     0; and its test, the three cases, and over the seed's two years each
     glyph shown on some day.
   - *The glyphs at every frame*: `portfolio_held`'s window `A1:D10`, each of
     `D5:D8` the hand's glyph on a day's first frame and the record byte
     for byte through the day, so at every frame of two years and two days
     and on every one of eight hundred days at one frame a day.
   - *The looks, drawn*: a test over the Portfolio's own record, which prints
     its formats, looks and styles, drawing each cell's format by the
     viewport's rule, a look by the value first, then the cell's style, then
     for an empty cell its column's: the band under `A4:D4`; under each
     glyph the formula can show, the house's fill for it, the mint under `▲`
     and none under `▼` and `=` (the second slice's ink then), the glyphs
     themselves held to the hand at every frame above; no other cell filled;
     no look on a view keyed on a number, which would shade every cell
     holding it (`MISE.3`); and every format a view's cell draws with
     reading at 4.5:1 or more, its ink, the view's `#1D1D1B` until the second
     slice gives one, over its fill or the sheet's white. The last holds an
     author's hex to legibility; an author who wants a color past it lowers
     the bar in the same commit.
   - *Kept*: the cartridge its source expanded; `BD-7`'s scan and the
     census's `(every-frame 0)`; nothing reads a view, the Portfolio's
     formula cells 18 of its 1,000; the describe golden blessed again and
     read in its diff with the shapes' numbers stripped.
   - *Mutants*, each written into the source by one anchor and expanded
     before the suite runs, each failing the test written for it: the glyph
     reversed; the flat day lost, `>=` for `>`; the glyph reading the frame;
     a look keyed on a number, `1`; `D4`'s band dropped; the gain's fill the
     burgundy, under the view's ink at 1.56:1; the glyph's look on the
     Stocks sheet; the band in `#6B4FC8` under the view's ink, 2.88:1.

10. **The looks' cost, re-measured on the real view (rec).** As `FRONT.1`
    took it: natively, a scratch runner pinning itself to the performance
    cores and opting itself out of Windows' throttling, alternated against a
    `git archive` of `965d141` built in a target of its own, its pin
    `alonzo` 0.1.4, whose code 0.1.5 carries unchanged but for its version
    test (P3 to P5); in the browser, `FRONT.1`'s instruments over the page
    built here, three runs of 600 frames in headless Chrome at the owner's
    full screen, 2,560 by 1,440 at 1.5, every process opted out and pinned,
    each tick filed by what its frame drew, `965d141`'s cartridge first on
    the same 0.1.5 page, so that the page's own change is not counted as the
    looks', and the probe copy in the owner's own Chrome (P8).

11. **The hand test, the owner's, on the page built here at 0.1.5:** the
    band across `A4:D4`, its shade by eye, one hex to change; each stock's
    glyph beside its change, `▲` on the mint on a gain, `▼` and `=` in the
    view's ink, each agreeing with its change's sign, every glyph seen
    within a minute; Build in the drawer, its text compared with
    `src/buffet.vla` by a command given at the handover, since the page
    prints no "same bytes"; the probe copy's corner, one screenshot after a
    minute.

12. **The stale places, rewritten at the build.** In `OMAKASE.md`: `FRONT.2`'s
    line, 🟡 from the approval, its text as decisions 1, 2 and 7 leave it,
    the first slice's close dated on it; `MISE.6`, the ask's row; `MISE.3`'s
    spelling of a background amended on its line (decision 6). The source's
    opening comment, the formats' section and the Portfolio's section;
    `oracle/src/buffet.rs`'s module comment and `glyph`; the module comment
    of `oracle/tests/cartridge.rs`; the describe golden; `Delta/CLAUDE.md`'s
    map, a format or a look among the edits that trip a test; the memory.
    The first slice's record goes under this scoping, and `FRONT.2` leaves
    `OMAKASE.md` for the ledger when its second slice closes. Not this item:
    the other views and their bands, each its dish's; the strip's colors,
    Alonzo's page; the pocket's look, `PLATE.2`'s; number formats in the
    record, `MISE.6`'s candidate; the replay golden, the tasting's.

**The predictions, before any number**: amended at the approval, below,
to the build approved, the colors deferred, before any number was taken
(the colored slice had predicted 439 rows, ten lines in). The numbers
column is filled at the build, P9 at the owner's hand test; P3 to P5 and P8
as decision 10 says.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 431 rows where 429 stand: `D`'s width and the glyph's fill; 111 formula cells where 107; the diff two lines in and none out, 104 characters, no other row moved | *Measurement*, as predicted but for the characters: the runner printed `(rows 431) (loads yes)`; 111 formula cells; the diff two lines in and none out, `D`'s width and the glyph's fill, no other row moved. **Missed** in the characters: 107 where 104, each quoted glyph five characters with its escapes, counted as four |
| P2 | the describe golden | `(budget 30 111 3330 2000000)`; `(plan (shapes 48) (texts 46) (formulas 111) (levels 16) (runs 10) (lanes 43) (alone 68) (every-frame 0) (cycles 0))`; `(readers "Portfolio" 1)` first among the readers, the rest unmoved; a shape 4, `"Portfolio" "D5"`, `(cells 4) (levels 15 15 1 4) (runs 1 4 4 4) (alone 0) (reads lookup)`; `A10` shape 5 and every shape after it one more, its `same` with it; nothing else | *Measurement*, as predicted in every count: `(budget 30 111 3330 2000000)`; `(plan (shapes 48) (texts 46) (formulas 111) (levels 16) (runs 10) (lanes 43) (alone 68) (every-frame 0) (cycles 0))`; `(readers "Portfolio" 1)` first among the readers; shape 4, `"Portfolio" "D5"`, `(cells 4) (levels 15 15 1 4) (runs 1 4 4 4) (alone 0) (reads lookup)`; `A10` shape 5 and the 42 shapes after it their own rows but for their numbers, `(same 12)` and `(same 13)` where 11 and 12 stood |
| P3 | the pass and the load | the pass 0.01 to 0.1 ms longer, two rows calling no picker; the load 0.01 to 0.06 ms longer | *Measurement*, a scratch runner over each tree, `965d141`'s and this one's, each in a target of its own under the profile the tests use, three runs each alternated, 200 passes and 200 loads a run, every run pinning itself to the performance cores and opting itself out of Windows' throttling, a fixed computation before and after within 2.5%: the pass 4.40 to 4.47 ms at p50 where it took 4.43 to 4.49, the medians 4.46 and 4.44, 0.02 apart, inside; the load 1.50 to 1.53 ms where 1.51 to 1.54, the medians 1.52 and 1.53. **Missed** in the load, below the range: two rows' load is inside the noise |
| P4 | the step | an ordinary frame unmoved, within 0.1 µs, one shape more; a day's first frame 0.5 to 2 µs more, four glyphs; the first frame 2 to 8 µs more | *Measurement*, the same runs, two years and two days at thirty frames a day: an ordinary frame 2.0 µs at p50 in every run of this tree, where it took 1.9 to 2.0, unmoved; a day's first frame on which the fine print turns 15.8 to 16.1 µs where 15.0 to 16.5, the medians 0.6 apart, inside; the first frame 80.0 to 82.9 µs where 78.9 to 82.5, the medians 2.2 apart, inside. **Missed** at the median in one: a day's first frame on which the fine print stays 15.0 to 15.1 µs where 14.0 to 15.1, the medians 0.4 apart, a tenth under the range, the runs overlapping |
| P5 | the suite | 46 tests green, the hand's glyph's test more, and in CI on Linux; within 0.3 s of `965d141`'s at the median | *Measurement*, here: 46 green, 16 of the crate and 30 of the cartridge. Each tree's suite run alternately, five rounds, the order turned each round, each run pinned and opted out: on eight threads 4.76 to 4.85 s where it took 4.73 to 5.71, the medians 4.78 and 4.76, 0.02 apart; on one thread 8.65 to 9.11 where 8.55 to 8.86, the medians 0.09 apart: inside. A first alternation on eight threads had the commit before's rounds at 5.51 to 5.70 s and this one's at 4.61 to 5.20, faster though it holds more, which no test timed alone bore out: alone, the longest test is 0.17 s dearer here, 4.69 against 4.52, and the thirty 0.26 s dearer in sum, so the first round's gap was the machine's. Linux at the push, run 38939138517 at this commit, `ec069e8`: the 46 green, 16 of the crate and 30 of the cartridge, the cartridge's in 8.25 s |
| P6 | the page | the cartridge's block 104 characters longer, its two rows, and the source's 800 to 2,000: the page near 1,823,200 where the pin left 1,821,787; Build writing the cartridge byte for byte | *Measurement*: 1,822,477 characters, 690 more than the pin's 1,821,787, the cartridge's block 107 longer and the source's 583. **Missed** in both: the cartridge's by the three characters of P1, and the source's under its range, its comments shorter than priced, so the page grew 690 where 904 to 2,104 was predicted. Build writing the cartridge byte for byte, 25,808 characters, checked headlessly over the probe copy |
| P7 | the checks | 4 checks and 4 controls green, no check edited, no file added under `src/` | *Measurement*, as predicted: ALL GREEN, 4 checks and 4 controls, the layout control's fourteen mutants and the offline control's fifteen as they stood; no check edited by this item, the pin's change to `check_vendored.ps1` being its own; no file added under `src/` |
| P8 | the glyph's cost in Chrome at full screen | `965d141`'s cartridge on the 0.1.5 page as `FRONT.1` measured it on 0.1.4's; then with the glyphs, a still frame's tick 0.2 ms at p50 and 0.3 at p95, nothing painted; a market day's patch 0.5 to 0.6 at p50, up to four cells more; 580 of 600 frames still and 20 patched, a glyph moving on a day's first frame alone; the main thread 0.72 to 0.82 ms a frame; the owner's Chrome a tenth faster or so | *Measurement*, six runs of 600 frames alternated in headless Chrome at 2,560 by 1,440 CSS pixels at 1.5, its eleven processes opted out and pinned, the clock within 1.0%: `965d141`'s cartridge on the 0.1.5 page, a still frame's tick 0.2 ms at p50 and 0.3 at p95, at its longest 0.4 to 0.5; a patch 0.5, at its longest 0.6 to 0.7; 581 of 600 frames still and 20 patched; the main thread 0.70 to 0.72 ms a frame. With the glyphs, the same at every percentile, at their longest 0.4 to 0.6 and 0.7; 580 or 581 still and 20 patched; the main thread 0.71 to 0.74: the glyphs cost nothing the page's clock, in steps of 0.1 ms, can see. **Missed**, by a hundredth, in the main thread's least, 0.71 where the range began at 0.72. The owner's own Chrome, the probe copy at full screen for 4,120 frames: still 0.2 ms at p50 and 0.3 at p95, at its longest 0.7; patched 0.4 and 0.6, at its longest 1.4; 3,981 still and 137 patched, and one other frame, a full paint, 1.3 ms: `FRONT.1`'s own within a tenth, and the patch a tenth under headless, as predicted |
| P9 | the hand test | every glyph beside its change, whole in its 3-character column and agreeing with its change's sign; Build's bytes the file's, checked headlessly | *Measurement*, the owner's two screenshots, in the owner's Chrome: the page windowed at frame 709, day 24 of the first year, rungs 1 to 3 at a change of 0 beside `=` and rung 4 at −0.01 beside `▼`; then the probe copy at full screen at frame 4,248, day 142, rungs 1 to 3 at 0 beside `=` and rung 4 at 0.01 beside `▲`; each glyph whole in its column and agreeing with its sign; and the owner's word, "Tests pass; commit." Not in the screenshots, checked over the probe copy in headless Chrome at the owner's full screen: Build in the drawer, the same bytes as `src/buffet.vla`, 25,808 characters; `B19` the sheet shown and the pointer's `B11` and `B12`, as at `FRONT.1` |

*Scoped 2026-10-16. Approved by the owner the same day with the colors
deferred: "Ignore colors for now; proceed with FRONT.2 as originally
scoped. COlors can e added later as an aesthetic pass." So the first slice
is the glyph alone. Decisions 4, 5 (with no cell in `D4`, the band it was
to carry deferred), 9's glyph half, 10, 11 and 12 stand, and 7's strikes;
decisions 1, 2, 3, 6 and 8 and 9's looks wait for the aesthetic pass, the
second slice, its place in the order the owner's. The ask is not made, its
row a candidate in `MISE.6`.*

**What the build found.** The glyph is two rows of the cartridge and one
shape of the plan: a view reading its own cells gives the census its first
`readers` row for a view, and a level, the sixteenth; the rule that nothing
reads a view holds, its test passing over a view's own reads. On the page
built here the live glyphs agree with their changes (headlessly, on days 23,
26 and 28 of the first year: `=` beside 0, `▼` beside −0.01 and −0.02), and
Build writes the file's bytes. Five mutants, each written into the source by
one anchor and expanded into the cartridge before the suite ran, each failed
the test written for it: the glyph reversed, the flat day lost to `>=` and
the flat day respelled, the two years' frames; the glyph reading the frame,
whose values do not move, `BD-7`'s scan alone; a model's sheet reading the
glyph, nothing reads a view. And a lesson in reading a suite's time: the
first alternation on eight threads had the commit before slower by 0.86 s at
the median, though this one holds more, and each test timed alone in both
trees found the opposite and where it lies, the two years' test 0.17 s
dearer, so the eight threads' difference was the machine's and a second
alternation agreed (P5).

### The second slice: the looks, over the pin to `alonzo` 0.3.0

*Scoped and built 2026-10-30 at the owner's word, "Move the Alonzo and vla-lang pins, then
proceed with FRONT.2's second slice", from this section's decisions 1, 2, 3 and 6, the line
in `OMAKASE.md` and its rows in `MISE.6`. Each decision below is built as recommended, for
the owner to amend at the hand test, and decisions 3 and 4 and the two after 9 stand as the owner's
two hand tests of the same day amended them. The pin is a `HOUSE.2` commit of its own, as `ce6f126`
stood beside the first slice.*

*Passed at the owner's third hand test, 2026-11-15, "Tests pass; commit", and committed after the
pin; closed in the ledger, 🛎 THE FRONT OF HOUSE.*

**The pin.** Tried in scratch earlier the same day, a target a copy. `alonzo` 0.3.0 and
`vla-lang` 0.12.0 compile under the oracle with no port, and over the old kit every frame
holds and the cartridge is its source expanded: only the describe golden moves, by the
census's words (`slot` for `lookup`, `(unlowered text)`), its plan's `(every-frame 0)`
standing. The cost is the kit. `KIT.5`'s verbs take expressions, and a text is one, so 35
calls (`hold` 15, `count` 11, `pressed` 2, `put` 7, two rows each) wrote 42 rows of quoted
texts, `=IF("B11=1",...)`: the pass said `(loads yes)`, `--check` said `same` once expanded
again, and 27 of the cartridge's 54 tests failed, its frames alone catching it. All 42 came
back through `formula_syntax::read` and `write` byte for byte, so each piece was respelled
as its expression, and the pass proved it: the rows that moved from the committed cartridge
were the manifest's three and no other. And the page: a character typed on a cell opens its
edit, and Delete empties it, on any sheet a `(spec 6)` manifest does not name in `(read ...)`
(`CART.9`, `SPEC.5`), a cartridge at `(spec 5)` among them. So:

1. **The views the game's and read-only, the Caveats read-only, every other sheet a person's
   (rec).** `(game ...)` and `(read ...)` each name the seven views, and `(read ...)` the
   Caveats too. The model stays the open book (`BD-3`): the waiver `Proctor!B3`, the paces
   `Day!B1` and `B28`, sandbox edits by the owner's word of 2026-10-19, and `PLATE.6`'s edit
   are a person's writes into it. *Alternative:* the model read-only too, which turns those
   three into rows no person may make. Seen in Chrome: Q and Delete on the Portfolio refused
   by the sheet's name and no cell moved; Q on the Day sheet opens its edit; the views' keys
   and clicks as before.

**The slice's decisions.**

2. **A view's figures numbers under a format's code (rec, the `MISE.6` row's own plan).**
   `dollars`, `signed-dollars` and `percent` keep their calls and write `=Stocks!E5/100` and
   a `style` row over the fill, where they wrote nested IFs of over a thousand characters a
   row, padded to a number's edge. The codes `#,##0.00`, `+#,##0.00;-#,##0.00;+0.00` and
   `+0.00%;-0.00%;+0.00%`, a flat figure keeping the `+0.00` the owner has seen; and `#,##0`
   on the shares, the owner's "Thousands separators" reaching them. `values`, the oracle's
   reader, reads what a cell draws, the record's `text` row. The hand spells the same cents by
   counting digits, sharing no rule with the code, and the padding's constants are gone.
   *Found:* a code draws fifteen significant digits, as Excel does, so past $10 trillion a
   view rounds its cents, $90,071,992,547,409.91 drawing `.90`, where the texts were exact to
   2^53 cents; the model's cents stay exact, and `every_amount_...` holds the drawing to 10^15
   cents less one and the rounding once. No caveat: it is Excel's own behavior. A figure too
   wide for its 12 characters draws `#`s, as Excel does and a person drags wider: past
   $9,999,999.99, as seen, and by the widths `CRITIC.1` measured a change past $999,999.99 and a
   percent past 99,999.99%; a code refuses a color,
   `[Red]`, so a glyph's shade stays a look.
3. **A button white on `#6B4FC8`, centered and bold, but for Trading's grid (rec, the owner's
   words at `GUARD.1`'s hand test).** 5.87:1, where white on the logo's `#A080E0` reads at 3.2:1, the shade the
   grid card already stands in for. An exam's option the same, reading from its left, its text
   long, first built not bold and bold at the owner's word at the hand test: "the white text over
   lavender background should be bolded for readability". The widest of the bank's and the firms'
   84 options, bold and with its letter, takes 580 of the column's 692 pixels at 100%, and the same
   share at every ratio measured to 225%, so none spills white onto the sheet's white. Trading's 24
   buttons plain at the second hand test, "perhaps E6:J9 need not be bolded?", under a bold band and
   beside a bold column of keys; the buttons standing alone and the options stay bold. A fill
   covers its cell's gridlines since `ENGINE.28`, which at the pin
   alone drew Trading's 24 buttons and the Exam's four options as one lavender block each, so
   each button says the gridlines' gray itself on its right and bottom, as Superposition's
   sites do. An option emptied keeps those edges and takes the default's fill by `(look "" 0)`.
4. **The band white and bold on `#6B4FC8`, spanning each table (the owner's word at the hand
   test: "white text on lavender fill, also, for readability").** The Portfolio's `A4:H4`,
   Trading's `A5:J5` and the Report's `A6:D6`, 5.87:1, a header to the left over a text's column
   and to the right over a figure's, and centered, with the gridlines' gray on its right and
   bottom, over a column that stands centered (format 14). First built in the logo's `#A080E0`
   under the view's ink,
   5.35:1, decision 6's shade; white on that shade reads at 3.2:1, under the bar, so the band
   takes the button's. A button's brackets now tell the two apart, every other cell of a view on
   that fill saying them. A `style` row needs a written cell, so each column a band spans and no
   header names holds an empty text.
5. **Each glyph its shade, by decision 1's roles (rec).** A gain's `▲` on the mint `#ADEBB3`
   under the view's ink, 12.31:1; a loss's `▼` in the burgundy `#800020` on the sheet's white,
   10.83:1; a flat day's `=` in the view's ink; each centered in its 3 characters. By a look of
   the glyph's own text, never a number. The mint says the gridlines' gray too.
6. **A bold label over what it heads (rec).** The Portfolio's and the Overview's headings and
   the Certificate's title and `Caveat:`. No size is said, so no row moves.
7. **The formats named once, by `define` (rec, over decision 3).** Fifteen formats in a section
   before the views, each a word, `button-look`, `dollars-look` and the rest, which a view's
   rows and the kitchen's verbs name; the expansion spells the indices as ever. *Alternative:*
   bare numbers, as decision 3 had them when the language held no names.
8. **The Report's year column 15 characters (found at the build).** "The year so far", bold
   and to the right, cut on its left in 12, its neighbor holding a text. The test holds every
   band's header to its column.
9. **The caveat on number formats rewritten, a draft (`BD-5`).** "Only the views format their
   numbers: in the model, $12.30 is 1230 and 2% reads 0.02", its `Where` `Stocks!E5:E8`, and
   its joke "The decimal point has returned from furlough, to the front office only." The
   model's cents keep no format, so the oracle reads them as the model holds them; a thousands
   separator on them is one row a range, not taken unasked.
10. **A button standing alone bordered (the owner's word at the hand test: "maybe it needs a
    border?").** Format 13, a button whose four edges are the view's ink, `medium`: the
    Certificate's `[ Open for business ]`, Trading's `[ Switch ]` and Gigs' `[ Proofread a
    prospectus ]`, one rule for the three. `thin` is one device pixel at every ratio, which at the
    owner's 150% could still read as no border, so `medium`, seen beside it at that ratio.
    Trading's 24 and the Exam's four stay among others, the gridlines' gray between them.
11. **Trading's keys where they act (the owner's word at the hand test).** In the band over each
    column of buttons, centered, the key that makes its trade: B, N, M, S, D and F. Beside each
    stock not picked, centered in column A, the key that picks it, 1 to 4, the stock picked keeping
    its mark, `=IF(Orders!$B$16=Orders!B6,"▸",Orders!B6)`, and at the second hand test on the band
    itself, "the same lavender background and white text as row 5", so that the band turns the
    corner down `A6:A9`. The shares and their header centered at the same word, `D5:D9`, by format
    14 and format 15, a count of shares in the middle of its column; the Portfolio's stay to the
    right among its figures, the word naming Trading. No column is ever picked, a key acting at
    once, so each stands always. The keys over the columns are text cells, as `or press C` is, and
    the hand reads each from its command's key, `key_hint`, so that a key moved on the Commands
    sheet with its hint left behind fails. A person's own binding in the drawer's keymap, `(bind "V"
    buffet.buy-1)`, moves a key no cell knows of, as it does for `or press C` and the line at `A15`,
    each saying the cartridge's own.

**The oracle.** `every_format_a_view_draws_reads`, each view's own record drawn by the
viewport's rule at the exam and in session, a look over the cell's own property by property,
every cell that holds something at 4.5:1 or more, no look keyed on a number and no format on
the model. `the_views_draw_the_houses_looks`, the bands, the glyphs, the buttons, the codes and
the headings. `the_codes_draw_the_hands_spellings`, each code as the cartridge's row spells it
through the language's `numfmt`, against the hand, to $5,000 by the cent, a change to $1,000
either way, a percent to 100.00% either way, and beside each power of ten.
`the_views_are_the_games_and_no_person_writes_them`, the manifest's sheets. 96 tests, 38 and
58, where 92 stood; every frame of every run held. 32 mutants, each written into a copy of the
source or the kitchen by one anchor and expanded, each failing: a text piece again; a view off
either list and the model on it; the glyph reversed; a look on a number and on the model; the
band dropped, off its row and in the deep shade; the gain's fill the burgundy and the loss's ink
the mint; a button's ink, shade and edges; an option centered and an emptied one still a button;
each code wrong; a figure at the wrong scale; a percent not yet made; the year's column narrow;
a format on the model; a heading lost. Two of them, written inside an `on` first, were refused
at the pass for their spelling, and were written again at the top level and caught. After the hand
test 26 more, each failing a test other than the describe golden: an option not bold; each band's
shade and ink and its weight; a lone button's edges, gone, gray and thin; each of the three among
others, and one of the 24 alone; a key wrong, gone and two swapped over the columns; a stock not
picked saying nothing or its row, the stock picked saying its key, the marks reversed; the keys and
the mark off center; the deep shade off a band with no brackets; and a command's key and a pick's
moved with the hint left behind. After the second, 11 more, by the same rule: the grid bold again; the
keys off the band and to its left; the centered band not bold and off center, and a key not on it; the
shares' header to the right; Trading's shares to the right and to the left, the Portfolio's centered,
and the centered code ungrouped.

**The measurements**, natively, a runner a pin pinned to the performance cores and opted out of
the throttling, seven alternated runs each, median:

| | `f5dcde1` at 0.2.0 | the pin at 0.3.0 | the slice at 0.3.0 |
|---|---|---|---|
| the load | 11.6 ms | 12.0 ms | 8.1 ms |
| an ordinary frame | 2.3 µs | 1.4 µs | 1.4 µs |
| a mark, a tick of the session | 38.0 µs | 37.9 µs | 20.0 µs |
| the Portfolio's window viewed whole | 46 µs | 51 µs | 82 µs |
| Trading's | 48 µs | 52 µs | 78 µs |
| the cartridge | 99,281 bytes | 99,451 | 84,227 |

The texts' nested IFs gone halve a mark; a whole window costs more to view, each figure's code
writing its text there, which a frame's patch pays for its changed cells alone. The page
3,882,538 characters where 2,521,977 and its module 1,866,609 bytes where 1,038,867, the
language's library and expander, 0.3.0's own (its `RELEASES.md`). In Chrome at 100% and 150%:
the exam by clicks and keys, the certificate's button, orders by click and by key, the switch,
Gigs' button, every view drawn, no error in the console; a card reached in about 3 s with the
paces written.

**Found, unminted.** (a) `MISE.6`, a finding for Alonzo: a text where a verb wants an expression
goes unrefused at the pass, and 0.3.0's notes for a game that carries the page do not name it. (b)
`MISE.6`, a finding for Alonzo: a press leaves the selection's wash on a button, white on it about
2.9:1 until the selection moves. (c) `MISE.6`, a finding for Alonzo that gates `HOUSE.4`'s second
slice: the page's loop oracle runs 15 cases on a game's page, 14 pass over Buffet's module, and
`grid` fails on the Exam at both ratios under Alonzo's own runner: scrolled to its last column the
viewport shows one column and one row past the record's window. The same at the pin's stage and
at a column A of 40 characters, so neither this slice nor the wide column; the case's moves are
newer than 0.2.0's page, whose `grid` passes. The cells there are 100 columns past anything Buffet
draws, so no person sees it today. (d) `MISE.6`, a finding for Alonzo: a stray key on a read-only
view paints the page's own sentence in red and it stays there, "Certificate is a read-only sheet,
named by the manifest's (read ...): a key there writes nothing, and the cartridge's own rows still
write it (version 6, docs/SPEC.md section 7.2)", seen in the owner's own hand test after the exam;
words for an author, where a player pressed a letter. (e) The owner's Snipping Tool captures brighten
every color by 3 in linear light and clip at white: `#6B4FC8` lands as `#B185FF`, `#A080E0` as `#FFD2FF`
and the view's ink as `#363633`, and the mint is lost altogether; measured from the shots themselves,
so a shot of the owner's says nothing of a color. Chrome reads that display as `(dynamic-range: high)`,
and its own capture of a window on it, the call DevTools' "Capture screenshot" makes, holds `#6B4FC8`
to the digit. (f) `MISE.6`, a finding for Alonzo, the owner's question at the second hand test, "Why
is only the first option highlighted": a format's font is lost on a patch's second and later rows.
`_repaint` paints each changed row under its own `clipped`, whose `ctx.restore()` takes the canvas's
font back, while the painter's note `p.font`, `ENGINE.28`'s, keeps the bold, so the next row's `if
(p.font !== font)` sets nothing and draws plain until a full paint. The Exam's four options change in
one frame, so `A7` alone draws bold from the second question on, and a tab away and back draws all
four bold; a pick moved on Trading leaves one key plain, its second row. Keeping the note beside the
canvas's state in `clipped` is one line, and in a scratch copy of the page every option draws bold at
every question. Buffet's vendored file is not edited, and the options stay bold, the owner's word
standing and no page public, until the pin that carries it. My own walks shot the first question
alone, and the hand's frames hold a cell's text and no font drawn: a look on a cell whose text moves
wants its picture after a patch, not only at the opening.

## `BACK.1` — the side hustle, the first button: a prospectus proofread for $25, by a click or a key

*Scoped 2026-10-16 at the owner's word, "Scope and implement BACK.1 in
Buffet (Delta/Buffet): the side hustle, the first button", to stop for the
owner's approval before building: from `OMAKASE.md`'s line (`COURSE.0`'s
item 8), the opening contract's hustle ($25 a claim, once a market day, at
most four a run; personal cash $100; money in whole cents) and the open
decisions' row "the hustle's cap, four a run or once a day for good", which
this scoping puts to the owner (decision 1); `FRONT.1`'s rescoping and
`FRONT.2`'s scoping and their records; `MISE.3`'s probe 3 and `PLATE.1`'s
approved spelling, which copies this dish's button six times a rung;
`BD-2`, `BD-7`, `BD-8` and `BD-9`; Alonzo's `SPEC.md` §2's rule 3, §3.3,
§3.4 and §3.9; the kit's verbs; and the owner's rule of the same day, that
Alonzo, Buffet and Frazaro are keyboard-first, every act a key and a button
its convenience, Alt reserved for the chords. It comes now because
`FRONT.2` closed (`ec069e8`, pushed): the Portfolio shows the day and four
stocks, and nothing a player does reaches the model yet. This is the first
tongs, the smallest click-to-cell pattern there is, which `PLATE.1`'s
buttons and `GUARD.1`'s four options a question copy. Each decision carries
its recommendation, for the owner to approve, amend or strike; the texts a
player reads are drafts, for the owner to approve or rewrite. The
predictions were written before any number was taken. Alonzo's and
Frazaro's trees were read and never touched.*

**What it stands on, read on the day** (`alonzo` 0.1.5 and `vla-lang` 0.9.4
as the registry holds them; `web/` as vendored at `v0.1.5`; Buffet at
`ec069e8`, level with its remote, its tree clean).
- *The cartridge.* `(spec 4)`, 431 rows, 111 formula cells in 48 formula
  rows. No formula reads the Input sheet and there is no Keys sheet, so no
  key is the game's. The tabs: the Screen hidden, then the Portfolio, Day,
  Dice, Market and Stocks, the Disclosures and the Questions hidden, the
  Caveats, and the engine's Clock, File and Input; the box "hidden sheets
  (15)".
- *The press* (`SPEC.md` §3.3 at version 4; `host.js`, `_inputRows` and
  `_pointerFromGrid`). The host writes `Input!B19`, the sheet its viewport
  shows; `B11` and `B12`, the pointer's column and row on it, 0 off its
  cells; and `B13`, the left button held; each only when it changed
  (`ENGINE.19`), as `(cell "Input" "<cell>" <value>)` rows of one write. A
  button is `B19`, `B11`, `B12` and the rising edge of `B13`, kept once in a
  cell that holds 0 between presses (§2's rule 3: no raw pointer in a
  cell's formula, a term that moves going through a cell that holds still).
  The buttons follow the pointer: `B13` is 0 over a tab, a header or a
  scrollbar and 1 over a cell while the button is held, so a press on a tab
  is never the game's, and a drag that enters a cell with the button held
  raises `B13` there, a press by the rule.
- *The keys* (§3.4; `host.js`, `_readKeys`, `keyDown` and `_listenGrid`;
  `viewport.js`, `_keyDown`). The Keys sheet is `A1:E10`, column A a
  button's label in the Input sheet's order and columns B to E the
  `KeyboardEvent.code` that holds it for players 1 to 4, read by the host
  after each step; while a bound key is held, its button's Input cell is 1.
  A bound key is the game's whatever sheet shows, taken before the viewport
  sees it; every other key reaches the viewport, which moves by the arrows,
  Page Up and Down, Home and End, and opens a cell by Enter. No key moves
  between sheets until the chords (`SPEC.md` decision 44). The host hears a
  key only while the focus is in the grid, and a cartridge picked leaves it
  on the picker, so the first click in the grid gives the game its keys.
  The kit's `default-keys` binds the arrows, Z, X, A, S, Enter and Space.
  `B17` and `B18` are the last key of a frame, any key, bound or not, for
  that frame alone, the browser repeating a key held.
- *The Keys device* (`alonzo`'s `cartridge.rs`). A cartridge's own sheet,
  an effect, its cells anywhere in `A1:E10`, no label required, the engine
  checking the Input sheet's alone; `describe` prints `(device "Keys"
  "A1:E10" effect)` after the Input's once it exists.
- *State* (the kit's `count`, `hold` and `pressed`; §2). A formula's twin
  holds no value until the first step's end, so a count reads 0 at the
  first frame, as the input card's counts do. The step follows a cell to its
  readers only when its value's bits change (`FRONT.1`'s first scoping), so
  a cell that holds 0 between presses shields its readers from every move of
  the pointer, and `describe`'s `every-frame` counts no shape for reading
  the Input sheet. A Write row lands a frame late (`MISE.3`'s probe 3), the
  ledger's (`PLATE.1`); nothing here needs one.
- *The dialect* (`vla-lang`'s `calc/library.rs` and `calc/eval.rs`). `IF`,
  `AND`, `OR`, `NOT`, `SUM`, `MIN`, `MAX`, `ABS`, `INT`, `MOD`, `ROW`,
  `COLUMN`, `CHOOSE` and `SIN`. Two texts compare without case, so
  `Input!B19="Gigs"` holds however the record spells the name. A cell
  cannot read a cell it moves in the same frame: a claim judged by the
  claims it changes is a cycle the load refuses.
- *A name.* A sheet's name is written bare in a formula, as the kit's verbs
  need and `every_sheet_is_written_bare_...` holds: one word, no space.
- *The pockets.* None: the personal and the firm's are `PLATE.2`'s, the
  plate and the ledger `PLATE.1`'s.
- *The cap against the first year's trades* (*derived*, by the hand,
  `buffet.rs`, in a scratch runner: seed 791886, whole shares, whole cents,
  an order at the day's price, no costs). A hundred dollars held from day 0
  to the report card's day 365 earns $21.00 in rung 1, $15.60 in rung 2,
  $10.16 in rung 3, the index's own year, and $9.52 in rung 4, its four
  dividends of 50 cents a share among them; perfect foresight, the cash
  whole each day in the rung whose next day pays most or else in cash,
  earns $52.14, invested on 252 of the 365 days. Four claims are $100: twice
  a perfect trader's first year on the pocket's $100, five times the best
  holding's. A claim a day for good is $9,125 a year: 175 times the perfect
  trader's, 434 times the best holding's. And the brief of the design
  reference, the owner's (`BUFFET.md` §8): avoid "the inevitable monotony of
  clicking and waiting for time to pass".
- *The loop* (`host.js`, `tick`). An animation frame's tick steps a frame
  each time its accumulator passes 33.3 ms at `(rate 30)`, so a press waits
  for the next stepping tick, between 0 and 33.3 ms, is stepped, viewed and
  painted inside it, and reaches the screen with the browser's next frame.
  On the Portfolio now (`FRONT.2`'s P8): a still frame's tick 0.2 ms at p50,
  a market day's patch 0.5, the main thread 0.71 to 0.74 ms a frame.
- *CI* at the last push: run 38939138517 green on both jobs at `ec069e8`,
  the crate job's 16 and 30 tests on Linux, the cartridge's in 8.25 s,
  written into `FRONT.1`'s and `FRONT.2`'s P5 cells in this item's commit.

**The decisions.**

1. **The cap: four claims a run (rec), the owner's open decision.** At most
   four claims, $100, so that the hustle doubles the personal pocket's $100
   once and is a bridge to the market and never its loop: four claims
   outearn a perfect trader's first year twice over and are spent in four
   days, where a claim a day for good outearns the perfect trader 175 times
   over and is the clicking the brief rules out. A run is a game from its
   load; a save (`ENGINE.5`, course 7) keeps the count, being a cell. The
   cap is a value of the model, `Hustle!B2`, so a person's edit raises it
   (`AD-6`), which is how the instrument times many claims (decision 13).
   *The alternatives:* once a market day for good, $9,125 a year; or four a
   year, which no line names.

2. **The view: Sideline, second among the views (rec).** A fund manager's
   word, and one word, as a sheet's name must be: a sideline is a second
   line of work, and cash on the sidelines is money out of the market,
   where the fees sit until `PLATE.1` can put them to work. It stands after
   the Portfolio, which stays the first visible sheet and the game's
   opening; a later view takes the place its dish gives it. *The
   alternatives:* Work, the docs' working name since `FRONT.1`'s first
   scoping; Moonlighting; Proofreading. *(Amended at the approval,
   2026-10-16, the owner's name: "I submit "Gigs" as the name of the sheet
   where various clicker jobs can produce cash at varying speeds,
   cooldowns, and pay rates." So the view is Gigs, second among the views,
   and every later mention of it in this scoping reads Gigs; the gigs to
   come, each its own speed, cooldown and pay, are later dishes', and the
   view's layout theirs to change, the button's place being values of the
   model (decision 5).)*

3. **The layout: the button, its key, the claims and the cap, in a
   workbook's plain cells (rec).**

   | | A | B |
   |---|---|---|
   | 1 | the date, the Portfolio's line (decision 10) | |
   | 3 | `Proofreading prospectuses at $25 apiece, paid by the prospectus and not by the page.` | |
   | 5 | the button, `[ Proofread a prospectus ]` | its key, `or press P` |
   | 7 | `Proofread` | the claims made |
   | 8 | `Left this run` | what remains of the cap |
   | 9 | `Fees earned ($)` | the claims' fees, whole cents over 100 once |
   | 11 | where the hustle stands, a sentence | |
   | 13 | the fine print (decision 10) | |

   Column A 26 characters, as wide as the button's text, so that every
   character of it is the button: a text spills only into the empty cells on
   its right, and a click on the spill is a click on the cell beside it.
   Column B 12; the default height; gridlines on; no look, the looks being
   `FRONT.2`'s second slice, so the brackets draw the button. The sentence,
   three drafts, the counts joined from the model: open, "A prospectus
   awaits. Nobody else has read one either."; done for the day, "Done for
   today: the next prospectus arrives tomorrow."; the cap reached,
   "Compliance has approved 4 a run, and you have proofread 4." The jokes:
   the job's line, a prospectus being long; the open line, a prospectus
   being unread; and Compliance, the house's voice (`BUFFET.md` §2). Six
   formula cells, read by nothing. *The alternatives:* the claims as `1 of 4`
   in one cell; the state a label and a word, `Today` and `Open`; the key
   inside the button, `[ Proofread a prospectus (P) ]`, a column of 30.

4. **The hustle's state: a model sheet of its own, Hustle, after the Stocks
   (rec).** Settings, then workings, each a label in A beside its value in B
   as the model's sheets hold them, visible (`AD-6`); its section after the
   Stocks' and before the content sheets', the views reading it after both:

   ```text
   ; ---- BACK.1: the hustle --------------------------------------------------------------
   (on "Hustle"
     (labels "A1" "a prospectus's fee, in cents" "claims a run" "the button's sheet" "its column" "its row")
     (cells
       ("B1" 2500)                     ; $25 a prospectus, whole cents
       ("B2" 4)                        ; claims a run: four, and then Compliance says no
       ("B3" "Gigs")                   ; the button: its view,
       ("B4" 1)                        ;   its column, A,
       ("B5" 5))                       ;   and its row, values a layout moves, and no formula
     (labels "A7" "clicked" "keyed" "claimed" "claims" "open from day" "shut" "earned, in cents")
     (formulas
       ("B7" "=IF(AND(Input!$B$19=$B$3,Input!$B$11=$B$4,Input!$B$12=$B$5,Input!$B$13=1,Input.last!$B$13=0),1,0)"))   ; a click: the left button's rising edge on the button, its sheet shown
     (pressed "B8" "Input!B5")         ; a key: the a button's rising edge, held by P (decision 9)
     (formulas
       ("B9" "=IF(AND(OR(B7=1,B8=1),Hustle.last!B12=0),1,0)"))   ; a claim: a press while the last frame showed it open
     (count "B10" "B9")                ; the claims so far
     (hold "B11" "B9=1" "Day.last!$B$3+1")   ; open from day: after a claim, the day after the day shown
     (formulas
       ("B12" "=IF(OR(B10>=$B$2,B11>Day!$B$3),1,0)")   ; shut: the run's claims spent, or today's made
       ("B13" "=B10*$B$1")))           ; earned, in whole cents
   ```

   Seven formula cells, each moving at a press alone. *The alternative:* the
   state on the view, which nothing could then read, `PLATE.2`'s pocket
   among the readers.

5. **The press: two cells that hold 0 between presses, the button's place
   values of the model (rec).** `B7` is §3.3's button, the sheet, the column
   and the row compared with the settings and never written into the
   formula, so that laying Gigs out again moves three values and no
   formula of the model, a test holding the view's cell at that place to the
   button's text; `B8` is the kit's `pressed` over the key's button. Each is
   1 in a press's frame alone, and everything else reads them and never the
   pointer. A drag onto the button with the button held is a press, as the
   host writes it. *The alternatives:* the place written into the formula,
   `Input!$B$11=1,Input!$B$12=5`, so that every layout moves a formula of
   the model; one cell for the click and the key.

6. **The day a press counts against: the day shown (rec).** A claim is a
   press while `shut`'s twin is 0, the last frame's, which is the state the
   person saw, since the claim cannot read this frame's `shut`, which it
   moves. So it counts against the last frame's day, `Day.last!$B$3`, as an
   order fills at the price shown, the last frame's quote (the contract). On
   a boundary frame, the day before: a press in the frame the day turns,
   while yesterday showed open, is yesterday's claim, and today opens after
   it; one while yesterday showed done is refused, as it showed. One cell,
   `shut`, is what the view shows and what the next press is judged by.
   *The alternative:* the frame's own day, `Day!$B$3`: a press on a boundary
   frame claiming a day the person had not yet seen, and the rule written
   twice, the claim's over the twins and this frame's day, the view's over
   this frame's cells.

7. **The once-a-day reset: the day moving, and nothing written (rec).**
   `Open from day` holds, after a claim, the day after the day it counted
   against, from 0, so that a game opens open; `shut` compares it with
   today. At a boundary the day reaches it and Gigs opens: no cell
   reads the boundary, the frame or the phase (`BD-7`), and nothing resets.
   A press is an edge, so a button or a key held across a boundary claims
   once. *The alternative:* a flag that a claim sets and the boundary,
   `Day!B5`, clears, its press on a boundary frame then needing an order of
   the two.

8. **The dollars: the claims alone, the fees a product (rec).** `Earned` is
   the claims times the fee, in whole cents, which the view shows over 100
   once. No cash cell enters: `PLATE.2` makes the personal pocket, its $100
   and its flows, and reads the hustle's claim or its fees into it, the
   ledger's rows being `PLATE.1`'s, and nothing here is taken over. *The
   alternative:* the personal cash cell now, `=10000+Hustle!B13`, the
   contract's $100 shown from today, the pocket's place and spelling settled
   a dish before `PLATE.2`. *(Taken at `GUARD.1`'s hand test, 2026-10-17, at
   the owner's word and choice: the Cash sheet's personal cash, `Cash!B4`,
   the start and the fees once paid, beside the firm's, on the Portfolio as
   its cash on hand; and the fee $250 where $25, the owner's word the same
   day.)*

9. **The key: P, on the `a` button, the one row this dish needs (rec).**
   `(sheet "Keys" visible)`, then `Keys!A5` `a` and `Keys!B5` `KeyP`,
   written last, so that the sheet stands after the Caveats with the
   engine's devices: P for the prospectus, a letter the viewport never
   takes; never an arrow, which the viewport moves by, nor Enter, which opens
   a cell, nor Alt, the chords'. It presses whatever sheet shows, the host's
   rule for a bound key and the only rule that leaves the act a key's while
   no key moves between sheets: P on the Portfolio proofreads, the claim
   landing on Gigs. Beside the button, `or press P`, a text a test
   holds to the code bound. The kit's whole ten-row sheet would take the
   arrows, Enter and Space from the viewport for buttons nothing reads.
   *The alternatives:* the ten labels with the one code; H or W; the key
   read from `Input!B18`, which the viewport also sees and the browser
   repeats. *(Amended at the hand test, 2026-10-16, the owner's words:
   "Pressing P anywhere in the workbook triggers the prospectus gig, but
   clearly keybindings should be scoped to each tab to avoid limiting the
   entire application to 26 key commands." So a key is bound on its own view
   alone: `Keys!B5` is a formula of the sheet shown,
   `=IF(Input!$B$19=Hustle!$B$3,"KeyP","")`, which the host reads after every
   step (`SPEC.md` §3.4: a cartridge may rebind keys by formula), so that P
   is Gigs' and on any other sheet unbound and the viewport's, and each later
   view binds keys of its own.)*

10. **The view's frame into the library at its second writing: `as-of` and
    `fine-print` (rec).** Gigs writes the Portfolio's date and fine
    print a second time, so by the ratchet they enter `kitchen.vla`, each a
    verb of one formula row taking a sheet and a cell, and the Portfolio is
    rewritten over them, expanding to its bytes unchanged, the hoist's
    proof. *The alternative:* written by hand again and hoisted at the third
    view.

11. **The notes and the comments (rec).** The manifest's notes gain a
    sentence after the Portfolio's: "Beside it, Gigs, where a
    prospectus is proofread for $25, by a click on its button or by P, once
    a market day and four times a run, Compliance permitting." The source's
    opening comment lists `BACK.1`.

12. **The oracle (rec).** In `oracle/src/buffet.rs` and
    `oracle/tests/cartridge.rs`:
    - *The hand's hustle*: the fee and the claims a run as the contract's
      constants; the claims as the days each counted against, in order; a
      press read from a frame's inputs and the last frame's as the host
      writes them, a click the left button rising on `Gigs!A5`
      while it shows, a key the `a` button rising; a claim when the run has
      claims left and the day shown has none. It shares no predicate with
      the cells, which judge `shut`'s twin where the hand reads its list's
      length and its last day. Its own test: four claims on four days, a
      second the same day refused, a fifth refused.
    - *A scripted session*, every frame of it held: the Input rows written as
      the host writes them, its changed rows alone, the key through the Keys
      sheet as the host finds it, the code's row read from the cartridge; at
      every frame `Hustle!B7:B13` and `Gigs!B7:B9` and `A11` held
      to the hand. Its presses, over about ten days: a click claiming; a
      second click and a key the same day refused; a click on the boundary
      frame after a claimed day refused, and the next frame's claiming; a
      click on `B5`, beside the button; a click at `A5` while the Portfolio
      shows; P from the Portfolio claiming; the button held across a
      boundary claiming once, and the key; a click and a key in one frame
      claiming once; a click on the boundary frame after an open day claiming
      that day, and the new day open after it; a drag onto the button
      claiming; the fourth claim, and every press after it refused.
    - *At rest*: Gigs held whole on each day's first frame inside
      `held`, so on every day of two years and two days and of eight hundred
      at one frame a day: no claim, four left, nothing earned, a prospectus
      awaiting, its date and its fine print the Portfolio's. Between days it
      cannot move without an input, which `BD-7`'s scan and the census hold,
      and the session holds every frame of the active path: about a tenth of
      a second, where holding it at every frame of the two years would cost
      about half a second (`FRONT.1`'s P5).
    - *The settings and the binding*: the Hustle's settings the hand's; the
      Gigs sheet's cell at the button's place its button text; the Keys sheet
      binding one code, `KeyP`, on the row `B8` reads, no arrow, Enter, Space
      or Alt bound, and the text beside the button naming it.
    - *Kept*: the cartridge its source expanded, the Portfolio's bytes
      unmoved by the hoist; `BD-7`'s scan and the census's `(every-frame 0)`;
      nothing reads a view, `VIEWS` naming Gigs, each view at most
      1,000 formula cells; the tab order's list; the game opening on the
      Portfolio; the describe golden blessed again and read in its diff with
      the shapes' numbers stripped.
    - *Mutants*, each written into the source by one anchor and expanded
      before the suite runs, each failing the test written for it: a press
      counted twice, the click's edge lost, so that a held button claims
      again the next day; a press off the button counted, the column's test
      lost; the cap lost; the reset lost, `B11>0` for `B11>Day!$B$3`; the key
      unbound, `Keys!B5` gone; a press while another sheet shows counted, the
      sheet's test lost; the frame's own day, `Day!$B$3` for `Day.last!$B$3`;
      and a refused press counted, the claims counting presses.

13. **What a click costs, measured (rec).** Natively, as `FRONT.2` took it:
    a scratch runner pinning itself to the performance cores and opting
    itself out of Windows' throttling, alternated against a `git archive` of
    `ec069e8` built in a target of its own: the pass, the load, the first
    frame, an ordinary frame and a day's first in both trees; and in this
    one, the cap raised by a write, a frame whose pointer moved, a press's
    frame that claims, a refused press's frame and the frame after a claim,
    each its write and its step timed apart. Each test of the suite timed
    alone in both trees before a difference in the suite's whole time is
    read. In the browser, `FRONT.2`'s instruments copied into this session's
    scratchpad: the page built here, its probe copy raising the cap by a
    person's edit at its start; three runs in headless Chrome at the owner's
    full screen, 2,560 by 1,440 CSS pixels at 1.5, every process opted out
    and pinned; in each, forty real clicks on the Gigs sheet's button through
    the DevTools protocol, one a day at a random point of the frame, then
    forty presses of P; each timed from the event's own timestamp to the end
    of the tick that painted its claim, and to the start of the next
    animation frame, the frame the browser shows it in; p50, p95 and the
    longest, against the contract's 100 ms at p95, the number `MISE.5` will
    time; and the still and patched ticks with Gigs shown. In-page
    times are Chrome's, in steps of 0.1 ms. And the owner's own Chrome, the
    probe copy printing the same in a corner.

14. **The hand test, the owner's, on the page built here:** the strip and
    its box; Gigs as laid out, the button whole in its column; a
    click claiming within a frame, and a second click before the date turns
    claiming nothing; P after the date turns, from Gigs, then from
    the Portfolio; a click at the Portfolio's `A5` claiming nothing; the
    fourth claim, the cap's sentence, and nothing after it; the drawer's
    viewer on the Hustle sheet; Build's text compared with `src/buffet.vla`
    by a command given at the handover, since the page prints no "same
    bytes"; the probe copy's corner after a minute of a click or a P each
    second.

15. **No caveat owed (rec).** The hustle claims nothing of the world but a
    fee, and the fee is fictional. Offered for the owner's taste, a
    thirteenth row in the house's voice, its claim marked for `MENU.8`'s
    read:

    | The field | The draft |
    |---|---|
    | Things this game gets wrong | A side job needs nobody's permission. |
    | In the real world | A registered representative gives the firm written notice before taking on an outside business activity (FINRA Rule 3270). |
    | Management's response | Management acknowledges receipt of this caveat, in writing. |
    | Where | `Gigs!A5` |

16. **The stale places, rewritten at the build.** In `OMAKASE.md`: `BACK.1`'s
    line, 🟡 from the approval, its text as decisions 1 to 9 leave it, the
    claims alone where it had the personal pocket's cash cell; the
    contract's hustle, the cap settled, dated; the open decisions' row,
    answered, dated; `PLATE.1`'s line, the click's pattern hoisted at its
    second writing and the keys its acts will want; `GUARD.1`'s line, its
    options' keys; `MISE.6`'s views row, the click met. The source's opening
    comment, notes, sheets row and two sections; `kitchen.vla`'s two verbs;
    `oracle/src/buffet.rs`'s module comment and the hand's hustle; the module
    comments of `oracle/src/lib.rs` and `oracle/tests/cartridge.rs`; the
    describe golden; `FRONT.1`'s and `FRONT.2`'s P5 cells, CI's Linux;
    `Delta/CLAUDE.md`'s map, the Keys sheet and a button's place among the
    edits that trip a test; the memory. `BACK.1` leaves `OMAKASE.md` for this
    file with its commit, in a room of its own, 🧽 THE BACK OF HOUSE. Not
    this item: the pockets (`PLATE.2`), the plate and the ledger
    (`PLATE.1`), the hustle's later rungs (the opening bell, the intern's
    autoclick, `BACK.5`), the looks (`FRONT.2`'s second slice), the chords,
    and the replay golden (the tasting).

**Found at the scoping**, measured or read, none minted:
- *A game's keys wait for the grid's focus* (Alonzo's page). A cartridge
  picked leaves the focus on the picker, so P does nothing until a click in
  the grid, and a letter typed meanwhile reaches the picker, which may pick
  and load another cartridge, an "A" naming "An empty sheet" (read; to be
  confirmed in headless Chrome at the build). A finding for Alonzo, the
  owner's to pass on.
- *Ten buttons for course 0's acts* (`SPEC.md` §3.4). `PLATE.1`'s six
  buttons a rung, twenty-four, and `GUARD.1`'s four options outnumber the
  Keys sheet's ten, four of which are the arrows the viewport moves by.
  Keyboard-first then needs the key rows, `B17` and `B18`, which the
  viewport also sees and the browser repeats, or the chords (decision 44):
  `PLATE.1`'s scoping's question, and a candidate row for `MISE.6`. *(The
  owner, at the approval: "The Alonzo session is scoping a Control + Shift
  + Backtick command palette CLI instead of Alt + keychord to avoid
  colliding with system defaults, so keyboard shortcuts may need to be
  deferred until that session implements a solution." So the keys for
  `PLATE.1`'s acts wait for that palette, and no item is asked here.)*
- *A key by its place* (§3.3). `KeyP` is the key where QWERTY's P stands:
  P on QWERTY, AZERTY and QWERTZ, and L on Dvorak; the text beside the
  button says P.

**The predictions, before any number.** The numbers column is filled at the
build, P9 at the owner's hand test; P3 to P5 and P8 as decision 13 says.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 474 rows where 431 stand, 43 more: the sheets row's two names; the Hustle's 24, twelve labels, five values and seven formulas; Gigs' 14, two widths, three texts, three labels and six formulas; the Keys' three; 124 formula cells where 111; the diff the notes' line and the sheets row's two lines in place, the Hustle's rows after the Stocks', Gigs' and the Keys' at the end; the Portfolio's rows unmoved | *Measurement*, as predicted: the runner printed `(rows 474) (loads yes)`; 124 formula cells in 61 formula rows; the diff one line out and 44 in, the notes' line, Gigs and Hustle in place in the sheets' rows, the Hustle's 24 rows after the Stocks', Gigs' 14 and the Keys' three at the end; the Portfolio's rows unmoved, the two verbs writing them byte for byte |
| P2 | the describe golden | the notes; `(device "Keys" "A1:E10" effect)` after the Input's; `(budget 30 124 3720 2000000)`; the plan's shapes 61, texts 57, formulas 124, alone 81, levels 16, runs 10, lanes 43, every-frame and cycles 0; readers rows new for the Hustle, 19 to 21 with its own reads, past sixteen and so indexed, `(most 5)` or `(most 6)` its row 10, the claims; its twin's 3, the Input's 5, the Input's twin's 2 and the Day's twin's 1; the Day's `(readers "Day" 19 (most 7))`, its row 3 read by `shut` too, and the Disclosures' 13; no row for Gigs; its six shapes after the Portfolio's, its date `(same 0)` and its fine print `(same 5)`; the Hustle's seven after the Stocks', single cells; every other shape's row unmoved but for its number and its `same` | *Measurement*: the notes; `(device "Keys" "A1:E10" effect)` after the Input's; `(budget 30 124 3720 2000000)`; `(plan (shapes 59) (texts 57) (formulas 124) (levels 16) (runs 10) (lanes 43) (alone 81) (every-frame 0) (cycles 0))`; `(readers "Hustle" 19 (most 5))`, `(readers "Input" 5)`, `(readers "Day.last" 1)`, `(readers "Hustle.last" 3)` and `(readers "Input.last" 2)`; `(readers "Day" 17 (most 7))`; no row for Gigs; Gigs' four other shapes after the Portfolio's and the Hustle's seven after the Stocks', single cells; every other shape's row its own but for its number. **Missed** in three, of one cause: Gigs' date and fine print are no shapes of their own but the Portfolio's two, each now a shape of two cells over two sheets, `(cells 2) ... (sheets 2)`, where `(same 0)` and `(same 5)` were predicted; so 59 shapes where 61, the Day's readers 17 where 19, and the Disclosures' 12, unmoved, where 13 |
| P3 | the pass and the load | the pass 0.2 to 0.5 ms longer, forty-three rows and two verbs; the load 0.1 to 0.25 ms longer | *Measurement*, this tree's runner over both cartridges, the commit before's texts copied into its tree for the runs marked before, three runs each alternated, 200 passes and 200 loads a run, every run pinning itself to the performance cores and opting itself out of Windows' throttling, a fixed computation before and after within 3.7%: the pass 4.94 to 4.96 ms at p50 where it took 4.53 to 4.60, 0.34 to 0.43 more, inside; the load 1.81 to 1.85 where 1.55 to 1.60, the medians 0.25 apart, inside at its edge. Each tree's own runner, alternated first, agreed within 0.06 ms; its fixed computation read 120 ms in the commit before's binary and 140 in this one's at every run, two layouts of one loop and not the machine, which is why one runner timed both |
| P4 | the step | an ordinary frame 0.1 to 0.3 µs dearer, thirteen shapes' flags; a day's first frame 0.5 to 2 µs more, Gigs' date and `shut`; the first frame 5 to 15 µs more. In this tree: a pointer's move, its two rows' write 2 to 6 µs and its step 2.1 to 2.8; a claim's write 1 to 3 µs and its step 3.5 to 7, about ten cells; a refused press's step 2.3 to 3.5; the frame after a claim 2.5 to 4.5 | *Measurement*, the same runs, two years and two days at thirty frames a day: an ordinary frame 2.1 µs at every run where it took 2.0, 0.1 more, inside at its edge; a day's first frame on which the fine print stays 17.2 to 17.7 µs where 15.3 to 15.9, the medians 1.7 apart, inside; one on which it turns 18.2 to 19.6 where 16.3 to 17.4, the medians 2.0 apart, inside at its edge; the first frame 97.7 to 99.7 µs where 83.4 to 89.0, the medians 12.1 apart, inside. In this tree, the run's claims raised by a write and 360 days played, each day a move onto the button, a claim, the frame after it, a release, a refused press, a release and a move away: a move's two rows' write 3.6 to 3.7 µs, inside; a claim's write 2.1 µs and its step 5.2 to 5.3, inside; the frame after a claim 4.1 to 4.2, inside. **Missed** in two: a move's step 3.5 µs where 2.1 to 2.8, and a refused press's step 3.6 to 3.7 where 2.3 to 3.5, a frame after a write of the Input rows costing about 1.4 µs over an ordinary one where the cells it computes were priced alone; not looked into |
| P5 | the suite | 49 tests green, the crate's 17 and the cartridge's 32, and in CI on Linux; within 0.4 s of `ec069e8`'s at the median | *Measurement*, here: 49 green, the crate's 17 and the cartridge's 32. Each tree's suite run alternately, five rounds, the order turned each round, each run pinned and opted out, on eight threads: 5.07 to 5.46 s where it took 4.82 to 5.11, the medians 5.13 and 4.87, 0.26 apart: inside. Each test timed alone first, two rounds alternated, one thread: the two years' test 5.03 s where 4.78, 0.25 dearer, the hundred years' 2.68 where 2.48, the eight hundred days' 0.27 where 0.23, the year at difficulty 0 1.37 where 1.34; the session 0.02 s and the binding's test 0.01, new; every other within a hundredth. The long tests' cost is the oracle's own: every record lists every sheet, 30 sheet rows where 24 stood, Gigs, Hustle, the Keys and their twins, and the oracle's `values` reads each record whole, so every held frame pays for each sheet a dish adds. Linux at the push, run 39088078269 at `7bbeb59`, three commits on, `POT.10`, `GUARD.1` and the pin to `alonzo` 0.2.0, CI running the pushed head alone: the 67 green there, the crate's 27 and the cartridge's 40, the cartridge's in 6.07 s |
| P6 | the page | the cartridge's block 2,800 to 3,800 characters longer, the source's 5,000 to 8,000 and the library's 800 to 1,500: the page between 1,831,000 and 1,836,000 where 1,822,477 stands; Build writing the cartridge byte for byte | *Measurement*: 1,831,874 characters, 9,397 more than `ec069e8`'s 1,822,477: the cartridge's block 2,248 longer, the source's 5,882 and the library's 1,267, the three adding to the page's growth exactly; the page, the source's block and the library's inside their ranges; Build writing the cartridge byte for byte, 28,056 characters, checked headlessly (P9). **Missed** in the cartridge's block, under its range: its rows shorter than priced |
| P7 | the checks | 4 checks and 4 controls green, no check edited, no file added under `src/` | *Measurement*, as predicted: ALL GREEN, 4 checks and 4 controls, the layout control's fourteen mutants and the offline control's fifteen as they stood; no check edited; no file added under `src/` |
| P8 | a click's cost in Chrome at full screen | click to painted 13 to 20 ms at p50 and 30 to 34 at p95, at its longest under 38; to the next frame about 16.7 more, 46 to 51 at p95: the contract's 100 ms met by half; P within a millisecond of a click; a still tick with Gigs shown 0.2 ms at p50 and 0.3 at p95, a claim's patch 0.4 to 0.7; the main thread 0.70 to 0.80 ms a frame; the owner's Chrome within a tenth | *Measurement*, three runs in headless Chrome at 2,560 by 1,440 CSS pixels at 1.5, its eleven processes opted out and pinned, the clock within 0.3%; in each, 40 real clicks on Gigs' button and then 40 presses of P through the DevTools protocol, one a market day at a random point of the day, the run's claims raised by a person's edit and every press claiming: a click to painted 16.0 to 18.1 ms at p50 and 32.0 to 33.2 at p95, at its longest 33.1 to 33.9; to the next frame, the claim shown, 32.0 to 34.4 at p50 and 48.2 to 49.3 at p95, at its longest 49.3 to 50.2: the contract's 100 ms at p95 met by half. P to painted 17.7 to 19.8 at p50 and 32.0 to 33.8 at p95; shown 48.2 to 49.9 at p95. A claim's patch 0.4 to 0.5 ms at p50 and 0.6 to 0.7 at p95. With Gigs shown and nothing pressed, three runs of 600 frames: a still tick 0.2 ms at p50 and 0.3 at p95, at its longest 0.4 to 0.5; a patch 0.4 to 0.5, at its longest 0.5 to 0.6; 580 or 581 still and 20 patched; the main thread 0.77 to 0.80 ms a frame, inside (0.96 to 0.98 while the instrument polled the day every 5 ms). **Missed** in one: P within a millisecond of a click, where its p50s stood 0.3 to 3.8 ms apart, forty presses a run, each waiting up to a frame's 33 ms, telling no finer difference; and one P of 120 at 45.0 ms. The owner's own Chrome, the probe copy at full screen for 2,297 frames, 31 clicks: painted 11.4 ms at p50 and 30.3 at p95, at its longest 33.0; shown 27.7 and 46.8, at its longest 49.2; one press claiming nothing; no P pressed; a still tick 0.2 ms at p50 and 0.3 at p95, at its longest 0.6; a patch 0.5 and 0.6, at its longest 1.3; 2,188 frames still and 106 patched: headless's p95s within 2 ms, the p50 lower over its 31 |
| P9 | the hand test | the strip, Portfolio, Gigs, Day, Dice, Market, Stocks, Hustle, Caveats, Keys, Clock, File and Input, and the box "hidden sheets (18)"; a click claiming within a frame and a second claiming nothing; P from either view; the cap's sentence after the fourth; Build's bytes the file's, checked headlessly | *Headlessly*, over the page built here, before the owner's test: the strip and its box as predicted; a click claiming within a frame and a second claiming nothing; P the next day claiming from Gigs; on the Portfolio a click on its `A5` claiming nothing and P claiming; the fourth claim and the cap's sentence, a click and P after it claiming nothing; Build's bytes the file's, 28,056 characters. *Measurement*, the owner's three screenshots, in the owner's Chrome: Gigs on the first year's day 16, a prospectus proofread, three left and $25 earned, a prospectus awaiting again the next day; day 66, the fourth claim, none left, $100 and Compliance's sentence, the drawer's viewer on the Hustle sheet, its settings and its claims 4; and the probe copy at full screen (P8). **Missed** in one, by the owner's word: P claimed from the Portfolio as from Gigs, as decision 9 had it, where "keybindings should be scoped to each tab"; amended the same day, the key bound on its view alone, and checked headlessly over the page built again: P on the Portfolio claiming nothing, P on Gigs claiming, Build's bytes the file's, 28,097 characters; then the owner's re-test, a screenshot in the owner's Chrome, Gigs on day 30 with one prospectus proofread and the drawer's viewer on the Keys sheet, `B5` reading `KeyP` while Gigs shows, and the owner's word, "Tests pass; commit." |

*Scoped 2026-10-16. Approved by the owner the same day with the view named
Gigs (decision 2's amendment): "All other recommendations are approved;
proceed with implementing BACK.1 as scoped." So the cap is four claims a
run (decision 1), the key P stands (decision 9), and no caveat is added
(decision 15). The predictions were amended to the name before any number
was taken; the name moves characters and no row.*

**What the build found.** The ratchet's proof held: the Portfolio written
over the two new verbs, `as-of` and `fine-print`, expands to its bytes
unchanged. The census makes one shape of a formula written alike on two
sheets, its cells counted over both, `(sheets 2)`: the views' date and fine
print are one shape each, whichever views write them, so a view's frame
costs the plan no shape (P2's one miss). A formula over an empty cell's twin
holds 0 in the record, so the claims and the day the hustle opens read 0
from the first frame and a game opens open. The scripted session passed at
its first run, and each of the eight mutants, written into the source by one
anchor and expanded into the cartridge before the suite ran, failed it: the
click's edge lost, a held button claiming again the next day; the column's
test lost, a click beside the button claiming; the cap lost; the reset lost;
the key unbound, which failed the binding's test too; the sheet's test lost,
a click at the button's address on the Portfolio claiming; the frame's own
day, the boundary's claim counting against a day the person had not seen;
and the claims counting presses. Every record the engine views lists every
sheet, so each sheet a dish adds makes every frame the oracle holds dearer,
its `values` reading a record whole: three sheets and their twins cost the
two years' test 0.25 s and the hundred years' 0.21 (P5), a cost of the
oracle's and none of the game's. On the page, a real click's claim is
painted within the frame that answers it and shown at the browser's next,
48 to 50 ms at p95 at the owner's full screen, the contract's 100 ms met by
half, and the button's 26 characters draw in 228 of the column's 273 device
pixels. And the scoping's finding stands, confirmed headlessly: a pick
leaves the focus on the picker, where an "a" typed picks "An empty sheet"
and loads it, the game gone, a finding for Alonzo's page, the owner's to
pass on.

**The owner's hand test, and what it changed.** The hand test passed but for
the key (P9): P claimed from the Portfolio as from Gigs, and the owner's word
was that "keybindings should be scoped to each tab to avoid limiting the
entire application to 26 key commands". Amended the same day, decision 9's
dated note: the Keys sheet's `B5` a formula of the sheet shown, P bound while
Gigs shows and on no other sheet. What moved: 474 rows still and 125 formula
cells; the describe golden's `(budget 30 125 3750 2000000)`, `(plan (shapes
60) (texts 58) (formulas 125) ... (alone 82) ...)`, `(readers "Hustle" 20
(most 5))` and `(readers "Input" 6)`; the hand's key a press only while
bound, by the sheet the last frame showed, which the host's keymap read after
the last step; the session's script, P on the Portfolio unbound and claiming
nothing, P on Gigs claiming, and P held on the Portfolio while Gigs is shown
under it, a press the frame the key is bound, refused there, the day claimed,
the session holding at every frame the keys the cartridge binds to the hand's;
a ninth mutant, the key bound on every sheet, caught with the eight by the
session and the binding's test; the page 1,832,337 characters, its
cartridge's block 28,097, Build byte for byte. Found with it: a key held
while its view is shown under it presses the frame it is bound, the host
holding keys and its keymap changing beneath them; and the drawer's line of
help names the kit's default keys, "the arrows, Z, X, A, S, Enter and Space
are player 1's", whatever a cartridge's Keys sheet binds, a finding for
Alonzo's page, the owner's to pass on.

The owner's word with it, for the next item and before any other dish: "Also,
it has become clear that one second per market day is too fast to
conceptualize. The game should run at one real-world second per hour of
in-game market play. I also think the market should have trading hours and
non-trading hours, so the player has time between trading days to focus on
other game tasks. I understand this is outside the scope of BACK.1, but after
playing the game as-is a few times these are necessary changes before
anything else gets built." What it moves, read on the day and decided at its
scoping: `POT.1`'s calendar and the contract's thirty frames a market day;
`BD-7`'s day as the market's clock; the hustle's once a market day; the
views' date; the fine print's month; and the oracle's runs, a year of 365
days at thirty frames an hour being 262,800 frames.

## `POT.10` — the market's hours: an hour a second, the bells, and a session's prices each half hour

*Scoped 2026-10-16 at the owner's word, "Scope and implement POT.10 in
Buffet (Delta/Buffet): the market's hours", to stop for the owner's approval
before building; minted the same day in `OMAKASE.md`, the stock pot's next
free, and placed before every other dish of `COURSE.0`, since the owner,
having played the game at `BACK.1`'s hand test, ranked it so: "one second per
market day is too fast to conceptualize. The game should run at one
real-world second per hour of in-game market play. I also think the market
should have trading hours and non-trading hours, so the player has time
between trading days to focus on other game tasks. ... these are necessary
changes before anything else gets built." Read for it: that record, above;
`POT.1`'s scoping and record (the Day sheet, the dice, the hand's calendar,
the tests that hold every frame); `BD-3` and `BD-7`; `COURSE.0`'s opening
contract; `POT.2`'s, `STATION.1`'s, `FRONT.1`'s, `FRONT.2`'s, `BACK.1`'s and
`MENU.1`'s records; and Alonzo's `SPEC.md` at the tag the pin names, §2,
§3.5, §3.9 and §5. Each decision carries its recommendation, for the owner to
approve, amend or strike; the texts a player reads are drafts, for the owner
to approve or rewrite. The predictions were written before any number was
taken; the figures marked *derived* were computed by the hand, `buffet.rs`,
in a scratch runner at seed 791886, and none of them is a measurement of the
cartridge. Alonzo's and Frazaro's trees were read and never touched.*

**What it stands on, read on the day** (`alonzo` 0.1.5 and `vla-lang` 0.9.4
as the registry holds them; `SPEC.md` at `v0.1.5`; Buffet at `518d414`, one
ahead of its remote, its tree clean).
- *The cartridge.* `(spec 4)`, `(rate 30)`, 474 rows, 125 formula cells in 62
  formula rows; the cartridge its source expanded, the runner's check saying
  `same`; 49 tests green, the crate's 17 and the cartridge's 32 in 4.99 s; CI's
  last run 38939138517 green at `ec069e8`, `BACK.1`'s push and its P5 Linux
  cell still to come.
- *The calendar* (`POT.1`). The Day sheet, the one sheet that reads the frame:
  `B1` the frames a day, 30, held to the manifest's rate by a test; `B2` the
  days a year, 365, every day a market day; `B3` the day from 0,
  `=INT((Clock!B1-1)/B1)`; `B4` the phase, the frame within the day, the one
  cell of it that changes every frame; `B5` the boundary, 1 in a day's first
  frame; `B6` the year and `B7` the day of the year. Its `B3`, `B4` and `B5`
  compute at every frame, and no other cell of the game does.
- *Every reader of the day*, the cartridge's text grepped (`Day!$B$3` five
  times, `$B$2`, `$B$5`, `$B$6` twice, `$B$7` three times, `Day.last!$B$3`
  once): the market's die, rolling on `B5` and taking the seed while `B3` is
  0; tier 1's four phases, held from day 0; the index, `Market!B14`
  `=Day!$B$3`, its look back `C14` twenty days before and yesterday `D14` the
  day before, each column the same fill of the waves at its day; the stocks'
  price today and yesterday over `B25` and `D25`; the days between two
  dividends off `B2`; the dividend today off `B7`; the fine print, a
  disclosure every 30 days off `B3`; the views' date, the library's `as-of`,
  off `B6` and `B7`; and the hustle, a claim counted against the day shown,
  `Day.last!$B$3`, the day after it held and compared with `B3`.
- *`BD-7`'s oracle.* A scan of the formula rows, that no sheet but the Day
  sheet reads `Clock!B1` or `Day!B4`; and the census's `(every-frame 0)`,
  which counts the shapes no change is followed to (`vla-lang` 0.9.4's
  `calc/census.rs`: a defined name, a span, a broken or an unread reference),
  and not the shapes that read the frame, the Day sheet's three being
  followed. So the calendar may grow cells that read the frame and its phase
  without moving the census, as long as no other sheet reads them.
- *The clock* (`SPEC.md` §3.5, §5, §6). The frame is the only clock; the rate
  is a person's dial, 0 or a whole number from 1 to 120, which the host reads
  after each step and steps at; no frame is ever skipped; wall time decides
  when a step happens and never what it computes. A derived write may not
  target the Clock (§3.9: "a host-written device cell ... is refused by
  name, `cart-write-derived`"), so a cartridge cannot set its own pace: hours
  outside trading can be made shorter only by the calendar, which maps frames
  to hours, and never by the host.
- *The dice* (`POT.1`'s decisions 5 to 8). The market's die jumps 1,017 draws
  at a day's first frame; four draws a day, `POT.2`'s phases reading them on
  day 0 alone; a day holds 1,016 draws, each dish taking the next free one;
  the exam's die and its two draws at `Dice!A7:C9`, directly under the
  market's fourth draw.
- *The dialect* (`vla-lang` 0.9.4's `calc/library.rs` and `formula.rs`). `IF`,
  `AND`, `OR`, `NOT`, `SUM`, `MIN`, `MAX`, `ABS`, `INT`, `MOD`, `ROW`,
  `COLUMN`, `CHOOSE` and `SIN`; `&` joins texts and `<>` compares; `MOD` takes
  the divisor's sign as Excel's does, `n - d*floor(n/d)`, so `MOD(-3,7)` is 4;
  no `TEXT` and no `RIGHT`, so a time is joined from its numbers.
- *The views.* The Portfolio (`A1` the date, `A4:D8` the stocks, `A10` the
  fine print) and Gigs (`A1` the date, `A3` the job, `A5` the button,
  `A7:B11` its books, `A13` the fine print), row 2 empty on both; both open on
  the library's `as-of`, "As of year 1, day 1 (unaudited)".
- *The oracle* (`oracle/tests/cartridge.rs`; `BACK.1`'s P5, each test timed
  alone). Every frame of two years and two days at thirty frames a day,
  21,960 frames, held whole (the calendar, the dice, the market, the stocks,
  the fine print, the Portfolio, Gigs at rest): 5.03 s, the suite's longest;
  a hundred years at one frame a day, `Day!B1` written 1, 36,500 frames,
  2.68 s; eight hundred days at one frame a day, 0.27 s; a year at difficulty
  0, 10,950 frames, 1.37 s; the hustle's scripted session, six days, 180
  frames. The cost is the oracle's own, every record listing every sheet and
  `values` reading each whole: 0.23 ms a frame held in the two years' test.
  At thirty frames an hour those two years are 525,600 frames, about two
  minutes at that cost (*derived*), and a hundred years at a frame an hour
  876,000.
- *The first year at seed 791886* (*derived*). Its low 0.98943 on day 91, a
  Monday if day 0 is one, and day 365 1.10155: `POT.2`'s dip of 1.1% and its
  close 10.2% up, to the digit. A week begun on a Monday at day 0 gives the
  first year 261 weekdays. The dividend's four days of the year, the 91st,
  182nd, 273rd and 364th, are days 90, 181, 272 and 363, all Sundays in the
  first year, all Mondays in the second and a weekday later each year after,
  365 days being 52 weeks and a day. Over the first year's 261 weekdays, a $1
  share's close differs from the last close on 25, the $100 share's on 230.
- *The loop in Chrome* (`BACK.1`'s P8, at the owner's full screen, Gigs
  shown): a still tick 0.2 ms at p50, a patch 0.4 to 0.5, the main thread
  0.77 to 0.80 ms a frame.

**The decisions.**

1. **The pace: an hour a second, every hour alike (rec).** Two minutes of the
   market's time a frame, so thirty frames an hour at `(rate 30)`: an hour a
   second, a day 24 seconds, a week 2 minutes 48, a year of 365 days 2 hours
   26 minutes (*derived*). The night and the weekend pass at the same pace as
   the session, since the owner asked for time between trading days, and a
   night made shorter is that time taken away. The teacher's dial, the
   Clock's rate, still scales it all with no formula moved: at 120, four hours
   a second, a week in 42 seconds and a year in 36.5 minutes. And the Day
   sheet is the one place that maps frames to hours (decision 9), so a night
   made shorter later moves that sheet and the hand's calendar and nothing
   else. *The alternatives:* the hours outside trading compressed by the
   calendar, the night at five hours a second say, a weekday 10 seconds and a
   weekend day 4.8, at the price of a time computed piece by piece on the Day
   sheet, eight cells or so more; or the host stepping faster at night, which
   needs a cartridge to write its own rate, refused today (§3.9), an ask of
   Alonzo not made here.

2. **The calendar's knob: minutes a frame, 2 (rec),** where `POT.1` had the
   frames a day, 30. Sixty over it is the manifest's rate, and a test holds
   that, as it held the frames a day to the rate; a person who writes 60
   plays a frame an hour, and 1,440 a frame a day, each in whole minutes and
   so exact, which is how the oracle's long runs play an hour or a day a frame
   (decision 14). A value and never the Clock's rate, for `POT.1`'s reason: a
   time read off the rate would jump when the teacher moves the dial. *The
   alternative:* the frames an hour, 30, which cannot be written for a frame
   a day, a twenty-fourth having no exact double, the day's turn then landing
   a rounding away from its frame.

3. **The first frame: Monday, day 0, at 9:30, the opening bell (rec).** The
   market opens on the game's first frame, so the first five minutes (`BD-4`)
   begin on a market that moves; day 0 is a Monday, so the first weekend
   comes after five sessions; the first day runs from the bell to midnight,
   fourteen and a half hours. The first frame's time a value, 570 minutes
   after midnight, beside the bells. *The alternatives:* midnight, the bell
   9.5 seconds in; 9:00, half a second before the bell.

4. **The trading hours: 9:30 to 16:00, Monday to Friday, no holidays (rec).**
   New York's core session, six and a half hours, so six and a half seconds
   of trading a weekday at the manifest's rate and seventeen and a half
   between; the weekend closed, Friday's close to Monday's bell 65 seconds
   and a half; 261 sessions in the first year where US exchanges keep about
   252, the holidays a caveat (decision 16). The bells and the trading days
   are values, 570, 960 and 5. The weekend is the owner's "time between
   trading days" at its longest, a fund manager's week, and the dearest of
   these to add later: the session before a Monday, the report card's
   sessions a year, the timers and the rules all read it. *The
   alternatives:* every day trades, 365 sessions a year, as `POT.1` has it,
   the session's arithmetic three terms shorter; 10:00 to 16:00, the New York
   Stock Exchange's hours from 1974 to 1985 (check), every bell on the hour
   and the time shown to the hour; holidays, which want a table of the year's
   sessions that the dialect, with no `INDEX`, cannot search.

5. **The market's clock: the day, with the session's hours inside it; `BD-7`
   restated (rec).** The waves stay a function of the day (`POT.2`), so each
   day's close is tier 1's index of that day to the bit, and the first year
   is `POT.2`'s: its dip of 1.1% to day 91, a Monday and so a session, and its
   close 10.2% up (*derived*). The session is that day's hours, eight marks,
   the opening bell and each hour from 10:00 to the closing bell at 16:00.
   `BD-7` restated, for the owner's words: *the calendar is the market's
   clock, and only the calendar reads the frame. A frame is two minutes of
   the market's time; prices move at the session's marks, and the dice,
   fees, timers and events at a day's first frame, and nothing at any other
   frame. The Day sheet alone reads `Clock!B1` and its time to the minute,
   `Day!B4`, the one calendar cell that changes every frame; every other
   formula reads the calendar's day, its time to the half hour, the session
   and its mark, each of which changes at most twice an hour, so that a frame
   between two half hours computes three of the calendar's cells and what
   the person touched.* Held as now, by the scan, its list `Clock!B1` and
   `Day!B4` with `B4`'s twin, and by the census's `(every-frame 0)`. *The
   alternative:* the hour as the market's one tick, the waves counting
   hours, which moves every day's close and so the kind first year, and asks
   the periods to be primes of hours.

6. **Inside the session: a price each hour, a path from the last close to the
   day's close, drawn from the day's dice (rec).** At the session's mark `m`,
   0 at the bell and 7 at the close, the index is the last close and the
   day's close weighed by `t = m/7`, times one and the swing at the mark:
   `(last*(1-t) + close*t) * (1 + s*(W(m) - t*W(7)))`, where `W(m)` is the sum
   of the session's first `m` steps, each `2u-1` for the uniform of the day's
   market draws 5 to 11, the next free seven (decision 8), and `s` the hour's
   swing, 0.004 of the index, the most one hour's step moves it. A bridge: at
   the bell `t` is 0 and `W(0)` is 0, and at the close `t` is 1 and
   `W(7)-W(7)` is 0, so the path leaves the last close and reaches the day's
   close exactly, in floating point too, each end an operand multiplied by 0
   or by 1; the hand held every open and every close of the first year bit
   for bit (*derived*). So nothing a day was moves, the hours wander between,
   and the market stays a function of the seed and the calendar (`BD-3`).
   What it does at 0.004 (*derived*, the first year's 261 sessions, 1,827
   hourly marks): the hour's price off its line by 0.12% at p50, 0.45% at p95 and
   0.75% at most; the $100 share moving at 1,798 marks, about 14 cents a
   mark, the $25 at 1,728, the $5 at 1,502, the $1 at 682, a cent at a time;
   the year's lowest mark 0.98382, on day 92, against its lowest close
   0.98943, so the kind first year stays above 0.98 at every hour. At 0.003
   the $1 share moves at 503 marks and the $100 about 10 cents; at 0.005 the
   lowest mark is 0.98231. The swing a value of the Market sheet, `B5`,
   beside the drift. *The alternatives:* prices at the bell alone, the day's
   close from 9:30 to the next bell, six and a half seconds of a still market
   a session; the waves read at a fraction of a day, which moves the index
   about 0.02% a day, a cent seldom. *(Amended at the owner's hand test,
   2026-10-16, the owner's words: "Why don't prices move on the half hour,
   too?", and at the question that followed, "Every half hour". So a mark
   each half hour, thirteen a session, 0 at the bell and 13 at the close,
   `t = m/13`; the steps the day's market draws 5 to 17, the next free
   thirteen; and the swing 0.0028 of the index, so that a session wanders as
   far as it did at 0.004 an hour, its middle mark off its line by 0.204% at
   p50 where 0.197 (*derived*, the first year's 261 sessions and 3,393
   half-hour marks): every open and every close bit for bit still; the half
   hour's price off its line by 0.13% at p50, 0.46% at p95 and 0.94% at
   most; the $100 share moving at 3,303 marks, about 10 cents a mark, the
   $25 at 3,106, the $5 at 2,513 and the $1 at 925; the year's lowest mark
   0.98013, on day 86 at 13:00, so that the kind first year stays above 0.98
   at every half hour. At 0.003 the lowest mark is 0.97947, under it.)*

7. **Outside the session: the last close, and its change; the first session
   from its own close (rec).** A price shown is the session's: during it, the
   hour's; outside it, the last close, until the next bell. The change beside
   it is since the session before's close, as a quote is: Monday before the
   bell shows Friday's close and Friday's change. The session before the
   game's first is the first itself, so day 0 opens and closes at the starts
   exactly, $1, $5, $25 and $100, every change 0 and every glyph `=` at the
   first frame, the hours of day 0 wandering between; `STATION.1`'s opening at
   the starts holds at the bell and at the close. So no price ever reads a
   day before day 0, and the least a price can be is a third of the trend at
   day 0 less the widest swing a bridge of seven steps allows, `24s/7`: the
   rungs' bound no seed passes 11, 94, 822 and 4,342 cents, where 11, 96, 833
   and 4,386 stand (*derived*). *The alternative:* the session before day 0
   the Friday before, day -3, every first frame then off the round starts.
   *(Amended with decision 6, 2026-10-16: the widest swing a bridge of
   thirteen steps allows is `84s/13`, at its sixth and seventh marks, so the
   rungs' bound is 11, 94, 818 and 4,327 cents (*derived*).)*

8. **The dice: a die a day, rolled at midnight and at the first frame; the
   session's seven draws the next free; the exam's stream moved to columns of
   its own (rec).** The die rolls when the day turns, at midnight, and at the
   game's first frame, as it rolls on a day's first frame now, so the market's
   dice are the same days' at any minutes a frame (`BD-3`). The session takes
   the market's draws 5 to 11, the day then holding eleven; they reach row
   12, through the exam's block at rows 7 to 9, so the exam's die and its
   draws move to `Dice!H1:J3`, labels in `H`, the die and its draws in `I`,
   their uniforms in `J`, out of the way of every draw a later course adds
   below row 12 (course 6's two hundred). Its numbers stay, being a function of
   the seed alone; nothing reads them until `GUARD.1`. *The alternative:* a
   die an hour, the session's steps its first draws, the day's draws then
   moving with the minutes a frame. *(Amended with decision 6, 2026-10-16:
   the session's thirteen draws are the market's 5 to 17, the day then
   holding seventeen, to row 18, beside the exam's block at `H1:J3`, which
   stays where this decision moved it.)*

9. **The Day sheet: the calendar, in minutes (rec).** `B2`, `B3`, `B5`, `B6`
   and `B7` keep their addresses and their meanings, so the dice, the phases,
   the fine print, the dividend, the hustle and the days between two
   dividends read them unchanged:

   | | A | B |
   |---|---|---|
   | 1 | minutes a frame | `2` |
   | 2 | days a year | `365` |
   | 3 | day | `=INT(B14/1440)` |
   | 4 | time, in minutes from day 0's midnight | `=(Clock!B1-1)*B1+B9` |
   | 5 | boundary | `=IF(OR(Clock!B1=1,B3<>Day.last!B3),1,0)` |
   | 6 | year | `=INT(B3/B2)+1` |
   | 7 | day of the year | `=MOD(B3,B2)+1` |
   | 9 | the first frame's time, in minutes after midnight | `570` |
   | 10 | the opening bell | `570` |
   | 11 | the closing bell | `960` |
   | 12 | days a week the market trades, from Monday | `5` |
   | 14 | the time, to the half hour | `=INT(B4/30)*30` |
   | 15 | the time of day, to the half hour | `=MOD(B14,1440)` |
   | 16 | the time shown | `=INT(B15/60)&":"&IF(MOD(B15,60)=0,"00","30")` |
   | 17 | the day of the week, from 0 a Monday | `=MOD(B3,7)` |
   | 18 | the day's name | `=CHOOSE(B17+1,"Monday",...,"Sunday")` |
   | 19 | open | `=IF(AND(B17<B12,B15>=B10,B15<B11),1,0)` |
   | 20 | the session's day | `=IF(AND(B17<B12,B15>=B10),B3,IF(B17>=B12,B3-B17+B12-1,IF(B17=0,B3-8+B12,B3-1)))` |
   | 21 | the session before it | `=IF(B20<=0,B20,B20-IF(MOD(B20,7)=0,8-B12,1))` |
   | 22 | the mark | `=IF(AND(B20=B3,B15<B11),INT(B15/60)-INT(B10/60),B23)` |
   | 23 | the closing mark | `=INT(B11/60)-INT(B10/60)` |

   Three cells compute at every frame, `B4`, `B5` and `B14`, as three do now;
   `B14` changes each half hour, and every cell below it at most then. The
   boundary compares the day with its twin, so it holds at any minutes a
   frame, a frame a day turning the day at every frame. The session's day is
   the last weekday whose bell has rung, Friday's through the weekend and
   until Monday's bell; the mark counts the session's hours, 0 at the bell,
   the closing mark after the close and before the bell. The literal 1,440
   is the minutes a day and 7 the days a week, arithmetic and not settings.
   *The alternatives:* the hour and its phase as `POT.1` spelled the day and
   its phase, with the long runs' frame a day lost (decision 2); a sheet of
   its own for the session, which would have to read the time to the minute
   or the half hour from the Day sheet, the calendar then two sheets.
   *(Amended with decision 6, 2026-10-16: the mark counts the session's half
   hours, `B22` `=IF(AND(B20=B3,B15<B11),INT(B15/30)-INT(B10/30),B23)`, and
   the closing mark `B23` `=INT(B11/30)-INT(B10/30)` is 13; every other row
   as drawn.)*

10. **The Market and the Stocks (rec).** The index's three columns read the
    session and not the day: `B14` `=Day!$B$20`, the session's day, its close
    in `B25`; `D14` `=Day!$B$21`, the session before, its close in `D25`,
    where `B14-1` stood; `C14` twenty days before the session, as now. The
    hour's swing, 0.004, at `B5` with the settings. Then the session, from
    row 27: a row an hour, `9:30 to 10:00` to `15:00 to 16:00`, its step
    `=2*Dice!$C6-1` down `B28:B34` and the steps so far down `C28:C34`; then
    the mark `B36` `=Day!$B$22`, its share of the session `B37`
    `=B36/Day!$B$23`, the swing at the mark `B38`
    `=$B$5*(CHOOSE(B36+1,0,C28,C29,C30,C31,C32,C33,C34)-B37*C34)`, and the
    index now `B39` `=(D25*(1-B37)+B25*B37)*(1+B38)`. Eighteen formula cells;
    the steps move at midnight with the dice, the mark's four at each mark,
    and the index now at each, the bell and the close among them. The Stocks'
    price today becomes the price now, `E5:E8`
    `=INT(B5*Market!$B$39^C5+0.5)`, and its price yesterday the price at the
    last close, `F5:F8` over `D25` as it stands, their headers renamed; the
    dividend's two columns unchanged. The Portfolio's change, `E` less `F`,
    is then the change since the last close with no edit. *The
    alternative:* the session's rows on a sheet of their own, `Session`,
    the Market sheet then a day's market alone. *(Amended with decisions 6
    and 11, 2026-10-16: the session a row a half hour, `9:30 to 10:00` to
    `15:30 to 16:00`, its steps down `B28:B40` and their sums down
    `C28:C40`; the mark `B42`, its share `B43`, the swing at the mark `B44`
    over a `CHOOSE` of fourteen, and the index now `B45`, which the price now
    reads, `E5:E8` `=INT(B5*Market!$B$45^C5+0.5)`: thirty formula cells
    where eighteen. And the Stocks gain the change since the last close in
    cents, `I5:I8` `=E5-F5`, which the Portfolio's change and its glyph read
    since its dollars became texts, where they had read the Portfolio's own
    numbers.)*

11. **The views: the date with its weekday and its time, and the bell's line
    (rec).** The library's `as-of` gains the day's name and the time to the
    half hour: "As of Monday, year 1, day 1, 9:30 (unaudited)", 52 characters
    at its longest (*derived*), spilling past the Portfolio's 49 into an
    empty column as the fine print does. A new verb beside it, `bell`, writes
    each view's row 2, one formula over the calendar, its four drafts: open,
    "The market is open until 16:00."; a weekday before the bell, "The market
    opens at 9:30."; after the close Monday to Thursday, "The market has
    closed. Analysts are now explaining why."; from Friday's close to Monday's
    bell, "The market is closed until Monday at 9:30. Compliance is not." The
    first two with no joke, being the lines a player acts on; the times and
    the weekday written into the texts, a test holding them to the bells'
    settings. Both views call both verbs, so `bell` enters `kitchen.vla` at
    its first dish, written twice, as `draws` did. The time shown to the half
    hour, so the bell reads 9:30 and a view's date changes once every fifteen
    frames: shown to the minute it would change at every frame, and every
    view with it. *The alternatives:* the time to the hour, the bell at 9:30
    then shown as 9:00; the bell's line in the date's own cell; no weekday.
    *(Amended at the owner's hand test, 2026-10-16, the owner's words: "All
    dollar cells should show two decimals, so Portfolio!B5:B8 should show
    cents at all times, even when the amount is a full dollar." The record
    holds no number format (`vla-lang` 0.9.4: `general` and `text`), so, at
    the owner's choice of a text now and the engine's format later, a view's
    dollars are a text: the library's `dollars`, a third verb beside `as-of`
    and `bell`, spells whole cents as the dollars, a point and two digits,
    `1.00`, padded on the left with two spaces for each of four digits the
    dollars lack, since a text is drawn from its cell's left and two spaces
    are a digit's width in the grid's font to within a sixth of a pixel
    (measured in the page at 15 pixels in Segoe UI: a digit 8.09 pixels, a
    space 4.11). The prices, `Portfolio!B5:B8`, and the fees, `Gigs!B9`,
    call it; the change, `C5:C8`, is its signed twin, written once, a `+`
    for a gain or none and a space and a `-` for a loss, 10.26 and 10.11
    pixels, so that every change's point stands within a sixth of a pixel
    of the others'; the glyph reads the change's cents, `Stocks!I5:I8`, and
    no longer the view's own cell. A sum or a comparison reads the model's
    cents and never these texts. The number format is asked of Frazaro and
    Alonzo (`MISE.6`), and when the record holds one the verb becomes a
    format and the texts numbers again. Caveat 12 rewritten with it.)*
    *(Amended again at the owner's second hand test, 2026-10-17, the owner's
    words: "The double-digited text appears to be left of center in the cells
    they occupy." The viewport draws a text from its cell's left pad and a
    number to its right pad, so a text padded to four digits of dollars
    ended about 30 CSS pixels short of where a number ends in a column of 12
    characters. So each is padded to end where a number ends: a price's or
    a fee's spaces and twice its dollars' digits make 16, and a change's
    spaces before its sign 14, from the font measured in the page, the
    numbers under the owner's second hand test below; and every column of a
    view's dollars is 12 characters, which a test holds.)*
    *(Amended at `GUARD.1`'s hand test, 2026-10-17, the owner's words:
    "Thousands separators need to be implemented": the thousands grouped by
    commas, a comma four fifths of a space and taking one off the padding,
    and the change the library's `signed-dollars` since (`GUARD.1`'s decision
    9, amended).)*

12. **The day's other readers (rec).** The hustle stays once a day, a
    calendar day, at any hour, the market open or closed: its formulas read
    `B3` and `Day.last!$B$3` unchanged, a day being 24 seconds now, so "the
    next prospectus arrives tomorrow" within the half minute. The fine print
    stays a disclosure every 30 days, twelve minutes at the manifest's rate.
    The dividend stays on its four days of the year, all day, paid by
    `PLATE.1` at the day's first frame whether the market trades or not, the
    first year's four on Sundays (Found). The report card's year stays the
    calendar's, read at day 365, 2 hours 26 minutes at the manifest's rate and
    36.5 minutes at the dial's 120; a card each quarter, the quarter already
    the model's in the dividend's 91 days, is `CRITIC.1`'s to scope. *The
    alternatives:* gigs while the market is closed alone, a claim refused
    during the session; the first report card at the quarter's end, day 91,
    36 minutes in, which at this seed reads the year's low. *(Amended at the
    approval, 2026-10-16, the owner's words: "report cards should come
    quarterly and annually instead of just annually." So a report card at
    each quarter's end, the first about 36 minutes in at the manifest's rate,
    near the year's low at this seed, and the year's at day 365; `CRITIC.1`
    builds them, its line and the contract saying so from this item's build,
    and no cell of this item moves for it.)* *(Amended at the owner's hand
    test, 2026-10-16, the owner's words: "Maybe prospectuses should be
    limited to one per week, and they take a full day to complete", and at
    the question that followed, "Weekly, paid 24 game hours later; but why
    limit to 4 per run? Is that to incentivize unlocking other,
    higher-paying gigs instead of farming prospectuses forever?" So the
    hustle is a prospectus a week and a day's work: a claim opens the next
    one seven days after the day it counted against, `Hustle!B13`
    `Day.last!$B$3+$B$6`, the days between a setting, `B6`, 7; its fee is
    paid a day after the half hour the person saw at the claim, `B15`
    `Day.last!$B$14+$B$7`, the work a setting, `B7`, 1,440 minutes, while
    `B16` says it is being proofread, `B17` counts those done and `B18`
    pays them; and Gigs' sentence says when the fee is paid, "Proofreading:
    the fee is paid tomorrow at 16:00.", or when the next one arrives, "Done
    for the week: the next prospectus arrives in 3 days." The cap stays four
    a run, `B2`, a value the owner may raise, the question answered in
    chat: it is `BACK.1`'s, the owner's word at its approval, four claims
    being $100, about twice a perfect trader's first year on the personal
    pocket's $100; the hustle starts the pocket and is not the game's loop,
    the gigs to come each with a pay of its own; and uncapped at a week each
    it would pay $1,300 a year, about 25 times that trader's $52.14,
    `BACK.1`'s figure over the market as it stood before this item.)*

13. **An order while the market is closed: refused, spelled here, built by
    `PLATE.1` (rec).** An order fills at the price shown, the last frame's,
    `Stocks.last!E5:E8`, and only while the last frame showed the market open,
    `Day.last!$B$19=1`, as a claim is judged by the state the person saw; a
    press while it showed closed is refused and fills nothing, its button
    saying so. No order waits for the bell at course 0: a queue is state, a
    market-on-open order a later dish's. Nothing of it is built here; this
    settles the cell `PLATE.1` reads and the rule it follows. *The
    alternative:* an order placed out of hours filled at the next bell's
    price, the plate keeping a queue a pocket wide.

14. **The oracle (rec).** In `oracle/src/buffet.rs` and
    `oracle/tests/cartridge.rs`:
    - *The hand's calendar counted in minutes*: a tick adds the minutes a
      frame, the minutes rolling into the day, the day into the week and the
      year; the session counted, a new one when the bell rings on a weekday,
      the one before it kept, the session before day 0's first being itself;
      the mark counted, 0 at the bell and one more at each hour of the
      session, the closing mark from the close. It shares no predicate with
      the cells, which divide the time and choose by the weekday.
    - *The hand's session*: the bridge in the cell's order (the market cannot
      share nothing, `POT.2`); its own test, every open and close of a year
      equal to tier 1's to the bit, and no mark off its line by more than
      `24s/7`.
    - *The runs*, at the pace each needs, so that every frame is held where a
      frame matters and every hour and every day where they do:
      - every frame of ten days at the manifest's pace, from Monday's bell on
        day 0 to the second Wednesday's last frame, the first weekend and the
        Monday after it among them, 6,915 frames (*derived*): the calendar at
        every frame, the dice moving at a day's first frame alone, the
        Market's columns, its session and the index now and the Stocks bit
        for bit at every frame, the fine print, the Portfolio held whole at
        each half hour and byte for byte between, and Gigs at rest;
      - every hour of a year and two days at a frame an hour, the minutes a
        frame written 60, 8,808 frames: the same, at every mark of 263
        sessions, the year's turn and the four dividends among them;
      - eight hundred days at a frame a day, 1,440 minutes, the bell's minute
        each frame: the same days rolling the same dice and pricing the same
        closes at a frame a day, an hour and two minutes;
      - a hundred years at a frame a day: the session's close, the look back
        and the session before, every rung now and at the last close, the
        dividend, and the bound;
      - a year at difficulty 0 at a frame an hour, 8,760 frames;
      - the hustle's scripted session at the manifest's pace, its acts written
        by day and time, every frame held as now.
    - *The settings and the texts*: the Day sheet's settings the hand's; sixty
      over the minutes a frame the manifest's rate; the bell's texts naming
      the bells' times and Monday.
    - *Kept*: the cartridge its source expanded; `BD-7`'s scan, its list
      `Clock!B1`, `Day!B4` and `Day.last!B4`; the census's `(every-frame 0)`;
      nothing reads a view; the tab order; the game opening on the Portfolio;
      the kind first year, its every mark above 0.98 too; the describe golden
      blessed again and read in its diff.
    - *Mutants*, each written into the source by one anchor and expanded
      before the suite runs, each failing the test written for it: the die
      rolling each half hour; the weekend trading, the trading days 7; the
      bridge unpinned, `-B37*C34` lost, a close off its day; the mark one
      ahead at the bell; the session before a Monday the Sunday; the bell's
      price today's close, a gap at 9:30; the closing bell at 16:30; a view's
      date reading the time to the minute; and the exam's die left at `B7`,
      which the load refuses, a cell written twice.

15. **What a frame costs, measured (rec).** Natively, as `BACK.1` took it:
    this tree's runner timing both cartridges, `518d414`'s texts copied into
    its tree for the runs marked before, pinned to the performance cores and
    opted out of Windows' throttling, a fixed computation before and after:
    the pass, the load, the first frame; in `518d414`, an ordinary frame and
    a day's first; in this tree, an ordinary frame, a half hour's turn, an
    hour's mark, the bell, the close and midnight, over ten days. Each test
    timed alone in both trees before the suite's whole is read. In the
    browser, `BACK.1`'s instruments copied: the page built here, three runs
    in headless Chrome at the owner's full screen, every process opted out
    and pinned; the Portfolio shown through a week, each tick filed by what
    its frame turned (nothing, a half hour, a mark, the bell, the close,
    midnight) and timed, a still tick and each kind of patch at p50, p95 and
    the longest, and the main thread a frame; then the dial at 120 over a
    week. And the owner's own Chrome, the probe copy printing the same in a
    corner.

16. **The caveats, drafts (rec).** A dish that brings a simplification brings
    its row (`BD-5`):

    | | Things this game gets wrong | In the real world | Management's response | Where |
    |---|---|---|---|---|
    | 2, rewritten | The market keeps no holidays: every weekday trades, 261 sessions in the first year. | US exchanges also close for about ten holidays a year, leaving about 252 trading days. | Holidays are available from the rank of Partner. | `Day!B12` |
    | 11, amended | A $1 share moves a penny at a time, and most days closes where it opened. | (as it stands) | (as it stands) | (as it stands) |
    | 13, new | Prices move once an hour, on the hour, and stand still between. | A share's price moves with every trade, many times a second, from the opening bell to the closing one. | Management prefers hourly prices, which are easier to explain to Management. | `Day!B22` |
    | 14, new | Each day's close is decided before the bell: the hours wander off it and always come back. | Nobody knows a day's close before the closing bell, which is the point of having one. | Management confirms the close was agreed in advance, as a courtesy to the auditors. | `Market!B39` |

    Row 11's "and on most days not at all" stops being true: within a
    session the $1 share moves at about a third of its marks (decision 6),
    though its close still differs from the last on 25 of 261 sessions.
    Rows 3 and 5 stay true, each of the day. Row 2's claim of 252 is marked
    for `MENU.8`'s read, as it already was. *(Amended at the owner's hand
    test, 2026-10-16, with decisions 6 and 11, drafts again for the owner:
    row 13 "Prices move on the half hour and stand still between.", its
    response "Management prefers prices that keep to a timetable."; row 14's
    "the half hours wander off it", at `Market!B45`; and row 12, on number
    formats, "$12.30 is 1230 in the model's cents and text on the
    Portfolio", its response "The decimal point has returned from furlough,
    on a text-only contract." Within a session the $1 share now moves at
    about a quarter of its marks, 925 of 3,393.)*

17. **The contract, `BD-7`, the notes and the comments (rec).** In
    `OMAKASE.md` at the build: the contract's "thirty frames a market day, so
    a day a second and a year about six minutes" becomes "two minutes a
    frame, thirty frames an hour, so an hour a second, a day 24 seconds and a
    year about two hours and a half, the market open from 9:30 to 16:00
    Monday to Friday, 261 sessions in the first year"; its hustle "once a
    day"; `COURSE.0`'s table "a market ... that moves once a day, a day a
    second" becomes "that trades from the opening bell to the closing bell,
    an hour a second"; `BD-7` as decision 5 restates it, dated. The
    manifest's notes: "the game opens on the Portfolio at Monday's opening
    bell, the day, the hour and four stocks on it, each its price and its
    change since the last close, an hour a second, the market open from 9:30
    to 16:00 on weekdays and moving each hour"; Gigs "once a day"; the back
    office "the market's calendar and its hours" and "four waves and a drift,
    and each session's hours between two closes". The source's comment on the
    market's clock rewritten to the new pace, the dial's 120 its example.

18. **The hand test, the owner's, on the page built here:** the Portfolio at
    the first frame, "As of Monday, year 1, day 1, 9:30 (unaudited)", the
    bell's line open, the four starts and every change 0; the date turning
    every half second; the prices at 10:00 and each hour after, the changes
    and the glyphs with them; the close at 16:00, the four starts again and
    the line closed; the night; Tuesday's bell; then the dial at 120 by a row
    written in the drawer, Friday's close, the weekend's line and Monday's
    bell; the Day sheet's cells through it; Gigs claiming once a day still;
    Build's text compared with `src/buffet.vla` by a command given at the
    handover, since the page prints no "same bytes"; the probe copy's corner
    after a few minutes.

19. **The stale places, rewritten at the build.** In `OMAKASE.md`: `POT.10`'s
    line, 🟡 from the approval, its text as the decisions leave it; the
    contract, the course's table and `BD-7` (decision 17); `PLATE.1`'s line,
    the order's rule of decision 13; `MISE.4`'s floors fixture, a frame a day
    by the minutes a frame written 1,440 where it had a manifest at
    `(rate 1)`, which no longer plays one; `MISE.5`'s phases at the new pace;
    `GUARD.1`'s line, the exam and the clock (Found); `PLATE.3`'s and
    `RECIPE.1`'s lines, the day or the session for their counts (Found). The
    source's opening comment, notes, the Day, Dice, Market and Stocks
    sections and the views'; `kitchen.vla`'s `as-of` and `bell`; the caveats'
    file; `oracle/src/buffet.rs`'s module comment, the calendar, the
    session and the constants; the module comments of `oracle/src/lib.rs`
    and `oracle/tests/cartridge.rs`; the describe golden; `BACK.1`'s P5 cell,
    CI's Linux at the push; `Delta/CLAUDE.md`'s map, the calendar's settings
    among the edits that trip a test; the memory. Not this item: the plate
    and its orders (`PLATE.1`), the report card (`CRITIC.1`), the weather word
    (`FRONT.3`), the looks (`FRONT.2`'s second slice), and the replay golden
    (the tasting).

**Found at the scoping**, measured or read, none minted:
- *The dividend falls on Sundays in the first year* (*derived*). The 91st,
  182nd, 273rd and 364th days of a year begun on a Monday are Sundays, then
  Mondays, a weekday later each year; no fixed day of the year stays a
  weekday, 365 being 52 weeks and a day. The cell holds it all day and
  `PLATE.1` pays it at the day's first frame, so nothing breaks; a dividend
  paid on the session nearest its day is `PLATE.1`'s or `STATION.1`'s second
  slice's to weigh, if the owner minds.
- *The exam and the clock* (`GUARD.1`). The market runs from the first
  frame, so an exam of two minutes would pass five days of it. Whether the
  calendar starts at the exam's end, its first frame's time a formula of the
  Exam sheet, which the Day sheet may read, is `GUARD.1`'s scoping's
  question.
- *A cartridge's own pace* (`SPEC.md` §3.9). A night made shorter by the host
  needs a cartridge to write its rate, a host-written cell the Write sheet
  refuses; the calendar can do it alone (decision 1's alternative). A
  candidate for Alonzo should the owner want the dial a game's.
- *`MISE.4`'s floors fixture.* "A build of the course's source whose manifest
  says `(rate 1)`, one frame a market day" no longer plays a day a frame,
  `(rate 1)` being a frame a second and the minutes a frame two; the fixture
  becomes the minutes a frame written 1,440 (decision 19).
- *`PLATE.3`'s timers and `RECIPE.1`'s cadence.* Each counts "days": a
  calendar day, or a session, is theirs to choose; the calendar gives both,
  `B3` and `B20`.

**The predictions, before any number.** The numbers column is filled at the
build, P9 at the owner's hand test; P3 to P5 and P8 as decision 15 says.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 535 to 550 rows where 474 stand, about 542: the Day sheet's 42 where 14 (21 labels, 6 values, 15 formulas); the Dice's seven labels more, its draws' two rows eleven deep; the Market's 23 more, the swing and its label and the session's 21; the views' two bells; the caveats' two rows of four fields; 165 to 172 formula cells where 125, about 169; the diff the notes, the Day sheet whole, the Dice's labels and the exam's block, the Market's `B5`, `B13:D14` and its session after row 25, the Stocks' `E` and headers, both views' first two rows and the caveats | *Measurement*: the runner printed `(rows 543) (loads yes)`, one more than about 542, the Day sheet's labels' column, as wide as its longest label, 36 characters, so that the calendar reads without a drag (`POT.1`'s hand test had found the Day sheet wants one); 169 formula cells in 81 formula rows where 62; the Day sheet's rows 44 where 15, the Dice's 32 where 25, the Market's 80 where 57, each view's one more and the caveats' eight more, the 69 rows exactly; the diff 35 lines out and 104 in, where predicted |
| P2 | the describe golden | the notes; `(budget 30 169 5070 2000000)`, give or take three cells; the plan's shapes 75 to 82 where 60, every-frame and cycles 0; the Day's readers rows grown, its own cells and the Market's, the views' and the bell's reads; `(readers "Day.last" 2)`, the boundary's read with the hustle's; the Dice's readers grown by the session's steps; the Market's grown by the Stocks' `E`; the views' date one shape over two sheets as now, and `bell` one more of the same kind; every other shape's row its own but for its number | *Measurement*: the notes; `(budget 30 169 5070 2000000)`; `(plan (shapes 78) (texts 76) (formulas 169) (levels 24) (runs 9) (lanes 37) (alone 132) (every-frame 0) (cycles 0))`; `(readers "Day" 53 (most 11))` where 17, `(readers "Day.last" 2)`, `(readers "Dice" 8)` where 7, `(readers "Market" 51 (most 6))` where 34, `(readers "Caveats" 60 (most 8))` where 52; the views' date one shape over two sheets and the bell one more of the kind, `(cells 2) ... (sheets 2)`. **Missed** in two: every shape below the calendar stands deeper in the order, the Portfolio's prices at level 22 where 14, so its row moved by more than its number; and the waves' fifteen terms, `Market!B15:D19`, one run of six lanes before, are now evaluated alone, `(runs 9) (lanes 37)` where 10 and 43, `alone` 132 being 82 and the calendar's 10, the dice's 14, the session's 18, the bells' 2 and those 6. A run is four cells or more of one shape together in the plan's order (`vla-lang` 0.9.4's `RUN`); not looked into further, the terms computing once a day, at the bell |
| P3 | the pass and the load | the pass 0.4 to 0.9 ms longer than `518d414`'s; the load 0.25 to 0.5 ms longer | *Measurement*, this tree's runner over both cartridges, `518d414`'s four texts (the source, the library, the caveats, the cartridge) copied into its tree for the runs marked before, three runs each alternated, 200 passes and 200 loads a run, every run pinned to the performance cores and opted out of Windows' throttling, a fixed computation before and after within 3.9%: the pass 5.50 to 5.51 ms at p50 where it took 4.86 to 5.02, the medians 0.53 apart, inside; the load 2.33 to 2.40 where 1.83 to 1.85, the medians 0.49 apart, inside at its edge |
| P4 | the step | an ordinary frame within 0.4 µs of `518d414`'s, three cells as now; a half hour's turn 4 to 9 µs; an hour's mark in the session 8 to 16 µs; the bell, the whole Market's three columns, 18 to 28 µs; the close 8 to 16 µs; midnight, the dice and the session's steps, 15 to 28 µs; the first frame 110 to 140 µs where about 99 | *Measurement*, the same runs, ten days at two minutes a frame, each frame filed by what the hand's calendar ticked beside it turned, against two years and two days at thirty frames a day: an ordinary frame 2.4 to 2.5 µs where 2.1, inside at its edge; an hour's mark 12.8 to 13.5 µs, inside; the bell 21.0 to 25.3, inside; the close 14.2 to 15.6, inside; midnight 22.9 to 26.5, inside; the first frame 118 to 126 µs where 99 to 100, inside; `518d414`'s day's first frame 17.5 to 17.8 µs, 18.8 to 19.9 where the fine print turns. **Missed** in one: a half hour's turn 9.2 to 9.5 µs, over its 4 to 9, the date joined as text on two views and the calendar's cells below the half hour priced short |
| P5 | the suite | 52 to 56 tests green, and in CI on Linux; the suite 3.0 to 4.5 s where 5.0, its longest test the hour's year at about 3 s; every test timed alone in both trees | *Measurement*, here: 54 green, the crate's 21 and the cartridge's 33; the suite's whole 4.0 to 4.2 s on eight threads in two runs, where 5.0, inside. Each test timed alone first, two rounds alternated, one thread: the hundred years at a frame a day 4.12 s, the hour's year 3.60, the ten days at every frame 2.23, the year at difficulty 0 at a frame an hour 1.48 where a year at thirty frames a day took 1.40, the eight hundred days 0.35, the scripted session 0.30 where 0.02, its six days 4,035 frames where 180; `518d414`'s two years took 5.03 s and its hundred years 2.68 (`BACK.1`'s P5). **Missed** in one: the longest test is the hundred years', its window grown to the index now and the hand's path computed at each of its 36,500 frames, not looked into, and not the hour's year at about 3 s. Linux at the push, run 39088078269 at `7bbeb59`, two commits on, `GUARD.1` and the pin to `alonzo` 0.2.0: the 67 green there, the crate's 27 and the cartridge's 40, the cartridge's in 6.07 s |
| P6 | the page | the cartridge's block 5,500 to 8,000 characters longer, the source's 9,000 to 14,000 and the library's 600 to 1,200: the page between 1,847,000 and 1,856,000 where 1,832,337 stands; Build writing the cartridge byte for byte | *Measurement*: 1,845,186 characters, 12,849 more than `BACK.1`'s 1,832,337: the cartridge's block 4,302 longer, the source's 6,612, the library's 1,337 and the caveats' 598, the four adding to the page's growth exactly; Build writing the cartridge byte for byte, 32,399 characters, checked headlessly (P9). **Missed** in all three blocks and the page: the cartridge's and the source's under their ranges, their rows and comments shorter than priced, the library's over it, its comment longer, and the caveats' block not priced at all, as `MENU.1` once forgot the library's |
| P7 | the checks | 4 checks and 4 controls green, no check edited, no file added under `src/` | *Measurement*, as predicted: ALL GREEN, 4 checks and 4 controls, the layout control's fourteen mutants and the offline control's fifteen as they stood; no check edited; no file added under `src/` |
| P8 | a frame's cost in Chrome at full screen | a still tick 0.2 ms at p50 and 0.3 at p95, as now; a half hour's patch 0.4 to 0.5; a mark's and the bell's 0.5 to 0.8; the main thread 0.78 to 0.90 ms a frame through a week, a view's date patched every fifteenth frame; at the dial's 120 the same per frame; the owner's Chrome within a tenth | *Measurement*, three runs in headless Chrome at 2,560 by 1,440 CSS pixels at 1.5, its eleven processes opted out and pinned, the clock within 4%, the Portfolio shown; in each, two days at the manifest's rate, 1,440 frames from about the 800th, each tick filed by what its frame turned, found from the frame number so that the probe reads no record: a still tick 0.2 ms at p50 and 0.3 at p95, as now, at its longest 0.6 to 1.7; a half hour's patch 0.4 at p50 and 0.5 at p95, inside; a mark's 0.5 at p50 and 0.7 to 1.0 at p95; the bell's and the close's 0.5 at p50, 0.5 to 0.7 at p95; midnight's 0.4 to 0.5; 96 of 1,441 frames patched, one in fifteen, the date's; the main thread 0.75 to 0.80 ms a frame, inside in two runs and the third 0.03 under. Then the dial at 120 by a person's edit, a week of 5,040 frames: the host stepping two frames a tick on the display's 60, a tick 0.4 ms at p50 and 0.6 at p95, at its longest 1.1 to 2.0, and the main thread 0.34 to 0.36 ms a frame. **Missed** in one: at the dial's 120 a frame costs the main thread half what it does at 30, two frames sharing each tick's draw, where the same was predicted. The owner's Chrome, the probe copy's corner at the hand test, 3,270 ticks of one frame each from Monday's 10:30 to Friday's night at the manifest's rate: a still tick 0.2 ms at p50 and 0.4 at p95, at its longest 1.0; a half hour's patch 0.4 and 0.6; a mark's 0.6 and 0.9; the close's and midnight's 0.6 and 0.8; 219 of 3,300 frames patched; within a tenth of the headless runs at p50 but for one, **missed**: the bell's 0.9 ms at p50 over its four ticks, where the headless runs read 0.5 |
| P9 | the hand test | the strip unchanged, no sheet added; the Portfolio opening on Monday's bell at the starts, every change 0; a price each hour, the starts again at day 0's close; the night; the weekend's line and Monday's bell at the dial's 120; Gigs once a day; Build's bytes the file's, checked headlessly | *Headlessly*, over the page built here, before the owner's test, its steps driven through the DevTools protocol from the first frame, the harness picking the game once attached: the first frame "As of Monday, year 1, day 1, 9:30 (unaudited)" over "The market is open until 16:00.", the four starts and every change 0, `====`; at 10:00 two prices and their changes moved, `==▼▼`, and at 11:00 three, `=▲▲▲`; at 16:00 the starts again, `====`, and "The market has closed. Analysts are now explaining why."; a click on Gigs' button claiming and a second claiming nothing; Tuesday's midnight, "The market opens at 9:30."; Tuesday's bell from Monday's close, every change 0; the dial written 120, Friday's close, "The market is closed until Monday at 9:30. Compliance is not.", Saturday's midnight showing Friday's session, Monday's bell opening from Friday's close with every change 0, and 10:00 moving; the strip and its box as they stood, "hidden sheets (18)"; Build's bytes the file's, 32,399 characters. The owner's, on the page built here, the probe copy's, from Monday's bell to Saturday's first hour: passed, Build's bytes the file's, "same bytes (32398 characters)", the command trimming the file's last newline; and three changes asked, a price each half hour, a prospectus a week and a day's work, and every dollar cell in two decimals, with two of the bell's lines rewritten in the owner's words, each in the paragraph below |

*Scoped 2026-10-16. Approved by the owner the same day with the report
cards amended (decision 12's amendment): "All recommendations approved,
except report cards should come quarterly and annually instead of just
annually. Proceed with implementing POT.10 as scoped." So every other
decision stands as recommended, the drafts among them until the owner
rewrites them; the amendment moves no cell of this item and no prediction.*

**What the build found.** The calendar in minutes held as decision 9 drew
it, its labels shortened to sit in a column of their own width, 36
characters, the one row P1 did not hold, so that the calendar reads without
a drag. Driven through the engine before the hand was written, the cartridge
opened at Monday's bell on the starts, moved its prices at 10:00 and 11:00,
closed day 0 on the starts again, opened Tuesday from Monday's close, showed
Friday's session through the weekend and opened Monday from Friday's. The
hand's calendar, counting minutes into hours and keeping the session whose
bell it last heard, met the cells at every frame of every run from the first:
the ten days at every frame, the hour's year, the eight hundred days and the
hundred years at a frame a day, and the hustle's session at six days of the
new pace, every act where it fell. The hand's own tests hold every open and
every close of the first year to the bit, a bridge's widest stray, 24/7 of a
step, over all 128 patterns of extreme steps, and the kind year at every
hour, its lowest 0.98382 on day 92, above the 0.98 its days keep. The census
found the waves' fifteen terms evaluated alone where six had run together,
every shape below the calendar standing deeper in the order (P2). The
oracle's runs fell in time but for one: the suite's whole 4.0 to 4.3 s where
5.0, its longest test now the hundred years at a frame a day, 4.12 s alone
(P5). Nine mutants, each written into the source or the library by one anchor
and expanded before the suite ran, each failed the test written for it: the
die rolling each half hour, the weekend trading, the bridge unpinned, the mark
one ahead at the bell, the session before a Monday the Sunday, the bell's
price today's close, the closing bell at 16:30, a view's date reading the time
to the minute, which `BD-7`'s scan caught too, and the exam's die left at
`B7`, which the load refused, "Dice!B7 was already written by line 77". At a
frame a day the die rolls every frame, so the first mutant passed the
frame-a-day runs and failed the ten days and the hour's year, as a mutant of
the hours should. In the page, driven headlessly by the hand test's steps from
the first frame (P9), every line the scoping drew appeared where it was drawn,
and in Chrome at the owner's full screen a half hour costs a patch of 0.4 ms,
a mark 0.5, the main thread 0.75 to 0.80 ms a frame at the manifest's rate and
half that at the dial's 120 (P8). Found with it: a harness that attaches to a
page after the game is picked misses its first frames, so the verifier picked
the game itself once attached.

**The owner's hand test, and what it changed.** On the page built here, the
probe copy, from Monday's bell to Saturday's first hour: the hand test passed
(P9), Build's bytes the file's, and the owner's Chrome drew a frame as the
headless runs did but for the bell, over four ticks (P8). Three changes were
asked, each in the owner's words, and settled the same day by three questions
answered: "Why don't prices move on the half hour, too?", a mark each half
hour (decisions 6 to 10, amended); "Maybe prospectuses should be limited to
one per week, and they take a full day to complete", a prospectus a week, its
fee paid a day after the claim, the cap of four kept, the owner's question of
it answered (decision 12, amended); and "All dollar cells should show two
decimals, so Portfolio!B5:B8 should show cents at all times, even when the
amount is a full dollar", the views' dollars spelled as texts now and the
record's number format asked of Frazaro and Alonzo for later, the owner
choosing "Text now, engine later" (decision 11, amended; `MISE.6`). And the
owner rewrote two of the bell's lines in `kitchen.vla`, the drafts' jokes
gone: "The market has closed. Analysts are analyzing the aftermath." and "The
market is closed until Monday at 9:30.", the hand's texts with them.

What moved: 569 rows where 543, and 201 formula cells in 86 formula rows
where 169 in 81; the describe golden's `(budget 30 201 6030 2000000)`,
`(plan (shapes 83) (texts 81) (formulas 201) (levels 30) (runs 10) (lanes 41)
(alone 160) (every-frame 0) (cycles 0))`, `(readers "Day" 56 (most 13))`,
`(readers "Market" 57 (most 6))`, `(readers "Stocks" 13)`, `(readers "Hustle"
31 (most 5))`, `(readers "Day.last" 3)` and `(readers "Hustle.last" 4)`, the
Portfolio's own readers row gone with the glyph's read of it; the hand's
calendar a mark each half hour, its bridge of thirteen steps straying at most
84/13 of a step over all 8,192 patterns of extreme steps, the kind year above
0.98 at every half hour, the dollars spelled by two functions held over every
count of cents to $10,000 and every change to $1,000 either way, and the
hustle's claims each a day and a half hour; the cartridge's runs as drawn,
the hustle's scripted session now four weeks and a day at a frame each half
hour, its four claims on days 0, 7, 14 and 21 and every fee paid; 55 tests
green, the crate's 22 and the cartridge's 33, the suite's whole 4.0 to 4.4 s
on eight threads in two runs. Each test alone, two rounds alternated, one
thread: the hour's year 4.17 s, now the longest; the hundred years at a frame
a day 3.90, its window narrowed to the session's close, the look back and the
session before, the index now being held at every hour by the hour's year;
the ten days at every frame 2.57; the year at difficulty 0 1.74; the eight
hundred days 0.40; the scripted session 0.14. Natively, this tree's runner
over both cartridges as before, six runs alternated, the fixed computation
within 5.7%: the pass 5.41 to 5.73 ms where `518d414`'s took 4.80 to 4.92,
the load 2.54 to 2.60 where 1.80 to 1.94, the first frame 136 to 141 µs where
95 to 110; an ordinary frame 2.5 to 2.6 µs where 2.1 to 2.2; a half hour's
turn 9.4 to 9.6; a mark 24.0 to 24.7 µs where the hour's was 12.8 to 13.5,
each mark now spelling eight dollar texts on the Portfolio, where it had
divided eight numbers; the bell 25.2 to 27.7,
the close 25.3 to 26.0 and midnight 27.7 to 32.2; `518d414`'s day's first
frame 17.4 to 18.2 µs. In Chrome, three runs at the owner's full screen as
P8's: a still tick 0.2 ms at p50 and 0.3 at p95; a half hour's patch 0.4 at
p50 and 0.5 to 0.6 at p95; a mark's 0.6 and 0.7; the bell's 0.4 to 0.5, the
close's 0.4 to 0.6 and midnight's 0.5 at p50; 96 of 1,441 frames patched, the
date's every half hour as before; the main thread 0.77 to 0.84 ms a frame; at
the dial's 120 a tick of two frames 0.4 ms at p50 and 0.6 to 0.7 at p95, the
main thread 0.36 to 0.40 ms a frame. The page 1,850,757 characters, Build
writing the cartridge byte for byte, 34,203 characters. Headlessly, the hand
test's steps: the first frame's prices `1.00`, `5.00`, `25.00` and `100.00`,
their points in one column, every change `+0.00`; the prices moving at 10:00,
10:30 and 11:00, `==▲▲`, `=▲▲▲` and `▲▲▲▲`; the close on the starts again
under the owner's line; a click on Gigs claiming, "Proofreading: the fee is
paid tomorrow at 16:00.", a second claiming nothing, Tuesday's "today at
16:00", and on Friday the fee paid, `25.00`, and "Done for the week: the next
prospectus arrives in 3 days."; Friday's close at `-0.01` and `-0.02` beside
`=▼=▼`; Monday's bell from Friday's close. Thirteen mutants, the nine with
their anchors moved to the session's new rows and four for the amendment, a
price spelled at its shortest, a change's sign lost, the week lost to a day
and the fee paid at the click, each caught by the test written for it. The
amended page is the owner's to test again before the commit.

Found with it: the figure space, U+2007, a digit's width in fonts that have
one, is not in the grid's: the page draws it from another font, 21.34 pixels
wide at 15, so the dollars pad with two spaces a digit, which hold to within a
sixth of a pixel in Segoe UI at 15 pixels, the one font measured; a system
whose font sets a space at another share of a digit, unmeasured, would draw a
column's points apart by the difference, one reason more for the record's
format.

**The owner's second hand test, and what it changed.** On the amended page
and its probe copy, 2026-10-17: Build's bytes the file's, "same bytes (34202
characters)", the command trimming the last newline; Gigs on the second
Monday at 21:00, a Sunday's prospectus proofread and paid, `25.00`, and "Done
for the week: the next prospectus arrives in 6 days."; the Portfolio on
Tuesday at 14:00 and on the third Thursday, every price and change in two
decimals; and the probe copy's corner at frame 12,148, the manifest's rate
throughout: a still tick 0.2 ms at p50 and 0.3 at p95, at its longest 1.6; a
half hour's patch 0.4 and 0.6; a mark's 0.5 and 0.9; the bell's 0.6 and 0.9
over twelve; the close's 0.6 and 1.2 over thirteen; midnight's 0.4 and 0.9
over seventeen; 809 of 12,148 frames patched, one in fifteen; each within a
tenth of the headless runs at p50. One finding, the owner's words: "The
double-digited text appears to be left of center in the cells they occupy."
Read in the page's `web/viewport.js`, Alonzo's at the pin's tag: a text is
drawn from its cell's left pad and a number to its right pad, each pad
`round(4·scale)` device pixels, in a column `round(84·scale)` wide for 12
characters, at `round(13·scale)` pixels of system-ui; so a text padded to four
digits of dollars ended about 30 CSS pixels short of a number's edge, and a
column of them sat left of center. Measured on a canvas in headless Chrome at
13 pixels of Segoe UI: a digit 7.008, a space 3.561, the point 2.818, a plus
8.893, a space and a minus 8.760. Over the scales 100%, 125%, 150%, 175% and
200% and one to seven digits of dollars, a price whose spaces and twice its
dollars' digits make 16 ends 0.6 to 1.7 device pixels short of a number's edge
at 150%, the owner's, and never past it at any scale, 6.0 short at the most;
at 17 it would end 3.8 to 4.9 past it at 150%, inside the pad but against the
gridline. A change whose spaces before its sign and twice its dollars' digits
make 14 ends 1.4 to 2.1 past it at 150%, and between 2.7 short and 2.1 past at
any scale. So decision 11 was amended again: the library's `dollars` pads to
16 and the change to 14, no price from eight digits and no change from seven;
the hand's `two_decimals` and `signed_two_decimals` with them, `PRICE_UNITS`
and `CHANGE_UNITS`; and a test that every column of a view's dollars is 12
characters, `DOLLARS_COLUMN`, the width the padding is measured for.
Headlessly at 150%, enlarged from the screenshots: Gigs' `0.00` ends at the
pixel where the `0` and the `3` above it end, and the Portfolio's prices and
changes at their cells' right pad.

What moved: 569 rows and 201 formula cells still, the census's counts
unchanged, the two shapes' formulas in the describe golden; 56 tests green,
the crate's 22 in 0.57 to 0.59 s, its test of the spelling reading each text
back as whole numbers where it had parsed a double, 0.90 s alone at first, and
the cartridge's 34 in 4.0 to 4.6 s on eight threads; the page 1,852,402
characters, Build byte for byte, 34,723 characters, checked headlessly;
fifteen mutants, two more for the padding, a price padded to four digits again
and a change padded as before, each caught. Natively, this tree's runner over
both cartridges, six runs alternated, the fixed computation within 1.6% (a
first set of six, one of whose runs moved 8.8%, set aside): a mark 23.9 to
24.0 µs, a half hour 9.4 to 9.5, an ordinary frame 2.5, the bell 24.6 to
26.1, the close 24.8 to 25.3, midnight 26.1 to 27.5 and the first frame 140
to 153 µs; the pass 5.43 to 5.48 ms and the load 2.68 to 2.71, 0.1 ms longer
for the longer formulas; `518d414`'s ordinary frame 2.1 µs and its day's first
17.2 to 17.3. In Chrome, three runs at the owner's full screen as P8's: a
still tick 0.2 ms at p50 and 0.3 at p95; a half hour's patch 0.4 and 0.5; a
mark's 0.5 to 0.6 and 0.7; the bell's 0.4 to 0.6, the close's 0.5 to 0.6 and
midnight's 0.5 at p50; 96 or 97 of 1,440 frames patched; the main thread 0.76
to 0.77 ms a frame; at the dial's 120 a tick of two frames 0.4 ms at p50 and
0.6 to 0.7 at p95, the main thread 0.35 to 0.36 ms a frame. So the padding
costs a frame nothing measured. The page is the owner's to test a third time
before the commit.

Found with it: a person who drags a column's divider moves the cell's right
edge and not the texts, the host's width overriding the record's (the page's
`columnWidthCss`), so the padding holds at the record's 12 characters alone;
a number format in the record would hold at any width.

**The owner's third hand test.** On the page with its dollars flush right and
its probe copy, 2026-10-17: passed, no finding. Build's bytes the file's,
"same bytes (34722 characters)", the command trimming the last newline; the
Portfolio on the fifth Wednesday at 11:00, `0.99`, `4.95`, `24.84` and `99.53`
at their cells' right pad beside `+0.00`, `-0.01`, `-0.02` and `-0.06`, `=▼▼▼`;
and the probe copy's corner at frame 21,598, the manifest's rate throughout: a
still tick 0.2 ms at p50 and 0.3 at p95, at its longest 1.2; a half hour's
patch 0.4 and 0.6; a mark's 0.6 and 1.0; the bell's 0.6 and 1.0 over 21; the
close's 0.7 and 1.1 over 22; midnight's 0.5 and 0.7 over 30; 1,439 of 21,598
frames patched, one in fifteen; each within a tenth of the headless runs at
p50.

## `GUARD.1` — the Series 777: the opening screen and its controls tutorial, nine questions drawn without a repeat, a certificate, then the game

*Scoped 2026-10-17 at the owner's word, "Scope and implement GUARD.1 in Buffet
(Delta/Buffet): the Series 777, the game's opening screen and its controls
tutorial", to stop for the owner's approval before building. Minted already,
`COURSE.0`'s item 14 in `OMAKASE.md`, and taken now, before `PLATE.1`, so that
the first button's second writing is the exam's. The owner's brief with it:
nine scored questions drawn from the bank by the exam's own die, a seeded
start and a stride among the primes above 1,547, so that none repeats, shown
one at a time on an Exam view with four option cells; an answer a press, as
`BACK.1`'s button is, and every answer a key too, "keyboard-first, keys
scoped to each tab"; the score a count, and the exam impossible to fail; a
tenth question, ungraded, that sets the archetype; then a certificate, then
the game; and the contract's numbers standing, the firm's capital $5,000 and
$500 a correct answer, the personal cash $100. Read for it: the item's line
and its four Found notes; `COURSE.0`'s opening contract; `BD-2`, `BD-4`,
`BD-5` and `BD-7`; `BACK.2`'s line, which names the firm from the archetype
later, and `PLATE.1`'s and `MISE.6`'s notes on buttons and keys; in this file
`MENU.2`'s record (the bank, its row shown, the stride's finding and its
decision 4), `MENU.5`'s (the plain statement the certificate shows),
`BACK.1`'s (the button, the keys a view binds, the scripted session) and
`POT.10`'s (the calendar, the exam's stream at `Dice!H1:J3`, the dollars'
padding, its measurements and its traps); `BUFFET.md` §3, the draft texts;
and Alonzo's `SPEC.md` at `v0.1.5`, the tag the pin names, §3.3, §3.4, §3.8
and §5, with the host and the engine at that tag where the page is silent.
Each decision carries its recommendation, for the owner to approve, amend or
strike; the texts a player reads are drafts, for the owner to approve or
rewrite. The predictions were written before any number was taken; the
figures marked *derived* were computed in a scratch script over the
cartridge's own rows by the rule the hand will follow, at seed 791886 and at
every seed from 1 to 1,000, and none of them is a measurement of the
cartridge. Alonzo's and Frazaro's trees were read and never touched: Alonzo's
holds a peer's `ENGINE.25` uncommitted, and was read at its tag and its
`HEAD` alone.*

**What it stands on, read on the day** (`alonzo` 0.1.5 and `vla-lang` 0.9.4
as the registry holds them; `SPEC.md` and the page at `v0.1.5`; Buffet at
`b82793c`, two ahead of its remote with `BACK.1`'s `518d414`, its tree clean).
- *The cartridge.* `(spec 4)`, `(rate 30)`, 569 rows, 201 formula cells in 86
  formula rows; the cartridge its source expanded; 56 tests green, the
  crate's 22 and the cartridge's 34, and the four checks with their controls;
  CI's last run 38939138517 at `ec069e8`, `BACK.1`'s and `POT.10`'s P5 Linux
  cells waiting for the push. The game opens on the Portfolio, the first
  visible sheet, with no Camera and the Screen hidden (`FRONT.1`), and the
  test of it says the Camera is this dish's to bring.
- *The exam's stream* (`POT.1`'s decision 8, moved by `POT.10`'s decision 8).
  `Dice!I1`, `=MOD(Clock!$B$3*$F$4,2147483647)`, the seed jumped half the
  generator's period, a function of the seed alone; its two draws, `I2` and
  `I3`, and their uniforms, `J2` and `J3`; read by nothing.
- *The bank* (`MENU.2`, as amended at its close). The Questions sheet, hidden,
  twenty rows of eight fields in `A2:H21`, the concept, the question, the
  four options, the right one's letter and the why, the right letters five to
  each; beside the table `K1` the count, 20; `K2` the row shown, a value, 1,
  the source's (where the brief says `M2`: the approver's strike moved it to
  `L2` and the vintage's to `K2`, both on 2026-10-15); and `K3` to `K10` a
  picker a field. Its longest question 130 characters and its longest option
  92 (*derived*). The ungraded question is no row of it: its options belong
  on the Firms sheet, each beside the firm it picks, and its prompt on the
  Exam view (its decision 4).
- *The plain statement* (`MENU.5`). The Caveats sheet's row shown, `G2`, a
  value, 1, so that `G3` to `G5` hold its first row's first three fields:
  "You. It knows nothing of your goals, your means or your timeline.";
  "Advice starts from those three, from someone licensed to give it. This
  game is not financial advice."; and the joke, "Any resemblance to financial
  advice, living or dead, is purely coincidental." The record has the
  certificate read `G4:G5`.
- *The button and the key* (`BACK.1`). `Hustle!B9`,
  `=IF(AND(Input!$B$19=$B$3,Input!$B$11=$B$4,Input!$B$12=$B$5,Input!$B$13=1,Input.last!$B$13=0),1,0)`,
  the button's place three values a layout moves; P on the `a` button, the
  Keys sheet's `B5` a formula of the sheet shown,
  `=IF(Input!$B$19=Hustle!$B$3,"KeyP","")`, the other nine buttons free; and
  the scripted session writing the Input rows as the host does, the rows that
  changed alone, reading the Keys sheet after every step, a key's binding
  lagging the tab by a frame.
- *The calendar* (`POT.10`). `Day!B4`, `=(Clock!B1-1)*B1+B9`, the time in
  minutes, `B9` the first frame's, 570, Monday's opening bell; `B19` open; the
  views' bell, four lines over the Day sheet; `BD-7` held by a scan of the
  formula rows, that no sheet but the Day reads `Clock!B1`, `Day!B4` or its
  twin, and by the census's `(every-frame 0)`. Every long run starts the
  market at the first frame; each test alone, at its last timing: the hour's
  year 4.17 s, the hundred years at a frame a day 3.90, the ten days at every
  frame 2.57, the year at difficulty 0 1.74, the eight hundred days 0.40, the
  hustle's scripted session 0.14; the cartridge's suite 4.0 to 4.6 s on eight
  threads, 6.2 s today, unpinned.
- *The page's version 4* (`SPEC.md` at `v0.1.5`). §3.8: the Camera's third
  row is the sheet the game shows; at the load it names the sheet the
  cartridge opens on, "a value or a formula's `value` row", else the first
  visible sheet; after a step, a change of its value that names a sheet is
  shown once at the frame's end, as a tab clicked is; between its changes the
  person's tabs win; and its own example is this dish's: "A game's exam that
  ends on its Portfolio is `=IF(Exam!B1=1,"Portfolio","Exam")`, its one change
  shown once." The engine takes a Camera of three rows at version 4, `A1:B3`
  (`alonzo` 0.1.5's `cartridge.rs`); the host reads its `B1` and `B2` as the
  Screen's window and says once in its line when either is not a whole
  number from 1 (`host.js`, `_cameraCell`); the load takes a formula's `value`
  row, and the host reads it as the cell's value at the load (`sheetCells`).
  §3.4: the host reads the Keys sheet after each step, so a cartridge "may
  even rebind keys by formula, by phase of play". §3.3: `B19` the sheet
  shown, `B11` and `B12` the pointer's cell on it.
- *The keys at the pin* (`host.js`, `viewport.js` and the template at
  `v0.1.5`). A key the Keys sheet binds is the game's while the focus is in
  the grid, taken before the viewport sees it; the viewport moves by the
  arrows, Page Up and Down, Home and End, opens a cell by Enter, and takes no
  letter; the page binds no letter of its own. A pick leaves the focus on the
  picker (`BACK.1`'s finding), where a typed "a" picks "An empty sheet" and
  loads it; mended in Alonzo's `ENGINE.23` (`dc32bbc`), unreleased since
  `v0.1.5`.
- *Alonzo's `ENGINE.25`*, a cartridge's own commands: approved by the owner
  2026-10-17, all nine decisions, and under construction there, its Commands
  sheet and the Input sheet's twentieth row a `(spec 5)` cartridge's alone;
  "Buffet takes it after a release, its pin the owner's to move" (its
  roadmap line).
- *The dialect* (`vla-lang` 0.9.4's `calc/library.rs`). `IF` computes the
  branch taken and `CHOOSE` the value chosen, and nothing else, so a guard
  shields a `CHOOSE` of 0; `AND` and `OR` compute every argument. `MOD` of a
  draw, below 2^31, is exact.
- *The first game's exam* (*derived*, seed 791886, the rule of decision 4).
  The start 8 and the stride 1,549, which is 9 modulo 20, so the nine rows
  are 9, 18, 7, 16, 5, 14, 3, 12 and 1: Liquidity, Time Value of Money,
  Bonds, Central Banks, Risk and Return, Sunk Costs, Inflation, Moral Hazard
  and Opportunity Cost, their right letters D, C, D, D, A, A, C, B and C. The
  first question is "Which is the most liquid?", its right option D, "cash".
- *A thousand seeds* (*derived*, seeds 1 to 1,000). No exam repeats a
  question. 160 exams are distinct, the bank's twenty starts by the eight
  strides coprime to 20, all eight reached by the first ten primes above
  1,547, 1,549, 1,553, 1,559, 1,567, 1,571, 1,579, 1,583, 1,597, 1,601 and
  1,607, which are 9, 13, 19, 7, 11, 19, 3, 17, 1 and 7 modulo 20 (the first
  eight miss 1, and meet 140). Each question is drawn 442 to 458 times of the
  9,000 and stands first 47 to 53 times of the thousand; the right letters
  are met 2,254, 2,250, 2,245 and 2,251 times, A to D; and no exam's nine hold
  more than five of one letter. Every bank a picker holds, 9 to 1,547 rows,
  with each of the ten strides: nine rows and no repeat.

**The decisions.**

1. **The market's clock during the exam: the bell waits for the certificate's
   button (rec).** `POT.10`'s finding stands: the market runs from the first
   frame, an hour a second, so an exam of two minutes passes five days, the
   first week's bells and the hustle's first prospectus with them. So the
   calendar stands at its first frame's time, Monday's opening bell, from the
   first frame until the game begins, and the bell rings then: not at the
   tenth answer, which would pass the certificate's reading in market hours,
   twenty seconds being twenty hours, but at the certificate's own button, the
   exam's end as the player meets it. While it waits the market is closed and
   still: the time 9:30 on day 0, the session day 0's at its mark 0, every
   price its start (`STATION.1`'s opening), and `Day!B19` 0, so that
   `PLATE.1`'s orders, filled only while the last frame showed open
   (`POT.10`'s decision 13), are refused through the exam with no rule of
   their own; the dice roll at the first frame as now; and the views' bell
   says so in a fifth line, "The market opens when you leave the exam room."
   The brief's question, the calendar starting at the exam's end, answered
   yes, by the Day sheet itself, as decision 2 draws it. *The alternatives:*
   the bell at the tenth answer; the market from the first frame, as `POT.10`
   left it; the market open and still through the exam, `Day!B19` and the
   bell's four lines untouched, an order then filling at the starts while the
   exam is up.

2. **The Day sheet counts the frames the bell waited; `BD-7` holds, a clause
   added (rec).** Two cells after the closing mark, a block of their own, and
   two formulas rewritten:

   | | A | B |
   |---|---|---|
   | 4 | time, in minutes | `=(Clock!B1-1-B26)*B1+B9`, where `=(Clock!B1-1)*B1+B9` stood |
   | 19 | open | `=IF(AND(B25=0,B17<B12,B15>=B10,B15<B11),1,0)`, the wait's test first |
   | 25 | the exam holds the bell | `=IF(Proctor!$B$41=1,0,1)` |
   | 26 | frames the bell waited | `=Day.last!B26+Day.last!B25`, the kit's `count` |

   `B25` is 1 until the Proctor's begun (decision 6) and 0 from it for good;
   `B26` counts the frames before this one that it held, its twin reading 0
   at the first frame. So a frame of the exam stands at 570 minutes, the
   game's first frame after it at 570 too, and every frame after that two
   minutes on; and a waiver before the first frame (decision 12) leaves `B26`
   at 0 and the calendar exactly as it is today. `B9` keeps its value and its
   meaning, the market's first frame's time. The brief's `Day!B9` as a formula
   of the Exam sheet would have the frames counted somewhere other than the
   calendar, an exam sheet reading its own twin at every frame being the
   frame by another name, and the first frame's time a negative number of
   minutes. `BD-7` holds as written: the Day sheet alone reads the frame and
   counts frames; the scan's list gains `Day!B26` and its twin, which change
   at every frame of the exam; and the census keeps `(every-frame 0)`, `B26`
   reading twins alone. A frame of the exam computes three of the calendar's
   cells, `B4`, `B5` and `B26`, `B4`'s value standing so that `B14` and
   everything under it wait; from the frame after the bell `B26` is still,
   and an ordinary frame computes the three it does today. The rule gains a
   dated clause saying so (decision 20). *The alternative:* `B9` the formula,
   as the brief frames it.

3. **The exam's state: a model sheet of its own, Proctor, after the Hustle
   (rec).** A view is read by nothing (`FRONT.1`'s decision 10), and the Day
   sheet, the Camera, the Keys and later `PLATE.2` and `BACK.2` read what the
   exam decides, so its books stand on a model sheet, visible (`AD-6`), a
   label in A beside each value as the model's sheets hold them: the
   proctor's, who administers every exam and is never promoted (`BUFFET.md`
   §2), so that `GUARD.2`'s exams take the same sheet over other banks. Its
   rows:

   | | A | B | C | D | E | F | G |
   |---|---|---|---|---|---|---|---|
   | 1 | the firm's capital before the exam, in cents | `500000` | | | | | |
   | 2 | the capital a correct answer adds, in cents | `50000` | | | | | |
   | 3 | waived: 1 skips the exam | `0` | | | | | |
   | 4 | a waiver's correct answers | `5` | | | | | |
   | 5 | a waiver's style | `1` | | | | | |
   | 6 | where the certificate leads | `Portfolio` | | | | | |
   | 8 | the bank's questions | `=Questions!$K$1` | | | | | |
   | 9 | the start, from 0 | `=MOD(Dice!$I$2,B8)` | | | | | |
   | 10 | the stride, a prime above 1,547 | `=CHOOSE(MOD(Dice!$I$3,10)+1,1549,1553,1559,1567,1571,1579,1583,1597,1601,1607)` | | | | | |
   | 12 | the draw: question | its row in the bank | | | | | |
   | 13 to 21 | `1` to `9` | `=MOD($B$9+(A13-1)*$B$10,$B$8)+1`, a fill | | | | | |
   | 23 | the buttons | view | column | row | key | clicked | keyed |
   | 24 to 27 | `A` to `D` | `Exam` | `1` | `7` to `10` | `KeyA` to `KeyD` | `clicked`, a fill (decision 8) | the kit's `pressed` over `Input!B6` to `B9`, a fill |
   | 28 | open for business | `Certificate` | `1` | `11` | `KeyO` | the same fill | the same fill, `Input!B10` |
   | 30 | the option pressed | `=IF(OR(F24=1,G24=1),1,IF(OR(F25=1,G25=1),2,IF(OR(F26=1,G26=1),3,IF(OR(F27=1,G27=1),4,0))))` | | | | | |
   | 31 | answered | `=IF(AND(B30>0,Proctor.last!B33=1),1,0)` | | | | | |
   | 32 | answers | `=Proctor.last!B32+B31`, the kit's `count` | | | | | |
   | 33 | a question open | `=IF(AND(B32<10,$B$3=0),1,0)` | | | | | |
   | 34 | the row shown | `=CHOOSE(MIN(B32+1,9),B13,B14,B15,B16,B17,B18,B19,B20,B21)` | | | | | |
   | 35 | the right option | `=IF(Questions!$K$9="A",1,IF(Questions!$K$9="B",2,IF(Questions!$K$9="C",3,IF(Questions!$K$9="D",4,0))))` | | | | | |
   | 36 | correct | `=IF(AND(B31=1,Proctor.last!B32<9,B30=Proctor.last!B35),1,0)` | | | | | |
   | 37 | correct so far | `=Proctor.last!B37+B36`, the kit's `count` | | | | | |
   | 38 | the score | `=IF($B$3=1,$B$4,B37)` | | | | | |
   | 39 | the firm's capital, in cents | `=$B$1+$B$2*B38` | | | | | |
   | 40 | the style | `=IF($B$3=1,$B$5,IF(AND(B31=1,Proctor.last!B32=9),B30,Proctor.last!B40))` | | | | | |
   | 41 | begun | `=IF(OR($B$3=1,AND(OR(F28=1,G28=1),Proctor.last!B32=10)),1,Proctor.last!B41)`, the kit's `hold` | | | | | |

   Thirty-four formula cells in eighteen formula rows. The draw's three cells
   and its nine compute at the first frame and never again; the buttons' ten
   follow the pointer and the keys, each 0 between presses (`SPEC.md` §2's
   rule 3); and the twelve workings move at a press alone. The bank is told
   the question to show through its own row shown, `Questions!K2`
   `=Proctor!$B$34`, where the source wrote 1 (`MENU.2`'s decision 5), so the
   pickers stay its own and the views read them. The capital is whole cents,
   the contract's two numbers values beside it. *The alternatives:* the state
   on the Questions sheet, which holds words alone; the name `Licensing` or
   `Registration`, a firm's records of its licenses, which `GUARD.2`'s
   `Licenses` sheet would stand beside confusingly.

4. **The draw: a start and a stride among ten primes, nine rows a table
   (rec).** The start is the exam's first draw modulo the bank's size, from 0,
   and the stride one of the first ten primes above 1,547 by its second draw,
   so that it is coprime to every bank a picker holds (`MENU.2`'s finding);
   the question `k`'s row is `MOD(start+(k-1)*stride,n)+1`, the nine rows a
   fill, which repeat no row while `n` is nine or more, since it shares no
   factor with the stride. Ten primes reach the eight strides coprime to 20,
   so the first thousand seeds meet 160 exams, each question first in about
   one of twenty and somewhere in nine of twenty (*derived*, above). The
   primes stand in the formula, where a reader of its bar sees them and the
   hand's test proves them prime; the bank's size is its own count, so a row
   added joins the draw with no formula edited (`MENU.2`'s decision 4) and
   moves every seed's exam (Found). The table is the draw whole from the
   first frame, which a player may open (`AD-6`; `BD-3`, the exam an open
   book as the market is, and one that cannot be failed), and which the
   oracle reads at every seed of a thousand after one step each. *The
   alternatives:* one prime, twenty exams in all; the primes as ten value
   cells; the draw computed for the question shown alone, one cell where
   nine, the oracle then walking ten answers at every seed.
   *(Amended at the owner's hand test, 2026-10-17, the owner's words: "The
   exam should have 11 question (10 of substance, the eleventh question being
   firm style)". Ten rows, `B12:B21`, the table's header a row higher, at
   `A11`, so that the buttons and the workings keep their addresses; ten rows
   repeat none while the bank holds ten or more, and the first game's tenth
   is the bank's row 10.)*

5. **An answer: a press on an option or its key, judged by the question the
   last frame showed (rec).** An answer is a click on one of the Exam view's
   four option cells, `BACK.1`'s button four times, or one of four keys
   (decision 7), each a cell that holds 0 between presses; the option pressed
   is the first by letter when two are pressed in one frame, so that a frame
   answers once. It is taken only while the last frame showed a question
   open, `Proctor.last!B33`, which also refuses a press in the first frame,
   whose last showed formulas' texts; and it is judged by the question the
   person saw, the last frame's right option, since the answer moves this
   frame's question, as a claim is judged by the hustle the person saw and an
   order fills at the price shown. A right answer counts while the question
   shown was one of the nine; the tenth's sets the style. Nothing is said of
   right or wrong on screen at this dish: the next question is the answer's
   echo, the proctor's remarks wait for an exam that answers on screen
   (`MENU.2`'s decision 9), and the whys for the concept cards (`FRONT.5`).
   *The alternatives:* this frame's question, which reads the answer's own
   effect and so the question after it; the last option pressed of two.
   *(Ten and the eleventh since the owner's word at the hand test, decision
   4's amendment.)*

6. **The score, the capital and the style (rec).** The score is the right
   answers of the nine, a count; the firm's capital `500000+50000*score`
   cents, $5,000 to $9,500, the contract's; the style the tenth answer, 1 to
   4, 0 until it is given, which `BACK.2` turns into the firm's name; begun
   once the certificate's button is pressed while the last frame showed all
   ten answered, or at a waiver. The personal cash, $100, stays `PLATE.2`'s,
   as `BACK.1`'s decision 8 left it, no pocket entering before its dish.
   "Style", a fund's investment style, is the game's word for the docs'
   archetype. *(Amended at the owner's hand test, 2026-10-17: the right
   answers of the ten, $5,000 to $10,000, the eleventh the style, begun after
   all eleven, decision 4's amendment. And the personal cash entered after
   all, at the owner's word the same day, "Portfolio should list "Cash on
   hand:" so the player can see their available cash inside their
   portfolio", both pockets at the owner's choice: the Cash sheet, after the
   Proctor, its personal cash at the start `B1`, $100, a value, the firm's
   cash `B3`, the exam's capital, and the personal `B4`, the start and the
   hustle's fees once paid; the Portfolio shows the two under its stocks,
   "Cash on hand ($)", a line a pocket. The pockets' own books, a switch and
   the orders stay `PLATE.1`'s and `PLATE.2`'s.)*

7. **The keys: the Keys sheet's free buttons, each bound on its view while
   its act can happen (rec).** A, B, C and D on the `b`, `x`, `y` and `start`
   buttons while the Exam shows and a question is open; O on `select` while
   the Certificate shows and waits for its button; Gigs' P on `a` as it
   stands. Each code a formula of the sheet shown, the owner's word of
   2026-10-16, as P's is:

   | | A | B |
   |---|---|---|
   | 6 to 9 | `b`, `x`, `y`, `start` | `=IF(AND(Input!$B$19=Proctor!B24,Proctor!$B$33=1),Proctor!E24,"")`, a fill |
   | 10 | `select` | `=IF(AND(Input!$B$19=Proctor!B28,Proctor!$B$32=10,Proctor!$B$41=0),Proctor!E28,"")` |

   Each button stands for one act on one view, so no reader of a press asks
   which view bound it, and Gigs' press is untouched; six of the ten are then
   the game's, the arrows' four left for acts that move. A key bound only
   while its act can happen leaves a later page's note of the game's keys
   (`ENGINE.24`) naming only keys that act. The texts beside the buttons
   name the keys, as "or press P" does, and a test holds each to its code.
   Not `ENGINE.25`'s commands: approved and under construction in Alonzo, a
   `(spec 5)` cartridge's, and unreleased; when a release carries them and
   the pin moves, the exam's five keys may become the cartridge's own
   commands, an item's of that move (Found). *The alternatives:* `ENGINE.25`
   first, the exam waiting for a release and the cartridge for version 5; the
   digits 1 to 4, one key on every layout and no picker to meet (Found), where
   the options are lettered; the key rows `B17` and `B18`, which the viewport
   also sees and the browser repeats (`BACK.1`'s decision 9).

8. **The button's second writing: `clicked` enters `kitchen.vla` (rec).**
   `BACK.1`'s click is written a second time, five times over, by the exam's
   options and the certificate's button, so by the ratchet it enters the
   library: a verb of one formula row over a cell or a fill, taking the
   sheet, the range and three pieces, the references to the button's view,
   column and row as the range's first cell reads them, and writing
   `=IF(AND(Input!$B$19=<view>,Input!$B$11=<column>,Input!$B$12=<row>,Input!$B$13=1,Input.last!$B$13=0),1,0)`.
   The Hustle's `B9` is rewritten over it, `(clicked "B9" "$B$3" "$B$4"
   "$B$5")`, expanding to its bytes unchanged, the hoist's proof; the
   Proctor's five are one fill over the buttons' table, `(clicked "F24:F28"
   "B24" "C24" "D24")`, a row a button. Its name is the Hustle's own label for
   the cell. `MISE.6`'s hoist ledger gains its row: *`clicked`, a left button
   pressed on a cell of a view; from `BACK.1`'s Gigs and `GUARD.1`'s exam and
   certificate; into `src/kitchen.vla`; course 0; the Hustle's row byte for
   byte.* A button's click and key together, `PLATE.1`'s pair, waits for its
   own second writing there. *The alternative:* the click written by hand
   again and hoisted with `PLATE.1`'s twenty-four.

9. **The views: the Exam and the Certificate, each a sheet of its own, first
   among the tabs, the game opening on the Exam (rec).** Each opens on its
   date and the bell's line and closes on the fine print, as every view does,
   those three one shape each over four sheets.

   The Exam, its column A 100 characters wide:

   | | A |
   |---|---|
   | 1 | the date (`as-of`) |
   | 2 | the bell's line (`bell`) |
   | 3 | `=IF(Proctor!$B$33=1,IF(Proctor!$B$32<9,"The Series 777, question "&(Proctor!$B$32+1)&" of 10. It cannot be failed.","The Series 777, question 10 of 10, ungraded: it sets your firm's style."),IF(Proctor!$B$3=1,"The Series 777: waived.","Pencils down. Your certificate is on its sheet."))` |
   | 5 | the question: `=IF(Proctor!$B$33=0,"",IF(Proctor!$B$32<9,Questions!$K$4,"Which best describes you?"))` |
   | 7 to 10 | an option each: `=IF(Proctor!$B$33=0,"",IF(Proctor!$B$32<9,"[ "&Proctor!A24&" ]  "&Questions!K5,"[ "&Proctor!A24&" ]  "&Firms!A2))`, a fill |
   | 12 | `=IF(Proctor!$B$33=1,"Click an answer, or press its letter: A, B, C or D.","")` |
   | 14 | the fine print (`fine-print`) |

   An option cell is its button whole: every option with its letter, 99
   characters at the longest (*derived*), stands inside the column, which a
   test holds over every question of the bank and the four styles, so that a
   click on any character of it answers (`BACK.1`'s decision 3: a click on a
   text's spill is the next cell's); the question, 130 characters at the
   longest, spills past it into empty cells and is no button. The tenth's
   prompt is the view's (`MENU.2`'s decision 4), its options the Firms
   sheet's rows. Ten formula cells.

   The Certificate, its column A 21 characters wide, as wide as its button's
   text, and B 12, the width the dollars are padded for:

   | | A | B |
   |---|---|---|
   | 1 | the date | |
   | 2 | the bell's line | |
   | 3 | `The Series 777: a certificate of completion` | |
   | 5 | `=IF(Proctor!$B$33=1,"Pending: the proctor certifies after the tenth question.",IF(Proctor!$B$3=1,"Waived: the regulator assumes you would have passed.","The bearer has sat the Series 777 and passed, as every candidate does."))` | |
   | 7 | `Correct answers` | `=Proctor!$B$38&" of 9"` |
   | 8 | `Firm capital ($)` | the capital's dollars, `dollars` over `Proctor!B39` |
   | 9 | `Style` | `=IF(Proctor!$B$40=0,"asked last",Firms!$D$3)` |
   | 11 | `[ Open for business ]` | `or press O` |
   | 13 to 15 | the plain statement whole: `=Caveats!G3`, a fill over `G3:G5` | |
   | 17 | `Signed for the regulator by the proctor, whose signature is, on inspection, a doodle of a bull.` | |
   | 19 | the fine print | |

   The plain statement's three fields, where the record had two, since the
   second, "Advice starts from those three", reads the first's three (Found).
   Ten formula cells. The tab order, each sheet's state and the box: the
   Exam, the Certificate, the Portfolio and Gigs, the views; the Day, Dice,
   Market, Stocks, Hustle and Proctor, the model; the Disclosures, Questions
   and Firms hidden, and the Caveats; then the Keys, Camera, Clock, File and
   Input: sixteen tabs in the strip where twelve, "hidden sheets (24)" where
   18, none numbered (`BD-2`). The game opens on the Exam, the first visible
   sheet, the Camera's third row a formula with no value at the load (§3.1).
   The cost of the five sheets, read before it is paid: every shape is
   renumbered, two views standing before the Portfolio; and every record the
   oracle reads lists forty sheet rows where thirty, the long tests' price
   (P5), the Certificate's own share of it about a hundredth of each. *The
   alternatives:* the certificate the Exam's lower half, a sheet and its twin
   fewer and every cell of the view two-faced; the two views after Gigs, the
   opening stated by a `(value "Camera" "B3" "Exam")` row after the Camera's
   formula, which the load and the host both take, so that a later Overview
   (`FRONT.3`) stands first with no reordering; the Certificate named
   License, beside `GUARD.2`'s Licenses.
   *(Amended at the owner's hand test, 2026-10-17. The Exam's `A3` in the
   owner's words, edited into the source: "The Series 777, question N of 11.
   Correct answers earn starting capital.", where it had read "It cannot be
   failed.", eleven questions since decision 4's amendment. The
   Certificate's plain statement under a heading, the owner's words: "The
   caveat should have a "Caveat:" cell above it so the caveat doesn't read
   like a random "You. ... statement" hanging on the certificate without
   context": `A13` `Caveat:`, the statement `A14:A16`, the signature `A18`
   and the fine print `A20`; its `B7` of ten. The buttons in Lovelace
   Lavender, the owner's words: "Buttons like [ Open for business ] should be
   highlighted with lavender background and white text": format 1, a fill of
   `#A080E0`, on the Exam's four options, the Certificate's button and Gigs',
   and a look of the empty text on the Exam drawing an option emptied after
   the exam in the default look; the record holds no color for a text
   (`vla-lang` 0.9.4), so the text stays the grid's dark, 5.3 to 1 against
   the lavender, its white asked of Frazaro and Alonzo (`MISE.6`; Found
   below). The views' dollars with their thousands grouped by commas, the
   owner's word: "Thousands separators need to be implemented": the
   library's `dollars`, and the Portfolio's change its signed twin, now the
   library's `signed-dollars`, both over one walk, a comma four fifths of a
   space and taking one off the padding. And the Portfolio's cash on hand
   under its stocks, decision 6's amendment, the fine print moved to `A14`.
   The Cash sheet stands after the Proctor: seventeen tabs in the strip,
   "hidden sheets (25)".)*

10. **The Camera: the hand-over, Exam to Certificate to the game (rec).** A
    device sheet the game writes, visible after the Keys: the kit's `camera`
    for its first two rows, the Screen's window at `A1`, unused and hidden but
    two whole numbers for the host to read without a word; and its third row,
    `A3` `sheet` beside `B3`
    `=IF(Proctor!$B$41=1,Proctor!$B$6,IF(Proctor!$B$33=1,Proctor!$B$24,Proctor!$B$28))`.
    At the load it has no value, so the game opens on the first visible sheet,
    the Exam; at the first frame it names the Exam, a change the host shows on
    the sheet already shown; at the tenth answer the Certificate; at the
    certificate's button or a waiver, where the certificate leads. Three
    changes in a game, each shown once, and between them the person's tabs
    win, so a player who opens the Portfolio mid-exam sees the market at its
    bell, and is shown the Certificate when the tenth is answered. The views'
    names stay values of the model, `Proctor!B24`, `B28` and `B6`, so that a
    layout moves no formula. *The alternative:* `B3` naming its sheets as
    texts in the formula.

11. **Where the certificate leads: the Portfolio, a value (rec).** The game's
    opening before this dish and the view of the market the player has, until
    the Overview (`FRONT.3`), which writes `Overview` into `Proctor!B6` and
    moves no formula. *The alternatives:* Gigs, the first thing a player can do
    for money; nowhere, the bell rung and the player's tab their own.

12. **A second run's skip: a waiver, the Proctor's value, written by a
    person's edit, at the owner's word (rec).** `Proctor!B3` written 1, by the
    drawer's edit row, `(cell "Proctor" "B3" 1)`, at any frame: the game begins
    at the next step, the exam closed, the score the waiver's, 5 of 9 and
    $7,500, the middle of 0 to 9 rounded up (the open decisions' "the score
    then the median"), and the style the waiver's, the first, both values a
    person may edit; the certificate says the exam was waived, and the Camera
    goes where it leads. Written before the first frame, it opens the market
    at the first frame as today, which is how the oracle's long runs and a
    hand test of the market play. No key or button on the Exam at this dish:
    no run can tell it is a second, the engine keeping nothing between loads
    until a save (`ENGINE.5`), so a key on the opening screen would offer
    every first player a way past the controls tutorial (`BD-4`); the
    manifest's notes say how, which a returning player reads in the drawer
    before playing, and the act is a typed row, a keyboard's. *The
    alternatives:* a key and its button on the Exam, W for waive, on every
    run, a row of the Keys sheet and a click cell more; no waiver, every long
    run answering the exam first, some twenty frames each, and every hand
    test of the market starting with the exam. *(Amended at the hand test,
    2026-10-17: five is the middle of 0 to 10, $7,500 as before, the
    certificate saying 5 of 10.)*

13. **The Firms sheet: the four styles alone (rec).** `src/content/firms.vla`,
    the fourth content sheet, hidden after the Questions: one field, `Style`,
    40 characters wide, its four rows the draft's answers, "I read the
    prospectus", "I read the headlines", "I read the room" and "I feel"; its
    row shown the source's `D2`, `=MAX(Proctor!$B$40,1)`, the style chosen and
    the first before one is, so that its picker, `D3`, is the certificate's
    style. The Exam's tenth reads its rows, `A2:A5`, by reference. `BACK.2`
    adds the firm beside each, a field added to four rows, with its names and
    its flavor. The layout check's content pin rises from 3 to 4. *The
    alternative:* the firms' draft names now beside the styles, Prudent &
    Sons, Headline Capital, Read-the-Room Partners and Vibes LLC (`BUFFET.md`
    §3), a field read by nothing until `BACK.2`, which `MENU.2`'s decision 9
    would leave for its dish. *(The fourth style in the owner's words at the
    hand test, 2026-10-17, edited into `firms.vla`: "I feel the market", where
    the draft had "I feel". The Exam's eleventh reads the rows since decision
    4's amendment.)*

14. **The texts, drafts (rec).** The bell's fifth line, "The market opens when
    you leave the exam room.", with no joke but the room, being a line a
    player acts on; the Exam's status lines, its line of keys and the tenth's
    prompt; the Certificate's title, its three headlines, its labels, its
    button and the signature, `BUFFET.md` §3's "a doodle of a bull" kept. Every
    one is a fund manager's word or an examiner's (`BD-2`), the jokes the
    regulator's and the proctor's. `BUFFET.md` §3's "you have failed in a way
    the regulator has decided to call passing" is offered for the owner's
    taste as the passed headline's joke, where the draft says "as every
    candidate does". *(The owner's words at the hand test, 2026-10-17, edited
    into the files: the Exam's status line, "Correct answers earn starting
    capital.", where the draft had "It cannot be failed."; and the bank's
    first question, "Which asset is the most liquid?", where `MENU.2` had
    "Which is the most liquid?". The passed headline's draft stands.)*

15. **The caveats, drafts (rec).** A dish that brings a simplification brings
    its row (`BD-5`), and this one brings two, the first owed since `MENU.5`:

    | | Things this game gets wrong | In the real world | Management's response | Where |
    |---|---|---|---|---|
    | 15, new | The Series 777 cannot be failed, and nine questions license you to manage other people's money. | FINRA's qualification exams can be failed: the Series 7 asks 125 scored questions and passes at 72% (check). | Management passed the Series 777 on its first attempt, as everyone does. | `Proctor!B38` |
    | 16, new | The market waits at its opening bell until you leave the exam room. | The opening bell rings at 9:30 whether or not anyone has finished an exam. | Management asked the exchange to hold the bell. The exchange agreed, this once. | `Day!B26` |

    Row 15's claim is marked for `MENU.8`'s read. *(Row 15 reads "ten
    questions" since the owner's eleven at the hand test, 2026-10-17.)*

16. **The notes and the comments (rec).** The manifest's notes open on the
    exam: "... the game opens on the Series 777, the regulator's exam for a
    fund manager, ten questions answered by a click or a key, nine of them
    scored, each right answer $500 more of the firm's $5,000, and the last its
    style; then its certificate, and the Portfolio at Monday's opening bell
    ... A returning candidate may waive the exam, a 1 in Proctor!B3, at five
    correct answers." The back office's list gains the proctor's books. The
    source's opening comment lists `GUARD.1`, and its comment on the market's
    clock says the bell waits; `kitchen.vla`'s header gains `clicked`, and
    `bell`'s comment its fifth line.

17. **The oracle (rec).** In `oracle/src/buffet.rs` and
    `oracle/tests/cartridge.rs`:
    - *The hand's exam*, sharing no predicate with the cells: the primes above
      1,547 found by the hand's own `is_prime`; the draw walked a row at a
      time, each the last plus the stride's remainder, wrapping, where a cell
      computes its row from the start; the answers kept as a list of the
      options given beside the rows shown, the score counted by comparing an
      option's letter with the bank's letter, as text; the capital, the style
      and the begin as the contract states them. Its own test: every bank from
      9 to 1,547 rows with every stride, nine rows and no repeat; the ten
      strides prime and above 1,547; every score's capital.
    - *The hand's calendar waits*: a frame before the bell stands at the first
      frame's time, the session day 0's at its mark 0 and the market closed;
      the game's first frame stands there too, and every frame after it is
      two minutes on.
    - *The draw at every seed of a thousand*: the seed written into the Clock
      as a person's edit, 1 to 1,000 and the contract's, a step each; the
      start, the stride and the nine rows the hand's, none repeated, each a
      row of the bank.
    - *A scripted session*, every frame held: the Input rows written as the
      host writes them, the keys through the Keys sheet as the host reads it
      after every step; at every frame the Proctor's workings, the Exam, the
      Certificate, the Camera's sheet, the keys each view binds, and the Day's
      wait, the hand's. Its presses: one in the first frame, refused; a click
      answering right, and a key answering wrong; a click on the question,
      beside the options; a click at an option's address while the Portfolio
      shows, and a key there, unbound; a key held on the Portfolio as the Exam
      is shown under it, a press the frame it is bound; a click and a key on
      two options in one frame, the first answering once; the Portfolio
      mid-exam, every price its start and the time 9:30; the tenth by its key,
      the style; the Camera showing the Certificate, A to D unbound and O
      bound there; the certificate's button pressed before the tenth, refused,
      and after it, the game begun; then the market's first frames, the bell
      at the game's first frame and 9:32 at the next, held to the hand's
      calendar with its wait.
    - *The score to capital*: ten sessions, right answers 0 to 9 by key, the
      capital and the certificate's dollars the contract's; and a waiver
      written at each score, the same.
    - *A waiver*: before the first frame, the market from the first frame and
      the Camera at the Portfolio; mid-exam, the exam closed at the next step.
    - *The settings, the views and the keys*: the Proctor's settings and
      buttons the hand's; every option of the bank and every style inside the
      Exam's column; each button's text at its place; the keys named beside
      them, each bound; no code the viewport's, and no Alt; every column of
      dollars 12 characters, the Certificate's among them.
    - *The long runs*, each with a waiver before its first frame, so that its
      market and its hand stand as today; the exam's views held at the run's
      first frame and its last, their date, bell and fine print being the
      Portfolio's own shapes over four sheets, which the census shows.
    - *Kept*: the cartridge its source expanded, the Hustle's row byte for
      byte under the verb; `BD-7`'s scan, its list grown by `Day!B26` and its
      twin; the census's `(every-frame 0)`; nothing reads a view, `VIEWS`
      naming the Exam and the Certificate; the tab order; each content sheet
      its file, `CONTENT` naming the Firms; the Questions' first question,
      now the draw's first row; the describe golden blessed again and read in
      its diff with the shapes' numbers stripped.
    - *Mutants*, each written into the source or the library by one anchor and
      expanded before the suite runs, each failing the test written for it:
      the stride 1,560, a multiple of 20; the start 0; an answer judged by this
      frame's question; the tenth counted in the score; the bell not waiting,
      `B4`'s `-B26` lost; the bell rung at the tenth answer, `B25` reading the
      question open; an answer's key bound on every sheet; the option keys
      bound after the exam; the certificate's button beginning the game
      before the tenth; the capital a correct answer adds lost; the waiver's
      score ignored; the Camera never showing the Certificate; the click's
      edge lost in the verb, which fails the hustle and the exam alike; and
      the Exam's column 60 characters, an option past its edge.

18. **What it costs, measured (rec).** As `POT.10` took it. Natively: this
    tree's runner timing both cartridges, `b82793c`'s texts copied into its
    tree for the runs marked before, pinned to the performance cores and
    opted out of Windows' throttling, a fixed computation before and after:
    the pass, the load and the first frame in both; an ordinary frame, a half
    hour's turn, a mark, the bell, the close and midnight in both, this tree's
    waived; and in this tree an exam's still frame, a pointer's move over the
    options, an answer by click and by key, the tenth and the begin, each its
    write and its step apart. Each test timed alone, in this tree and in a
    `git archive` of `b82793c` built in a target of its own, before the
    suite's whole is read. In the browser, `POT.10`'s instruments copied into
    this session's scratchpad: the page built here, three runs in headless
    Chrome at the owner's full screen, every process opted out and pinned; an
    exam played by real clicks and keys through the DevTools protocol, each
    answer timed by `BACK.1`'s latency instrument from the event's own
    timestamp to the end of the tick that painted the next question, and to
    the next animation frame, at p50, p95 and the longest, against the
    contract's 100 ms at p95; the Camera's two hand-overs; the still ticks of
    an exam left alone; then a week of the market as `POT.10`'s P8. And the
    owner's own Chrome, the probe copy printing the same in a corner.

19. **The hand test, the owner's, on the page built here:** the strip and its
    box; the Exam at the first frame, its date, the bell's waiting line, the
    first question and its four options; a click in the sheet first (Found);
    answers by clicks and by keys, the question's number climbing; the
    Portfolio mid-exam, the market at its bell, every price its start; the
    tenth and its styles; the Certificate shown by itself, its score, its
    capital in dollars, its style, the plain statement and the signature; O
    or its button, the Portfolio at 9:30 and the market open, its first move
    at 10:00; the Proctor's books; a second load waived by the drawer's edit
    row, the Portfolio at once and the certificate saying so; Build's text
    compared with `src/buffet.vla` by a command given at the handover, since
    the page prints no "same bytes"; and the probe copy's corner after a few
    minutes.

20. **The stale places, rewritten at the build.** In `OMAKASE.md`:
    `GUARD.1`'s line, 🟡 from the approval, its text as the decisions leave it
    (it still says "a draw of nine scored and one ungraded"); the contract's
    exam, its waiver and the bell's wait, dated; `BD-7`'s dated clause, the
    frames the bell waited counted by the Day sheet alone; `BD-5`'s list, the
    exam's caveat taken; `BACK.2`'s line, the Firms sheet arrived with its
    styles and the certificate's style where its name will go; `PLATE.1`'s,
    the click hoisted at `GUARD.1` and its orders refused while the bell
    waits; `PLATE.2`'s, the firm's capital at `Proctor!B39`; `FRONT.3`'s, the
    certificate's lead at `Proctor!B6`; `MISE.6`'s hoist ledger, `clicked`,
    and its key chords' row, `ENGINE.25` approved and under construction; and
    the open decisions' row on the exam's skip, answered, dated. The source's
    opening comment, notes, sheets row and the Day's section, the Hustle's
    click, the Proctor's section, the Questions' row shown, the views'
    sections, the Keys' and the Camera's; `kitchen.vla`'s header, `clicked`
    and `bell`; `src/content/firms.vla`, and `src/content/README.md`'s fourth
    sheet; the caveats' file; `check_layout.ps1`'s pin, header and control;
    `oracle/src/buffet.rs`'s module comment, the exam and the calendar's wait;
    the module comments of `oracle/src/lib.rs` and `oracle/tests/cartridge.rs`;
    the describe golden; `BACK.1`'s and `POT.10`'s P5 cells, CI's Linux at the
    push; `Delta/CLAUDE.md`'s map, the exam's sheets, the waiver and the
    Camera among the edits that trip a test; the memory. Not this item: the
    firm's name (`BACK.2`), the pockets (`PLATE.2`), the plate and its orders
    (`PLATE.1`), the overview (`FRONT.3`), the proctor's remarks (`MENU.4`),
    the license ladder (`GUARD.2`), the looks (`FRONT.2`'s second slice), and
    the replay golden (the tasting).

**Found at the scoping**, measured or read, none minted:
- *The opening's first key.* At the pin a pick leaves the focus on the
  picker, where a typed "a" picks "An empty sheet" and loads it, the game
  gone (`BACK.1`'s finding, mended in Alonzo's `ENGINE.23`, `dc32bbc`,
  unreleased); a "b" names the game already picked, and "c", "d" and "o" name
  nothing. The exam's first answer by key, A, meets it until the pin moves
  past that release, so the hand test clicks the sheet first.
- *A key by its place* (`SPEC.md` §3.3). `KeyA` is the key where QWERTY's A
  stands: A on QWERTY, QWERTZ and Dvorak, Q on AZERTY, where the view says A.
- *The plain statement's second field reads its first.* "Advice starts from
  those three" names the goals, the means and the timeline of `G3`, and the
  record's `G4:G5` (`MENU.5`'s decision 4) shows it without them; so the
  certificate shows `G3:G5`.
- *A question added moves every exam.* The draw reads the bank's count, so a
  row added changes the start and the stride's remainder at every seed, the
  first game's exam among them: a replay's answers then meet other questions,
  and its golden moves with them, the tasting's to know.
- *The hustle through the exam.* Gigs never asks whether the market is open,
  so a claim made during the exam counts against day 0 at 9:30, its fee due
  at day 1's 9:30; harmless while the bell waits, where `PLATE.1`'s orders are
  refused by the market's own `B19`.
- *The Questions' row shown a formula.* A person's edit of `Questions!K2`
  replaces the formula for the run, as `MENU.2`'s tests write it to walk every
  row; the exam then shows the row written, and its judging follows it.
- *`ENGINE.25` at version 5.* When it is released and pinned, the exam's five
  keys may become `buffet.<command>` lines of the palette, the cartridge then
  `(spec 5)`; a move of the pin's own item.

**The predictions, before any number.** The numbers column is filled at the
build, P9 at the owner's hand test; P3 to P5 and P8 as decision 18 says.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 715 to 740 rows where 569 stand, about 725: the sheets row's three names more; the Day's four; the Proctor's 89, its labels, its values and its 34 formula cells; the Firms' 13; the Exam's 8 and the Certificate's 17; the Keys' seven more and the Camera's seven; the caveats' two rows of four fields; 255 to 275 formula cells where 201, about 266, in about 127 formula rows where 86; the diff the notes, the sheets row, the Day's `B4`, `B19` and its new block, the Hustle's `B9` unmoved, the Proctor after the Hustle, the Questions' `K2` a formula, the Firms after the Questions, the caveats' rows, the two views before the Portfolio, and the Keys' and the Camera's rows at the end | *Measurement*: the runner printed `(rows 725) (loads yes)`, as about 725 predicted, each sheet's rows as priced: the sheets row's three names, the Day's four, the Proctor's 89, the Firms' 13, the Exam's 8 and the Certificate's 17, the Keys' seven more, the Camera's seven and the caveats' eight; 266 formula cells in 127 formula rows where 201 and 86, as predicted; the diff 11 lines out and 167 in, the Hustle's `B9` unmoved. **Missed** in one: the diff's list left out the Portfolio's and Gigs' `A2`, the bell's formula, which its new line, the market waiting for the exam, moves on both views that stood; the caveats' count `G1` and its four pickers moved with the caveats' rows, as the list's "the caveats' rows" meant |
| P2 | the describe golden | the notes; `(device "Camera" "A1:B3" effect)` after the File's; `(budget 30 266 7980 2000000)`, give or take ten cells; the plan's shapes 112 to 125 where 83, every-frame and cycles 0; readers rows new for the Proctor, its twin and the Firms; the Questions' grown by the Exam's and the Proctor's reads, the Caveats' by the certificate's, the Day's by the wait's, the Dice's by the start and the stride, the Input's and its twin's by the clicks and the keys, and the Day's twin's by the count; the views' date, bell and fine print each one shape over four sheets, `(sheets 4)`; every other shape's row its own but for its number | *Measurement*: the notes; `(device "Camera" "A1:B3" effect)` after the File's; `(budget 30 266 7980 2000000)`, the cells exactly; `(plan (shapes 117) (texts 115) (formulas 266) (levels 31) (runs 15) (lanes 68) (alone 198) (every-frame 0) (cycles 0))`; readers rows new, `(readers "Proctor" 67 (most 8))`, `(readers "Proctor.last" 8)` and `(readers "Firms" 7)`; the Questions' 172 where 168, the Caveats' 69 where 60, the Day's 59 where 56, the Dice's 10 where 8, the Input's 13 where 6 and its twin's 4 where 2, the Day's twin's 5 where 3; the views' date, bell and fine print each one shape over four sheets, `(sheets 4)`; every shape renumbered, the two views standing before the model. **Missed** in one: the plan's levels 31 where 30, and every shape below the calendar a level deeper, so that its row moved by more than its number: the time to the minute, `Day!B4`, now reads the frames the bell waited, `B26`, a count over twins at level 0, which sets the time at level 1 where 0 and makes the calendar's chain, the longest, one longer, `Day!B14` at 2 where 1 and the Stocks' prices at 28 where 27 |
| P3 | the pass and the load | the pass 0.6 to 1.4 ms longer than `b82793c`'s 5.4 to 5.5; the load 0.4 to 0.9 ms longer than its 2.7 | *Measurement*, this tree's runner over both cartridges, `b82793c`'s four texts (the source, the library, the caveats, the cartridge) copied into its tree for the runs marked before, six runs alternated, 200 passes and 200 loads a run, every run pinned to the performance cores and opted out of Windows' throttling, a fixed computation before and after within 3.2%: the pass 6.51 to 6.61 ms at p50 where it took 5.53 to 5.59, about 1.0 longer, inside. **Missed** in one: the load 3.66 to 3.72 ms where 2.62 to 2.77, about 1.0 longer, over its 0.4 to 0.9, priced short of its rows: `POT.10`'s 69 rows more had cost 0.5 ms, and these are 156 |
| P4 | the step | waived, an ordinary frame within 0.3 µs of `b82793c`'s 2.5, three cells as now; a half hour's turn and a mark each 1 to 3 µs dearer, the date on four views where two; the bell, the close and midnight each 1 to 4 µs dearer, the bell's line on four; the first frame 160 to 195 µs where 140 to 153. In this tree: an exam's still frame 2.4 to 3.2 µs; a pointer's move over the options, its write 3.5 to 4.5 µs and its step 3.5 to 6; an answer, its write 2 to 3 µs and its step 15 to 40; the tenth 15 to 45; the begin 10 to 35 | *Measurement*, the same runs, ten days at two minutes a frame, the exam waived before the first frame, each frame filed by what the hand's calendar ticked beside it turned: an ordinary frame 2.8 to 2.9 µs where 2.5 to 2.6, inside at its edge; the first frame 184 to 189 µs where 140 to 151, inside. In this tree, over 200 exams each a fresh load, the Input rows written as the host writes them: a pointer's move, its write 3.8 to 3.9 µs and its step 3.6 to 3.7, inside; an answer's write 2.3 to 2.4, inside, and its step 15.5 to 15.7 by a click and 17.5 to 17.8 by a key, inside; the frame after an answer 8.3, and after the begin 4.7. An exam's still frame, timed first on frames 2 to 4, 5.0 to 5.1 µs, those frames still carrying the first frame's state through the twins; timed again from the fifth frame, twenty an exam, in three runs of this tree alone, 2.9 µs, inside. **Missed** in two: every turn of the calendar dearer than priced, a half hour 13.9 to 14.4 µs where 9.5 to 9.7, a mark 28.9 to 29.5 where 24.0 to 24.3, the bell 30.2 to 36.2 where 24.8 to 30.1, the close 31.6 to 32.6 where 25.0 to 25.8, and midnight 32.2 to 38.7 where 26.6 to 34.0, each 4.4 to 6.8 µs more where 1 to 4 were priced: a run with the Exam's and the Certificate's dates taken out read a half hour at 9.9 µs and a mark at 24.6, so each view's date, its text joined anew at every half hour, costs about 2 µs, where a quarter to three quarters of that was priced; and the tenth's step 13.8 µs and the begin's 7.1, under their 15 to 45 and 10 to 35 |
| P5 | the suite | 62 to 66 tests green where 56, and in CI on Linux; each long test 3 to 12% dearer alone, its records listing forty sheet rows where thirty and a waiver written before it; the draw at a thousand seeds 0.2 to 0.6 s, the scripted exam 0.05 to 0.25, the ten score sessions 0.05 to 0.3; the suite's whole 4.2 to 5.2 s on eight threads where 4.0 to 4.6 | *Measurement*, here: 66 green, the crate's 27 and the cartridge's 39, inside at its edge; the suite's whole on eight threads 4.45 to 4.73 s, the median 4.55, where `b82793c`'s took 4.12 to 4.67, the median 4.19, four rounds alternated, pinned and opted out, inside. Each test timed alone, two rounds alternated, one thread, the least of each: the ten days at every frame 2.81 s where 2.53, 11% dearer; the hour's year 4.38 where 4.07, 7%; the year at difficulty 0 1.83 where 1.74, 5%; the eight hundred days 0.44 where 0.40, 9%; the hustle's session 0.17 where 0.14, its script rewritten over the new verb and its keys read as a set; the scripted exam 0.053, inside at its edge, and the ten score sessions 0.146, inside; the waiver's two sessions 0.029, the settings 0.012 and the opening 0.012; the whole 14.77 s where 13.11. A record of one cell, counted in each tree, lists 40 sheet rows where 30, 1,196 characters where 921. **Missed** in two: the hundred years at a frame a day 4.38 s where 3.84, 14% dearer, over its 3 to 12%; and the draw at a thousand seeds 0.065 s, under its 0.2 to 0.6, one load and the seed written into the Clock a step. Linux at the push, run 39088078269 at `7bbeb59`, a commit on, the pin to `alonzo` 0.2.0 and `vla-lang` 0.10.0: the 67 green there, the crate's 27 and the cartridge's 40, the cartridge's in 6.07 s |
| P6 | the page | the cartridge's block 8,500 to 13,000 characters longer, the source's 10,000 to 18,000, the library's 1,000 to 2,000, the caveats' 600 to 1,100, and a block new, `library-content-firms`, 900 to 1,600: the page between 1,872,000 and 1,890,000 characters where 1,852,402 stands; Build writing the cartridge byte for byte | *Measurement*: 1,881,861 characters, 29,459 more than `b82793c`'s 1,852,402, inside: the cartridge's block 9,599 longer, the source's 15,936 and the library's 1,963, each inside, the last at its edge, and the new block, `library-content-firms`, 1,293 characters of text, inside, the five and the new block's tags adding to the page's growth exactly; Build writing the cartridge byte for byte, 44,322 characters, checked headlessly (P9). **Missed** in one: the caveats' block 571 longer, under its 600 to 1,100, its two rows shorter than priced |
| P7 | the checks | 4 checks and 4 controls green; one file added under `src/`, `src/content/firms.vla`; `check_layout.ps1`'s pin raised from 3 to 4, its header and its control's mutant past the pin moved to a fifth; no other check edited | *Measurement*: ALL GREEN, 4 checks and 4 controls; one file added under `src/`, `src/content/firms.vla`; `check_layout.ps1`'s pin raised from 3 to 4, its header with it, and its control's mutant past the pin a fifth sheet; no other check edited. **Missed** in one: the control wanted three edits more, which its first run failed for: its scratch copy the nine files where eight, the Firms file among them, its clean copy held to nine, and the Firms file put back after each mutant with the others, the mutant past the pin having failed for the file's absence where it should have failed for its count |
| P8 | an answer's cost in Chrome at full screen | a click to painted 13 to 20 ms at p50 and 30 to 34 at p95, and to the next frame 46 to 51 at p95, the contract's 100 ms met by half, as `BACK.1`'s claim is; a key within a frame of a click at p95; an answer's patch 0.5 to 0.9 ms, the question, its options and the status; each hand-over 1 to 3 ms, a sheet shown whole; an exam's still tick 0.2 ms at p50 and 0.3 at p95; then the market's week as `POT.10`'s, the main thread 0.76 to 0.86 ms a frame; the owner's Chrome within a tenth | *Measurement*, three runs in headless Chrome at 2,560 by 1,440 CSS pixels at 1.5, its eleven processes opted out and pinned, the clock within 1.8%; in each, four exams, the game picked again for each, a click on the question first, which answers nothing and gives the game its keys, then ten answers, a real click on an option and a key by turns at a random wait, then O: a click to painted 16.3 to 16.5 ms at p50 and 32.8 to 33.2 at p95, at its longest 33.1 to 33.4, inside; to the next frame, the answer shown, 32.4 to 32.7 at p50 and 49.0 to 49.1 at p95, at its longest 49.1 to 49.4, inside: the contract's 100 ms at p95 met by half. A key to painted 17.2 to 21.7 at p50 and 31.8 to 34.0 at p95, shown 47.7 to 49.5 at p95, within a frame of a click, inside; O, four a run, shown 32.2 to 47.8 at p95. An answer's patch 0.6 ms at p50, inside; a hand-over, the Certificate or the Portfolio shown whole, 2.4 to 2.6 at p50, inside; an exam's still tick 0.2 at p50, over 1,737 to 1,903 a run. Then the market's week, three runs, the exam waived by a person's edit once the host ran and each frame filed by the market's own time, the frames the bell waited read once: a still tick 0.2 ms at p50 and 0.3 at p95, as `POT.10`'s; a half hour's patch 0.4 and 0.5 to 0.7; a mark's 0.5 to 0.6 and 0.6 to 1.0; the bell's 0.5 to 0.6 and 0.5 to 0.7; midnight's 0.4 to 0.5 and 0.5 to 0.6; 96 or 97 of 1,440 frames patched; the main thread 0.79 to 0.80 ms a frame, inside; at the dial's 120, a week of about 5,045 frames, a two-frame tick 0.4 at p50 and 0.7 at p95, at its longest 1.3 to 1.9. **Missed** in five: an answer's patch 0.9 to 1.5 ms at p95, over its 0.9 in two runs of three; a hand-over 3.3 to 3.5 at p95, over its 3; an exam's still tick 0.4 at p95 where 0.3; the close's patch 0.6 to 0.7 at p50 and 0.7 to 1.3 at p95, two ticks a run, where `POT.10`'s read 0.5 and 0.5 to 0.7; and at the dial's 120 the main thread 0.37 to 0.38 ms a frame where `POT.10`'s 0.34 to 0.36. The owner's own Chrome, the probe copy's corner at the hand test over 6,233 frames, the exam answered by keys: ten keys painted 13.1 ms at p50 and 31.5 at p95, shown 29.2 and 45.8, at their longest 45.8; O shown at 27.7; a still tick 0.2 ms at p50 and 0.3 at p95, at its longest 0.6; a patch 0.4 and 0.6, at its longest 1.2, over 388 frames patched, the market's most of them; 5,833 frames still: headless's p95s within 2.5 ms and the owner's lower, the still tick's within a tenth. **Missed** in one more: a sheet shown, two ticks, 3.0 ms where headless read 2.4 to 2.6 at p50 |
| P9 | the hand test | the strip of sixteen and "hidden sheets (24)"; the Exam at Monday's 9:30, the bell's waiting line and "Which is the most liquid?"; the answers by click and by key, the Portfolio still at its starts mid-exam; the tenth, and the Certificate shown by itself, the capital the contract's; O to the Portfolio, the market open and moving at 10:00; a waiver's second load opening on the Portfolio; Build's bytes the file's, checked headlessly | *Headlessly*, over the page built here, before the owner's test, its steps driven through the DevTools protocol: the first frame on the Exam, "As of Monday, year 1, day 1, 9:30 (unaudited)" over "The market opens when you leave the exam room.", "The Series 777, question 1 of 10. It cannot be failed." and "Which is the most liquid?", its four options and A to D bound; Build's bytes the file's, 44,322 characters; the strip of sixteen, from the Exam to the Input sheet, and "hidden sheets (24)"; a click on the question answering nothing; nine answers by a click and a key in turns, each judged as the hand judges it, six right, the capital rising $500 at each right one; the Portfolio mid-exam at 9:30 on the starts, `====`, under the bell's waiting line; the tenth, "Which best describes you?" over the four styles; its answer by C showing the Certificate by itself, A to D unbound there and O bound: "The bearer has sat the Series 777 and passed, as every candidate does.", 6 of 9, 8000.00, "I read the room", the plain statement in its three fields and the proctor's signature; A there doing nothing; O beginning the game, the Portfolio shown under "The market is open until 16:00.", the first prices moving at 10:00, fifteen frames on, `==▲▲`; the Exam then "Pencils down. Your certificate is on its sheet."; and a second load waived after its first frame by an edit through the host, as the drawer's edit row writes one, opening on the Portfolio at 9:30, the bell having waited that one frame, its certificate "Waived: the regulator assumes you would have passed.", 5 of 9, 7500.00, "I read the prospectus". The owner's, on the page built here and the probe copy, 2026-10-17: passed, "Tests pass.", the exam sat, the certificate at 1 of 9 and 5500.00, the Proctor's books and the market's second week seen; with three texts reworded in the tree and seven changes asked, each in the owner's words, in the paragraph below; and the second the same day, over the changes: passed, "Tests pass; commit." |

*Scoped 2026-10-17. Approved by the owner the same day, whole:
"Recommendations approved; proceed with implementing GUARD.1 as scoped." So
every decision stands as recommended, the drafts among them until the owner
rewrites them, and the waiver is a person's edit of `Proctor!B3` with no key on
the opening screen (decision 12); the roadmap's line went to 🟡 the same
hour.*

**What the build found.** The exam held as the decisions drew it, and the
hand learned one rule from the cartridge: an answer is judged by what the
last frame showed, so a press in the first frame, whose last frame showed no
question, is refused, `Proctor.last!B33` reading 0 there, where the hand had
taken it; the hand now counts its frames and takes no answer in its first,
its own tests hold that refusal, and the scripted exam opens with such a
press. One test of the market from the first frame failed on its own load:
the disclosures' row added, its fine print turning with the day, never saw
its day come, the bell waiting for an exam no one sat; it now writes the
waiver with its other edits, as the long runs do through `cartridge()`. The
hand then met the cells at every frame of every run from the first: the
scripted exam's proctor's books, both views, the keys each view binds, the
Day sheet's wait and the market at its bell, through every press the script
draws, the tenth and the begin among them; the draw at every seed of a
thousand and the contract's, no row repeated; the score at each of its ten
values, $5,000 to $9,500; and a waiver before the first frame and in the
middle of an exam. The census found the calendar's chain a level longer, the
time to the minute reading the frames the bell waited (P2), and the timings
found each view's date about 2 µs at every half hour, four views joining it
where two did (P4); a frame timed after a load carries the first frame's
state through the twins to its fourth, so an exam's still frame was timed
again from its fifth, 2.9 µs. Fifteen mutants, each written into the source
or the library by one anchor and expanded before the suite ran, each failed
the test written for it: the stride a multiple of twenty and the start 0,
each caught by the draw; an answer judged by this frame's question; the
tenth counted in the score; the bell not waiting, and rung at the tenth
answer; an option's key bound on every sheet, and after the exam; the
certificate's button beginning the game before the tenth; the capital's $500
lost; the waiver's score ignored; the Camera never showing the Certificate;
the click's edge lost in `clicked`; the Exam's column 60 characters, an
option past its edge; and the bell's waiting line never shown. The click's
edge survived the exam's session at its first run, the script holding no
click across two frames, so a click held across three was added, and the
mutant failed it then, with the hustle's session. The layout check's control
failed its first run on the new file it did not yet copy and put back (P7),
and the offline check on a page built before the sources moved, until the
page was built again. Every prediction measured missed in part, each named
beside its number. In the page, driven headlessly (P9), every line the
scoping drew appeared where it was drawn, and in Chrome at the owner's full
screen an answer is painted 16.3 to 16.5 ms after its click at p50 and shown
49.0 to 49.1 ms after it at p95, the contract's 100 ms met by half (P8).
Found with it: a waiver written after the first frame lands at the next
step, the bell waiting the frames before it, one in the headless run, as
decision 12 has it; and each view joins its own date, so a fifth view
(`FRONT.3`'s Overview) adds about 2 µs to every half hour unless the date is
joined once on the model and each view reads it, a choice left to that dish.

**The owner's hand test, and what it changed.** On the page built here and
the probe copy, 2026-10-17: passed, "Tests pass.", the exam sat to 1 of 9 and
5500.00, the Proctor's books and the market's second week seen, and the probe
copy's corner sent (P8, P9). The owner reworded three texts in the tree, each
kept: the Exam's line, "Correct answers earn starting capital.", where it read
"It cannot be failed."; the fourth style, "I feel the market", where "I feel";
and the bank's first question, "Which asset is the most liquid?", where "Which
is the most liquid?". And asked seven things, each in the owner's words and
each settled the same day, one by the owner's answer to one question, "Both
pockets":

- "The exam should have 11 question (10 of substance, the eleventh question
  being firm style)": ten questions scored and an eleventh, the style
  (decisions 4, 5, 6, 9, 12 and 15, amended): the draw's ten rows, the score
  of ten and the capital $5,000 to $10,000, the Certificate's "of 10", a
  waiver's five the middle of 0 to 10, and the caveat's row 15, "ten
  questions".
- "Thousands separators need to be implemented": the views' dollars grouped
  by commas (decision 9 and `POT.10`'s decision 11, amended), a comma taking
  a space off the padding; the Portfolio's change the library's
  `signed-dollars`, the price's signed twin. The model's numbers, cents in
  number cells, wait for the record's number format (`MISE.6`).
- "Buttons like [ Open for business ] should be highlighted with lavender
  background and white text": the fill on every button, the Exam's options
  undrawn once emptied; the white waits for a text's color in the record
  (`MISE.6`), and takes the deep shade with it (Found).
- "Is it possible to change the cursor to a pointer when hovering over
  specific cells? That would help indicate that it's a button.": not from a
  cartridge at the pin, the viewport giving every cell one cursor and no row
  of the record saying a cell is a button; a candidate for Alonzo (`MISE.6`),
  its item the owner's to mint.
- "The caveat should have a "Caveat:" cell above it so the caveat doesn't read
  like a random "You. ... statement" hanging on the certificate without
  context": the heading, `Certificate!A13` (decision 9, amended).
- "Portfolio should list "Cash on hand:" so the player can see their available
  cash inside their portfolio": both pockets, the owner's choice when asked
  which: the Cash sheet after the Proctor, its personal cash at the start,
  $100, the firm's cash, the exam's capital, and the personal, the start and
  the hustle's fees once paid, shown under the stocks a line a pocket
  (decision 6 amended, `BACK.1`'s decision 8's alternative taken).
- "Proofreading a prospectus should earn $250, not $20, unless you think the
  math works out better at $25 per prospectus?": $250, my answer being that
  the math works better there. A day's proofreading at $250 is a plausible
  fee where $25 was a joke at the player's expense; four claims bring the
  personal cash to $1,100, enough for whole shares of every rung and a
  session's moves in dollars rather than cents, and still a fifth of the
  least firm's $5,000, so that the firm stays the game; the cap of four keeps
  the hustle the pocket's start and not its loop.

**What the changes cost.** The cartridge 748 rows where 725 and 271 formula
cells where 266: the Cash sheet's 8 rows, the Portfolio's 5, the draw's
tenth, the format, six styles and a look, and the Certificate's heading. The
describe golden blessed again, `(budget 30 271 8130 2000000)`, `(plan (shapes
120) ...)` and `(readers "Cash" 2)`, the Portfolio's two cash cells one shape.
67 tests green, the crate's 27 and the cartridge's 40, one new: every amount
spelled with its thousands grouped, each cell of `dollars` and
`signed-dollars` held to the hand's spelling beside every power of ten to
10^15 cents and at 2^53 cents less one, either way. 28 mutants, the fifteen
moved to their new anchors and thirteen new, each caught by the test written
for it; the eleventh counted in the score was caught only once the score test
answered the eleventh with the option that was right for the tenth, where it
had answered A, which no tenth's right option at the contract's seed was. The
page 1,895,219 characters; Build the file's bytes, 50,547 characters, checked
headlessly; `run_checks.ps1` ALL GREEN with its controls. Natively, one runner
over GUARD.1 as built and as amended, six runs alternated, pinned and opted
out, the clock within 3%: the pass 6.63 to 6.74 ms where 7.01 to 7.20; the load
3.65 to 3.78 ms where 4.90 to 5.05, 1.3 ms more; the first frame 185 to 190 µs
where 211 to 216; a half hour, a mark, the bell and the close as they stood,
a price under $1,000 spelled at its first comparison. The grouping was written
three ways to get there, each timed: nested, each group written in every
branch, its formulas 1,708 to 1,960 characters where 363, cost the load 2.2 ms;
flat, each group once behind its own test, 1.4 ms, but 3.5 µs at every mark,
every price testing four groups; and kept, the hybrid, a price under $1,000
at its first comparison and the groups flat above it, 1,126 to 1,303
characters. In Chrome at the owner's full screen, the probe copies rebuilt:
two runs of four exams of eleven answers, a click painted 16.2 to 16.6 ms at
p50 and 33.1 to 34.7 at p95, shown 48.9 to 49.1 at p95, a key shown 48.1 to
48.2 at p95; a still tick 0.2 and 0.4, an answer's patch 0.6 and 0.8 to 0.9, a
sheet shown 2.5 to 2.8 and 3.5 to 3.8; and two runs of the market's week, a
still tick 0.2 and 0.3, a mark's patch 0.6 and 0.8, the main thread 0.75 to
0.77 ms a frame and 0.35 at the dial's 120: as the build stood, the lavender
costing nothing measured. Headlessly, the hand test's steps over the page
built here: the Exam's "question 1 of 11. Correct answers earn starting
capital." over "Which asset is the most liquid?", its four options lavender;
the strip of seventeen, the Cash sheet among them, and "hidden sheets (25)";
ten answers and the eleventh, "I feel the market" among the styles; the
Certificate, 7 of 10, 8,500.00, "Caveat:" over the plain statement, its
button lavender; the Exam after the exam, its options undrawn; the
Portfolio's cash on hand, Firm 8,500.00 and Personal 100.00; Gigs' "$250
apiece" and a prospectus claimed, its fee paid a day later, 250.00, and the
personal cash 350.00; and a waiver, 5 of 10 and 7,500.00.

Found with them, none minted:
- *The white on the lavender.* White reads at 3.2 to 1 on Lovelace Lavender
  itself, under the 4.5 a text of normal size wants, and at 5.9 to 1 on the
  family's deep `#6B4FC8`; the grid's dark reads at 5.3 to 1 on the lavender.
  So when the record holds a text's color, the buttons take the deep shade
  with the white.
- *A dollar's text costs the load.* About a tenth of a millisecond a cell, by
  its formula's length, while the record holds no number format: a ledger of
  a hundred rows of two dollars each would cost a load about 20 ms
  (*derived*), so `PLATE.1`'s rows want the format, or their dollars as
  numbers until it comes.
- *The pointer over a button* waits on Alonzo: the viewport's cursor is the
  hit's kind, never the cell's (`MISE.6`).

**The owner's second hand test.** On the page built here and the probe copy,
the same day, over the changes: passed, "Tests pass; commit." The Certificate
at 2 of 10 and 6,000.00, "I feel the market" its style and the plain statement
under "Caveat:", its button lavender; the Portfolio mid-exam, its cash on hand
Firm 6,000.00 and Personal 100.00 under the starts, the bell waiting; and Gigs
on the second Saturday, "$250 apiece", its button lavender, one prospectus
proofread and its fee paid, 250.00, the next arriving in four days. The
owner's own Chrome, the probe copy's corner over 8,999 frames: eleven answers
by key, painted 12.1 ms at p50 and 31.9 at p95, shown 28.0 and 45.2; O shown
at 21.0; a still tick 0.2 ms at p50 and 0.3 at p95, at its longest 1.3; a
patch 0.4 and 0.6, at its longest 1.4, over 574 frames patched; and two sheets
shown, 3.4 ms: the headless runs' p95s or under them, as at the first.

## `PLATE.1` — the plate and the ledger: orders by click and by key, fills at the price shown

*Scoped and built 2026-10-18 at the owner's word, "Scope and implement PLATE.1
from Buffet's OMAKASE.md", in one sitting: each decision below stands as
recommended and was built, the owner's to approve, amend or strike at the
hand test, and the texts a player reads are drafts. Read for it: the item's
line in `OMAKASE.md` and its five notes; `COURSE.0`'s opening contract;
`BD-2`, `BD-5`, `BD-7`, `BD-8` and `BD-9`; `PLATE.2`'s line, whose switch this
dish leaves to it; in this file `MISE.3`'s probe 3 and its decision 5, the
ledger's spelling, `STATION.1`'s decision 11, the dividend, `FRONT.1`'s
rescoping, the market's view and the Portfolio's columns, `BACK.1`'s and
`GUARD.1`'s records, the button, `clicked`, the keys a view binds and the
scripted sessions, and `POT.10`'s decision 13, an order while the market is
closed; Alonzo's `SPEC.md` at `v0.2.0`, the tag the pin names, §3.3, §3.9,
§3.10 and §5, and `ENGINE.25`'s scoping in its `REARVIEW.md`, which names this
dish its client and leaves Buffet's move to version 5 to it; and the page at
that tag where the specification is silent. Alonzo's and Frazaro's trees were
read and never touched.*

**What it stands on, read on the day** (`alonzo` 0.2.0 and `vla-lang` 0.10.0;
Buffet at `7bbeb59`, even with its remote, this file carrying the P5 Linux
cells of `BACK.1`, `POT.10` and `GUARD.1` uncommitted).
- *The cartridge.* `(spec 4)`, 748 rows and 271 formula cells; 67 tests
  green, the crate's 27 and the cartridge's 40 in 5.0 s; the four checks
  green.
- *The price shown.* `Stocks!E5:E8`, the price now over the index now, which
  moves at the session's marks; its twin, `Stocks.last!E5:E8`, the price the
  last frame showed; `Day!B19`, the market open, and its twin, what the
  person saw.
- *The pockets.* `Cash!B3`, the firm's cash, the exam's capital,
  `Proctor!B39`; `Cash!B4`, your own, $100 and the hustle's fees once paid,
  `Hustle!B18`.
- *The dividend.* `Stocks!H5:H8`, 50 cents a share of the fourth rung all day
  on the year's 91st, 182nd, 273rd and 364th days, paid by nothing until this
  dish: "`PLATE.1` pays it once, at that day's first frame, to the shares a
  pocket held as the day before closed" (`STATION.1`'s decision 11).
- *The click and the keys.* `kitchen.vla`'s `clicked`, written twice, Gigs'
  button and the exam's five; the Keys sheet's ten buttons, five of them
  bound by formula on their own views, P on Gigs, A to D on the Exam and O on
  the Certificate.
- *Version 5* (`SPEC.md` §3.10 at `v0.2.0`). A Commands sheet, `A1:D101`, a
  row a command: its name, lowercase letters, digits and hyphens, checked at
  the load; its words; whether it holds; and its key, a `KeyboardEvent.code`.
  The palette lists the commands that hold as `buffet.<name>`; a command run
  reaches the game as its name in `Input!B20` for one frame, one a frame;
  its key is the command's on the viewport while it holds and no Ctrl, Cmd
  or Alt is held, before a Keys button of the same code, a repeat running
  nothing; and `holds` is read after each step, so a formula of the sheet
  shown reaches a tab a frame late. The specification's own example is this
  dish's: `=IF(Input!$B$20="buy-1",...)`.
- *The Write sheet* (§3.9). A row a write, made by the host after the step
  as a value a frame late; the kit's `put`; `MISE.3`'s probe 3, through the
  real host, each order's six fields landing a frame late and its replay
  deriving them again.
- *The keys free.* The viewport takes the arrows, Page Up and Down, Home,
  End and Enter, and no letter or digit; the page's own keymap binds no key
  without Ctrl, Cmd or Alt (`web/viewport.js` and the template at `v0.2.0`).
- *The dialect* (`vla-lang` 0.10.0). Fourteen functions, `IF`, `AND`, `OR`,
  `NOT`, `SUM`, `MIN`, `MAX`, `ABS`, `INT`, `MOD`, `ROW`, `COLUMN`, `CHOOSE`
  and `SIN`, and no `SUMIF` or text function; `IF` and `CHOOSE` compute the
  branch taken; a `style` row takes one written cell, a `cell` row a range,
  and a `look` draws every cell of its sheet whose value is its own, before
  the cell's own style.

**The decisions.**

1. **The version: `(spec 5)`, and the order desk's acts the cartridge's own
   commands (rec).** The move `ENGINE.25` left to this dish, "Buffet's pin,
   its `(spec 5)` and its Commands sheet are Buffet's dishes'": the Input
   sheet's twentieth row, and the Commands sheet a device. The Keys sheet
   keeps the exam's and the hustle's keys, each bound on its own view, so no
   act already built moves (Found). *The alternatives:* the Keys sheet's ten
   buttons for twenty-four acts, which they cannot hold; version 4 and no
   key, the owner's keyboard-first rule broken.
2. **The views: Trading, a view of its own beside the Portfolio, where the
   buttons stand; and the Portfolio's columns of the holding (rec).**
   `FRONT.1`'s rescoping: "the market's view and its buttons with
   `PLATE.1`", and beside the Portfolio's prices "the shares, the cost, the
   value and the gain are `PLATE.1`'s columns". Its name settles `FRONT.1`'s
   "one is renamed at `PLATE.1`" with no rename: the model's Market sheet,
   which the Stocks sheet, the hand, the tests and every record read, keeps
   its name, and the view is Trading, the desk's own word. *The
   alternatives:* the buttons on the Portfolio's rows, a view 160 characters
   wide; the view named Market and the model's sheet renamed Index.
3. **The buttons: six a stock, Buy 1, Buy 10, Buy max, Sell 1, Sell 10 and
   Sell all (the line's), a grid of four rows and six columns; each a text in
   brackets in a column of 12 characters, drawn in the buttons' lavender by a
   look keyed on its text (rec).** Six looks for twenty-four cells, where a
   style row would stand at each: a look draws before a cell's own style
   (`SPEC.md` decision 19). The presses are `clicked`'s third writing, one
   fill over the Orders sheet's grid, `(clicked "D6:I9" "$B$2" "D$5"
   "$C6")`, its places values: the view, a column an act and a row a stock.
   *The alternatives:* a style row a button; a table a button a row, as the
   Proctor's, twenty-four rows of five values.
4. **The keys: a stock picked, then an act (rec).** Ten commands:
   `rung-1` to `rung-4`, on the digits 1 to 4, pick a stock; `buy-1`,
   `buy-10`, `buy-max`, `sell-1`, `sell-10` and `sell-all`, on B, N, M, S, D
   and F, order an act of the stock picked, the keyboard's lowest row buying
   and its home row selling, a share, ten and the most in the order of the
   buttons' columns. The stock picked is the last a click or a pick named,
   marked ▸ on the view, none at the start, and an act with none picked is
   refused by a sentence that says how to pick. Every command holds while
   Trading shows and on no other view (the owner's word at `BACK.1`'s hand
   test, "keybindings should be scoped to each tab"), so the exam's B and D
   stay the Exam's; each is a line of the palette and a person's to rebind.
   *The alternatives:* twenty-four commands, a button each, and no key but
   the palette's and a person's keymap; twenty-four default keys, which no
   layout spells memorably.
5. **An order: one a frame, judged by what the last frame showed (rec).** A
   click's before a command's: its pocket, its stock and its act, filled at
   the price shown, `Stocks.last!E5:E8`, only while the last frame showed the
   market open, `Day.last!$B$19` (`POT.10`'s decision 13), and against the
   pocket's cash and its shares as the last frame showed them, so that the
   state the person saw decides, as a claim's and an answer's do. Buy 1 and
   Buy 10 fill whole or not at all; Buy max fills as many as the cash
   covers; Sell 1 and Sell 10 whole or not at all; Sell all every share held.
   A refusal says its reason: the market closed, no stock picked, the cash
   short or the shares short. *The alternative:* Buy 10 and Sell 10 filled in
   part, up to what the cash or the shares cover, which Buy max and Sell all
   already offer.
6. **The books: a row a pocket and a stock, both pockets', on a sheet of
   their own, Positions (rec).** The line's "shares held in each pocket":
   eight rows, the firm's four, then your own. Formula state over their twins
   (`BD-8`): the shares; the cost basis at the average cost, a purchase adding
   its cost and a sale taking off the shares' share of it, rounded once to
   whole cents, all of it with the last share; the cents spent buying and
   received selling; and the dividends. Functions of them: the price now, the
   value, the gain not yet taken, the value less the basis, and the gain
   taken, the cents received less those spent, and back with the basis still
   held. Positions never pass through the Write sheet, as the line has it. The
   line's *weight* is left to the dish that shows one, the overview or the
   report card, since a percent wants the number format no view draws yet
   (Found). *The alternatives:* each pocket's columns side by side, every
   shape twice; lots kept first in, first out, `PLATE.7`'s.
7. **The pocket an order funds: the firm's, a value, `Orders!B1`, 1 the firm
   and 2 your own (rec).** `PLATE.2` makes it the switch, a click and a key,
   and holds that a switch moves no balance (`BD-9`); until then a person's
   edit of 2 is the switch. The views show that pocket's holdings, and both
   pockets' cash and net worth. *The alternative:* the firm's book alone,
   your own cash idle until `PLATE.2`.
8. **The cash: each pocket's a sum of its flows (rec).** The firm's, the
   exam's capital, less the cents spent, with those received and the
   dividends; your own, $100 and the hustle's fees, the same way; then each
   pocket's holdings at the price now and its net worth, on the Cash sheet.
   *The alternative:* cash as state over its twin, its history in no cell.
9. **The dividend: paid at a day's first frame to the shares the last frame
   held, into the pocket's dividends, with no ledger row (rec).** `STATION.1`'s
   decision 11 had it "into that pocket's cash with a ledger row". The ledger
   is of orders, and a day's first frame may pay both pockets and fill an
   order besides at a frame a day, three rows of seven writes; the dividends
   are a column of the books and a flow of the cash instead.
10. **The ledger: a sheet of its own, Ledger, a ring of the last hundred
    fills, seven fields, written by seven Write rows (`MISE.3`'s decision 5,
    rec).** The order's number; when, the half hour the person saw; the
    pocket; the stock; the shares, a sale negative; the price in cents; and
    the pocket's cash after it, in cents. A refusal is no row. The row is
    computed once, `=MOD(n-1,100)+2`, on the Orders sheet.
11. **The texts, drafts (rec).** Trading's line, "Orders fill at the price
    shown, while the market is open, from the firm's cash, which is to say
    other people's money.", or "from your own cash."; its headers Security,
    Price ($) and Shares; the last order, "No order yet.", "Bought 10 shares
    of rung 3 at the price shown.", and four refusals with no joke, each a
    line a player acts on: "Refused: the market was closed. Orders fill from
    the opening bell to the closing bell.", "Refused: no stock is picked.
    Press 1 to 4 to pick one, or click a button on its row.", "Refused: the
    cash does not cover 10 shares of rung 3." and "Refused: no shares of rung
    3 to sell." or "fewer than 10 shares"; the keys' line; the Portfolio's
    "The firm's holdings, at the price now.", its headers Shares, Cost ($),
    Value ($) and Gain ($), and "Net worth ($)"; and the commands' words,
    live, "Buy 10 shares of rung 3".
12. **The caveat (`BD-5`, owed by this dish).** "An order fills whole at the
    price shown, at once, with no spread, no commission and no day to
    settle." beside "A buyer pays the ask and a seller gets the bid, a broker
    may charge, and a trade settles a business day later.", and Management's
    response, "Our broker works for exposure.", its place `Orders!B23`, the
    price shown.
13. **The oracle (rec).** The hand gains the order desk: the buttons by their
    places, the commands by their names, an order judged by what the last
    frame showed, the books folded from a list of fills, the cash, the
    dividends, the ledger's ring and both views' texts. The tests: a scripted
    session of every press and refusal, the host's derived writes made as
    `host.js` makes them; the ledger past a hundred fills; the dividend paid
    at a day's turn to both pockets; the desk's settings, its buttons and the
    commands' names and keys; and every test standing moved to version 5 and
    the views' new rows.
14. **What it costs, measured** natively, as `GUARD.1` measured it, the
    browser's latency left to the hand test and the playtest pass (`MISE.5`).
15. **The stale places, rewritten at the build:** the item's line (🟡),
    `BD-5`'s owed list, the contract's `(spec 4)`, `MISE.6`'s row of chords and
    its hoist row of `clicked`; and `Delta/CLAUDE.md`'s map.

**Found at the scoping**, none minted:
- *The exam's and the hustle's keys as commands.* A to D, O and P could be
  commands of their views at version 5, lines of the palette; they stand on
  the Keys sheet as `GUARD.1` and `BACK.1` built them.
- *A weight as a percent* wants the number format (`MISE.6`'s row): a text
  percent would be padded as the dollars are.
- *A click and a command in one frame*: the click's order is taken and the
  command's dropped, one order a frame, as the exam takes the first of two
  options pressed together.
- *The ledger in cents*: a hundred rows of dollars as texts would cost a load
  about 20 ms (`GUARD.1`'s finding); the ledger is the model's, and no view
  reads it.

**The predictions.** The cartridge's size was read from the build's first
expansion, 1,028 rows and 520 formula cells, before these were written, so
it is no prediction; the rest were written before any of their numbers was
taken.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | read, not predicted: 1,028 rows where 748 stood, 520 formula cells where 271 | *Measurement*: 1,028 rows and 520 formula cells in 214 formula rows, where 131 stood |
| P2 | the describe golden | `(spec 5)`; `(device "Input" "A1:E20" input)`; the Write and Commands devices; `(budget 30 520 15600 2000000)`; every shape renumbered from the Portfolio's columns on, the views' date, bell and fine print each a shape over five sheets; readers rows new for the Orders, the Positions, the Commands and their twins; `(every-frame 0)` | *Measurement*: `(spec 5)`; `(device "Input" "A1:E20" input)`; `(device "Write" "A1:C1048576" effect)` and `(device "Commands" "A1:D101" effect)`; `(budget 30 520 15600 2000000)`; `(plan (shapes 198) (texts 195) (formulas 520) (levels 34) (runs 45) (lanes 260) (alone 260) (every-frame 0) (cycles 0))`; readers rows new, `(readers "Orders" 102 (most 14))`, `(readers "Positions" 39 (most 6))`, `(readers "Commands" 10)`, `(readers "Orders.last" 4)` and `(readers "Positions.last" 14)`; the views' date, bell and fine print `(sheets 5)`. **Missed** in one: no row for the Commands' twin, which nothing reads, and two twins' rows new that I did not name, the Stocks' 4, the price shown, and the Cash's 2, the cash shown; and the Stocks' readers 27 where 13, the Input's 19 where 13 and the Day's twin's 10 where 5 |
| P3 | the pass and the load | the pass 1.0 to 2.5 ms longer than `GUARD.1`'s 6.5; the load 2 to 4 ms longer than its 3.7, nineteen dollars' texts and 250 formula cells more | *Measurement*, one runner over `HEAD`'s texts and this tree's, four rounds alternated, 200 passes and 200 loads a round, pinned to the performance cores and opted out of Windows' throttling, a fixed computation 1.9 ms before and after: the pass 7.10 to 7.27 ms where `HEAD`'s took 5.44 to 5.85, 1.6 to 1.8 longer, inside; the load 7.10 to 7.23 ms where 4.18 to 4.38, 2.8 to 3.0 longer, inside. `HEAD`'s own pass read 5.4 to 5.8 where `GUARD.1` read 6.5, the pin to 0.2.0 at opt-level 3 between them |
| P4 | the step | an ordinary frame within 0.4 µs of 1.9; a mark 20 to 45 µs dearer than its 26, fourteen dollars' texts more moving with the prices; an order's frame 20 to 60 µs; the day's first frame within 10 µs of its own | *Measurement*, the same runs, ten days at two minutes a frame, the exam waived, each frame filed by what the hand's calendar turned, `HEAD`'s beside: an ordinary frame 2.0 to 2.1 µs where 1.6 to 1.7, inside; midnight 26.2 to 27.3 where 21.2 to 24.1, inside; a half hour 12.9 to 13.2 where 10.6 to 10.8; the bell 22.8 to 26.3 where 19.6 to 23.5; the close 34.5 to 36.1 where 25.9 to 27.5; the first frame 252 to 307 where 179 to 254. **Missed** in two: a mark 30.9 to 31.7 where 22.9 to 23.7, 8 µs dearer where 20 to 45 were priced, the holdings' dollars standing still while their shares are none; and an order's frame 118 to 122 µs, and the frame after it 180 to 188, where 20 to 60 were priced: the order's own cells 33 to 45 µs and the frame after 14 to 19 when its seven derived writes land on cells already written, and the rest the writes into the ledger's empty rows (What the build found). The derived writes themselves 14.6 to 15.2 µs, the Input rows 4.0 to 4.4 |
| P5 | the suite | 72 to 80 tests where 67; each long test 10 to 30% dearer, every record listing twelve sheet rows more; the scripted session of orders 0.05 to 0.3 s | *Measurement*, the two test binaries alternated, each pinned and opted out, two rounds: 71 green, the crate's 27 and the cartridge's 44; the session of orders 0.27 s, inside; the ring 0.25 s and the dividend at a frame a day 1.0 s; the hour's year 5.49 to 5.53 s where 4.56 to 4.60, 20% dearer, and the hundred years 5.04 to 5.11 where 4.09 to 4.12, 24%, inside; the whole on eight threads 5.8 to 6.0 s where 4.7 to 5.1. **Missed** in four: 71 tests, under 72, four new where five were priced; the ten days 3.03 to 3.04 s where 2.74 to 3.17, no dearer to the noise; the year at difficulty 0 1.97 to 1.99 where 1.81 to 1.84, 7 to 10%; and the eight hundred days 0.65 to 0.66 where 0.44 to 0.45, 47%, Trading held at each of its frames, every one a day's turn. Trading held at every half hour made the hour's year 46% dearer at the first build, so the long runs hold it at each day's turn |
| P6 | the page | the cartridge's block 25,000 to 35,000 characters longer, the source's 20,000 to 30,000, the caveats' 300 to 500; Build writing the cartridge byte for byte | *Measurement*: 2,457,379 characters where `HEAD`'s texts make 2,410,194; the source's block 23,757 longer, inside. **Missed** in two: the cartridge's 23,144, under its 25,000, and the caveats' 284, under its 300. Build's bytes not checked here, no headless harness this sitting: the hand test's |
| P7 | the checks | four checks and four controls green, none edited | *Measurement*: ALL GREEN, the four checks and their four controls, none edited, as predicted |
| P8 | the hand test | Trading by its tab; a click on Buy 10 filling at the price shown and the ledger's row landing; 3 then B buying a share; an order at a closed hour refused; the Portfolio's columns and net worth; the palette's `buffet.` lines on Trading alone | The owner's, 2026-10-18, on the page built here: passed, "Tests pass; commit", every line as predicted (the paragraph below) |

**What the build found.** The order desk held as the decisions drew it: the
scripted session of orders and the ledger's ring passed at their first run,
every cell of the Orders, the Positions, the Cash, the Ledger and the Commands
sheets and of both views the hand's at every frame, and the Write sheet's rows
seven at a fill and none else. The hand kept its fills in a list and folded
them, the cost of the shares sold found in integers rounded half up, and met
the cells' doubles at every sale. One test's own edit, your own start raised
to $10,000 so that it could buy ten shares of the fourth stock, met the cash
holder's check of the contract's $100, and the check moved to the holder of
the sessions that write none. Twenty mutants, each written into the source by
its anchor, applied the counted number of times and expanded before its test
ran, each failed the test named for it: the price now in place of the price
shown; the market open now; a command before a click; Buy 10 and Sell 10 filled
in part; the basis taken off at the sale's price; the dividend paid to this
frame's shares; the ring a row late; the commands holding on every sheet; a
click picking nothing; the gain taken, its sign turned; the ledger written on
a refusal; the pocket ignored; a button's look gone; Sell all leaving one share;
Buy max one share more; the firm's cash without its dividends; a refusal
counted a fill; Trading showing the firm's shares; and the ledger's cash before
the order. At the first run the price now lived: the session's order at the
10:00 mark sold the first stock, a $1 share the half hour's swing seldom moves
a cent, so its frame showed no new price. A sale of the third at the 11:00 mark
was added, its price's move asserted, and the mutant failed it. Two of the
mutants wanted tests of their own: Buy 10 in part, a pocket that covers some of
ten, your own $100 against the third stock; and the dividend paid to this
frame's shares, an order on a dividend's own frame, which only a pace of a day a
frame or more meets, a day's first frame being midnight at the manifest's, so
the dividend's test plays three years at a frame a day and sells the firm's
shares on the ninth dividend's frame, a Tuesday.

Found at the build, none minted:
- *A derived write into a cell that held nothing costs the next two steps.*
  Natively, the ledger's seven writes into a row not yet written cost the step
  after them about 150 µs more and the step after that about 90; the same
  writes into a row already written, nothing measurable. So while the ledger
  fills its hundred rows each fill costs about a quarter of a millisecond, and
  after the hundredth none. The cartridge could fill its rows with empty texts
  at the load, a row of the source, to avoid it; it does not, the cost being a
  hundred fills' and the language's to price (`vla-lang` 0.10.0's machine): a
  candidate ask, with these numbers, if a later ledger is dearer.
- *The plan's levels 34 where 31*: the order desk's chain, the click's grid, the
  order, the fill, the books, the cash and the views' dollars, the longest.
- *The dollars a view shows of a holding* stand still while no share is held,
  so a mark costs a fifth of what fourteen moving texts would.

**The owner's hand test.** On the page built here, 2026-10-18: passed, "Tests
pass; commit", over eight screenshots and no change asked. The exam sat to two
of ten, the firm's $6,000.00; the Portfolio at Monday's 22:30, "The firm's
holdings, at the price now.", Shares, Cost ($), Value ($) and Gain ($) at 0
and 0.00, and the net worth beneath the cash on hand. On Trading, ten shares
each of the first three stocks bought on Tuesday, the fourth picked, and a
click at Wednesday's 1:00 refused, "Refused: the market was closed. Orders fill
from the opening bell to the closing bell."; then "Bought 5690 shares of rung
1 at the price shown.", the cash available $0.70. The Portfolio on Friday at
13:00: 5,700 shares of the first at their cost, the second's ten at $50.00
worth $49.90, a loss of 0.10, the third's at $249.30 worth $249.70, a gain of
0.40, the firm's net worth $6,000.30. The Ledger's four rows, each the half
hour the person saw, the firm, the stock, the shares, the price and the cash
after in cents, the fourth "Wednesday, year 1, day 3, 14:30", 5,690 at 100,
70 left. Ctrl+K on Trading listing `buffet.rung-1` to `buffet.sell-all`, each
with its live words, "Buy 1 share of rung 1", and its key "on the Screen", and
on the Portfolio none of them. Your own pocket by the drawer's edit,
`(cell "Orders" "B1" 2)`: Trading's line "from your own cash.", ten shares of
the second bought from it, $50.10 left, and the drawer's count, 35 derived
writes landed and none refused, its note naming the keys 1 to 4, B, N, M, S, D
and F. Found in the screenshots, not asked: column D drawn about 24
characters wide on every sheet shown, the Portfolio's, Trading's and the
Ledger's alike, where the record gives it 3, 12 and 10. The viewport keeps a
width a person drags by the column's letters alone (`state.widths`,
`web/viewport.js` at `v0.2.0`), so one drag of D's divider on any sheet widens
D on all of them, where a spreadsheet keeps a column's width a sheet: a
finding for Alonzo, the owner's to pass on.

## `PLATE.2`, first slice — the two pockets' switch: a button and a key, and no money moved

*Scoped and built 2026-10-18 at the owner's word, "Scope and implement PLATE.2
in Buffet (Delta/Buffet): the two pockets' switch", in one sitting after
`PLATE.1`'s close: each decision below stands as recommended and was built,
the owner's to approve, amend or strike at the hand test, and the texts a
player reads are drafts. Read for it: the item's line in `OMAKASE.md` and its
three notes; `BD-5`, `BD-7`, `BD-8` and `BD-9`; `PLATE.1`'s record above, its
decisions 4, 5 and 7 and what its build found; Alonzo's `SPEC.md` at `v0.2.0`,
the tag the pin names, §3.3, §3.4 and §3.10; and the page at that tag for the
keys it keeps. Alonzo's and Frazaro's trees were read and never touched.*

**What it stands on, read on the day** (Buffet at `c7cd4f3`, one ahead of its
remote, the tree clean).
- *The pocket.* `Orders!B1`, a value, 1 the firm's and 2 your own, read by
  one cell, `B19`, `=IF($B$1=2,2,1)`; and `B19` read by everything that
  follows the pocket: the order's cash and shares shown, `B24` and `B25`; the
  fill's row, `Positions!D2:D9`; the pocket's cash now, `B31`; the rows shown,
  `Positions!A12:A15`, so the Portfolio's holding and Trading's shares;
  Trading's line and cash available; the Portfolio's line; and the ledger's
  pocket and cash after, the Write sheet's `C4` and `C8`. A person's edit of
  `B1` the only switch.
- *One order a frame, a click's before a command's* (`PLATE.1`'s decision 5):
  a button clicked and a command run in one frame, the click's order taken.
- *The keys.* Trading's commands hold 1 to 4, B, N, M, S, D and F; the exam's
  A to D and O and the hustle's P bind on their own views alone; the page
  binds no letter without Ctrl, Cmd or Alt (`KEYMAP_DEFAULTS` at `v0.2.0`).
- *The suite.* 71 tests green, the crate's 27 and the cartridge's 44; the four
  checks green.

**The decisions.**

1. **The selector: `Orders!B1`, formula state over its twin (rec).**
   `=IF($B$47=1,IF(Orders.last!$B$1=2,1,2),IF(Orders.last!$B$1=2,2,1))`: the
   last frame's pocket read as `B19` reads it, 2 your own and any other value
   the firm's, so the firm's at the first frame, whose twin is empty, and the
   other pocket at a switch; always 1 or 2. `B19` and every formula after it
   stand byte for byte, so the views and the ledger follow the switch with no
   formula moved, the line's words; and `B1` is the selector the line names,
   the toggle its note asked for (`BD-8`: a state is a cell over its twin). A
   person's edit of `B1` now replaces the switch's formula, as an edit of any
   formula does, and the notes stop offering it. *The alternatives:* `B1` kept
   a value, the pocket at the start, and the switches counted beside it,
   `B19` flipping the start by their parity, which moves `B19` and keeps a
   setting no player needs, a switch being one key; the kit's `hold`, whose
   untaken branch is the bare twin, 0 until the first switch, a pocket the
   label does not name.
2. **The switch: a button beside the cash it turns, `[ Switch ]` at
   Trading's `D11`, "or press C" beside it at `E11`, drawn in lavender by a
   style row (rec).** Row 11 reads "Cash available ($) 7,500.00 [ Switch ] or
   press C", the line above saying whose cash, as Gigs' and the Certificate's
   buttons carry their keys beside them; a style row, as their one button
   each has, where Trading's twenty-four take looks. Its place is values on
   the Orders sheet, `B43`, column 4, and `B44`, row 11, the view `B2`'s, so a
   layout moves no formula, and its press is the library's `clicked` at its
   fourth writing, `B45`, the verb unchanged. *The alternatives:* two buttons,
   `[ Firm ]` and `[ Personal ]`, the chosen marked, two places for one
   toggle; a button naming the pocket it turns to, which 12 characters hold
   only ambiguously, "[ Your own ]" reading as the pocket shown as well as
   the pocket to come; the switch at row 4, under the line.
3. **The command: `switch-cash`, on C, its words live, holding while Trading
   shows (rec).** The Commands sheet's row 12, listed in the palette as
   `buffet.switch-cash`: "Switch to your own cash: the buy and sell commands
   spend it", or "Switch to the firm's cash: ..." while your own is the
   pocket, as each pick's words say what the commands then trade. C for the
   game's own word on the row beside the switch, free on Trading. Its hold is
   the commands' fill, `C2:C11` grown to `C2:C12`, and the order desk reads its
   name from `Commands!$A$12`, `B46`, as it reads the others. *The
   alternatives:* P, for a pocket, which the game never names, P being Gigs'
   on its own view; Tab, the page's for its focus; two commands, a pocket
   each, two keys for one toggle.
4. **One act a frame, a click's before a command's: an order, a pick or a
   switch (rec).** `PLATE.1`'s rule widened to the switch, so that no order
   lands on a switch's frame and the pocket an order funds is always the
   pocket its person saw, the house's rule (an act judged by what the last
   frame showed) kept with no view's formula moved, the views reading this
   frame's pocket. Switched, `B47`, is a click on the switch, or its command
   while no button of an order is clicked; an order's act, `B21`, drops a
   command's beside the switch's click; the stock picked, `B16`, drops a
   command's pick beside it; and the order's stock, `B20`, follows the act.
   *The alternatives:* both acts taken, the order funding the pocket switched
   to in the same frame, which its person never saw; the order's pocket read
   from the twin, which moves the formulas the line keeps and shows every
   switch a frame late (`BD-7`: a person's act is answered at the next
   frame).
5. **No balance moves (`BD-9`), held by a test of its own (rec).** At a
   switch's frame and at the frame after it, each pocket's cash, each book's
   shares, cost basis, cents bought and sold and dividends, the fills counted
   and the ledger's rows are as the last frame left them; a value and a net
   worth move with the price at a mark, being valuations and no balance. A
   switch is allowed at every hour, the market open or closed, since it moves
   nothing. The line's "mutant page" is read as mutants of the cartridge, the
   page's own loop oracle being `HOUSE.4`'s second slice.
6. **The views follow, at the switch's own frame (rec).** Trading's line on
   whose cash, its shares column and its cash available; the Portfolio's line
   and its holding's four columns; the command's words. The last order's
   sentence stays the last order's (Found). Both pockets' cash and net worth
   on the Portfolio stand as they did.
7. **The caveat owed (`BD-5`): row 18, cash that earns nothing and loses
   nothing to inflation, its place `Cash!B3:B4` (rec).** "Cash earns no
   interest and loses nothing to inflation: a dollar today is worth a dollar
   in any year." beside "Savings earn interest, and prices rise: US consumer
   prices rose about 3% a year on average over the last century.", and
   Management's response, "Inflation has been classified as transitory,
   permanently." The 3% is marked for the finance educator's read (`MENU.8`):
   the consumer price index's annual average from 17.5 in 1925 to 313.7 in
   2024 (*check*, the Bureau of Labor Statistics' CPI-U), 2.96% a year
   (*derived*, (313.7 ÷ 17.5)^(1/99)).
8. **The texts (drafts):** `[ Switch ]`, "or press C", the command's words,
   and the game's notes, the clause offering an edit of `Orders!B1` struck
   and the switch named beside Trading's buttons.
9. **The oracle (rec).** The hand gains the switch: its place, its command,
   the pocket flipped, and one act a frame, the switch taken first and nothing
   after it. The tests: a scripted session of the switch held at every frame;
   `BD-9`'s comparison at each switch's frame and the frame after; the desk's
   settings grown by the switch's place, text, style, command, key and hold;
   `PLATE.1`'s two sessions switching by the switch where a person's edit
   stood; a hand test of the rule; the describe golden.
10. **What it costs, measured** natively, as `PLATE.1` measured it, the
    browser's latency left to the hand test and the playtest pass (`MISE.5`).
11. **The stale places, rewritten at the build:** the item's line (🟡),
    `BD-5`'s owed list, the source's notes and the comments that named the
    switch to come, and `Delta/CLAUDE.md`'s map.

**Found at the scoping**, none minted:
- *The switch on the Portfolio too.* Its holding follows the switch made on
  Trading; a button there and the command holding there would let it turn
  its own view. One more place and one more hold, at the owner's word.
- *The last order's sentence names no pocket.* After a switch, Trading's
  `A13` still reads the last order, which may be the other pocket's: "Bought
  10 shares of rung 2 at the price shown." under your own cash. "..., for the
  firm." would say it, a view's formula moved, so left to the owner.
- *A caveat on a manager's own trades.* The game lets both pockets buy the
  same stock at the same price in the same minute, approved by no one, where
  an adviser's people report their own trades to compliance, and firms
  commonly approve them first and forbid trading ahead of a client (Rule
  204A-1's code of ethics, *check*); offered as a row with its joke,
  "Compliance has approved all personal trading in advance, and in
  arrears.", and not added.
- *A transfer's row wants two pockets.* `BD-9`'s second half, every transfer a
  row of the ledger with the pocket it left and the pocket it entered, is
  `BACK.6`'s; the Ledger's pocket field holds one, so its seven fields grow by
  one then.

**The predictions.** The cartridge's size was read from the build's first
expansion, 1,048 rows, before these were written, so it is no prediction;
the rest were written before any of their numbers was taken.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | read, not predicted: 1,048 rows where 1,028; 526 formula cells where 520, predicted | *Measurement*: 1,048 rows and 526 formula cells, as predicted |
| P2 | the describe golden | `(budget 30 526 15780 2000000)`; `(formulas 526)`, `(every-frame 0)`, `(cycles 0)`; shapes 198 to 201 to 205; levels 34 or 35; the readers of Orders 2 to 6 more, Commands 11, Input 20, `Input.last` 6, `Orders.last` 5; the devices unchanged | *Measurement*: `(budget 30 526 15780 2000000)`; `(plan (shapes 203) (texts 200) (formulas 526) (levels 34) (runs 44) (lanes 256) (alone 270) (every-frame 0) (cycles 0))`; Commands 11, `Input.last` 6 and `Orders.last` 5; the devices unchanged; every shape renumbered from the Orders sheet's on, the views' shapes reading the pocket five levels deep where one. **Missed** in three: the readers of Orders 113 where 104 to 108, the switch's six cells and the desk's reads of them each counted; Input's 23 where 20, the switch's click reading four of its cells; and the Caveats' 77, unnamed, a picker a field reading the new row |
| P3 | the pass and the load | the pass 0.0 to 0.3 ms longer than `HEAD`'s; the load 0.0 to 0.3 ms longer | *Measurement*, one release runner over `HEAD`'s texts and this tree's, four rounds alternated, 200 passes and 200 loads a round, pinned to the performance cores and opted out of Windows' throttling, a fixed computation 91 ms before and after: the pass 7.90 to 8.23 ms where `HEAD`'s took 7.81 to 8.04, inside; the load 7.41 to 7.54 ms where 7.24 to 7.31, inside. `HEAD`'s own pass read 7.8 to 8.0 where `PLATE.1` read 7.1 to 7.3, the day's machine |
| P4 | the step | an ordinary frame within 0.2 µs of `HEAD`'s; a switch's step 15 to 45 µs with no share held; the frame after a switch within 5 µs of an ordinary frame; an order's step within 5 µs of `HEAD`'s once the ledger's rows are written | *Measurement*, the same runs, ten days at two minutes a frame filed by the hand's calendar, then twenty loads each Trading's: an ordinary frame 2.0 to 2.2 µs where 2.0 to 2.2, inside, and the half hour, the mark, the bell, the close and midnight each `HEAD`'s to the noise; the frame after a switch 5.4 to 6.6 µs, 3.4 to 4.4 above an ordinary frame, inside; an order's step 29.8 to 29.9 where 29.4 to 29.9, inside, the frame after it 10.7 to 10.9 where 10.4 to 10.5. **Missed** in one: a switch's step 13.1 to 13.3 µs with no share held, under the 15 priced, where a command no cartridge knows costs 5.1 to 5.2; 34.0 to 34.4 with the firm holding four stocks, the Portfolio's twelve texts of the holding spelled again |
| P5 | the suite | 73 to 75 tests where 71; the new session 0.1 to 0.4 s; each long test within 5% of `HEAD`'s; the whole on eight threads within 0.4 s | *Measurement*, the two test binaries alternated, each pinned and opted out, two rounds: 73 green, the crate's 28 and the cartridge's 45, inside; the switch's session 0.29 to 0.30 s, inside; the ten days 3.07 to 3.10 s where 3.00 to 3.06, the hour's year 5.51 to 5.55 where 5.47 to 5.49, the hundred years 5.12 to 5.14 where 5.01 to 5.05, the orders' session 0.28 to 0.29 where 0.27 to 0.28 and the dividend's 1.02 to 1.03 where 0.99, each 1 to 3%, inside; the whole on eight threads 5.96 to 6.44 s where 5.89 to 6.45, inside |
| P6 | the page | the cartridge's block 1,500 to 2,500 characters longer, the source's 2,500 to 4,500, the caveats' 300 to 450 | *Measurement*: 2,462,137 characters where `HEAD`'s texts make 2,457,379; the source's block 3,080 longer and the caveats' 313, inside. **Missed** in one: the cartridge's 1,365, under its 1,500, the switch's rows short |
| P7 | the checks | four checks and four controls green, none edited | *Measurement*: ALL GREEN, the four checks and their four controls, none edited, as predicted |
| P8 | the hand test | `[ Switch ]` on Trading turning the line to your own cash, the cash available to $100.00 and the shares to yours; C turning it back; the palette's `buffet.switch-cash` on Trading alone, its words naming the pocket it turns to; each pocket's orders in the ledger by name; no cash moved by a switch; the Portfolio's line and holding following; caveat 18 on the Caveats sheet | The owner's, 2026-10-19, on the page built here: passed, "Tests pass", every line as predicted (the paragraph below) |

**What the build found.** The switch held as the decisions drew it: the
session of the switch and `PLATE.1`'s two sessions, switching by the click and
by C where a person's edit of `B1` had stood, passed at their first run, every
cell of the Orders, the Positions, the Cash, the Ledger and the Commands
sheets and of both views the hand's at every frame, the views following at
each switch's own frame. Sixteen mutants, each written into the source or the
caveats' file by its anchor, applied the counted number of times and expanded
before its test ran, each failed the test named for it: a switch moving a cent
into your own cash; a switch counted as a fill; the switch one way only; the
pocket read a frame late; an order taken beside the switch's click; the
switch's command taken beside an order's click; a pick taken beside the
switch's click; the order's stock beside the switch's click; the commands
holding on every sheet; the switch a row lower; its words fixed; its key
moved; its look gone; the pocket starting your own; the command renamed; and
the caveat's place empty. At the first run the two that move money, the cent
and the fill, were caught by the hand's books, `Orders!B31` and `B29`, before
`BD-9`'s own comparison ran, so the comparison moved ahead of the hand in each
frame, read from the cells alone at the switch frames the script names; then
both, and the order beside the switch's click, failed it by its own words,
"frame 6: a switch moved a balance" and "frame 14". A switch costs under half
a mark with no share held and about a mark with four stocks held, its cost the
views' texts of the holding, none of the books'.

Found at the build, none minted:
- *A harness trap.* The mutant script piped into `Select-Object -First 1`
  stopped its child process in the middle of putting the texts back, and left
  the scratch copy's source cut short and its cartridge mutated, which the
  next control caught; the copy was restored, and the repository, which the
  script never writes, checked by the pass's own `--check`. Its output goes
  to a file since.

**The owner's hand test.** On the page built here, 2026-10-19: passed, "Tests
pass", over nine screenshots, every line as predicted. The exam waived at frame
156; Trading on Monday at 21:30, the market closed, its row 11 "Cash available
($) 7,500.00 [ Switch ] or press C", the switch in lavender; on Tuesday at
21:30 your own pocket switched in, "from your own cash.", rung 2 marked ▸, the
cash available $100.00 and every share 0 though the firm held ten, the last
order still the firm's, "Bought 10 shares of rung 2 at the price shown." (the
scoping's Found: the sentence names no pocket); the Portfolio on Wednesday at
6:30, "Your own holdings, at the price now.", the firm's cash $7,450.10 and
your own $100.00, their net worth $7,500.10 and $100.00; Trading on Friday at
19:00, twenty shares of rung 2 bought from your own cash, $0.20 left; the
palette on Trading, its eleven lines from `buffet.rung-1`, `buffet.switch-cash`
reading "Switch to the firm's cash: the buy and sell commands spend it" with "C
on the Screen", and on the Portfolio none of them; the Ledger's two rows, the
firm's 10 at 499 cents on Tuesday at 10:30 with 745,010 after and your own 20
at 499 on Friday at 13:00 with 20 after; the Cash sheet, 745,010 and 20 cents,
holdings 4,990 and 9,980, net worth 750,000 and 10,000; and the Caveats
sheet's eighteenth row. Asked the same day, of the game and not of this dish,
the owner's words: "the gameplay feels too fast. The trading windows is too
narrow, so I wonder if we should default the gamplay to 1 real-world second to
1 in-game minute, meaning a real-world minute would be an in-game hour, and so
on. To spice up the gameplay a little biut, we should add a volatility factor
to the market which increases the price movement so the game doesn't feel like
the prices are flat for the first 5-10 minutes of play." Taken up as `POT.11`.

## `POT.11` — the pace and the volatility factor: ten minutes a second while the market trades, an hour a second between, a price every five minutes

*Minted, scoped and built 2026-10-19 at the owner's word, "Mint, scope and
implement POT.11 in Buffet (Delta/Buffet): the pace and the volatility
factor, at the owner's adjudications of 2026-10-19", in one sitting as
`PLATE.1` and `PLATE.2` were: each decision below stands as recommended and
was built, the owner's to approve, amend or strike at the hand test, and the
texts a player reads are drafts. Minted alone, the stock pot's next free;
what else the scoping found stands under Found, unminted. Read for it:
`BD-3`, `BD-5` and `BD-7`, the opening contract and `POT.5`'s line in
`OMAKASE.md`; `PLATE.2`'s record above, whose hand test holds the owner's
first words on the pace; `POT.10`'s (the calendar of minutes, the session's
bridge, its decision 13), `POT.1`'s (the calendar's design, `BD-7`'s oracle)
and `GUARD.1`'s (the bell's wait); the source's Day, Dice and Market
sections, the hand's calendar and path, and the tests' paces. Alonzo's and
Frazaro's trees were read and never touched.*

**The owner's words**, 2026-10-19, after `PLATE.2`'s hand test, deliberated
in chat the same day. The pace: "5 minutes per second when the market is
open, an hour a second when the market is closed, originally 9:30-16:00
market hours with 4:00-20:00 unlocked after licensing"; then "78 trading
seconds to 17.5 feels disproportional, so what if we bump it to 10 minutes
per second during trading hours and keep everything else the same?"; then
"Pre- and post-market trading should probably move at the same speed as
closed market hours to avoid changing the pace of the game once pre- and
post-market trading hours are unlocked. Otherwise two players could play the
same game at two different paces if only one opts to license for extended
trading hours." The ticks every five minutes approved, and the clock "to the
minute" with them while a minute was a second. The factor's reach: "If this
item belongs in POT.5, then we don't need to pull it forward arbitrarily if
POT.5 just needs to be rescoped to handle added volatility." And, declining
the speed keys offered as a later dish: "Pacing is key to passive games, and
I'm wary to expose pacing to the player because it can demolish the
objectivity of the game by allowing players to arbitrarily bend time and lose
sight of the real game: trading and investing."

**What it stands on, read on the day** (Buffet at `11f6803`, two ahead of
its remote, the tree clean; `alonzo` 0.2.0 and `vla-lang` 0.10.0).
- *The calendar* (`POT.10`, `GUARD.1`). The time in minutes, `Day!B4`
  `=(Clock!B1-1-B26)*B1+B9`: the frames since the first times the minutes a
  frame, `B1`, 2, from the first frame's 570, less the frames the bell
  waited, `B26`; every other cell of the calendar reads the time to the half
  hour, `B14`. `B4`, `B5` and `B14` compute at every frame, and no other cell
  of the game.
- *Every reader of the half hour*, the cartridge grepped: `Day!B14` by the
  hustle's due, `Hustle!B15` through its twin and `B16`; `Day!B16`, the time
  shown, by the five views' dates (the library's `as-of`) and by the
  ledger's when, `Write!C3` through its twin; the mark and the closing mark,
  `Day!B22` and `B23`, by the Market's session, `B42` and `B43`; the half
  hour spelled once more in `Hustle!B19`, the due shown,
  `IF(MOD(B15,60)=0,"00","30")`; and the caveats that name it, row 13 at
  `Day!B22` and row 14 at `Market!B45`. Nothing outside the Day sheet reads
  the time to the minute (`BD-7`'s scan).
- *The session's path* (`POT.10`'s decision 6, amended): thirteen marks a
  session, each half hour's step the uniform of one of the day's market
  draws 5 to 17 set between -1 and 1, the half hour's swing 0.0028 of the
  index at `Market!B5`, and the index now, `B45`, a bridge from the last
  close to the session's own, each end exact.
- *The dialect* (`vla-lang` 0.10.0's `calc/library.rs`): fourteen functions
  and no `SQRT`; `^` is the platform's power.
- *The clock* (`SPEC.md` §3.5, §3.9): the host steps at the Clock's rate and
  a cartridge cannot write its own rate, so a pace that changes with the
  hours is the calendar's to keep (`POT.10`'s Found).
- *The suite*: 73 tests green, the crate's 28 and the cartridge's 45.
  `PLATE.2`'s P5, each alone: the ten days at every frame 3.07 to 3.10 s, the
  hour's year 5.51 to 5.55, the hundred years 5.12 to 5.14, the orders' and
  the switch's sessions 0.28 to 0.30, the dividend's 1.02 to 1.03; the whole
  on eight threads 6.0 to 6.4 s. At the new pace the ten days would be 14,715
  frames where 6,915 stand (*derived*).

**The decisions.**

1. **The pace: twenty seconds of the market's time a frame in the regular
   session, two minutes outside it, none while the exam holds the bell; the
   time a sum over its twin, in seconds (the owner's).** At the manifest's
   rate of 30, ten minutes a second from 9:30 to 16:00 on a weekday and an
   hour a second at every other hour, as today. The time, `Day!B4`, becomes
   seconds from day 0's midnight,
   `=IF(Clock!B1=1,B9*60,Day.last!B4+Day.last!B30)`: the first frame's time,
   then the last frame's time and the last frame's pace. A frame's pace,
   `B30`, is none while the bell waits, the session's in the regular
   session's hours and the pace between sessions at every other hour,
   `=IF(B25=1,0,IF(B29=1,B28,B1))`. The bells are multiples of 120 seconds,
   9:30 and 16:00 and the 4:00 and 20:00 of a license to come, so the steps
   between sessions land on the opening bell exactly and the session's on the
   closing bell, 1,170 steps a session. `B1`, the minutes a frame, becomes the
   seconds a frame between sessions, 120, today's pace in seconds; the
   session's, 20, is a value of its own, `B28`. A pace a person writes now
   moves the clock from the next frame, where the time read off the frame
   jumped at every edit. *Derived* at the manifest's rate: a session 39 s, a
   weeknight 17.5 s, a weekend 65.5 s, a weekday 56.5 s, a week 5 min 30.5 s;
   day 30 begins at 23:45 of play, the hustle's fourth fee is paid at about
   17:20 at the earliest, the first dividend and the first quarter's end come
   at 1:11:03, and the first year at about 4 h 47 min; the market open 59% of
   the time played, and 73% with the extended hours of the license to come.
   *The alternatives:* the time computed from the frame piece by piece, the
   session's frames and the night's counted from each day's turn, which
   reads the frame in more cells and breaks at every edit of a pace; a pace
   the host changes, which a cartridge cannot ask.

2. **The pace keyed on the regular session's bells, never on the cell that
   says orders fill (the owner's).** `B29`, in session,
   `=IF(AND(B17<B12,B15>=B10,B15<B11),1,0)`: a trading day between the bells
   `B10` and `B11`, whatever the exam or a license says; open, `B19`, reads
   it, `=IF(AND(B25=0,B29=1),1,0)`, where it repeated the comparisons. So the
   license's item widens `B19` alone, its 4:00 to 20:00 trading at the pace
   between sessions, an hour a second, and every player plays a seed at one
   pace: the license opens 9.5 seconds of trading a weekday, 24% on the
   session's 39, and never a different clock. Held by a test that writes
   `B19` open at every hour, as a license would widen it, and finds every
   step of the clock unmoved. *The alternative:* the pace keyed on `B19`,
   which the license would move.

3. **A price every five minutes of the market's time, 78 a session (the
   owner's, approved).** The time to the five minutes, `B14`
   `=INT(B4/300)*5`, in minutes, so that every cell below it reads minutes as
   now; the mark becomes the tick, `B22`
   `=IF(AND(B20=B3,B15<B11),INT(B15/5)-INT(B10/5),B23)`, 0 at the opening
   bell and one more each five minutes, and the closing tick, `B23`
   `=INT(B11/5)-INT(B10/5)`, 78, from the close to the next bell. The day's
   market draws grow from 17 to 82, the session's steps the draws 5 to 82,
   and the Market's session table from 13 rows to 78. At this pace a tick is
   fifteen frames in the session, two a real second. *The fallback, flagged
   for the hand test:* ten-minute ticks, 39 a session, one a second, each
   step scaled by 1/√3.

4. **Each tick's step scaled by 1/√6, so that the day's spread holds at a
   factor of 1.** A half hour of six steps, each 1/√6 of the half hour's
   swing, wanders as far as one step of it did: the five minutes' swing,
   `Market!B110`, `=$B$5*$B$6/6^0.5`, the half hour's swing, 0.0028 at `B5`,
   the owner's tuned number kept, times the factor (decision 5), over the
   root of six by the dialect's power, there being no `SQRT`; the swing at the
   tick, `B111`, `=B110*(CHOOSE(B108+1,0,C29,...,C106)-B109*C106)`; and the
   index now, `B112`, `=(D26*(1-B109)+B26*B109)*(1+B111)`, `POT.10`'s bridge,
   so that every open and every close is its day's index to the bit
   (`BD-3`). The widest a bridge of 78 steps strays is 39 steps, at its
   middle tick, where thirteen strayed 84/13 of a step; so the rungs' bound no
   seed passes falls, recomputed at the build.

5. **The volatility factor: a setting of the Market sheet, 4, within the day
   only (the owner's, the number theirs to tune at the hand test).**
   `Market!B6`, beside the half hour's swing, multiplies every tick's move
   and never a close: each close stays the waves' (`BD-3`), so the factor
   widens the hours between two closes and leaves the days as they were.
   Across days is `POT.5`'s, its line rewritten so that its AR(1) walk takes
   the same factor. The settings block grows a row, so the waves' table and
   the workings move down one: the index `Market!B26` where `B25` stood, the
   last close the Stocks read `D26`. *The alternative:* the factor at the head
   of the session's table, every address above it standing, which puts the
   knob the owner tunes and `POT.5`'s walk reads in the middle of the sheet.
   *(Amended at the owner's hand test, 2026-10-19, the owner's words: "Factor
   of 16 approved", after "I'm almost tempted to bump intra-day volatility
   factor to 16 to promote day trading": `B6` 16, the five minutes' swing
   0.018290 of the index, the day's widest stray 6.9% at p50, the first year's
   lowest tick 0.8210, and the sure dip of the hand test's paragraph a caveat
   of its own, decision 10's.)*

6. **The clock on every view to the five minutes (the owner's, approved as
   "to the minute" when a minute was a second).** `Day!B16`
   `=INT(B15/60)&":"&IF(MOD(B15,60)<10,"0","")&MOD(B15,60)`, 9:30, 9:35,
   16:00; every view's date and the ledger's when read it as they did. In the
   session it turns with the prices, twice a real second; between sessions it
   turns every two or three frames, twelve times a second, the hour a second
   shown by its minutes (flagged for the hand test; the alternative, the
   clock between sessions shown to the half hour by `B16` alone). *(Amended
   at the owner's hand test, 2026-10-19, the owner's words: "only show the
   half hour while the market is closed". So the time shown is a cell of its
   own, `Day!B31` `=IF(B29=1,B14,INT(B14/30)*30)`, in minutes from day 0's
   midnight, the five minutes in session and the half hour between, which
   `B16` spells; keyed on the session's bells as the pace is, `B29`, and not on
   the open, `B19`, so that it shows the half hour exactly while the market is
   closed today, the exam's wait standing at 9:30 either way, and a license's
   extended hours, at an hour a second, keep the half hour with the pace. A
   test writing `B19` open at every hour holds it.)*

7. **The other readers of the half hour, settled.** The hustle's due moves
   to the five minutes: `Hustle!B15` the five minutes the person saw and a
   day's work, `B16` comparing it with `Day!B14`, and `B19` spelling any
   minute, `...&IF(MOD(B15,60)<10,"0","")&MOD(B15,60)` where it spelled 00 or
   30, so that a prospectus claimed at 10:15 is paid at 10:15 tomorrow, as the
   hand's `time_shown` already spells it. The ledger's when follows `Day!B16`,
   the five minutes the person saw, its header "when, the time shown" where it
   named the half hour. The views' clock, decision 6. Nothing outside the
   Day sheet reads the time in seconds. *(Amended with decision 6 at the
   owner's hand test: the hustle's due runs a day from the time shown,
   `Hustle!B15` reading `Day.last!$B$31`, so that a claim between sessions,
   seen at 20:00 when the five minutes are 20:05, is paid tomorrow at 20:00,
   a time the clock shows, and a claim in session at its five minutes; and
   the ledger's when is always a session's five minutes, an order filling only
   in session.)*

8. **The exam's wait: a pace of none while the bell waits; `B26` kept as
   its record.** `GUARD.1` took the frames the bell waited out of the time;
   with the time a sum, a pace of none stands it still by itself, and the
   game's first frame after the wait stands at 9:30 too, as before. `B26`, the
   frames waited, enters the time no more and stays the record of the wait
   that caveat 16 names, computing at each frame of the exam and never after.
   *The alternative:* `B26` struck and caveat 16 pointed at `B25`.

9. **`BD-7` restated.** A frame is twenty seconds of the market's time in
   the regular session and two minutes between sessions; the Day sheet alone
   reads the frame, at the first frame, and its time in seconds, `B4`, the one
   calendar cell that changes every frame once the bell has rung; every other
   formula reads the day, the time to the five minutes, the session and its
   tick, each changing at most once every five minutes of the market's time,
   fifteen frames in the session and two or three between sessions; so an
   ordinary frame computes `B4`, `B5` and `B14` as now. Held as now by the
   scan, its list `Clock!B1`, `Day!B4` and `Day!B26` with their twins, and by
   the census's `(every-frame 0)`.

10. **The caveats (`BD-5`), drafts.** Row 13 rewritten, "Prices move every
    five minutes and stand still between.", its place the tick, `Day!B22`;
    row 14's "the half hours" become "the five minutes", its place the index
    now, `Market!B112`; rows 3 and 5 at `Market!B26` and row 4 at
    `Market!A9:E12`, where the rows moved. And row 19, owed by the factor:
    "Within a day prices swing four times wider than the closes move, by a
    setting, and every swing is back by the close." beside "A day's swings
    and its close are one market: a market that swings widely within its days
    moves widely between them.", and Management's response, "Volatility keeps
    business hours.", its place `Market!B6`. Row 11 stays true: the $1 share
    moves a penny at a time, and its closes are the waves'. *(Amended at the
    owner's hand test, 2026-10-19, the drafts kept, "The caveat reads fine for
    now": row 19 at the factor of 16, "sixteen times wider", its field widened
    to 105 characters; and a twentieth, the sure dip the hand test's numbers
    found, drafts: "A share bought below the day's close is a sure gain: every
    dip comes back by the closing bell." beside "A price that falls during the
    day can keep falling, and studies of day traders find most of them lose
    money." and Management's response, "Management attributes the trading
    desk's results to skill.", its place `Market!B26`, the close shown from the
    bell; the studies marked for `MENU.8`'s read (*check*: Barber, Lee, Liu and
    Odean on Taiwan's day traders, 2014, and Chague, De-Losso and Giovannetti on
    Brazil's, 2019).)*

11. **The kind first year, restated.** The closes keep it, the waves
    untouched (`BD-3`): the year's low 0.98943 on day 91 and day 365 at
    1.10155. The hours wander four times wider, so the hand's test that every
    half hour stays above 0.98 becomes every tick above the bound the build
    finds at the factor of 4, within the year's first four months, recorded
    beside it. *(Found at the build: 0.95006 on day 39 at 12:25. Amended with
    decision 5 at the owner's hand test: 0.82 at the factor of 16, the year's
    lowest tick 0.8210 the same day and minute, and the rungs' bound no seed
    passes 1, 15, 239 and 1,719 cents.)*

12. **No player control of time (the owner's standing rule).** No speed key,
    no fast-forward act, no license or upgrade that changes the clock. The
    paces, `Day!B1` and `B28`, and the Clock's rate stay cells a person may
    edit (`AD-6`), as the tests write them; whether a scored run fixes them at
    its start is `CRITIC.1`'s to scope (Found).

13. **The oracle (rec).** The hand's calendar counts seconds at its two
    paces, a step chosen by where the last frame stood, each five minutes
    reached turning the tick while the session is today's and the bell
    ringing at 9:30 within a step; its path of 78 ticks with the factor; the
    bridge's widest stray, 39 steps, proved over every tick by the extreme of
    a sum linear in each step, where `POT.10` enumerated all 8,192 patterns of
    thirteen. The runs: every frame of a weekday and its night at the default
    pace, Monday's bell through Tuesday's first ticks, where ten days stood;
    the hour's year, the eight hundred days and the hundred years at a frame a
    day, and the year at difficulty 0, each at its own pace, both paces
    written; the hustle's session at a frame each quarter hour, where each
    half hour stood, so that its claims fall at 10:15 and its due is spelled
    to the minute; the exam's and the waiver's frames at the default pace;
    `PLATE.1`'s and `PLATE.2`'s sessions at the default pace, their acts at
    their frames, now the ticks at 9:35, 9:40 and 9:45 where 10:00, 10:30 and
    11:00 stood, and their closing bell's acts at frames 1,171 and 1,172
    where 196 and 197; a factor written as a person's edit; and `B19` written
    open at every hour. Mutants, each into the source or a content file by
    one anchor, each failing the test named for it: the session's pace the
    one between; the pace between sessions off the bells, 100 seconds; the
    bell's wait walked, no pace of none; the pace keyed on `B19`; the tick at
    the half hour; the steps unscaled; the factor not read; the bridge
    unpinned; the clock to the half hour; the due spelled to the half hour;
    the draws 17 a day; the tick's `CHOOSE` a step behind; the weekend in
    session; caveat 19's place empty.

14. **What it costs, measured (rec).** Natively, as `PLATE.2` measured it: a
    release runner over `HEAD`'s texts and this tree's, alternated, pinned to
    the performance cores and opted out of Windows' throttling, a fixed
    computation before and after: the pass and the load; an ordinary frame,
    a tick, a five minutes' turn between sessions, the bell, the close,
    midnight and the first frame, each beside `HEAD`'s ordinary frame, half
    hour, mark, bell, close, midnight and first frame. The browser's frames
    at the hand test.

15. **The stale places, rewritten at the build.** In `OMAKASE.md`: the line
    (🟡); the contract's pace; `COURSE.0`'s table; `BD-7` (decision 9); `BD-5`'s
    taken rows; `POT.5`'s line, the factor across days; `CRITIC.1`'s first card
    at 1:11:03; `MISE.4`'s fixture, both paces written a day; `MISE.5`'s year of
    frames; the open decisions' pace. The source's opening comment, its notes
    and the Day, Dice, Market, Hustle, Ledger and views' comments;
    `kitchen.vla`'s `as-of`; the caveats' file; the hand's module comment, its
    calendar, path, constants and tests; `oracle/tests/cartridge.rs`' module
    comment and the sessions' comments; the describe golden; `Delta/CLAUDE.md`'s
    calendar paragraph; the memory.

16. **The hand test, the owner's, on the page built here:** the Portfolio's
    clock at 9:30, 9:35 and on, five minutes a half second, the prices with
    it; the session's 39 seconds; 16:00 and the night at an hour a second, the
    clock's minutes turning fast; Tuesday's bell 17.5 seconds after the close;
    the factor written in the drawer, 1 and then 6, and the swings with it;
    the ticks' flicker judged (decision 3's fallback); Gigs' due to the five
    minutes; the Day sheet's paces and the caveats' rows 13, 14 and 19;
    Build's bytes the file's.

**Found at the scoping**, none minted:
- *The license's extended hours.* Its item decides what prices do outside
  the session, where they stand at the last close today, a pre-market price
  wanting a path of its own; and that five-minute ticks fall two and a half
  frames apart at the pace between sessions, an hour a second, twelve ticks
  a second where the session's are two.
- *The closes' texture is `POT.5`'s.* With the first week inside the first
  five minutes of play, the closes at the seed move 0.09% across it, rung 4
  $100.00 to $99.93, and slide 1.0% across the first hour's eleven weeks, a
  smooth line (*derived*, verified at the build); only `POT.5`'s walk gives
  them texture, and its place in the order is the owner's call.
- *A scored run's pace.* Whether a scored run fixes the paces and the rate at
  its start, as `POT.6` fixes the tier, is `CRITIC.1`'s to scope.
- *The bound and the factor.* No price reaches zero while the five minutes'
  swing times 39 stays under 1, a factor up to 22 at the half hour's 0.0028;
  a person's edit past it can, at some seed, in principle.
- *The time spelled twice.* `Day!B16` and `Hustle!B19` spell a time of day
  alike, each inside a formula of its own; a third writing would bring a verb
  to `kitchen.vla`.

**The predictions, before any number.** The numbers column is filled at the
build, P9 at the owner's hand test.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 1,185 to 1,200 rows where 1,048, about 1,193: the Dice's 65 labels more, the Market's 65 session labels, its two settings' cells and one working with its label, the Day's six, caveat 19's four; 785 to 795 formula cells where 526, about 789: the Dice's draws and uniforms 130 more, the session's steps and sums 130, the Day's two and the Market's one | *Measurement*: the runner printed `(rows 1192) (loads yes)`, one under about 1,193; 789 formula cells, as about 789 predicted, the describe golden's count |
| P2 | the describe golden | `(budget 30 789 23670 2000000)`, give or take five cells; the plan's shapes 204 to 208 where 203, `(every-frame 0)`, `(cycles 0)`; levels 34 or 35; the readers of `Day.last` 11 or 12 where 10, `Clock` 4; the Market's and the Dice's readers rows as before but for their counts; every shape renumbered from the Day sheet's on | *Measurement*: `(budget 30 789 23670 2000000)`, exactly; `(plan (shapes 206) (texts 203) (formulas 789) (levels 98) (runs 44) (lanes 256) (alone 533) (every-frame 0) (cycles 0))`; `(readers "Day.last" 12)`, `(readers "Clock" 4)`, `(readers "Dice" 10)`, `(readers "Market" 124 (most 8))` where 57 and `(readers "Caveats" 81 (most 8))` where 77; the shapes renumbered from the Day sheet's on. **Missed** in one: the plan's levels 98 where 34 or 35, the day's 82 draws a chain each reading the one above, 82 levels from the die, then their uniforms, the steps and the 78 steps so far, another chain, so that the index now stands at level 91 and the views after it; `alone` 533 where 270, the chains' 319 cells each evaluated alone, as the thirteen were |
| P3 | the pass and the load | the pass 0.5 to 1.5 ms longer than `HEAD`'s; the load 0.5 to 1.5 ms longer | *Measurement*, one release runner over `HEAD`'s texts and this tree's (the source, the cartridge, the library and the caveats), six runs alternated, 200 passes and 200 loads a run, pinned to the performance cores and opted out of Windows' throttling, a fixed computation before and after within 1.1%: the pass 9.14 to 9.20 ms where `HEAD`'s took 8.04 to 8.14, the medians 1.05 apart, inside. **Missed** in one: the load 7.72 to 8.01 ms where 7.49 to 7.57, the medians 0.36 apart, under the 0.5 priced |
| P4 | the step | an ordinary frame within 0.3 µs of `HEAD`'s; a tick within 15% of `HEAD`'s mark; a five minutes' turn between sessions within 20% of `HEAD`'s half hour's turn; the bell and the close within 15% of `HEAD`'s; midnight 10 to 25 µs above `HEAD`'s, its 65 more draws, steps and sums; the first frame 30 to 80 µs above `HEAD`'s | *Measurement*, the same runs, ten days of each text's own frames, the exam waived, 6,914 at `HEAD`'s pace and 14,714 at this one, each frame filed in a first pass by what its own cells say turned and timed in a second that reads nothing, three times over: an ordinary frame 2.3 µs where 2.0 to 2.1, inside at its edge; a tick 32.2 to 33.0 µs where `HEAD`'s mark took 31.1 to 31.8, inside; a five minutes' turn between sessions 13.3 to 13.6 where `HEAD`'s half hour took 13.0 to 13.3, inside; the close 37.2 to 39.2 where 34.8 to 35.7, inside; the first frame 287 to 294 µs where 258 to 267, the medians 31.5 apart, inside at its edge. **Missed** in two: midnight 52.0 to 62.5 µs where 25.9 to 30.0, the medians 31.5 apart, over the 25 priced, its 260 more cells each alone at about a tenth of a microsecond; and the bell 27.2 to 27.5 µs where 22.7 to 22.9, a run's 35.4 set aside, 20% over, past the 15% priced, the session's own cells, in session and the pace now, turning with it |
| P5 | the suite | 75 to 77 tests where 73; the weekday every frame 0.8 to 1.6 s alone; `PLATE.1`'s and `PLATE.2`'s sessions 1.2 to 2.2 s each, to the closing bell's frame 1,171; the hour's year within 30% of `HEAD`'s, its windows wider; the whole on eight threads within 1.5 s of `HEAD`'s | *Measurement*, the two test binaries alternated, each pinned and opted out, two rounds: 76 green, the crate's 29 and the cartridge's 47, inside; the weekday at every frame 0.88 to 0.89 s alone, where the ten days took 3.07, inside; `PLATE.1`'s session 1.50 s and `PLATE.2`'s 1.63 to 1.64 where 0.27 and 0.29, inside; the hour's year 6.23 s where 5.50 to 5.53, 13%, inside; the whole on eight threads 6.93 to 7.09 s where 6.19 to 6.23, inside. At the first green run the hour's year took 9.50 s and the whole 11.34, each frame parsing the day's 82 draws and 78 steps afresh; the build holds them whole at each day's turn and byte for byte between, as the Portfolio is held between its ticks (below). The hundred years 6.12 to 6.31 s where 5.07 to 5.12, the year at difficulty 0 2.38 to 2.40 where 1.95 to 1.97, the eight hundred days 1.22 where 0.65, a frame a day turning the day at every frame; the hustle's session 0.62 to 0.64 where 0.31 to 0.32, its frames doubled |
| P6 | the page | the cartridge's block 8,000 to 12,000 characters longer, the source's 6,000 to 10,000, the caveats' 250 to 400 | *Measurement*: 2,473,548 characters where `HEAD`'s texts make 2,462,137, built in scratch with the same module: the caveats' block 296 longer, inside. **Missed** in three: the cartridge's 5,749, under its 8,000, and the source's 5,233, under its 6,000, the 130 labels of the draws and the five minutes shorter than priced; and the library's 133, not priced at all, its comments, as `MENU.1` and `POT.10` once forgot it; the four adding to the page's growth exactly |
| P7 | the checks | four checks and four controls green, none edited, no file added under `src/` | *Measurement*, as predicted: ALL GREEN, four checks and four controls, none edited, no file added under `src/`; `cargo fmt --check` and `cargo clippy -D warnings` clean |
| P8 | the numbers given | the real-time figures of decision 1 exact; at the seed over day 0's first 300 market minutes at five-minute ticks, the $1 share moving on 10 of 60 ticks at a factor of 1 and on 26 at 4, from $0.97 to $1.02, the $100 share on 59 of 60, $98.84 to $100.93, and half-hour marks at a factor of 1 moving the $1 share 2 of 10 times, as the deliberation's scratch found them; the closes 0.09% across the first week and 1.0% across the first hour | *Measurement*, by the hand at the seed in a scratch runner, as predicted: the frames of the game's pace put day 30's midnight at 23:45.5 of play, the first dividend's day and the quarter's end at 1:11:03.0, day 365 at 4:47:13.0, the first session's close at 0:39.0, Tuesday's bell at 0:56.5 and the second Monday's at 5:30.5; at five-minute ticks over day 0's first 300 minutes, the $1 share moving on 10 of 60 ticks at a factor of 1, $0.99 to $1.01, and on 26 at 4, $0.97 to $1.02; the $100 share on 58 of 60 at 1 and 59 at 4, $98.84 to $100.93; and `POT.10`'s half hours at 1 moving the $1 share 2 of 10 times; the closes: day 4 at 0.99913, rung 4 $99.93, and day 77 at 0.98970, 1.03% down. And the year's lowest tick, 0.95006 on day 39 at 12:25 at the factor of 4 (0.98233 at 1), where its lowest close is 0.98943 on day 91 |
| P9 | the hand test | the clock and the prices turning together twice a second through a session of 39 seconds; the night at an hour a second; the factor in the drawer changing the swings and no close; Gigs' due to the five minutes; the caveats' three rows; Build's bytes the file's | The owner's, on the page built here (`web/index.html`, 2,473,548 characters), 2026-10-19: two moves a second kept, "it feels more like a live, actual, active market"; the Day sheet on Saturday at 2:40, its paces 120 and 20, `B29` 0 and `B30` 120, the time 441,840 seconds and the bell having waited 362 frames; the Market's factor and its session's rows; the caveats' rows read "fine for now"; Build's bytes the file's, "same bytes (80804 characters)"; then the factor set at 16 and the clock between sessions to the half hour, the owner's words, and the commit (the paragraph below) |

**What the build found.** The pace held as the decisions drew it. The time
summed over its twin met the hand's clock, counted five minutes at a time in
seconds, at every frame of every run from the first: the weekday and its
night at the game's pace, 1,170 steps of twenty seconds landing on the
closing bell and 525 of two minutes on Tuesday's opening one; the hour's
year, the eight hundred days and the hundred years at their own paces, both
paces written; the exam's frames, the time standing at 9:30 through the wait
by a pace of none, the game's first frame there too and 9:40 thirty frames
on; and the hustle at a frame each quarter hour, where every one of its four
claims fell at a quarter past or a quarter to, so that each due is spelled to
the minute, where three were priced. The hand's own tests hold the calendar's
turns at the frames the arithmetic gives, 1,171 the closing bell, 1,696
Tuesday's, 9,916 the second Monday's and 516,991 the year's turn, a step that
crosses the bell ringing it within the step, every open and every close of the
first year to the bit at every five minutes, and the bridge's widest stray of
39 steps by the extreme of a sum linear in its steps, the rule held first
against all 8,192 patterns of thirteen. The rungs' bound no seed passes falls
to 8, 72, 685 and 3,786 cents at the factor of 4, where it stood at 11, 94,
818 and 4,327. Fifteen mutants, each written into the source or the caveats'
file by one anchor, applied once and expanded before its test ran, each failed
the test named for it: the session's pace the one between and the pace between
off the bells, the weekday's first frame; the bell's wait walked, the exam's
first frame; the pace keyed on the open, `B19` written open, at the 1,171st
frame; the time summed over this frame's pace, refused at the pass as a cycle;
the tick at the half hour, the clock to the half hour, the steps unscaled, the
bridge unpinned, the draws 17 a day and the tick's `CHOOSE` a step behind, the
weekday; the factor not read, the factor written 1; the due spelled to the
half hour, the hustle at its first claim; the weekend in session, the hour's
year at Saturday's 9:30; and caveat 19's place empty. At the first green run
the suite's whole took 11.3 s where `HEAD`'s took 7.3, the long runs parsing
the day's 82 draws and 78 steps at every frame; held whole at each day's turn
and byte for byte between, as the Portfolio is held between its ticks, the
dice's and the table's records compared as texts, the hour's year fell from
9.5 s to 6.2 and the whole to 7.0 (P5). Natively a tick costs what a mark did
and a five minutes between sessions what a half hour did, an ordinary frame a
quarter of a microsecond more, and midnight twice its cost, its 260 more cells
of the dice and the session's table each alone (P4): at the game's pace about
130 µs of stepping a second in session and 200 between, natively.

Found at the build, none minted:
- *The plan's depth.* The day's draws are a chain, each the one above times
  16807, and the steps so far another, so that the plan stands 98 levels deep
  where 34 and the index now at level 91. Nothing measured pays for it beyond
  midnight's cells; a draw found by its place from the die, the hand's way,
  would let the dice compute in lanes, at the price of a jump's multiplier a
  draw, `16807^k` modulo the prime, in a column of constants.
- *The clock between sessions.* At an hour a second the clock to the five
  minutes turns twelve times a second, and the views' date with it, 2,125 of
  ten days' 14,714 frames each a patch of every view's date (P4's turns): the
  hand test's to judge, decision 6's alternative one cell's spelling.

**The owner's hand test.** On the page built here, 2026-10-19, over three
screenshots: the Day sheet on Saturday at 2:40, frame 8,635, its paces 120 and
20, out of session and the pace now 120, the time 441,840 seconds and the bell
having waited 362 frames; the Market sheet at frame 8,976, the factor 4 at `B6`
and the session's rows from "9:30 to 9:35"; and the Caveats sheet's rows 13, 14
and 19. Build's bytes the file's, "same bytes (80804 characters)". Decision 3
kept, the owner's words: "Two moves per second is fine, it feels more like a
live, actual, active market this way." Decision 10's drafts kept: "The caveat
reads fine for now. I expect them to change as the game matures to account for
gaps in realism." And the factor's number opened again: "I'm almost tempted to
bump intra-day volatility factor to 16 to promote day trading to alleviate the
feeling of waiting for assets to grow over time. Day trading is, after all, the
most common activity in the real world market which traders use to stave off
the boredom of value investing (wink)." Asked of the hand test, not of this
dish: "Why does changing the column width on one sheet change the same column
on every other sheet? I thought that was already minted and addressed in an
Alonzo item where each tab becomes its own independent sheet?" Answered: Alonzo's
`ENGINE.18` gave each view the widths its own record names; the width a person
drags is one map for every sheet, keyed by the column's letters
(`web/viewport.js` at `v0.2.0`, `state.widths`), which `PLATE.1`'s hand test
found and `MISE.6` holds as a finding for Alonzo, never minted there, its next
free `ENGINE.26`.

What a factor of 16 does, by the hand at the seed over the first year at a
frame each five minutes (*derived*, beside 4 and 8): the day's widest stray
from its line 6.91% at p50 and 17.33% at the most, where 4 strays 1.73% and
4.33%; the year's lowest tick 0.8210, on day 39 at 12:25, where 4 reaches
0.9501, so the kind year's floor, `KIND_TICK`, would fall to 0.82; the $1 share
moving on 39% of ticks where 21%; and the rungs' bound no seed passes 1, 15,
239 and 1,719 cents, the $1 share's a cent, every price above zero up to a
factor of 22. And what it pays: the session is a bridge to a close fixed by the
waves before the bell (caveat 14), shown at `Market!B26` from the bell, and the
next bell opens at it to the cent, so a share bought below the close is sold at
or near it for a gain known in advance. The best such trade a session, bought at
the session's lowest tick below its close, averages at p50 2.9% on the $1 share
and 1.0% on the $100 at a factor of 4, 5.4% and 2.1% at 8, and 11.4% and 4.3% at
16: day trading made riskless, its gains growing with the factor, until
`POT.5`'s walk lets a close wander from the waves as far as the hours do.

Then, the same day, the owner's words: "Factor of 16 approved; only show the
half hour while the market is closed; then commit before suggesting next steps
from Buffet's OMAKASE.md." So decisions 5, 6, 7, 10 and 11 were amended, each
dated above: the factor 16; the time shown a cell of its own, `Day!B31`, the
five minutes in session and the half hour between, keyed on the bells as the
pace is; the hustle's due a day from the time shown; caveat 19 at sixteen times
and a twentieth naming the sure dip, drafts; and the kind year's floor 0.82.
What moved: 1,198 rows and 790 formula cells, `(budget 30 790 23700 2000000)`,
`(plan (shapes 207) (texts 204) (formulas 790) (levels 98) ...)`; 76 tests
green, the test writing `B19` open now holding the time shown too; eighteen
mutants caught, four new, each by the test named for it: the clock to the half
hour in session too and to the five minutes between sessions, the weekday's;
the clock keyed on the open, the open test's at 16:05, `B31` 965 where 960; and
the due counted from the model's five minutes, the hustle's at its claim seen at
0:15 on day 7. Natively, the same runner over `HEAD`'s texts and these, six runs
alternated: a five minutes' turn between sessions 2.6 to 2.7 µs where it took
13.3 to 13.6 with the clock to the five minutes, the views' date now turning at
the half hour alone, so that a second between sessions steps in about 75 µs
where 200; a tick 33.5 to 34.0 where `HEAD`'s mark took 31.1 to 31.7; midnight
50.4 to 59.3; an ordinary frame 2.3. The page 2,475,244 characters, its blocks
the files'. Committed at the owner's word.

## `FRONT.3` — the overview, the ticker, the weather word: where the certificate leads, two net worths, a word and a headline

*Scoped and built 2026-10-19 at the owner's word, "Scope and implement FRONT.3
in Buffet (Delta/Buffet): the overview, the ticker, the weather word", in one
sitting as `PLATE.1`, `PLATE.2` and `POT.11` were: each decision below stands
as recommended and was built, the owner's to approve, amend or strike at the
hand test, and every text a player reads is a draft. Minted already; nothing
else was minted, and what else the scoping found stands under Found. Read for
it: the item's line in `OMAKASE.md` and its Found note; `BD-2`, `BD-4`, `BD-5`,
`BD-6` and `BD-7`; the opening contract; `BACK.2`'s and `MENU.7`'s lines;
`FRONT.1`'s records above (no sheet stands empty waiting, a view reads the model
and nothing reads a view, at most 1,000 formula cells a view); `MENU.1`'s (the
content sheets by `strings` and `picker`, the fine print's row by the day, the
names table keeping "Headlines"); `GUARD.1`'s decisions 10 and 11, the Camera's
`B3` and the lead; `POT.2`'s finding on the weather word; `POT.11`'s record;
`PLATE.1`'s and `PLATE.2`'s; `BUFFET.md`'s course 0 row and its Headlines
appendix, as design reference; the source's views, the library's verbs and the
Market's look back; the tests' views and their holding; and Alonzo's `SPEC.md`
at `v0.2.0`, the tag the pin names, §3.8. Alonzo's and Frazaro's trees were read
and never touched.*

**What it stands on, read on the day** (Buffet at `7386c3d`, three ahead of its
remote, the tree clean; `alonzo` 0.2.0 and `vla-lang` 0.10.0).
- *The lead.* `Proctor!B6`, a value, "Portfolio", which the Camera's `B3` names
  once the game begins (`GUARD.1`'s decisions 10 and 11), so one value moves the
  game's first view after the exam. The host shows a sheet when the Camera's
  third row changes and names a sheet the grid holds, and between its changes
  the person's tabs win (§3.8).
- *The look back.* `Market!B4`, twenty days; `C15` `=B15-$B$4`, twenty days
  before the session's day, its column the index's own fill, read by nothing in
  the cartridge (grepped). The session's close, `B26`, is the waves' index of the
  session's day, known from its opening bell, which the price shown reaches only
  at the closing bell (caveats 14 and 20); the last close, `D26`, the session
  before's.
- *The weather word's finding* (`POT.2`): read off the slope's sign alone it never
  turned in seed 777's first year; at seed 791886 the index's twenty-day slope
  first turns on day 102 (the hand's kind-year test) and again on days 1,105 and
  1,681 in five years.
- *The dice.* 82 market draws a day, `POT.2`'s four phases (read on day 0 alone)
  and `POT.11`'s 78 steps; the next free is the 83rd (the source's rule: a dish
  that needs a draw takes the next free one, and none an earlier dish reads
  moves).
- *The strings.* `MENU.1`'s `strings` and `picker`; its names table kept
  "Headlines" for the ticker's headline of the day, read by the Overview; a
  content file a sheet, hidden, called after the sheets row; `check_layout.ps1`
  pins content sheets at 4.
- *The views.* Five, the Exam, the Certificate, the Portfolio, Trading and Gigs,
  each opening on the library's `as-of` and `bell` and closing on `fine-print`,
  their dollars by `dollars` in 12-character columns. The Portfolio already
  shows each pocket's cash on hand and net worth, `Cash!B3:B4` and `B9:B10`
  (`GUARD.1`'s hand test, `PLATE.1`).
- *The suite.* 76 tests green, the crate's 29 and the cartridge's 47.

**The decisions.**

1. **The view: the Overview, third among the tabs, and where the certificate
   leads (rec).** A fund manager's word for the page that sums the book up, and
   the records' own (`FRONT.1`'s views, `MENU.1`'s names table, `GUARD.1`'s
   decision 11). It stands after the exam's two views, before the Portfolio,
   `(sheets "Exam" "Certificate" "Overview" "Portfolio" ...)`: the strip reads
   the views in the order a player meets them, and the game still opens on the
   first visible sheet, the Exam, its opening rule unchanged. `Proctor!B6` reads
   `Overview`, one value and no formula moved, so the certificate's button lands
   there (`BD-4`: two numbers, a word and a headline make the first screen after
   the exam, the Portfolio's 32 cells of stocks and holdings one tab over).
   *The alternatives:* the Summary, Excel's word, or the Dashboard, software's,
   or a Tear Sheet, a fund's one-pager few players know; the Overview first of
   all, the opening stated by a `(value "Camera" "B3" "Exam")` row (`GUARD.1`'s
   named alternative), the strip's head after the exam held by a row a later
   edit of the Camera must keep; the lead kept on the Portfolio, the Overview a
   tab the player finds.

2. **What it shows at course 0, and what it leaves (rec).**

   | | A | B |
   |---|---|---|
   | 1 | the date, every view's | |
   | 2 | the bell's line, every view's | |
   | 3 | `Your firm: name pending registration.` | |
   | 5 | `Net worth ($)` | |
   | 6 | `  Firm` | its net worth, `Cash!B9` |
   | 7 | `  Personal` | its net worth, `Cash!B10` |
   | 9 | `Market weather: Overcast, the market down 0.5% in the 20 days to the last close.` | |
   | 11 | `Today's headline: SEC announces new rule against funny business; definitions to follow.` | |
   | 13 | the fine print | |

   Column A 22 characters and B 12, the width the dollars are padded for; seven
   formula cells, where a view may hold 1,000. The firm's line is a value, a
   placeholder with its own wink, regulation's pace, until `BACK.2` names the
   firm from the Firms sheet beside the style and makes the cell a formula. Each
   pocket's net worth is the line's own; the Portfolio keeps the stocks, the
   holdings and the cash on hand, which the Overview does not repeat.
   *The alternatives:* the style answered in the name's place, `Firms!D3`, "I read
   the room", which the Certificate already prints and which reads as no name;
   each pocket's cash and holdings beside its net worth, the Portfolio's block
   twice; the change of each net worth since the certificate, the report card's
   (`CRITIC.1`).

3. **The weather word: by the change's size, at the last close made, five words
   (rec).** The Market sheet's foot, rows 114 to 124: a table of words, each
   beside the change in whole tenths of a percent it begins at, Sunny from 10,
   Partly sunny from 1, Calm at 0, Overcast from -9 and Rain below them all,
   settings a person may edit (`AD-6`); the last close made, `B121`
   `=IF(Day!$B$29=1,D26,B26)`, the session before's while the session trades
   and the session's own from its closing bell; the change over the look back,
   `B122` `=B121/C26-1`; in tenths, `B123` `=INT(B122*1000+0.5)`, rounded half up;
   and the word, `B124`, the first whose tenths the change reaches, from the top.
   The look back moves with it, `C15` `=IF(Day!$B$29=1,D15,B15)-$B$4`, twenty days
   before the last close made. So the word changes at most once a session, at its
   close, keyed on the bells as the pace is (`BD-7`, a license's hours moving no
   weather), and never reads a close the session has not made: the open book's
   `B26` stays the Market's own (caveat 20), off the player's first screen. The
   Overview spells it with the change it is read off, up, down or flat, in tenths
   of a percent over the look back's days, `B4`, so a person who edits the look
   back reads its days. The words are plain, a line a player reads at a glance,
   as the bell's lines are; the jokes are the caveat's. The thresholds, chosen
   from the hand's measurement after the predictions were written: at the seed
   the word reads Overcast at the first bell, the market easing into the year's
   dip, then Calm from day 93's close, Partly sunny from day 112's and Sunny from
   day 263's; over the seed's century of trading days 4.9% Rain, 23.4% Overcast,
   4.5% Calm, 42.1% Partly sunny and 25.0% Sunny, and over 2,000 seeds' first five
   years alike, 1.3 changes a year. *The alternatives:* the slope's sign alone,
   two words that turn three times in five years at the seed, once in the first
   80 minutes of play and then not for thirteen hours; `POT.2`'s `B26` less `C26`,
   the session's close against twenty days before it, which reads at the bell the
   close the session has not yet made; the session before's close always, `D26`,
   a session late from each close to the next bell and Thursday's all weekend;
   thresholds of half a percent, 48% Sunny and 15% Rain over the century, or of a
   percent and a half, Rain on 0.7% of days.

4. **The ticker: the Headlines sheet, ten rows, a headline a calendar day by the
   day's 83rd market draw (rec).** `src/content/headlines.vla`, `MENU.1`'s shape,
   one field, `Headline`, 100 characters wide, hidden after the Firms sheet; its
   row shown, `D2` `=INT(Dice!$C$84*D1)+1`, the uniform of the day's 83rd draw
   times the rows, from the first, the Dice sheet's draws growing from 82 to 83,
   so that a row added joins the draw with no formula edited; the Overview reads
   its picker, `D3`. A headline a calendar day, weekends among them, changing at
   midnight with the die, drawn where anyone can watch it roll (`BD-3`), so
   tomorrow's is no rota a player counts off. Course 0's headlines are color
   alone and move nothing until `SPECIAL.1` gives the ticker teeth. A draw repeats
   yesterday's on about one day in ten (days 7 and 8 at the seed). *The
   alternative:* a headline a day round the rows, as the fine print's month is,
   no draw, the same ten in the same order every ten days, eight minutes of play.

   The ten, of `BUFFET.md`'s fifteen, each held to `BD-2`:

   | The seed (`BUFFET.md`, Appendix A) | The verdict |
   |---|---|
   | "Fed holds rates steady; markets read it as 'a sort of hug'." | kept: punches up at a market that needs reassurance |
   | "Economists predict nine of the last three recessions." | kept: the oldest joke in the profession, and it teaches humility about forecasts beside a weather word |
   | "Analyst upgrades stock to 'buy' after discovering he owns it." | rewritten: "... after discovering they own it." |
   | "Soybean futures bean-up after surprise harvest." | struck: a forced pun, `BD-2`'s weak comedy |
   | "Crypto exchange loses all customer funds, describes it as 'a learning process'." | struck: the target is the exchange, but its customers' losses are real ones, a line the policy keeps (never at people who lost their savings) |
   | "Board assures investors the losses are merely unrealized, 'like everything else'." | kept: the Portfolio shows an unrealized gain beside it |
   | "Yield curve inverts; bonds observed walking backward." | held for course 1, when the curve exists (`POT.3`) |
   | "SEC announces new rule against funny business; definitions to follow." | kept: the first game's first headline, day 0 at the seed |
   | "Company announces 'strategic pivot to AI'; stock up 40%; company still sells ice cream." | kept |
   | "Local man buys everything at the top; market tops." | kept: an archetype, and ourselves |
   | "Hurricane season: insurers 'cautiously terrified'." | held for course 6's insurance: beside a weather word, a hurricane reads as the market's forecast (clarity first) |
   | "Tulip prices surge; historians sigh." | kept |
   | "Intern discovers rounding error worth $4 million; asks about a bonus; there isn't one." | kept: the spreadsheet's own wink |
   | "Private-island index hits record; sea levels also." | struck: the one seed whose punchline is not finance |
   | "Gold remains calm as the world panics; the world finds this annoying." | kept |

5. **The alerts and the strategy guide: each waits for the dish that gives it
   something to hold (rec).** At course 0 nothing can alert as the line's three
   do, no margin (`PLATE.4`, course 3), no maturity (`PLATE.3` and `STATION.2`,
   course 1) and no redemption (`BACK.3`, course 2); and nothing a player buys has
   both a cost and an income, the two numbers a payback time divides, until
   course 2's staff (`BACK.5`) or a later gig. So the Overview keeps no row for
   them, as no sheet stands empty waiting (`FRONT.1`), holds no button and binds
   no key, having no act of its own: an alert that links to its panel is an act,
   and enters with the dish that raises it, its key and its command holding on
   the Overview beside it. *The alternative:* a column of dividend payback times
   now, the price over a year's dividends, the $100 rung's fifty years and the
   rest never, a price-to-dividend ratio by another name for `MENU.8` to read.

6. **The caveats (`BD-5`), drafts.** Row 21, the weather's, its place `Market!B124`:
   "The weather is one word for one number: the market's change in the twenty
   days to the last close." beside "A trend says where prices have been, not where
   they will go; forecasters weigh many signals and still miss turns." and
   Management's response, "Management checks the forecast in hindsight, where it
   has never been wrong." Row 22, the headlines', its place `Headlines!D3`: "The
   news moves nothing: a headline here is color, and no price reads it." beside
   "Prices react to news, often within seconds, and traders pay to read a headline
   a moment sooner." and "Management reads only the headlines, which the Series
   777 recognizes as a style." Both real-world lines marked for `MENU.8`'s read.

7. **The oracle (rec).** The hand shares no rule with the cells: the cells choose
   the last close made by whether the clock stands between the bells, `Day!B29`,
   and the hand by how far its session has ticked; the cells nest comparisons from
   the top and the hand counts the words whose tenths the change reaches; the
   change itself is the market's, taken in the cells' order as the waves are. The
   cells pick the headline by the draw's uniform times the rows, rounded down, and
   the hand divides the draw times the rows by the modulus in whole numbers, the
   two the same at every draw (*derived*: the modulus is prime, so the quotient is
   never whole and stands at least 4.7e-10 from the next whole number, where the
   double's two roundings move it less than 4e-13 for any sheet a picker holds),
   held at every day of the seed's century at twenty-one counts of rows. The runs:
   the Overview at rest and the Market's weather held to the hand at every bell and
   every day's turn of the long runs, the weather reading the session before's
   close at the bell and the session's own outside it; the look back's column in
   the waves' workings, held whole at each session and each close; the Overview at
   every frame of the order desk's sessions, its net worth the holdings at the
   price now; the weather at every frame of the run that writes `B19` open, to
   half past four on Tuesday, Monday's session being its own session before; the
   headline at every day's turn; the hand's first year at a frame each five
   minutes, the weather turning three times, each at a close; and a test of the
   view, its table and its keys, and one of the Headlines sheet. Mutants, each
   into the source or a content file by one anchor, each failing the test named
   for it: the weather reading the session's close in session; the look back at
   the session's day; the weather keyed on the open; a word begun past its tenths;
   the tenths rounded down; the table's words out of order; the sun from half a
   percent; the flat line spelled otherwise; the headline round the rows; the
   headline from the session's last draw; the draws 82 a day; the certificate
   leading to the Portfolio; each net worth its cash alone; the Overview reading a
   view; P bound on the Overview; the Overview after the Portfolio; and each
   caveat's place empty.

8. **What it costs, measured (rec).** Natively, as `POT.11` measured it: a release
   runner over `HEAD`'s texts and this tree's, alternated, pinned to the
   performance cores and opted out of Windows' throttling; the pass, the load, the
   first frame, and ten days of frames filed by what each turned, a half hour's
   turn between sessions now filed apart from a five minutes'. The suite, the two
   test binaries alternated. The browser at the hand test.

9. **The stale places, rewritten at the build.** In `OMAKASE.md`: the line (🟡);
   `BD-5`'s taken rows; `BACK.2`'s line, the placeholder at `Overview!A3`;
   `MENU.7`'s, the Awards on the Overview; `SPECIAL.1`'s, the ticker's draw;
   `POT.4`'s, the weather's table it replaces. The source's served list, its notes,
   the tabs', the Dice's, the Market's, the Proctor's and the Portfolio's comments,
   `Proctor!B6` and its comment; the caveats' and the headlines' files; the
   content's `README.md`; `check_layout.ps1`'s pin and control; the hand's module
   comment, its draw, its lead and its tests; `oracle/tests/cartridge.rs`' module
   comment, the views, the content sheets and the holders; the describe golden;
   `Delta/CLAUDE.md`'s views paragraph; and the memory.

10. **The hand test, the owner's, on the page built here:** the strip and the lead;
    the Overview at the bell, its date, bell, firm line, net worth, weather and
    headline; the headline at the first midnight; the net worth after a purchase;
    the weather turned by the paces written a day a frame; the drafts read; no
    key on the Overview; Build's bytes the file's.

**Found at the scoping**, none minted:
- *A link's mechanism.* The host shows a sheet when the Camera's third row
  changes (§3.8), so a link that sets `B3` to its panel shows it once: clicked
  again after the person tabs away, `B3` still names it and nothing moves. A
  Camera whose `B3` follows the sheet shown, `Input!B19`, between links, and
  names the link's panel at its frame, shows it at every click. The first
  alert's dish settles it; no ask of Alonzo is needed.
- *The weather is slow at tier 1.* About 1.3 changes a year over 2,000 seeds, the
  first game's first hour reading one word: the waves are smooth, and `POT.5`'s
  walk is what gives the closes texture, its place in the order the owner's.
- *The headline repeats.* About one day in ten shows yesterday's headline again.
  A draw among the other nine wants yesterday's row as state over the twin;
  `SPECIAL.1`'s schedule replaces the draw.
- *The Market sheet's labels.* Its column A keeps the default width, 8.43
  characters, so every label beside a value is cut where B begins, the
  weather's "Partly sunny" among them; a column row would show them whole, the
  looks' pass or the next Market dish. (*Seen at the hand test:* its numbers too,
  the session's steps and sums, the five minutes' swing, the index now and the
  weather's last close and change, each drawn `#######` in a column too narrow
  for its digits.)
- *The first screen's call to action.* The Overview names no act a first player
  can take, the tabs doing it; lines such as "a prospectus awaits" or "the
  market is open" are alerts of a kind at course 0, the owner's call.

**The predictions, before any number.** Written before any number was taken;
the numbers column was filled at the build, P9 at the owner's hand test.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 1,250 to 1,270 rows where 1,198, about 1,259; 803 to 808 formula cells where 790, about 805 | *Measurement*: `(rows 1258) (loads yes)`, one under about 1,259; 805 formula cells, as about 805 predicted, the describe golden's budget |
| P2 | the describe golden | `(budget 30 805 24150 2000000)` give or take three cells; shapes 214 to 222 where 207; `(every-frame 0)`, `(cycles 0)`; levels 98; the views' date, bell and fine print at `(sheets 6)`; a readers row for Headlines; Cash's readers 9 where 7; every shape after the Certificate's renumbered | *Measurement*: `(budget 30 805 24150 2000000)`, exactly; `(plan (shapes 216) (texts 213) (formulas 805) (levels 98) (runs 45) (lanes 260) (alone 545) (every-frame 0) (cycles 0))`; `(sheets 6)`; `(readers "Headlines" 13)`; every shape after the Certificate's renumbered. **Missed** in one: Cash's readers 8 where 9, the net worth one fill of two cells and so one reader |
| P3 | the pass and the load | the pass 0.2 to 0.8 ms longer; the load 0.3 to 1.0 ms longer | *Measurement*, one release runner over `HEAD`'s texts and these, six runs alternated, pinned and opted out, the fixed computation within 2.1%: the pass 9.78 to 9.83 ms where 9.11 to 9.29, the medians 0.56 apart; the load 8.45 to 8.73 where 7.96 to 8.11, 0.48 apart. Inside, each |
| P4 | the step | an ordinary frame within 0.3 µs; a tick 1 to 3 µs dearer; a half hour's turn between sessions 1 to 3 µs dearer; the bell 1 to 6 µs dearer; the close 5 to 15 µs dearer; midnight 2 to 6 µs dearer; the first frame 20 to 60 µs dearer | *Measurement*, the same runs, ten days of each text's own frames, the exam waived, three passes each, the medians: an ordinary frame 2.3 µs either way; a tick 35.7 where 34.1, 1.6 dearer; a half hour between sessions 15.5 where 13.8, 1.7 dearer; a five minutes' turn 2.6 either way; the close 49.1 where 39.9, 9.2 dearer; midnight 61.5 where 57.7, 3.8 dearer. Inside, each. **Missed** in two: the bell 29.5 µs where 30.3, no dearer, the look back's column having moved its work from the bell to the close (at the bell the last close made keeps its day, so the column's twelve cells stand); and the first frame 319.2 µs where 299.8, 19.4 dearer, under the 20 priced |
| P5 | the suite | 79 to 82 tests where 76; the hour's year 5 to 15% longer alone; the eight hundred days 5 to 20% longer; the weekday 3 to 10% longer; the whole on eight threads 0.3 to 1.0 s longer | *Measurement*, the two test binaries alternated, each pinned and opted out, two rounds: 81 green, the crate's 32 and the cartridge's 49, inside; the eight hundred days 1.34 to 1.35 s where 1.22 to 1.24, 9% longer, inside. **Missed** in three, each cheaper than priced, the Overview and the weather held at the bells and the days' turns alone: the hour's year 6.41 to 6.43 s where 6.22 to 6.29, 3% longer; the weekday 0.90 to 0.91 s where 0.89 to 0.91, 1%; the cartridge's whole on eight threads 7.34 to 7.43 s where 7.19 to 7.25, 0.15 to 0.18 s longer. Beside them the hundred years 6.47 to 6.50 s where 6.26 to 6.29, the order desk's sessions 4 to 6% longer, the Overview held at every frame of them, and the run that writes the open 0.26 to 0.27 s where 0.11 to 0.12, its frames 2,881 where 1,711 and the weather held at every one |
| P6 | the page | the cartridge's block 5,000 to 8,000 characters longer, the source's 4,000 to 7,000, the caveats' 500 to 800, the headlines' new block 1,800 to 2,500, the library's 0 to 300; the page 12,000 to 18,000 longer | *Measurement*: 2,489,690 characters where 2,475,244, 14,446 longer; the cartridge's block 5,423 longer, the source's 6,486, the caveats' 621, the headlines' 1,811 and the library's none, with the new block's tags. Inside, each |
| P7 | the checks | four checks and four controls green, `check_layout`'s pin 4 to 5 and its clean copy 9 files to 10, the only check edited; one file added under `src/`; fmt and clippy clean | *Measurement*, as predicted: ALL GREEN, four checks and four controls, the layout's control a clean copy of ten files and fourteen mutants; `cargo fmt --check` and `cargo clippy -D warnings` clean once `cargo fmt` had wrapped two calls |
| P8 | the numbers given | the weather word changes 3 to 5 times in the first year, the first before day 110, from Overcast; no word under 3% of the century's days but Rain; each headline on 24 to 50 of the first year's days; the hand's headline rule the cell's on every day of a century | *Measurement*, by the hand at the seed: Overcast at the first bell, "down 0.5%", then Calm from day 93's close, Partly sunny from day 112's and Sunny from day 263's, three changes; over the century Rain 4.9%, Overcast 23.4%, Calm 4.5%, Partly sunny 42.1%, Sunny 25.0%; the first year's headlines 27 to 46 days each; the two rules equal on all 36,500 days. Inside, each |
| P9 | the hand test | the certificate's button lands on the Overview; its lines; the headline at the first midnight, about 47 seconds of play after the bell; the weather turned by the paces; Build's bytes the file's | The owner's, on the page built here, 2026-10-19 (the paragraph below): the Overview after the exam sat at 2 of 10, its firm $6,000.00 and you $100.00, Overcast and down 0.5%, the SEC's headline; the AI pivot's at Tuesday's 0:30; the firm's net worth moving after a purchase; the weather Sunny at 15 tenths once the paces ran a day a frame; the drafts read; passed with no change asked |

**What the build found.** The Overview held as the decisions drew it, and the
suite was green at its first run, 81 tests. The hand's weather met the cells at
every bell and every day's turn of the long runs and at every frame of the run
that writes the open, and its headline at every day's turn. Eighteen mutants,
each written into the source or a content file by one anchor, applied once and
expanded before its test ran, each failed the test named for it but one at the
first run: the weather keyed on the open lived, since Monday's session is its own
session before, so that the first weekday's two closes are one close and its
night reads it either way; the test that writes the open now runs through
Tuesday's close and half an hour past it, 2,881 frames where 1,711, and caught it
at the close, frame 2,866. Natively the look back's column moved its work from the
bell to the close, at the bell the last close made keeping its day: a close now
costs 9.2 µs more and a bell nothing more, though the Overview's date and bell
line turn there too.

**The owner's hand test.** On the page built here (`web/index.html`, 2,489,690
characters), 2026-10-19, over seven screenshots. The strip read Exam,
Certificate, Overview, Portfolio, Trading, Gigs and the model's sheets. The exam
sat by hand at two right answers of ten, the certificate led to the Overview:
at frame 345, Monday at 10:05, the date, "The market is open until 16:00.", "Your
firm: name pending registration.", the firm's net worth $6,000.00 and yours
$100.00, "Market weather: Overcast, the market down 0.5% in the 20 days to the
last close.", "Today's headline: SEC announces new rule against funny business;
definitions to follow." and the fine print. At frame 1,656, Tuesday at 0:30, the
bell's "The market opens at 9:30." and the AI pivot's headline, the day turned;
at frame 2,887, Tuesday at 14:50, the firm's net worth $6,003.40 after a purchase
on Trading, the weather still Monday's close's. The paces written a day a frame,
`Day!B1` at frame 3,148 and `B28` at 3,259, the game paused at frame 3,869: the
Market sheet's weather table, the change at 15 tenths and the word Sunny; the
Headlines sheet's ten rows, `D1` 10, `D2` 4 and `D3` the board's; the Caveats
sheet's rows 22 and 23; the Proctor's `B6`, Overview. The Market sheet's numbers
at its foot drew `#######` in its default columns (Found, above). The owner's
words: "Hand tests pass; commit before suggesting next steps." So every decision
and every draft stands as built. Committed at the owner's word.

## `BACK.2` — the firm's name and flavor, and `MENU.3`'s first slice: the exam's last answer names the firm, and four stocks named

*Scoped and built 2026-10-19 at the owner's word, "Scope and implement BACK.2
in Buffet (Delta/Buffet): the firm's name and flavor, with MENU.3's first slice,
four names", in one sitting as `PLATE.1`, `PLATE.2`, `POT.11` and `FRONT.3`
were: each decision below stands as recommended and was built, the owner's to
approve, amend or strike at the hand test, and every text a player reads is a
draft. Minted already, `MENU.3`'s first slice riding with it by its line;
nothing else was minted, and what else the scoping found stands under Found.
Read for it: the item's line in `OMAKASE.md` and its three notes; `STATION.1`'s
second slice and `CRITIC.1`'s line; `BD-1`, `BD-2`, `BD-4` and `BD-5`;
`GUARD.1`'s record above (the Firms sheet, its decision 13, the style at
`Proctor!B40`, a waiver's style `B5`, the Certificate's style line `B9`);
`MENU.1`'s (Names became Firms, a rung's name on the rung's own row, its
decision 2); `STATION.1`'s decision 2 and `FRONT.1`'s decision 9 (the labels
`rung 1` to `rung 4`, the views reading the model's names); `FRONT.3`'s (the
Overview's `A3`, a value to become a formula; Found: the Market sheet's default
columns); `BUFFET.md`'s §2, §3 and Appendix A, as design reference;
`CONTRIBUTING.md`'s content policy and `TRADEMARK.md`; the source's Firms
section, the Certificate, the Overview, the Stocks sheet and every reader of its
names; `src/content/firms.vla` and the content's `README.md`; and the tests that
hold them. Alonzo's and Frazaro's trees were read and never touched; the drafts'
names were searched on the web for living companies of the same name.*

**What it stands on, read on the day** (Buffet at `af36987`, four ahead of its
remote, the tree clean; `alonzo` 0.2.0 and `vla-lang` 0.10.0).
- *The Firms sheet* (`GUARD.1`'s decision 13). One field, `Style`, 40
  characters wide, four rows, the styles the eleventh question offers; its row
  shown `D2`, `=MAX(Proctor!$B$40,1)`, the style answered or the first before
  it, and its picker `D3`, which the Certificate's `B9` printed, "asked last"
  until the eleventh answer. A waiver's style is `Proctor!B5`, 1, a value.
- *The Overview's firm line* (`FRONT.3`'s decision 2). `A3`, a value: "Your
  firm: name pending registration."
- *The names.* `Stocks!A5:A8`, values, `rung 1` to `rung 4` (`STATION.1`'s
  decision 2), in column A at the default width, 8.43 characters; read by the
  Portfolio's `A5:A8` and the order desk's `A6:A9`, each `=Stocks!A5` filled,
  by the Positions' `C2:C9` through a `CHOOSE` and their `C12:C15`, and through
  the order desk by Trading's `B6:B9`, its picks' and acts' words on the
  Commands sheet, the stock picked and the order's (`Orders!B17`, `B18`, held
  at `B32`), Trading's last order's sentence and the Ledger's stock, the Write
  sheet's fourth row. The columns that show them: the Portfolio's `A` and
  Trading's `B`, 22 characters; the order desk's `A`, 46; the Positions' `C` and
  the Ledger's `D`, 10; the Commands' words, `B`, 64.
- *The oracle.* The tests read the names off the Stocks sheet's cells
  (`stock_names`) and held every reader to them, so the hand had no names of its
  own and a reader was held to the cells it reads.
- *The policy.* `BD-2`: punch up, fictional names, a parody of a living mark
  gets the glance, PG-13, a forced pun weak comedy; `CONTRIBUTING.md`: a parody
  that points at a living company or a real mark is marked `(check)` in the row;
  `TRADEMARK.md`: the name's pun's other half is a living person, whose name,
  face and words the game keeps out.
- *The suite.* 81 tests green, the crate's 32 and the cartridge's 49.

**The decisions.**

1. **The firm's name: a field beside its style on the Firms sheet, the eleventh
   answer picking both (rec).** `src/content/firms.vla` grows a second field,
   `Name`, 30 characters wide, so the sheet's table runs from A to B and its
   values move from D to E, as the library's `strings` lays out a sheet of two
   fields: the rows counted `E1`, the row shown `E2`, `=MAX(Proctor!$B$40,1)` as
   `D2` was, the style `E3` and the name `E4`, beside the labels `D1:D4`, the last
   the field's header, `Name`. One row shown picks both, so the style and the
   name can never disagree, and a contributor adds a firm as one line of two
   fields (the content's `README.md`). The four drafts, held to `BD-2`:

   | The style | `BUFFET.md` §3's draft | Built | The verdict |
   |---|---|---|---|
   | I read the prospectus | Prudent & Sons | Prudent & Sons | kept: the old-line family firm for the reader of the fine print; no firm of the name was found; "Prudent" is a word Prudential's mark shares and points at no company, so no `(check)` |
   | I read the headlines | Headline Capital | Above the Fold Capital | renamed: Headline is a living venture firm, e.ventures until its rebranding in 2021, so "Headline Capital" in a fund manager's game points at it; a front page's headlines run above the fold, and the Overview sets the style beside the name; nothing was found of the name |
   | I read the room | Read-the-Room Partners | Read-the-Room Partners | kept: the idiom as a partnership; nothing found |
   | I feel the market | Vibes LLC | Vibes LLC | kept: the slang as a limited liability company; Vibes, a Chicago marketing company, and Vibe Capital, a venture firm, share a word and are not its target |

   Four legal forms for four firms, a family firm, a capital fund, a partnership
   and an LLC, and no forced pun among them. A waiver names the firm of its
   style, the first, Prudent & Sons, and a person's edit of `Proctor!B5` from 1
   to 4 picks another, which the manifest's notes now say. The Certificate's
   style line becomes the firm's, `A9` "Firm name" and `B9`
   `=IF(Proctor!$B$40=0,"pending registration",Firms!$E$4)`, the placeholder's
   wink kept for the state before the eleventh answer. The Overview's `A3`
   becomes a formula: "Your firm: name pending registration." until then, and
   then "Your firm: Above the Fold Capital. Investment style: I read the
   headlines.", the style printed as a fund's fact sheet prints its investment
   style. And the Exam's eleventh line says what the answer does: "The Series
   777, question 11 of 11, ungraded: it names your firm.", where it said "it
   sets your firm's style"; the options stay the styles, so the name is the
   certificate's reveal. *The alternatives:* the names as the eleventh's
   options, the reveal lost and the question no longer the candidate's
   self-description; the certificate keeping the style beside the name, a row
   more and its button moved; a waiver's firm of its own, a fifth row the exam
   never offers; "Headline Capital" kept and marked `(check)`, or "Front Page
   Capital".

2. **The flavor, never the power: words alone at course 0 (rec).** The style
   changes two lines a player reads, the Certificate's name and the Overview's
   firm line, and no number, rule, key or price. At course 0 every mechanic is
   a number, the capital, the fills, the prospectus's fee, the prices, so any
   flavor that was not words would be power (`BD-4`), and a style that moved a
   number would split the field into four games where a score compares on the
   same seed (`BD-3`, `CRITIC.1`). Held two ways: no formula of another sheet
   reads the style or the Firms sheet but the Firms sheet's row shown, the
   Exam's options, the Certificate's name and the Overview's line; and the four
   styles' waived games, cell for cell over every model sheet, the style's two
   cells excepted, are one game. Later dishes may color by the same row in
   words: the report card's header (`CRITIC.1`), Mildred's memos (`MENU.4`), the
   rivals as the other archetypes (`CRITIC.2`). *The alternatives:* a motto, a
   third field, four more strings to review and the Overview's line longer; an
   archetype's own lines on every view, a `CHOOSE` in each; a flavor of
   mechanics, a default pocket or a headline kind by style, power by another
   name.

3. **`MENU.3`'s first slice: the four stocks named on their own rows (rec).**
   `Stocks!A5:A8`, where the labels stood (`MENU.1`'s decision 2: a rung's name
   belongs on the rung's own row), each a fictional company of its rung's kind:

   | Rung | Start | Beta | The name | Why |
   |---|---|---|---|---|
   | 1 | $1 | 2 | Longshot Mining | the penny miner whose odds the name states, the wildest rung |
   | 2 | $5 | 1.5 | Disruptr | the small startup, spelled as startups spell themselves |
   | 3 | $25 | 1 | Average Co. | the market itself, the average company, which the report card scores against: losing to Average Co. is the lesson |
   | 4 | $100 | 0.75, 2% | General Widget | the economics textbook's firm under an old conglomerate's name, steady and paying its dividend |

   None points at a living company or a real mark, so none is marked
   `(check)`; nothing of the four names was found but a historic Arizona mine
   site called Long Shot and a few small businesses called Disruptr, none a
   stock. Set aside: Moonshot (Moonshot AI, a living company preparing a
   listing); Mr. Market (Graham's allegory, which Buffett's letters to his
   shareholders made famous, near enough to his words that `TRADEMARK.md`
   keeps out); Penny Dreadful (a television series' mark beside the Victorian
   term); Shell Co. (Shell's mark); Upstart (a listed lender); Benchmark (a
   venture firm). Every reader stands as it was, each a formula of the rung's
   row, so the names reach the Portfolio, Trading, the order desk, the books,
   the ledger and the commands with no formula edited. The columns that show
   them: the Stocks sheet's `A` given its first width, 20, its longest label's,
   "the days between two", where the default cut every name; the Positions' `C`
   and the Ledger's `D` from 10 to 15, the longest name's, Longshot Mining. The
   commands keep their names, `rung-1` to `rung-4`, since a person's keymap
   binds a command by its name (`ENGINE.23`), and their words name the stocks.
   *The alternatives:* names of ten characters or fewer and no width moved,
   Longshot and Widget Co.; the names on a content sheet the Stocks sheet reads,
   which `MENU.1`'s decision 2 set aside; tickers beside the names (Found); the
   commands renamed by their stocks, `buffet.longshot-mining`, every saved
   binding orphaned.

4. **The caveat (`BD-5`), a draft.** Row 23, its place `Firms!E4`: "Your
   firm's name is one of four, and the exam's last answer picks it." beside "A
   firm chooses its own name and registers it where it is formed; an adviser
   also reports it on Form ADV." and Management's response, "Management's first
   choice, Guaranteed Returns LLC, was declined by Compliance." (the phrase
   Mildred's memo forbids). The real-world line marked for `MENU.8`'s read. The
   stocks' names bring no caveat of their own: caveat 7 already says the four
   follow one index and none can go bust.

5. **The oracle (rec).** The hand states the stocks' names on its rungs,
   `RUNGS[k].name`, as the Stocks sheet keeps each on its rung's row, and the
   order's sentence and the commands' words read the hand's names; every test
   that read the Stocks sheet's names to hold a reader now holds it to the
   hand's: the Stocks sheet's rows, the Portfolio at every hold, Trading, the
   order desk, the books, the ledger and the commands at every frame of the
   order desk's sessions. The firm: the hand takes the content file's row at
   its own exam's style, read by the language's reader, and spells the
   Certificate's line and the Overview's (`firm_line`), where the cells pick the
   row by the Proctor's `B40` through the Firms sheet's pickers; held at every
   frame of the scripted exam, pending until the eleventh answer and then the
   third firm, the Overview's line beside the Certificate's; at every bell and
   day's turn of the long runs, a waiver's first firm; and at each of the four
   styles by a waiver. The new tests: the hand's names and the firm line;
   every stock's name inside every column that shows it, by the cartridge's own
   column rows, and every command's words inside theirs; the style naming the
   firm and moving no number, by the scan and the four games compared. The
   describe golden blessed again. Mutants, each into the source or a content
   file by one anchor, each failing the test named for it: the row shown read
   off the waiver's style alone; the Certificate naming the style; the
   Certificate naming the firm before the eleventh answer; the Overview's line
   pending for good; the Overview naming the firm before the eleventh; the name
   and the style swapped; the capital moved by the style; a cash reading the
   Firms sheet; a waiver naming the first firm whatever its style; a stock left
   its working label; a stock named as another; the books' names by the wrong
   rungs; the picks' words a row off; the Portfolio's names a row off; each of
   the four narrow columns at its old width; the eleventh's line as it read; a
   firm without its name; two firms of one name; and the caveat's place empty.

6. **What it costs, measured (rec).** Natively, as `FRONT.3` measured it: a
   release runner over `HEAD`'s texts and this tree's, alternated, pinned to
   the performance cores and opted out of Windows' throttling; the pass, the
   load, the first frame and ten days of frames filed by what each turned. The
   suite, the two test binaries alternated. The browser at the hand test.

7. **The stale places, rewritten at the build.** In `OMAKASE.md`: the line
   (🟡); `BD-5`'s taken row; `STATION.1`'s second slice, the first slice of
   `MENU.3` taken; `CRITIC.1`'s line, the header's name and the benchmark's.
   The source's served list, its notes, the Stocks', the Firms', the
   Certificate's, the Overview's, the Portfolio's and the Proctor's comments;
   `firms.vla` and its header; the caveats' file; the content's `README.md`;
   the hand's module comment, its rungs, its sentence, its commands' words and
   its tests; the oracle's crate comment; `oracle/tests/cartridge.rs`' module
   comment and every holder of a name; the describe golden;
   `Delta/CLAUDE.md`'s Buffet paragraph; and the memory.

8. **The hand test, the owner's, on the page built here:** the eleventh's line;
   the Certificate and the Overview pending before the eleventh answer and
   naming the firm after it; the four names on the Portfolio, Trading, an
   order's sentence, the palette, the Positions, the Ledger, the Stocks and the
   Commands, each whole; caveat 23; a waiver's firm by `B5`; Build's bytes the
   file's.

**Found at the scoping**, none minted:
- *Tickers.* A stock list usually carries a ticker beside the name, four or
  five letters, which would suit the books' and the ledger's narrow columns; a
  field of the Stocks sheet's rows and a joke each, `MENU.3`'s second slice's
  or a looks pass's.
- *The Stocks sheet's other headers.* Its row 4's headers stand in default
  columns and are cut where a value stands beside them, as the Market sheet's
  labels and numbers are (`FRONT.3`'s Found): a looks pass or the next Stocks
  dish.
- *A style out of range.* A person's edit of `Proctor!B5` past the Firms
  sheet's rows, 5 or more, draws an error value on the Certificate and the
  Overview, the picker's `CHOOSE` past its last reference, as any sandbox edit
  may (`AD-6`); `PLATE.6`'s error jokes would land there.
- *The archetype's later flavor.* The report card's header (`CRITIC.1`, noted on
  its line), Mildred's memos addressed to the firm (`MENU.4`) and the rivals as
  the other three archetypes (`CRITIC.2`), each words read off the same row.

**The predictions, before any number.** Written before any number was taken;
the numbers column was filled at the build, P10 at the owner's hand test.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 1,268 to 1,274 rows where 1,258, about 1,271; 806 to 808 formula cells where 805, about 807 | *Measurement*: `(rows 1271) (loads yes)`, as about 1,271 predicted; 807 formula cells, as about 807 predicted, the describe golden's budget |
| P2 | the describe golden | `(budget 30 807 24210 2000000)` give or take a cell; shapes 217 to 219 where 216; texts up by 1 to 3 where 213; levels 98; `(every-frame 0)`, `(cycles 0)`; alone up by 1 to 3; the Firms' readers up by 2 to 4 where 7, the Proctor's by 1 where 69; every shape from the Overview's `A3` on renumbered | *Measurement*: `(budget 30 807 24210 2000000)`, exactly; `(plan (shapes 218) (texts 215) (formulas 807) (levels 98) (runs 45) (lanes 260) (alone 547) (every-frame 0) (cycles 0))`; `(readers "Proctor" 70 (most 8))`; every shape from the Overview's `A3` on renumbered. **Missed** in one: the Firms' readers 14 where 9 to 11, the second picker's five reads of its own sheet counted with the Certificate's and the Overview's; and, unpriced, the Caveats' 97 where 93, each of the four pickers reading one more row |
| P3 | the pass and the load | the pass 0.05 to 0.30 ms longer; the load 0.05 to 0.30 ms longer | *Measurement*, one release runner over `HEAD`'s texts and these, six runs alternated, pinned and opted out, the fixed computation within 1.4% but a first run's warm start: the pass 9.53 to 9.61 ms where 9.56 to 9.60, the medians 0.03 apart; the load 8.44 to 8.53 where 8.39 to 8.47, 0.02 apart. **Missed**, each cheaper than priced and inside the noise |
| P4 | the step | an ordinary frame, a tick, a half hour, the bell, the close and midnight each within 3% of `HEAD`'s; the first frame 0 to 8 µs dearer | *Measurement*, the same runs, ten days of each text's own frames, the exam waived, three passes each, the medians: an ordinary frame 2.3 µs either way; a tick 35.3 where 35.5; a half hour between sessions 15.4 where 15.5; a five minutes' turn 2.6 either way; the bell 26.8 where 27.2; the close 44.2 where 44.5; midnight 53.2 where 54.2; the first frame 312.1 where 308.9, 3.2 dearer. Inside, each |
| P5 | the suite | 83 to 86 tests where 81, about 84; every long run within 3% of `HEAD`'s, the order desk's two sessions 0 to 5% shorter; the whole on eight threads within 0.3 s | *Measurement*, the two test binaries alternated, each pinned and opted out, two rounds: 84 green, the crate's 33 and the cartridge's 51; the weekday 0.88 to 0.89 s either way; the hour's year 6.34 to 6.36 where 6.35 to 6.36; the hundred years 6.39 to 6.45 where 6.39 to 6.42; the eight hundred days 1.30 to 1.31 where 1.33; the order desk's session 1.51 to 1.52 where 1.56 to 1.57, and the switch's 1.64 to 1.65 where 1.69 to 1.71, each 2 to 4% shorter; the whole on eight threads 7.03 to 7.40 s where 7.10 to 7.18. Inside, each |
| P6 | the page | the cartridge's block 800 to 1,600 characters longer; the source's 800 to 2,000; firms.vla's 150 to 400; the caveats' 300 to 500; the library's none; the page 2,000 to 4,500 longer | *Measurement*: 2,493,215 characters where 2,489,690, 3,525 longer; the cartridge's block 988 longer, the source's 1,859, firms.vla's 386 and the library's none, inside. **Missed** in one: the caveats' 292, eight under |
| P7 | the checks | four checks and four controls green, no check edited and no file added under `src/`; fmt and clippy clean | *Measurement*, as predicted: ALL GREEN, four checks and four controls, the content pin at 5; `cargo fmt --check` and `cargo clippy -D warnings` clean once `cargo fmt` had wrapped three calls |
| P8 | the numbers given | the longest stock name 15 characters, inside every column that shows it once two are widened; the commands' words at most 56 characters of their 64; the Overview's firm line at most 76 characters; the scripted exam's eleventh, the third style, naming the third firm on the Certificate and the Overview at frame 41 | *Measurement*: Longshot Mining, 15; the commands' words that name a stock 56 at most, buy-max's for Longshot Mining; the firm line 74 at most, Above the Fold Capital's; the scripted exam's frame 41, the Certificate's "Read-the-Room Partners" and the Overview's line. **Missed** in one, and one found: three columns given widths where two were priced, the Stocks sheet's `A` having none of its own; and beside the words that name a stock, a line that names none passes the Commands' column, `PLATE.1`'s buy-max words before a stock is picked, 67 characters (below) |
| P9 | mutants | 15 to 20, each into the source or a content file by one anchor; at most one lives at the first run | *Measurement*: 22 written, each anchor applying once, each expanded and caught by the test named for it at the first run. **Missed** in one: 22 where 15 to 20 |
| P10 | the hand test | the Exam's eleventh line names the firm; the Certificate names the firm of the style chosen; the Overview's `A3` the name and the style; the stocks' names on the Portfolio, Trading, an order's sentence, the palette's words, the Positions and the Ledger, each whole; caveat 23 read; Build's bytes the file's | The owner's, on the page built here, 2026-10-19 (the paragraph below): the Certificate and the Overview pending at the eleventh question, the Certificate naming Read-the-Room Partners at the eleventh answer and the Overview the name and its style after O; the four names whole on the Portfolio, Trading, its order's sentence, the palette, the Stocks sheet, the Positions, the Ledger and the Commands; caveat 23; the waiver's fourth style naming Vibes LLC; passed with no change asked |

**What the build found.** The source expanded to 1,271 rows and loaded at its
first expansion, and the suite was green at its second run, 84 tests. The
first run's two failures were the describe golden, blessed again, and the new
column test, which found a line of `PLATE.1`'s too wide for its column: the
Commands sheet's words for buy-max before a stock is picked, "Buy as many
shares of the stock picked, none yet as the cash covers", 67 characters where
the column held 64, cut on the Commands sheet where its `holds` stands beside
it (the palette shows it whole); the column was widened to 67 at the build, a
value of the device sheet's, a mutant its control. Writing the mutants found a
hole in the first form of the style's test: it scanned the formulas of other
sheets for a read of `Proctor!B40`, so a formula of the Proctor sheet reading
`B40` bare, or a range over it, would have moved a number past it; the test now
also plays the four styles' waived games side by side, every cell of the
model's sheets compared at each of the first frames, and the mutant that adds
the style to the capital failed there, at `Cash!B3`, `B9`, `Orders!B31` and
`Proctor!B39`. Twenty-two mutants, each written into the source or a content
file by one anchor, applied once and expanded before its test ran, each failed
the test named for it at the first run. Natively nothing moved but the first
frame, 3.2 µs dearer, two cells more computed once; in the suite the order
desk's sessions run 2 to 4% shorter, the hand's names read from its constants
where the tests read the Stocks sheet at every frame.

**The owner's hand test.** On the page built here (`web/index.html`, 2,493,215
characters), 2026-10-19, over fourteen screenshots. The exam sat by hand at two
right answers of ten: at frame 677, the eleventh question still open, the
Certificate's "Pending: the proctor certifies after the eleventh question.",
2 of 10, $6,000.00 and "Firm name" beside "pending registration", and at frame
1,118 the Overview's "Your firm: name pending registration."; at frame 1,581,
the eleventh answered, the Certificate's "The bearer has sat the Series 777 and
passed, as every candidate does." and "Firm name" beside "Read-the-Room
Partners", whole; at frame 1,993, after O, Monday at 10:40, the Overview's "Your
firm: Read-the-Room Partners. Investment style: I read the room." over the net
worths, the weather and the headline. The Portfolio at 13:35, its Security column
Longshot Mining, Disruptr, Average Co. and General Widget, each whole beside its
price, change and glyph. Trading at frame 4,886, Wednesday at midnight: the four
names whole, the first marked picked, "Bought 1 share of Longshot Mining at the
price shown.", and the palette opened on "pick", `buffet.rung-1` to `rung-4`
worded "Pick Longshot Mining: the buy and sell commands trade it" to "Pick
General Widget: ...". The Stocks sheet's `A` with its labels and the four names
whole; the Positions' `C`, every name whole in both pockets' rows and the rows
shown; the Ledger's first row, "Tuesday, year 1, day 2, 14:20", Firm, Longshot
Mining, 1 at 108 cents; the Commands sheet's every line whole, the trades worded
for Longshot Mining. The Caveats sheet's row 24, its four fields, and its count
23; the hidden Firms sheet's two fields and four firms, its pickers "I read the
room" and "Read-the-Room Partners". A second load: the waiver written at frame
416 and the fourth style at 604, the Overview's "Your firm: Vibes LLC.
Investment style: I feel the market." over $7,500.00, and the Certificate's
"Waived: the regulator assumes you would have passed.", 5 of 10, $7,500.00 and
"Firm name" beside "Vibes LLC". Not among the screenshots: the Exam's eleventh
line, held at every frame by the scripted exam; the Commands sheet's buy-max
words before a pick, held by the column test; and Build's line. Seen on the
hidden Firms sheet, and the page's (Found, below): its columns `A` to `C` drawn
at the widths of the Caveats sheet shown before it, and `E` about a character
wide, so that its count and its row shown drew `#` and its pickers' texts
spilled into `F`; and its label "the row shown" cut at `D`'s default width, as on
every content sheet but the Caveats'. The owner's words: "Tests pass; commit."
So every decision and every draft stands as built. Committed at the owner's
word.

**Found at the hand test**, none minted:
- *A sheet drawn at another's widths.* The page at `alonzo` 0.2.0 draws a
  column at a width a person dragged, kept by its letters for every sheet
  (`web/viewport.js`, `state.widths`, `PLATE.1`'s finding, `ENGINE.26` closed in
  Alonzo and not yet released), else at its sheet's record's: the hidden Firms
  sheet drew `A` to `C` at the Caveats sheet's widths and `E` about a character
  wide, the Caveats sheet's `E` as narrow. Whether a drag or the hidden sheet's
  widths not read, it is the page's, the owner's to pass on to Alonzo; the
  cartridge's widths are held by its tests.
- *A content sheet's labels.* `strings` writes the labels "rows", "the row
  shown" and "shown" in a column it leaves at the default width, so "the row
  shown" is cut by the value beside it on every content sheet but the Caveats',
  whose source gives its labels' column a width; the library could give the
  column its longest label's width, a row more for each content sheet.

## `CRITIC.1` — the report card: alpha over the same seed

*Scoped and built 2026-10-19 at the owner's word, "Scope and implement CRITIC.1
in Buffet (Delta/Buffet): the report card, alpha over the same seed", in one
sitting as `PLATE.1`, `PLATE.2`, `POT.11`, `FRONT.3` and `BACK.2` were: each
decision below stands as recommended and was built, the owner's to approve,
amend or strike at the hand test, and every text a player reads is a draft.
Minted already, its line in `COURSE.0` and a line of its own in each later
course; nothing else was minted, and what else the scoping found stands under
Found. Read for it: the item's line and its notes in `OMAKASE.md`, its later
lines (the income line at course 1, the firm line at 2, the drawdown line at 3,
the option book's at 4, the bot's at 5, every class's at 6), `CRITIC.2`,
`CRITIC.3` and `CRITIC.5`, which read the card, `POT.6`, `BD-3`, `BD-4`, `BD-5`,
`BD-7` and `BD-9`, and the opening contract; `POT.10`'s record above (the
quarters, the owner's word at its approval, and the quarter the model's in the
dividend's 91 days); `GUARD.1`'s (the Camera's third row, its decisions 10 and
11); `FRONT.3`'s (the weather's tenths spelled inline, its decision 5 and its
Found on a link's mechanism); `PLATE.1`'s and `PLATE.2`'s (each pocket's
books); `BACK.1`'s (the hustle's fees into your own cash); `BACK.2`'s (the four
styles' games compared, the hand's names); `BUFFET.md`'s §2, §4, §5 and §16, as
design reference; the source's Day, Market, Stocks, Positions, Cash, Proctor and
Camera sheets and the views; `kitchen.vla`'s dollars; the tests' long runs and
their holders; the hand's plate, calendar and rungs; and Alonzo's `SPEC.md` at
`v0.2.0`, the tag the pin names, §3.8, with its host's reading of the Camera
(`web/host.js`, `_readCameraSheet` and `_cameraSheetName`). Alonzo's and
Frazaro's trees were read and never touched.*

**What it stands on, read on the day** (Buffet at `c56d22f`, five ahead of its
remote, the tree clean; `alonzo` 0.2.0 and `vla-lang` 0.10.0).
- *The pockets.* The firm's cash, `Cash!B3`, the exam's capital, final at the
  begin (`Proctor!B39`), and its books' flows; its holdings, `B6`, and its net
  worth, `B9`. Your own, `B4`, `B7` and `B10`: $100 at the start and the
  hustle's fees, $250 each and four at most, each paid at the five minutes its
  prospectus is done (`Hustle!B18`). At course 0 the firm's capital takes no
  flow but its own trades and its dividends.
- *The broad rung.* Average Co., `Stocks!A7`, a beta of 1, $25.00 at the start,
  no dividend. The game always begins at day 0's bell, the calendar waiting for
  the exam, where the index is 1 at every seed, so its price at the begin is its
  start; and every capital the exam gives, $5,000 and $500 a right answer, is a
  multiple of $25.
- *The quarter.* `Stocks!B2`, 91, the year over the dividends a year: the
  dividend's days, the year's 91st, 182nd, 273rd and 364th, each a quarter's
  last day but the year's.
- *The Camera.* `B3` named the Exam, the Certificate, then the lead,
  `Proctor!B6`, from the begin for good. The host shows a sheet when the value
  changes and names a sheet the grid holds, compared without case; the empty
  text names none and moves nothing (`SPEC.md` §3.8; `_readCameraSheet` sets a
  show only for a changed value `_cameraSheetName` names). So a `B3` left naming
  the lead could show a card's Report once and never again, and one that went
  back to the lead after a card would take the person to the Overview the frame
  after.
- *A percent.* The Overview's weather spells its change inline in tenths, "down
  0.5%", the first percent a view writes, with no verb.
- *The suite.* 84 tests green, the crate's 33 and the cartridge's 51.

**The decisions.**

1. **What the card grades: the firm's book, by its plain return, against Average
   Co. bought at the begin and held (rec).** The firm's pocket alone. Its
   capital takes no flow at course 0, no client's money, no draw and no
   contribution, only its own trades and dividends, so its return over a
   period is its net worth at the period's end over its net worth at the start,
   less one, which is the time-weighted return with no flow to take out; and it
   is the book a fund manager is graded on, where your own pocket is the
   partner's. Your own pocket takes the hustle's fees, up to $1,000 against a
   start of $100, so a raw return would count a fee as a gain, and a return that
   takes them out wants each fee's day (modified Dietz) or the worth at each fee
   (time-weighted): a flow-adjusted return built for $100 and four fees, which
   the firm line at course 2 needs anyway for draws, contributions and clients
   (`BACK.6`, `BACK.3`). So your own pocket goes ungraded at course 0, and the
   Report says so in a line (decision 5). The benchmark: Average Co. bought with
   the firm's capital at the begin's price, $25.00 at every seed, in whole
   shares, as Buy max at the first bell would buy them, the cents left over held
   as cash, and held; its worth at a card its shares at the price now and the
   cents. Every capital the exam gives buys whole shares with nothing left over,
   200 to 400 of them; a capital a person's edit makes indivisible keeps its
   cents. No dividend: Average Co. pays none at course 0, and the income line
   (course 1) makes both total returns by name. The returns over the quarter and
   over the year so far, each from the last card or the begin; alpha, the firm's
   return less the benchmark's, in percentage points, each return rounded once to
   a whole hundredth of a percent, half up and exactly, so that alpha is a
   difference of two whole numbers and the card's figures subtract as printed.
   *The alternatives:* each pocket graded, your own by a time-weighted return over
   its fees, by modified Dietz over the fees' days, or with the fees taken out,
   a fee's own gain counted as the pocket's; your own alone, `BUFFET.md` §4's
   "personal wealth is the score", against `BD-3`'s never a raw net worth and the
   smaller book by far; the benchmark in fractions of a share, the price's own
   return, the same at every capital the exam gives; the benchmark bought again
   at each quarter's start, which forgets the year; alpha after beta, Jensen's,
   a regression the dialect has no function for (the caveat says so).

2. **What else the card holds at course 0 (rec).** A header naming the firm,
   `Firms!E4`, pending registration until the exam's last answer names it
   (`BACK.2`'s rule), and the seed and the tier it was graded at, printed as they
   stand, `Clock!B3` and `Market!B3`, since every firm at a seed and a tier plays
   one market (`BD-3`). A grade for each of its two columns by one table of five,
   A from two points ahead of the market, B from half a point, C within half a
   point either way, D to two points behind and F past that, settings a person
   may edit (`AD-6`), chosen from the hand's measurement after the predictions
   were written: at the seed, a firm that sits in cash grades B, D, F and F over
   the first year's quarters and F for the year, the benchmark returning -1.04%,
   +1.09%, +3.64% and +6.17%, +10.08% for the year; Average Co. itself, held,
   grades C at every card, average as in Average Co.; Longshot Mining, the beta
   of 2, D, B, A and A, A for the year, which the caveat flags (decision 7); and
   General Widget B, C, C and D, D for the year. Over 2,000 seeds the benchmark's
   first quarter runs from -3.88% to +7.72% between its 5th and 95th percentiles,
   so a firm in cash meets every grade. The proctor's remark, the Remarks
   sheet's, a content sheet of its own (`MENU.1`'s rule, a sheet a kind of
   string): a remark a grade, a row's place its grade, the quarter's at the first
   three cards and the year's at the fourth, drafts; `check_layout.ps1`'s pin 5 to
   6. Sharpe and the maximum drawdown wait for course 3, where the drawdown has
   its line already and Sharpe joins it, its line amended. At tier 1 the closes
   are four smooth waves and a drift (`POT.2`), so a quarter's daily returns
   barely vary: Average Co.'s daily closes give an annualized Sharpe ratio of
   -12.5, 10.7, 25.6 and 25.7 over the seed's first year's quarters and 26.0 to
   28.2 over the second's, a daily spread of 0.02% to 0.07%, where a real fund's
   ratio runs about 0.5 to 1.5; and a maximum drawdown of 1.04% in the first
   quarter and 0.000% in each of the next seven. Cash earns nothing until
   `POT.3`, the rate a Sharpe ratio subtracts, and `POT.5`'s noise (course 3) is
   what gives the days a spread. No scored run is built: whether a run's tier,
   paces and rate are fixed at its start is `POT.6`'s, whose picker fixes the tier
   (`POT.11`'s Found settled here). The card's every figure is a function of the
   days and the books, which no pace moves (`BD-3`, `BD-7`), so a pace written
   mid-run changes how long a person has to act and not what the card reads, and
   the replays `CRITIC.3` and `CRITIC.5` grade record every edit. *The
   alternatives:* Sharpe and the drawdown on the card now, as the line wrote them;
   the drawdown alone now, its course-3 line struck; the remarks as rows of the
   Caveats or the Headlines sheet, words of another kind on a sheet a player reads
   whole or one the ticker draws from; the tier by its name, the Day Trader, which
   `POT.6`'s picker names; the seed and the tier held with each card, a
   sandbox's edit of either after it then not printed.

3. **When a card comes: at each quarter's turn, the year's card the fourth's
   second column, each a snapshot until the next (rec).** The year's days in four
   quarters of the year's days over four, 91, the fourth taking the year's last
   day: 91, 91, 91 and 92 days, the year's 91st, 182nd, 273rd and 365th each a
   quarter's last, as calendar quarters run 90 to 92 days and end with the year.
   The Day sheet keeps them beside the calendar: the quarter, `B33`; the quarters
   counted from the game's first, `B34`; and a quarter's turn, `B35`, 1 in the
   frame the count moves, the first frame of a quarter's first day, midnight at
   the game's pace, and never the game's first frame; four the quarters a year,
   arithmetic and no setting. A card is read off the books at that frame, whose
   prices stand at the quarter's last close (between sessions the price now is
   the session's close, and at a bell the last one, so every pace the oracle
   plays turns there), and held until the next turn: a report card is a document
   of a date. The year's card is the fourth's second column, the year so far
   become the year, so the year's and the fourth quarter's come together and are
   shown once, where a year's card apart would come a day after the fourth's and
   show the Report twice in a row. At the game's pace the cards come at 1:11:27.0
   of play after the exam, 2:23:03.5, 3:34:40.0, and the year's at 4:47:13.0, the
   first 24 seconds after the first dividend's day begins (`POT.11`'s 1:11:03.0).
   The quarter is the calendar's and not the dividend's, so a person who writes a
   dividend a month moves no card. No cell of it computes every frame: the Day
   sheet's three change once a day at most, and the census holds
   `(every-frame 0)`. *The alternatives:* the quarter read off `Stocks!B2`, a
   dividend's setting moving the card; quarters of 91 and the year's card apart, at
   the end of the year's 365th day, or the year's last day a quarter's stub; the
   turn at the quarter's last close, Friday's in the first year, a session keyed on
   a day the quarter may outlive by two; live figures beside the last card, the
   quarter so far, which is the Overview's kind of line.

4. **How the player meets it: the Camera shows the Report at every card (rec).**
   The game shows its card as it showed the certificate: the Camera's `B3` names
   the Report at a card's turn, the Performance sheet's value `B1` as the lead is
   the Proctor's, and once the game has begun names nothing between, the empty
   text, so that the lead at the begin's frame and the Report at each card are
   each a change from nothing, shown once at the frame's end, every card and not
   the first alone, and between them the person's tabs win with nothing shown
   again. `FRONT.3`'s Found is settled so: a `B3` that followed the sheet shown,
   `Input!B19`, between links would work too, but would show again, a frame after
   each click of a tab, the sheet the person just clicked, a view and a draw each
   time, and would read the Input sheet. No pause and no control of time: the
   market keeps its pace through the card, the turn at midnight and the next bell
   9.5 seconds of play later. The Report stands after Gigs among the tabs, where a
   player meets it, an hour and eleven minutes in; it holds no button and binds
   no key, having no act (keyboard-first: no act, no key). *The alternatives:* a
   line on the Overview linking to the Report, an alert, an act with its key and
   its command (`FRONT.3`'s decision 5), which a player who never looks at the
   Overview never sees; the Report beside the Overview, before the Portfolio;
   `B3` following `Input!B19` between links.

5. **The view: the Report (rec).**

   | | A | B | C | D |
   |---|---|---|---|---|
   | 1 | the date, every view's | | | |
   | 2 | the bell's line, every view's | | | |
   | 3 | `Report card for Prudent & Sons, graded by the proctor at seed 791886 and tier 1.` | | | |
   | 5 | `The first quarter of year 1, to its last close.` | | | |
   | 6 | | `Worth ($)` | `The quarter` | `The year so far` |
   | 7 | the firm, by its name | its worth | its return | its return |
   | 8 | `Average Co., bought and held` | its worth | its return | its return |
   | 9 | `Alpha, the difference` | | alpha | alpha |
   | 11 | `Grade: B for the quarter, and B for the year so far.` | | | |
   | 12 | `The proctor remarks: ...` | | | |
   | 14 | `Your own account is not graded: the proctor grades the firm's book, and your own money is your own business.` | | | |
   | 16 | the fine print | | | |

   Column A 30 characters, the longest name a firm's row holds; B to D 12, the
   width the dollars and the percents are padded for. The last column reads "The
   year" at a fourth quarter's card. Before the first card the title says when it
   comes, "No card yet: the first is issued when the first quarter ends, after its
   91st day.", the worth column is the begin's, the firm and Average Co. side by
   side at the start, and the card's other cells are empty. The dollars by the
   library's `dollars`; the percents by its new `percent`, a column's figure, whole
   hundredths of a percent, signed as the change's dollars are, a gain's or
   nothing's + and a loss's space and -, and padded so that the spaces and twice
   the whole percent's digits make 10, which ends short of a number's edge at every
   scale from 100% to 200%, the percent sign measured at the scoping at 10.639
   pixels of 13-pixel Segoe UI, about three spaces (11 would pass the edge by two
   device pixels at 150%); the empty text for the model's empty text. The verb
   enters the library at its first dish, written six times over two fills, as
   `bell` came with both views writing it. The Overview's weather is the first
   percent a view writes, but a phrase of a sentence, up or down by words and in
   tenths, and keeps its own spelling; this verb is a column's figure, which a
   number format replaces when the record draws one (`MISE.6`). 18 formula cells
   where a view may hold 1,000; nothing reads it, the Camera reading the
   Performance sheet's name of it. *The alternatives:* the card's figures as
   sentences, no columns and no verb; the percents in tenths, the weather's
   precision; each period's start and end in dollars, two columns more; the
   weather rewritten over a verb of a phrase, its bytes moved, the sentence's
   percent and the column's one pattern.

6. **The model: the Performance sheet, after the Ledger (rec).** The card's
   books, a model sheet a player may open (`AD-6`), each formula the card's:

   | | A | B | C | D | E | F | G |
   |---|---|---|---|---|---|---|---|
   | 1 | where a card is shown | `Report` | | | | | |
   | 3 | Average Co.'s price now | `=Stocks!$E$7` | | | | | |
   | 4 | its shares, bought at the begin | held from the begin: `INT(capital/B3)` | | | | | |
   | 5 | the cents left over, held | held: the capital less the shares | | | | | |
   | 6 | a quarter's turn | `=Day!$B$35` | | | | | |
   | 7 | a year's turn | a turn into the first quarter | | | | | |
   | 8 | cards issued | a count of `B6` | | | | | |
   | 9 | the card's quarter | held at a turn: `Day.last!$B$33` | | | | | |
   | 10 | the card's year | held at a turn: `Day.last!$B$6` | | | | | |
   | 12 | worth, in cents | now | at the last card | at the one before | at the year's start | at the last year's | at the card's year |
   | 13 | the firm | `=Cash!$B$9` | held: the begin, then each turn | held: the last card's, at each turn | held: the begin, then each year's turn | held: the year's start's, at each year's turn | the last year's at a fourth card, else the year's |
   | 14 | Average Co., bought and held | its shares at the price now and the cents | each the firm's row's formula, filled | | | | |
   | 16 | the card, in hundredths of a percent | the quarter | the year so far | | | | |
   | 17 | the firm's return | `INT(((C13-D13)*20000+D13)/(2*D13))` | over `G13` | | | | |
   | 18 | Average Co.'s return | the same, row 14 | | | | | |
   | 19 | alpha, the difference | `B17-B18` | `C17-C18` | | | | |
   | 20 | the grade, 1 for A to 5 for F | the first whose alpha it reaches, from the top | | | | | |
   | 21 | the grade | its letter | | | | | |
   | 22 | the remark's grade | the year's at a fourth card, else the quarter's | | | | | |
   | 24 to 29 | the grades | A 200, B 50, C -50, D -200, F below | | | | | |

   Each hold takes the books "until the game has begun", `Proctor.last!$B$41<>1`,
   so that the begin's frame, a waiver's mid-exam included, takes the capital as
   final, and each turn after it. The card's figures are the empty text until the
   first card. A return is `INT(((e-s)*20000+s)/(2*s))`, the gain times 10,000
   over the start rounded half up by a quotient of two whole numbers rounded down,
   which is the exact rounding whenever the numerator stays below 2^53 (*derived*:
   both numbers are whole and held exactly, and a quotient of two whole numbers,
   if not whole, stands at least one over the divisor below the next whole number,
   while a double's rounding moves it less than the quotient over 2^53), any worth
   below about $4.5 billion. 31 formula cells. *The alternatives:* the card's
   books on the Report, a view holding state; the benchmark's worth recomputed at
   each card from the index's return, no shares; the returns as doubles, their
   rounding left to the view, alpha a difference of two unrounded numbers that
   prints a hundredth off now and then.

7. **The caveats (`BD-5`), drafts.** Row 24, its place `Performance!B19`, the
   quarter's alpha: "A grade reads returns alone: a book wilder than the market
   beats it in any rally, however bumpy the ride." beside "Alpha is measured after
   taking out the market's own push, beta, and judged beside risk, as the Sharpe
   ratio does." and Management's response, "Management finds volatility
   character-building, for the clients." Row 25, its place `Day!B35`, the turn: "A
   card is issued the moment its quarter ends, its figures checked by no one."
   beside "A fund reports its quarter weeks later, once its administrator has
   checked the figures; an auditor checks the year." and "Management's figures are
   unaudited, which saves everyone time." Both real-world lines marked for
   `MENU.8`'s read. The card brings no caveat for your own pocket ungraded, which
   its own line says, nor for the quarters' 91 to 92 days, as calendar quarters run.

8. **The oracle (rec).** The hand shares none of the card's rules with its cells.
   The cells divide the day of the year by the year's days over four, count the
   quarters and mark a turn by the count against its twin, and hold the worths at
   the last card and the one before it, at the year's start and the last year's,
   over their twins; the hand lists the quarters' last days, `QUARTER_ENDS`,
   counts the quarters as its calendar turns, and keeps every turn's worth in a
   list, `Cards`, reading a card's starts off it by search. The cells round a
   return by dividing two doubles; the hand rounds the same quotient in whole
   numbers, the two the same below 2^53, held over every pair of worths from 1 to
   1,000 cents and their next 2,000 and beside every power of two to 2^40. The
   cells grade by comparing from the top; the hand counts the grades it reaches.
   The cells pad a percent by comparing with powers of ten; the hand counts its
   digits, and holds its width at every scale by the measured glyphs. The Camera:
   the hand's exam knows the frame the game began, so the lead stands there alone,
   and the Report at the card's. The runs: the Day sheet's three new cells at
   every frame the calendar is held; the Performance sheet and the Report at every
   card of the hour's year and of the hundred years at a frame a day, 399 cards,
   the firm at rest, and at each day's turn and bell of the hour's year and of the
   eight hundred days; a scripted quarter at two paces, every frame held whole,
   ten shares of Longshot Mining and ten of Average Co. bought for the firm on day
   0, a share of Average Co. for your own on day 1 between two switches by C, the
   firm's Longshot sold on Friday, day 88, a prospectus claimed on Sunday, day 90,
   and paid on Monday, after the turn, and a share of General Widget bought for
   the firm in the new quarter: the card grading the firm's book and nothing of
   your own; the Camera at every card of a year at a frame a day, a tab clicked
   between each, the firm in cash three quarters and then Average Co. bought with
   all of it, so the year's card grades the quarter C and the year F and the
   proctor remarks on the year's; the Report's own test, its lines before the
   first card, no key and no command while it shows, its columns, every firm's
   name inside its first, the remarks whole, and a capital the begin's price does
   not divide keeping its cents; the style's test, the Report's two lines among
   the readers of the name and the Performance sheet among the model compared; the
   tab order; the columns of dollars and of percents; and the describe golden.
   Mutants, each into the source, the library or a content file by one anchor,
   each failing the test named for it: a turn at the game's first frame; the
   year's last day a fifth quarter; the card's quarter the new one; the benchmark
   bought again at every frame; its shares in fractions; its worth without its
   cents; a return rounded down; alpha's sign turned; B from a point and a
   twentieth; the letters a place off; the remark the quarter's at the fourth
   card; the remark the year's at every card; the year so far from the quarter's
   start; the year's start never turned; the Report shown at the first card alone;
   the lead named after the begin; the Report never shown; your own pocket graded;
   the fee in the firm's book; the firm's book read off its twin; the card's worth
   the year's start; the year's header at every card; the grades' sentence the
   quarter's twice; the remark's row a place off; the Report before Gigs; P bound
   on the Report; the percent padded by eleven; a loss's percent without its
   space; and each caveat's place empty.

9. **What it costs, measured (rec).** Natively, as `BACK.2` measured it: a
   release runner over `HEAD`'s texts and this tree's, alternated, pinned to the
   performance cores and opted out of Windows' throttling, the pass, the load,
   the first frame, ten days of frames filed by what each turned, and a third
   pass to the first quarter's turn at the game's pace, timing the card's frame
   beside the midnights before it. The suite, the two test binaries alternated.
   The browser at the hand test.

10. **The stale places, rewritten at the build.** In `OMAKASE.md`: the line (🟡),
    its Sharpe and drawdown and its first card's time; course 3's line, the risk
    line; course 1's and course 2's lines, the benchmark's dividends and your own
    pocket's return; `BD-5`'s taken rows; `POT.6`'s scored run; the hoists'
    table, `percent`. The source's served list, notes, the tabs', the Day's, the
    Camera's and the views' comments; `kitchen.vla`'s history and its new verb; the
    caveats' file; the new remarks' file; the content's `README.md`;
    `check_layout.ps1`'s pin, its baseline's history and its control; the hand's
    module comment, its exam's begin, its calendar's quarter and the card; the
    oracle's crate comment; `oracle/tests/cartridge.rs`' module comment, the views,
    the content sheets, the holders, the style's readers and model, and the
    dividend's script, which shows Trading again after the eighth card it crosses;
    the describe golden; `Delta/CLAUDE.md`'s Buffet paragraph; and the memory.

11. **The hand test, the owner's, on the page built here:** the strip; the
    Report before its first card; both paces written a day a frame in the drawer
    before the game begins, so that every frame lands on a bell (Found: a turn's
    frame at another pace), the Camera showing the Report at the first quarter's
    turn about three seconds later and at each card after; a tab clicked between
    kept; the year's card; the
    drafts read, the grades, the remarks and the two caveats; no key on the
    Report; Build's bytes the file's.

**Found at the scoping**, none minted:
- *A history of cards.* The Report shows the last card, so the year's goes when
  the next year's first quarter comes; a ledger of cards written through the Write
  sheet, as the order desk's ledger is, would keep them all. Course 7's careers
  and their saves (`ENGINE.5`) would want it.
- *Since the begin.* A third column, the return since the game began, which equals
  the year so far through the first year; a second year's card would want it.
- *A turn's frame at another pace.* The card reads the books at the turn's frame,
  which at the game's pace is midnight, where no order fills, no dividend falls and
  the price now is the quarter's last close; and so at a frame an hour (0:30) and
  at a day a frame from the first bell (9:30, the price now the last close). A day
  a frame written in the afternoon lands every frame in the afternoon, so the
  turn's frame stands after the new quarter's first session: its books are valued
  at that session's close, and an order there counts in the quarter it closes.
  Valuing the books at the quarter's last trading day's close, with the shares as
  the last frame left them, would make every pace alike: a column of the Market
  sheet's for that day and the four prices over it, a formula of the twins of every
  pocket's books. A sandbox's pace, `AD-6`'s, and no game's; the hand test writes
  its paces before the first bell.
- *The tier's name.* The card prints "tier 1"; the picker's names, the Kiddie Pool
  to the Documentary (`BUFFET.md` §5), are `POT.6`'s.
- *A seed written mid-run.* The market reads the seed on day 0 alone, so a seed a
  person writes later moves no market, but the card would print it; `CRITIC.4`'s
  host's seed decides what a seed written mid-run means.
- *The remarks past the fifth.* A sixth row of the Remarks sheet never shows, as a
  fifth firm is never offered; several remarks a grade, drawn, want a lookup
  (`KERNEL.8`) or a fixed count a grade.
- *The Overview's line on the last card.* A summary line, "Last report card: B",
  is a kin of `FRONT.3`'s alerts; the Camera shows each card instead.

**The predictions, before any number.** Written before any number was taken
but one, the percent sign's width, measured at the scoping on a canvas in headless
Chrome beside the record's widths, which it reproduced, to decide a spelling; the
numbers column was filled at the build, P10 at the owner's hand test.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 1,365 to 1,400 rows where 1,271, about 1,383; 852 to 868 formula cells where 807, about 861 | *Measurement*: `(rows 1389) (loads yes)` at the first expansion, inside; 861 formula cells, as about 861 predicted, the describe golden's budget |
| P2 | the describe golden | `(budget 30 861 25830 2000000)` give or take five cells; shapes 245 to 258 where 218; texts 240 to 256 where 215; levels 103 to 110 where 98; `(every-frame 0)` and `(cycles 0)`; the views' date, bell and fine print at `(sheets 7)`; readers rows new for Performance and Remarks; Cash's readers 9 where 8; every shape from the Report's on renumbered | *Measurement*: `(budget 30 861 25830 2000000)`, exactly; `(plan (shapes 254) (texts 250) (formulas 861) (levels 107) (runs 45) (lanes 260) (alone 601) (every-frame 0) (cycles 0))`; `(sheets 7)`; `(readers "Performance" 57 (most 9))`, `(readers "Remarks" 7)` and `(readers "Performance.last" 9)`; `(readers "Cash" 9)`; every shape from the Report's on renumbered, the year's percents one text with the quarter's, `(same 43)`. Inside, each |
| P3 | the pass and the load | the pass 0.5 to 1.5 ms longer; the load 0.4 to 1.2 ms longer | *Measurement*, one release runner over `HEAD`'s texts and these, two sets of six runs alternated, pinned and opted out, the fixed computation within 1.9%: the pass 10.51 to 10.67 ms where 9.64 to 9.84, the medians 0.82 and 0.93 apart; the load 9.40 to 9.65 where 8.37 to 8.69, 1.09 and 0.95 apart. Inside, each |
| P4 | the step, natively | an ordinary frame within 0.2 µs; a tick 1 to 5 µs dearer; a half hour between sessions within 1 µs; the bell 1 to 4 µs dearer; the close 1 to 5 µs dearer; an ordinary midnight 2 to 8 µs dearer; the first frame 30 to 90 µs dearer; a card's turn 15 to 60 µs dearer than an ordinary midnight | *Measurement*, the same runs, ten days of each text's own frames, the exam waived, three passes each, the medians: an ordinary frame 2.3 µs either way; a tick 37.7 where 35.6, 2.1 dearer; the first frame 337 to 341 µs where 307 to 310, 30 to 32 dearer; the first quarter's turn 110.4 to 111.7 µs where `HEAD`'s same midnight is 50.9 to 51.5, and 51.8 above this tree's thirty-one midnights before it, 58.6. Inside, each. **Missed** in four: a half hour between sessions 17.3 to 17.4 µs where 15.5 to 15.6, 1.8 dearer, the Report's own date line turning there, a seventh view's at about 1.8 µs; the bell no dearer, 28.6 where 28.9, and the close 0.9 dearer, each over seven or eight frames a run and inside their noise; and the first ten days' midnights 1.0 to 1.1 dearer, under the 2 priced, the thirty-one before the turn 2.4 |
| P5 | the suite | 90 to 95 tests where 84; the hour's year 2 to 6% longer; the hundred years 3 to 10% longer; the eight hundred days 3 to 10% longer; the whole on eight threads within 0.6 s | *Measurement*, the two test binaries alternated, each pinned and opted out, two rounds: 92 green, the crate's 38 and the cartridge's 54; the hour's year 6.69 to 6.74 s where 6.35 to 6.41, 5%; the hundred years 6.83 to 6.84 where 6.42 to 6.43, 6%; the whole on eight threads 7.47 to 7.55 s where 7.17 to 7.18, 0.30 to 0.37 longer. Inside, each. **Missed** in one: the eight hundred days 1.51 to 1.53 s where 1.32 to 1.33, 14%, every frame there a day's turn, so the Performance sheet and the Report held at each. Beside them the weekday 3 to 5% longer, the order desk's sessions 1 to 3%, the dividend's 3%; and the new tests, the Camera's year 0.55 s, the scripted quarter 0.36 and the Report's own 0.04 |
| P6 | the page | the cartridge's block 8,000 to 12,000 characters longer; the source's 9,000 to 15,000; the library's 1,200 to 2,500; the remarks' new block 1,300 to 2,200; the caveats' 700 to 1,100; the page 21,000 to 32,000 longer | *Measurement*: 2,521,961 characters where 2,493,215, 28,746 longer; the cartridge's block 10,155 longer, the source's 13,644, the library's 2,472 and the remarks' new block 1,854 with its tags, inside. **Missed** in one: the caveats' 623, under the 700 priced |
| P7 | the checks | ALL GREEN, four checks and four controls; `check_layout.ps1` the only check edited; one file added under `src/`; fmt and clippy clean | *Measurement*, as predicted: ALL GREEN, four checks and four controls, the layout's control a clean copy of eleven files and fourteen mutants; `src/content/remarks.vla` the one file added; `cargo fmt --check` and `cargo clippy -D warnings` clean once `cargo fmt` had rewrapped three places |
| P8 | the numbers given | the cards at 1:11:27.0, 2:23:03.5, 3:34:40.0 and 4:47:13.0 of play; Average Co. bought and held -0.8% to -1.3% over the first quarter, +9.6% to +10.4% over the year, each later quarter -1% to +7%; a firm in cash 0.00% at every card, its alpha the benchmark's return negated, positive at the first card and negative over the year; each stock held, its alpha about its beta less one times the benchmark's, General Widget's lifted about half a point a quarter; Average Co.'s quarterly Sharpe ratio at tier 1 between 2 and 40 in size, its first quarter's maximum drawdown 0.8% to 1.5%; the percent at 10 units never past a number's edge | *Measurement*, by the hand at the seed: the turns at frames 128,611, 257,506, 386,401 and 516,991, 1:11:27.0, 2:23:03.5, 3:34:40.0 and 4:47:13.0, as predicted; Average Co. -1.04%, then +1.09%, +3.64% and +6.17%, +10.08% for the year; the firm in cash 0.00% and alphas +1.04, -1.09, -3.64, -6.17 and -10.08; Longshot Mining's -0.96, +0.95, +4.36 and +5.87, the year's +10.92, General Widget's +0.75, +0.24, -0.45 and -1.18, the year's -0.62; the Sharpe ratios -12.5 to +28.2 and the first quarter's drawdown 1.04%, then 0.000%; the percent at 10 units 2.3 to 5.7 CSS pixels short of a number's edge from 100% to 200%, never past it. Inside, each |
| P9 | mutants | 20 to 26, each by one anchor, each applying once; at most one lives at the first run | *Measurement*: 30 written, each anchor applying once, each expanded and caught by the test named for it at the first run. **Missed** in one: 30 where 20 to 26 |
| P10 | the hand test | the strip's Report after Gigs; the paces written a day a frame, the Camera showing the Report at the first quarter's turn about three seconds later, its header the firm's name, its title the first quarter of year 1, both worths, the returns and alpha, a grade and the proctor's remark; a tab kept; the Report shown again at the next card; the year's card, "The year"; no key on the Report; caveats 24 and 25; Build's bytes the file's | The owner's, on the page built here, 2026-10-19 (the paragraph below): the strip's Report after Gigs and the Performance sheet after the Ledger; the Report before any card at frame 545, mid-exam, the firm pending registration and $5,000.00 twice; the paces and the waiver written in the drawer at frames 828, 938 and 1,076; the four cards, B and B, D and C, F and F, and the year's F and F under "The year", each figure the hand's; caveats 24 and 25 and the count 25; passed with no change asked |

**What the build found.** The source expanded to 1,389 rows and loaded at its
first expansion, and the cartridge stepped by hand to the year's card read the
hand's every figure before the hand was written: a firm in cash, B, D, F and F,
the Report shown at each turn and the Camera empty between. The suite's first run
failed two tests: the describe golden, blessed again, and the dividend's, whose
script at a frame a day crosses eight quarters' turns before its last two acts:
the Camera showed the Report at each, and its palette's switch and its click then
landed on the Report, where no command holds and no button stands. Its script
now shows Trading after the year's turn; a player at a day a frame meets a card
every three seconds. Thirty mutants, each written into the source, the library
or a content file by one anchor, applied once and expanded before its test ran,
each failed the test named for it at the first run; the remark's mutants needed
a card whose quarter's grade and year's differ, which a firm in cash never gives
at the seed (F and F at the fourth), so the Camera's test buys Average Co. with
the firm's cash at the fourth quarter's second day, C for the quarter and F for
the year; and the fractional shares needed a capital the price does not divide,
which no exam gives, so the Report's test writes one. Natively a card's turn
costs about 52 µs more than a midnight, the card's thirty cells once a quarter,
and the views' date line about 1.8 µs a turn more, the Report's own.

**The owner's hand test.** On the page built here (`web/index.html`, 2,521,961
characters), 2026-10-19, over six screenshots. The strip's Report after Gigs
and the Performance sheet after the Ledger. At frame 545, the exam open, the
Report before any card: "As of Monday, year 1, day 1, 9:30 (unaudited)", "The
market opens when you leave the exam room.", "Report card for a firm pending
registration, graded by the proctor at seed 791886 and tier 1.", "No card yet:
the first is issued when the first quarter ends, after its 91st day.",
$5,000.00 beside "Your firm" and beside "Average Co., bought and held", every
percent empty, and the line on your own account. In the drawer, the paces
written a day a frame at frames 828 and 938 and the waiver at 1,076, the firm
Prudent & Sons; every as-of line after it at 9:30, each frame at this pace
landing on the bell, so that by the calendar the turns fell at frames 1,167,
1,258, 1,349 and 1,441, a card every three seconds. Paused at frame 1,213,
day 138, the first card: "The first quarter of year 1, to its last close.",
Prudent & Sons $7,500.00 at +0.00% and +0.00%, Average Co. $7,422.00 at -1.04%
and -1.04%, alpha +1.04% and +1.04%, "Grade: B for the quarter, and B for the
year so far." and B's remark; at frame 1,289, day 214, the second, $7,503.00,
+1.09% and +0.04%, alpha -1.09% and -0.04%, D and C, and D's remark; at frame
1,369, day 294, the third, $7,776.00, +3.64% and +3.68%, alpha -3.64% and
-3.68%, F and F, and F's remark; at frame 1,465, year 2, day 25, the year's
card, its last column "The year", $8,256.00, +6.17% and +10.08%, alpha -6.17%
and -10.08%, "Grade: F for the quarter, and F for the year." and F's remark:
each figure the hand's at the seed, as the hand test listed them. The Caveats
sheet's rows 25 and 26, caveats 24 and 25 with their four fields, and its count
25. Not among the screenshots: the keys pressed on the Report before the first
card, the tab kept between cards, and Build's line, each a check the owner's
word passed. The owner's words: "Tests pass; commit, then recommend next
steps." So every decision and every draft stands as built: the bands, the five
remarks, the two caveats, the labels and the headers. Committed at the owner's
word.

## `PLATE.6`, first slice — the limit, as a test: a balance past what a double holds

*Scoped and built 2026-11-15 at the owner's word, "Scope and implement PLATE.6's
first slice in Buffet (Delta/Buffet): the limit, as a test", in one sitting as
`FRONT.3` and `CRITIC.1` were: each decision below stands as recommended and was
built, the owner's to approve, amend or strike at the hand test, and every text a
player reads is a draft. Minted already, its line in `COURSE.0`; the second slice,
the ending as a designed thing, is course 7's (`FRONT.7`). Nothing was minted, and
what else the scoping found stands under Found. Read for it: the item's line and
`BD-3` in `OMAKASE.md`; `BUFFET.md`'s "Numbers" and "The ceiling" and its appendix's
error values; `BACK.2`'s Found above, a style out of range; `FRONT.2`'s second
slice's decision 1, the model's sheets work sheets; and, to answer the scoping's
questions from the code: `vla-lang` 0.12.0's `calc/eval.rs` (`finite`, `binary`,
`arithmetic`), `calc/program.rs` and `calc/vector.rs` (the lowered programs and the
lanes), `calc/library.rs` (`IF`, `AND`, `OR`, `CHOOSE`), `calc/lookups.rs`
(`IFERROR`, `ISERROR`), `calc/criteria.rs` (an error's criterion), `calc/value.rs`
(`spell`), `numfmt.rs` (`Code::text`) and `view.rs` (the `text` row); `alonzo`
0.3.0's `Cartridge::write`, `view` and `step`; and the vendored page's
`cellEditRow` and `cellClearRow` (`web/index.template.html`) and `valueOf` and
`alignOf` (`web/viewport.js`). Alonzo's and Frazaro's trees were read and never
touched; the registry's sources were read in place.*

**What it stands on, read on the day** (Buffet at `6d4a275`, pushed, CI green, the
tree clean; `alonzo` =0.3.0 and `vla-lang` =0.12.0; 96 tests).
- *The machine past the end.* `eval::finite` turns every result no double holds,
  an infinity or a NaN, into `#NUM!`: each of `+`, `-`, `*`, `/` and `^`, negation
  and the percent, `SUM` and the aggregates, `SIN`, `MOD`, the criteria's sums and
  the array functions. A lowered program hands a result that is no number back to
  the evaluator (`program::num`), and the lanes call the same `arithmetic`. An
  error passes through every operator, the left operand's first, `IF`'s condition,
  `AND` and `OR`, `SUM`, `CHOOSE`'s index and every comparison; `IF` takes no
  branch it does not choose. So `BUFFET.md`'s rule, "no NaN reaches a cell", holds
  at 0.12.0 by the machine, and no cell of Buffet's needed a change for it. The
  record spells a number no cell can hold as `#NUM!` too (`Value::spell`), so the
  record cannot tell the two apart; COUNTIF can, an error's criterion meeting the
  error value alone (`criteria.rs`, `Target::Error`), and so the joke is the test's
  witness (decision 6).
- *What a person may type.* The page writes a typed number bare,
  `(cell "Proctor" "B2" 1E+308)` (`cellEditRow`), and Delete the empty text
  (`cellClearRow`), which arithmetic reads as `#VALUE!`. 1E+308 and the end itself,
  1.7976931348623157E+308, are held; 1E+309 is refused by the row's shape,
  `grid-row-malformed`. So no person types a balance past the end, and a sum of a
  typed number and the game's cents never passes it either: the end and $250 round
  back to the end. Only a product passes it, or two numbers near the end summed.
- *The products of money.* The firm's capital, `Proctor!B39`, the capital before
  the exam and the score at a correct answer's worth, `B2`; the hustle's fees,
  `Hustle!B18`, the claims done at a fee; the books' costs and values, shares at a
  price; the benchmark's worth; and the card's rounding, a worth times 20,000.
- *Undone.* `alonzo` 0.3.0 has no undo: `ENGINE.30`'s step back is open there, and
  at rate 0 alone. An edit is undone by a second edit writing the old value back.
- *State.* The books are counts over their twins (`Positions!E2:I9`, the fills
  counted `Orders!B29`), and so are the card's worths at its turns
  (`Performance!C13:F14`) and the benchmark's shares before the begin (`B4:B5`): an
  error any of them takes stands there after the edit is undone. The host writes no
  derived row whose value is an error (`web/host.js`, `_readWrite`).
- *What a figure in error draws.* An error under a format's code has no `text` row
  (`Code::text` spells numbers and texts alone), so the viewport draws the error's
  name, in the middle of its cell where its format says no alignment (`alignOf`), as
  Excel draws one. The glyphs read the prices' change, `Stocks!I5:I8`, which no
  balance reaches.

**The decisions.**

1. **The edit: a correct answer's worth written 1E+308 (rec).** A person writes
   1E+308 into `Proctor!B2`, after the begin, so the waiver's five right answers make
   the capital 5E+308, past the end; and undoes it by writing the contract's 50,000
   back. The capital is the one product of a setting and a count that is a balance,
   and after the begin every cell it reaches is a function of it, no state among
   them but the order desk's (decision 4). *The alternatives:* your own cash at the
   start, `Cash!B1`, which no number a person may type pushes past (above); the fee,
   `Hustle!B1`, which passes the end at the second claim done, a week and a day's
   work into the game; a stock's start, `Stocks!B5`, a price and no balance, which
   reaches the glyphs, the books' values and the holdings; a formula typed, `=10^309`,
   the error the edit's own cell and no sum's.
2. **Where it lands, held cell by cell against a twin (rec).** The capital; the
   firm's cash, the capital and its books' flows (`Cash!B3`); its net worth, the cash
   and the holdings (`B9`); the card's worth now (`Performance!B13`); the views'
   figures of them, the Certificate's capital, the Overview's net worth and the
   Portfolio's cash on hand and net worth; and while the firm's pocket funds the order
   desk, the cash now (`Orders!B31`), the Write sheet's field of a fill's cash after
   (`Write!C8`), which writes nothing while no order fills, and Trading's cash
   available, with the cash shown (`Orders!B24`) a frame behind, at the edit and at
   its undo. Your own pocket's every sum stays a number, and so does every other
   cell. The oracle: two cartridges, the edited and a twin with no edit, stepped with
   the same inputs; the twin held to the hand at every frame by the order desk's
   holders, its books, cash, ledger, commands and views; and every cell of every sheet
   of the edited game, the twins and the Input sheet aside, equal to the twin's at
   every frame but where the hand says the error lands and the order desk's record of
   the orders the edited game alone took. Undone, every cell is the twin's from the
   frame after, the cash shown from the frame after that. *The alternatives:* the
   edited game held by the hand alone, every holder taught an error, which the twin
   makes unnecessary; the landing cells alone, which cannot say the error went
   nowhere else.
3. **The Errors sheet, a content sheet of its own (rec, `MENU.1`'s name for it).**
   `src/content/errors.vla`, two fields, the error's name as a cell draws it and its
   joke, `BUFFET.md`'s five, word for word, `#NUM!` first; hidden after the Remarks;
   its row shown, `Errors!E2`, a value, 1, the error past the end, its pickers its name
   and its joke (`E3`, `E4`). The layout check's content pin 6 to 7, its control's
   clean copy twelve files. *The alternatives:* the row shown by the error's kind,
   which wants `ERROR.TYPE`, not in the language at 0.12.0 (Frazaro's `KERNEL.68`,
   released in 0.13.0, which Buffet takes at the pin after Alonzo's next release), so
   the gap is named and not built around: five COUNTIFs a joke would be a row of the
   sheet no row added could join; the jokes as rows of the Caveats sheet. *(Amended at
   the owner's hand test, 2026-11-15, the owner's words: "The joke should read 'Net
   worth exceeds the present value of the known universe. Please purchase another
   universe to continue playing.' to make more sense than merely 'exceeds the known
   universe' (known universe's *what*?)": `#NUM!`'s joke in those words, `BUFFET.md`'s
   appendix and its list of the game's texts with it, and the Joke field 112
   characters wide where 80, since the Errors sheet's test holds each joke inside its
   column; the cartridge 86,433 bytes where 86,386, its rows and formulas unmoved, no
   test, golden or floor moved but the content's.)*
4. **An order against cash shown as an error, refused (rec).** `Orders!B26` fills
   nothing while `ISERROR(B24)`, a sale too, and `B27` says why, the sixth outcome, its
   label "6 an error"; Trading's sentence: "Refused: the cash shown is an error. Orders
   fill against cash the books can count.", plain as every refusal is. The closed
   market and the stock unpicked keep their reasons first. Without it a buy fills
   `#NUM!` (the cash shown compared with the price), and the shares, the cost, the
   cents bought, the fills counted and the ledger's ring take it for good; a sale fills
   by the shares, its books right, but its ledger row loses its cash after, the host
   writing no error. Either way nothing recovers when the edit is undone, which is the
   line's claim. *The alternatives:* no guard, the test placing no order while the error
   stands and the latch a Found; buys alone guarded; an `IFERROR` on the fill, the
   error hidden, which the line forbids.
5. **What a figure in error draws: the error's name, as Excel draws it (no decision,
   the record's rule).** In the middle of its cell, its format's code drawing numbers
   alone; the figures' formats unchanged.
6. **The joke beside every balance a view shows (rec).** The cell to the figure's
   right, where its error's joke stands while it holds the error the Errors sheet
   shows, and the empty text else: the Certificate's `C8`, the Overview's `C6:C7`, the
   Portfolio's `C11:C12` and `C15:C16`, eight cells in all with Trading's `C12`, under
   the cash available since the switch stands beside it. A text spills into the empty
   cells on its right, so a joke reads whole. The library's `error-joke`, at its first
   dish since four views write it: `=IF(COUNTIF(<the model's cell>,Errors!$E$3)=1,
   Errors!$E$4,"")`, the model's cell the figure divides, never the view's figure, as a
   glyph reads the model's cents. A Delete's `#VALUE!` lands where `#NUM!` did and shows
   no joke, the jokes being `#NUM!`'s until the row shown can read an error's kind.
   *The alternatives:* a joke a view, beside its first error; beside the net worths
   alone, which every balance's error reaches; keyed on `ISERROR`, which would tell a
   Delete that the universe had been exceeded.
7. **The caveat this dish owes (`BD-5`), a draft (rec).** Row 26: "Money ends where a
   double does, near 1.8E+308 cents: past it, every sum that reads it is an error.";
   "A ledger keeps as many digits as a sum needs and has no ceiling; all the world's
   wealth is under $500 trillion."; "Management considers the ceiling a long-term
   goal."; its place `Cash!B9:B10`, the net worths. The world's wealth marked for
   `MENU.8`.
8. **The hand (rec).** A typed number as its digits and a power of ten; the end as its
   seventeen digits and their power; past the end by counting places and comparing
   seventeen digits, never in a double, the end's seventeen and one more the last a
   double reads back as the end, held against the platform's own reading of thirteen
   decimals at and around it; the capital at a worth, `#NUM!` past the end and
   `#VALUE!` at the empty text, judging no capital within eight of the end's
   seventeenth digit, where the double nearest the worth decides; the landing by a
   list of the firm's books written by hand; and the sixth outcome in the plate.

**Found at the scoping and the build**, none minted; the first three measured in
scratch at the build, a frame a day from the first bell, the exam waived:
- *A card's turn while the error stands.* The card's worth at its turn holds what
  `B13` holds then (`Performance!C13`), so the edit standing at frame 92, the first
  quarter's turn, gives the first card `#NUM!` for the firm's worth, both returns,
  alpha, both grades, the grades' sentence and the remark, Average Co.'s row a number;
  undone at frame 120, the second card (frame 183) reads the firm's worth again and
  still `#NUM!` for the quarter's return, the card before it being the first, and the
  third card (frame 274) is the hand's. An edit standing at a year's turn would hold the
  year's start (`E13`) in error for the year. The card is history, a quarter's books as
  they stood, so the error it took is true of that quarter; whether a card should say
  so in words is the second slice's or the card's next dish's.
- *The begin while the error stands.* The benchmark's shares and cents are taken until
  the game begins and held from then (`Performance!B4:B5`), so an edit during the exam
  that stands at the begin gives Average Co. `#NUM!` shares for good, its worth and the
  Report's second row with them, and the firm's worths at the begin (`C13`, `E13`) until
  the first card and the year's turn; undoing it after the begin restores the firm's
  cash and nothing of the benchmark. A sandbox edit before the bell; the second slice's,
  or `GUARD.1`'s next dish's, which may want the benchmark taken from the capital the
  contract writes rather than the cell.
- *An order in the edit's own frame.* The cash shown is the last frame's, a number, so
  an order landing in the frame the edit does fills, rightly, and the books are the
  hand's after the undo; but its ledger row's cash after is this frame's cash, `#NUM!`,
  and the host writes no error, so that row holds six of its seven fields for good.
  Not reachable by hand in one frame, the edit written on another sheet or through the
  palette, but a keymap's own command can run a row and a command in one step, and a
  replay can. The cash after read a frame later, or the guard reading the cash now,
  would close it; the second is a cycle, the cash now being the fill's.
- *The books past the end by play.* No edit is needed to pass the end once a balance is
  near it: your own cash written 1.7E+308 buys every share it covers, a book's cents
  bought counting a second purchase past the end, and a count over its twin keeps
  `#NUM!` for good. The game's own ending, which the second slice designs (`FRONT.7`),
  decides what stands there: an island before the end, or a ceiling the books refuse.
- *Other errors and other cells.* A price in error, a stock's start deleted or past the
  end, reaches the fill through the price shown, which the guard does not read, and a
  dividend's day multiplies the shares by it, the dividends' count latching; neither
  is a balance. Each is the dish's that makes the price or the dividend (`STATION.2`,
  `PLATE.3`), written knowing an error passes through. The fees, Gigs' `B9`, and the
  card's figures draw an error with no joke beside them.
- *The card's own ceiling.* The returns' rounding multiplies a worth by 20,000
  (`Performance!B17:C18`), so the card is `#NUM!` at worths past about 9E+303 cents,
  four orders of ten below the end.
- *A number past 10^15 under General.* The record spells 1E+308 in full, its 309
  digits, so `Proctor!B2` draws `#` across its cell where Excel's General draws
  `1E+308`: the viewport's or the record's (Alonzo's `hashesOf`, `vla-lang`'s
  `number_text`), on the asks table, the owner's to pass on.
- *Each error its own joke.* `ERROR.TYPE`, Frazaro's `KERNEL.68`, released in `vla-lang`
  0.13.0, gives the row shown an error's kind, so every error a joke of its own and a
  Delete's `#VALUE!` its analyst; at the pin after Alonzo's next release, on the asks
  table.
- *The guard's price.* `ISERROR` leaves the desk's `B26` and `B27` to the evaluator,
  `(unlowered error)` in the census where both were lowered; they compute when an order,
  a bell or a pocket's cash moves, so no frame pays it measurably (below).

**The predictions, before any number.** Written at the build, after the expansion,
the suite's first run and the describe's bless had shown the rows, the formula
cells and the tests, which stand below as measurements beside no prediction; every
other line was written before its number was taken.

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | none: seen at the expansion | *Measurement*: `(rows 1478) (loads yes)` where 1,446, 86,386 bytes where 84,227; 871 formula cells where 861, the describe golden's budget `(budget 30 871 26130 2000000)`, shapes 261 where 254, `(every-frame 0)` and `(cycles 0)` standing, a readers row for the Errors sheet, 22 |
| P2 | the pass and the load | each 0.1 to 0.4 ms longer | *Measurement*, one release runner over `HEAD`'s tree and this one, three rounds alternated, each a median of 21: the load 7.59 to 7.80 ms where 7.28 to 7.59, inside; the pass 10.87 to 11.55 where 10.53 to 10.75, about 0.5 longer. **Missed** in one: the pass dearer than priced, a sheet and 32 rows more |
| P3 | the step, natively | an ordinary frame and a tick within 3% of `HEAD`'s; the edit's own frame and the undo's 0 to 10 µs dearer than `HEAD`'s same frames | *Measurement*, the same runner, five sessions of 1,200 frames, each frame's median: an ordinary frame 1.3 µs either way, a tick 20.0 to 20.1 where 19.5 to 20.5; the edit's frame 9.9 to 10.6 where 6.2 to 6.4, about 4 µs dearer, the frame after it 3.7 to 3.9 where 2.9; the undo's 6.8 to 7.0 where 3.6 to 3.8, about 3 dearer; inside, each. Unpriced: the first frame 155 to 161 µs where 137 to 139, the Errors sheet and the eight jokes computed once |
| P4 | the suite | the limit's test 0.5 to 1.5 s alone; the whole on eight threads within 0.5 s of `HEAD`'s | *Measurement*: 103 green where 96, the crate's 42 and the cartridge's 61; the limit's test 0.47 to 0.49 s alone, three runs; the cartridge's whole on eight threads 9.88 to 9.89 s where 9.63 to 10.27, alternated, inside. **Missed** in one: the limit's test just under its range |
| P5 | the page | 4,000 to 8,000 characters longer | *Measurement*: 3,891,936 characters where 3,882,538, both built over the same module, 9,398 longer. **Missed**: the source's comments and the notes longer than priced |
| P6 | the checks | ALL GREEN, four checks and four controls; `check_layout.ps1` the only check edited; one file added under `src/`; fmt and clippy clean | *Measurement*, as predicted: ALL GREEN, the layout's control a clean copy of twelve files and fourteen mutants, its content pin at 7; `cargo fmt` wrapped lines of the hand's and the tests' twice, the second time a test written after the first, then clean; clippy clean |
| P7 | mutants | 20, each into the source, the library or a content file by one anchor or two; at most one lives at the first run, likeliest a joke reading its view's own figure | *Measurement*: 20 written, each anchor applying once, each expanded and loading. At the first run 19 were caught by a test other than the describe golden, and one lived, the Overview's jokes reading their own figures, which hold the same error: as predicted. A test of the rule the library states, each joke reading the cell its figure divides, was written for it, and the second run caught it and two more (below) |
| P8 | the hand test | the owner's, below | The owner's, 2026-11-15, on the page built here: passed, "Hand tests pass", one change asked, `#NUM!`'s joke in the owner's words (decision 3's amendment); `Proctor!B2` drawn as `#`s, predicted under Found, seen |

**What the build found.** The source expanded to 1,478 rows and loaded at its first
expansion, the `error-joke` verb's expression written whole, its empty text a text
of the formula's and no quoted piece (the trap of `KIT.5`, `SPIKE.md`'s pin record),
and the rows that moved from the committed cartridge were the notes, the order desk's
three, the Errors sheet's 27, caveat 26 with the Caveats' count and four pickers, the
eight jokes in five rows and Trading's sentence. Before any test the probe confirmed
the scoping's questions: `#NUM!` reaching the Certificate, the Overview, the Portfolio
and Trading with no `text` row, `Write!C8` holding it every frame, the cash shown a
frame late at both ends, and COUNTIF telling the `#NUM!` of 1E+308 from the `#VALUE!` of
a Delete. The suite's first run failed two of the hand's own tests and nothing of the
cartridge's but the describe golden: the hand refused to judge the end itself, a guard
drawn too wide, since a decimal of seventeen digits never needs one, a product's
rounding alone does; and a score of none raised ten to the 308th. The limit's test
held at its first run, every cell of every sheet of the edited game the twin's but the
hand's seventeen at most, and the order desk's record of the three refused orders. The
twenty mutants: the views' dollars, the firm's cash, the capital and the net worth each
hiding the error by `IFERROR`, and the joke beside any error, never shown, reading the
other pocket's cash, Trading's reading the firm's, the row shown the second, the
`#DIV/0!` row first, the fill unguarded, the outcome unguarded, the guard on buys alone,
the guard keyed on the price shown and Trading's sentence gone, each caught by the
limit's test, the two rows' by the Errors sheet's test too; the joke shown always caught
by fourteen, every long run's views holding each balance's joke cell empty; the
Certificate's joke gone by five; the Errors sheet shown by the tab order's; the caveat's
place an empty cell by the caveats'; and the Overview's jokes reading their own figures,
which lived, then caught by the new test with the other pocket's and Trading's. So
seven tests were written, four of the hand's and three of the cartridge's, beside the
joke cells every run's views now hold empty; and twenty mutants, nineteen caught at the
first run and all twenty at the second. Natively the
edit's frame is about 4 µs dearer than at `HEAD`, the five joke cells' COUNTIF over the
capital's cone, and no ordinary frame or tick moves.

**The owner's hand test.** On the page built here (`web/index.html`, 3,891,936
characters), 2026-11-15: passed, "Hand tests pass; if everything looks good, commit
before proceeding with slice 2 of PLATE.6", with one change asked, the joke's words
(decision 3's amendment). The owner's shots, in order: the Overview after the waiver
at frame 289, Monday at 13:50, the firm's $7,500.00 and your own $100.00 with nothing
beside either; `Proctor!B2` written 1E+308 at frame 1719 and drawn `##############`,
the record's spelling under Found; the Overview on Tuesday at 14:30, the firm's net
worth `#NUM!` in the middle of its cell with the joke beside it, your own $100.00 with
none; the Portfolio on Wednesday at 7:30, the firm's cash on hand and net worth each
`#NUM!` with the joke, your own two rows numbers; Trading in Wednesday's session, the
cash available `#NUM!`, `[ Switch ]` and "or press C" in their places, the joke on the
row under, and the last order's line "Refused: the cash shown is an error. Orders fill
against cash the books can count."; and, after the undo written at frame 6116, the
Errors sheet's five rows with `E1:E4` 5, 1, `#NUM!` and its joke, and the Caveats' row
27, caveat 26's four fields. The steps no shot shows, the Certificate, a sale refused,
your own cash's buy, the Delete's `#VALUE!` with no joke and the firm's buy after the
undo, passed by the owner's word. Amended the same day and committed once the suite,
the expansion and every check with its control had run green over the amendment, the
page built again, 3,892,029 characters.

## `PLATE.6`, second slice — the ceiling: no book counts past the end of the numbers

*Scoped and built 2026-11-15, the sitting of the first slice's close, at the owner's
word, "commit before proceeding with slice 2 of PLATE.6"; and, asked how to take a
slice that stood in `FRONT.7`'s line of course 7, beside the endings' panels and after
`STATION.7`'s island rung, "Ceiling now, island later": the part no later course
rewrites built now, an order whose fill would take a book past the end refused so that
no count, book or ledger row keeps `#NUM!` once an edit is undone; the island at a
stated net worth left to `FRONT.7`, the slice's third. Each decision below stands as
recommended and was built, the owner's to approve, amend or strike at the hand test,
and every text a player reads is a draft. Nothing was minted. Read for it: the first
slice's record above and its Found, the books past the end by play; `FRONT.7`'s line;
`BUFFET.md`'s "Numbers", "The ceiling" and its pitch, a net worth that needs
scientific notation; the order desk, the positions and the cash in the source; and
`vla-lang` 0.12.0's `calc/value.rs` (`number_to_text`, the join's spelling).*

**The gap, named first.** No break-infinity convention exists in the formula language
(`FRONT.7`'s line): a number ends near 1.8E+308, as Excel's does. Filling it would
change the rows the cartridge writes, a ceiling moving or going, but not cheaply, and
against the treaty Frazaro keeps with Excel, whose numbers end where a double does; and
`BUFFET.md` asks for the end of the game designed within it ("Numbers"). So nothing here
waits on the language, and the slice designs the books within the end.

**What it stands on, read on the day** (Buffet at `9afabd3`, the first slice, committed
and not pushed; 103 tests).
- *The books.* Each pocket's each stock keeps its shares, cost basis, cents bought,
  cents sold and dividends as counts over their twins. Bought and sold are gross and
  grow with every order, while the cash, the start and the fees less the four bought
  with the four sold and the dividends, nets them: turnover alone passes the end long
  before any balance does, an edit of your own cash near the end and two orders (the
  first slice's Found).
- *The sums the books feed.* The cash sums four rows of each kind (`Cash!B3:B4`); a
  sale of part of a holding takes its share of the basis as the shares sold times the
  basis over the shares held, ten times the basis before the division at Sell 10, the
  largest part a button sells, Sell all taking the basis whole.
- *A guard's own arithmetic.* A book near the end and an order's cents summed are
  `#NUM!` themselves, and an error in `IF`'s condition is the `IF`'s value (the first
  slice's reading of `IF`), so a guard that sums would hand the fill its own error.
- *Exact cents.* Whole cents are exact below 2^53 cents, $90,071,992,547,409.91; past
  it a sum of them rounds (`CRITIC.1`'s record).

**The decisions.**

1. **The ceiling: 1E+307 cents, a setting (rec).** `Orders!B49`, a value a person may
   edit, the open book (`BD-3`), below the end of the numbers by a factor of nearly
   eighteen, so that four books under it summed, and ten times a basis under it, are
   numbers (the hand's `the_ceiling_leaves_every_sum_of_the_books_a_number`, by digits:
   seventeen times it a number, eighteen past the end). *The alternatives:* the end
   itself, each sum asked whether it is an error, which the cash's four rows and a
   partial sale's ten times the basis pass while each book is a number, and which
   rounds at the boundary where the guard and the books add in different orders; 2^53
   cents, $90 trillion, where whole cents stop being exact, under the pitch's net worth
   that needs scientific notation, so that `FRONT.7`'s island would move it; a ceiling
   for each pocket's sums.
2. **The books the guard reads (rec).** A purchase: the pocket's cents bought of its
   stock, which bounds the stock's basis and shares. A sale: its cents sold, and the
   pocket's cash after it. Each as the last frame showed it, `Orders!B52:B53` over
   `Positions.last!G2:H9`, as the cash shown and the shares are. Dividends are paid to
   the shares held, at no order, and stand unguarded (Found). *The alternatives:* each
   pocket's sums of four, which no single book passes first; the shares and the basis,
   bounded already by the cents bought.
3. **Each book compared with the ceiling less the order's cents, never summed (rec,
   found at the build).** `B52>B49-B51`: the ceiling and the cents are each a number
   under the end, so is their difference, and no operand of the guard is a sum that
   could pass it. Written first as `B52+B51>B49`, the near-end session, written before
   its first run, found it failing: a book near a person's ceiling near the end and an
   order's cents summed to `#NUM!`, and the guard's own error became the fill's. Its
   mutant is kept (M6).
4. **Cents no number holds, refused (rec).** A sale's shares at a price a person's edit
   sent past the end make cents of `#NUM!`; `ISERROR(B49-B51)` refuses that order, and a
   ceiling a person wrote as no number refuses every order. *The alternative:* the error
   passed through, the fill `#NUM!` and every book with it.
5. **The seventh outcome and its sentence, a draft (rec).** `B27`'s 7, after the error's
   6 and before a fill, its label "7 the ceiling"; Trading's sentence, "Refused: the
   order would take the books past their ceiling.", plain, with no number, since the
   ceiling is a setting and the sentence speaks for the last order after the setting
   moves; the number stands in `Orders!B49` and in caveat 27. The order asked is `B50`,
   the fill the desk makes but for the ceiling, and `B26` it unless refused. *The
   alternative:* the number in the sentence, read from the setting now, which would name
   a ceiling the order never met.
6. **The hand takes a person's ceiling (rec).** `Plate::ceiling`, `None` for the
   cartridge's own, which no book of whole cents below 2^64 reaches, and a person's
   ceiling in whole cents judged exactly, a purchase by its cents bought and a sale by
   its cents sold and its cash shown, the order keeping what it asked and the books it
   was judged by. So the oracle is a session at lowered ceilings held cell by cell by the
   order desk's holders, every rule of the guard refusing alone and each boundary
   filling, and a second session near the end for what the hand's integers cannot hold,
   its refusals witnessed by the cells' own numbers. *The alternative:* a decimal hand
   near the end, which the first slice's counting of digits held for one cell and the
   books' every cell would not.
7. **Caveat 27 (`BD-5`), a draft (rec).** "The books stop counting at 1E+307 cents: an
   order that would take them past it is refused."; "A broker refuses an order too large
   for its limits, against a fat finger; a ledger never refuses a digit."; "Management
   has capped ambition at a level it describes as aspirational."; its place
   `Orders!B49`. Marked for `MENU.8`.
8. **No view shows the ceiling (rec).** The refusal says it, the caveat states it and the
   Orders sheet holds it; a view's line for a limit no play at course 0 meets would teach
   nothing.

**Found at the scoping and the build**, none minted:
- *A person's edit near the end still passes it.* The ceiling refuses orders and shrinks
  nothing: cash edited past it may buy, a purchase lowering it, and while the cash shown
  is past it no sale fills, not even a share's; an edit of the start, the capital or a
  price past the end is the first slice's error.
- *A holding's value past the end by its price.* Shares bought under the ceiling are
  worth past the end at a price eighteen times their cost's: a value's error, shown with
  its joke, which recovers as the price falls; and a sale of them is refused, its cents
  no number (decision 4), until it does.
- *Dividends unguarded.* At the ceiling the dividends' count passes the end after about
  670 years of game time, General Widget's 50 cents a share a quarter on the 1.3E+303
  shares its start's price lets the ceiling buy.
- *The card's own ceiling* stands below this one (the first slice's Found, 9E+303 cents a
  worth): a firm's book under the ceiling can still give a card `#NUM!` at its turn.
- *An undo after orders near the end* leaves your own cash at the orders' real result,
  near the end and of either sign, every book a number: the ceiling keeps the books
  counting, and the sandbox's money is the sandbox's.
- *Exact cents end at 2^53 cents*, $90 trillion, far under the ceiling; past it whole
  cents sum inexactly, the owner's decimal-precision item's question (Frazaro's,
  unminted), not this slice's.

**The predictions, before any number.** Written at the scoping and the build, each
before its number was taken (the rows' range before the first expansion).

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the cartridge | 1,490 to 1,500 rows; 876 formula cells; `(every-frame 0)` and `(cycles 0)` standing | *Measurement*, as predicted: `(rows 1494) (loads yes)`, 88,008 bytes where 86,433; 876 formula cells, the budget `(budget 30 876 26280 2000000)`, shapes 266 where 261, alone 616 where 611, levels 107 either way; the Orders sheet's readers 130 where 115; the shapes below the fill three levels deeper, 10 to 13 |
| P2 | the pass and the load | each 0.0 to 0.3 ms longer than `HEAD`'s | *Measurement*, one release runner over `HEAD`'s tree and this one, three rounds alternated, each a median of 21: the load 7.66 to 8.20 ms where 7.45 to 8.35, inside; the pass 11.08 to 11.40 where 10.75 to 10.86, its medians 0.38 apart. **Missed** in one: the pass dearer than priced, the order desk's comments and sixteen rows |
| P3 | the step, natively | an ordinary frame and a tick within 3% of `HEAD`'s; an order's frame 0 to 2 µs dearer | *Measurement*, the same runner, five sessions of 1,200 frames, a share bought every fifteen frames, each frame's median: an ordinary frame 1.3 to 1.4 µs either way; a tick 22.4 to 24.1 where 23.0 to 23.5, its medians 2% apart; an order's frame 17.4 to 18.3 where 16.8 to 17.1, 0.4 to 1.4 dearer; inside, each. Unpriced: the frame after an order 9.1 to 9.6 where 8.9 to 9.1 |
| P4 | the suite | 4 to 6 tests more; the ceiling's exact session 0.3 to 1.5 s alone; the near-end session under 0.5 s; the whole on eight threads within 0.5 s of `HEAD`'s | *Measurement*: 107 where 103, the hand's 44 and the cartridge's 63; the two new sessions 0.12 s together on one thread, three runs; the cartridge's whole on eight threads 9.28 to 9.36 s where 9.20 to 9.52, alternated, inside. **Missed** in one: the exact session under its range, its 32 frames cheaper than priced |
| P5 | the page | 2,000 to 5,000 characters longer | *Measurement*: 3,897,066 characters where 3,892,029, 5,037 longer. **Missed**, by 37: the source's comments |
| P6 | the checks | ALL GREEN, four checks and four controls; no check edited; no file added | *Measurement*, as predicted: ALL GREEN; fmt and clippy clean; `expand --check` the same |
| P7 | mutants | 10 to 14, at most one living at the first run | *Measurement*: 13 written, each anchor applying once, each expanded and loading, all caught at the first run by a test other than the describe golden (below) |
| P8 | the hand test | the owner's, below | The owner's, 2026-11-15, on the page built here: passed, "Tests pass", no change asked; `Orders!B49` drawn as `#`s, as predicted; a paste by Control+V into a cell found missing, Alonzo's (below) |

**What the build found.** The source expanded to 1,494 rows and loaded at its first
expansion, and the rows that moved were the outcome's label, `B26` and `B27`, the
ceiling's block of twelve, caveat 27 with the Caveats' count and its four pickers,
Trading's sentence and the notes. The suite's first run failed the describe golden
alone: every frame of every existing session held the six new cells of the desk, the
ceiling spelled as the record spells 1E+307, its 308 digits. The ceiling's session
failed once, at frame 28, where your own pocket held nineteen shares and ten sold left
nine, so ten more were a shortage of shares and no ceiling's; a share more was written
in its place. The near-end session, written before it ran, found decision 3: the guard
as first written summed. Its witnesses were added after its first pass, so that each
refusal is shown to be the one the script means: the cents alone past the cartridge's
own ceiling, the sale's cents and the holding's value errors, and the book and the cents
summing past the end in a double. The thirteen mutants: the fill unguarded and a
purchase unguarded, each caught by both new sessions; the guard summing and cents no
number holds unguarded, by the near-end session alone; a sale's cents sold unguarded, its
cash after unguarded and a refusal at the ceiling itself, by the exact session alone; the
cartridge's own ceiling at the end, the books shown of the other pocket and the cents sold
read as the cents bought, by both and every session the desk's holders hold, nine, eight
and seven tests; the seventh outcome unsaid and Trading's sentence gone, by both, the
near-end session finding Trading's last line an error after the undo; and the caveat's
place an empty cell, by the caveats' own test. Natively an order's frame is about a microsecond
dearer, the guard's five cells, and no other frame moves.

**The owner's hand test.** On the page built here (`web/index.html`, 3,897,066
characters), 2026-11-15: passed, "Tests pass; if everything looks good, commit before
suggesting next steps", no change asked. The owner's shots, in order: the Orders sheet
after the waiver at frame 344, the ceiling's block in rows 49 to 54 under its six labels,
`B49` drawn `#######` as predicted (1E+307 spelled in full, the first slice's Found), the
guard's cells 0 with no order; Trading after Monday's closing bell, "Bought 10 shares of
Longshot Mining at the price shown." and the firm's cash available $7,490.00; and, after
`Cash!B1` written 1.7E+308 at frame 4234, the Caveats' row 28, caveat 27's four fields.
The steps no shot shows, the lowered ceiling's three orders, your own pocket's buy and
sale, the ceiling written back and the refusal near the end, passed by the owner's word.

**Found at the hand test**, none minted: *a paste by Control+V into a cell.* The owner,
pasting 1E+307 into `Orders!B49`: "I realized we have not yet implemented copy/paste
keyboard shortcuts in Alonzo." The page writes a typed cell (`cellEditRow`) and Delete's
empty text (`cellClearRow`), and a copy of a cell or a paste into one by the keyboard is
neither; Alonzo's, its `ENGINE` family's, the owner's to mint. *Minted there 2026-11-16 as
`ENGINE.39`, "Mint 1 and 2 as one page item", copy, cut and paste in the grid beside a held
key repeating a command, citing this Found; being scoped there on 2026-11-18, and not in
`alonzo` 0.4.0 (`HOUSE.2`'s pin of that day; `OMAKASE.md`, `MISE.6`).*

## `HOUSE.2` — the pin to `alonzo` 0.4.0: the engine at 0.4.0, the language at `vla-lang` 0.14.0, the kit and the page copied again at `v0.4.0`

*Built 2026-11-18 at the owner's word, the day Alonzo published 0.4.0: "Move Buffet to Alonzo
0.4.0 ... respell the eight calls the new kit changed, and hold every frame, golden and check.
Record it under HOUSE.2, as the pin to 0.3.0 was (2213bf1)." No decision was asked and nothing
was minted; the manifest stays `(spec 6)`. Read for it: Alonzo's `RELEASES.md`, `## 0.4.0`,
"For a game that carries the page"; the pin to 0.3.0's record (`FRONT.2`'s second slice, "The
pin", above), whose lesson is that a re-expansion is never blessed at a kit move; and a trial of
2026-11-15 in scratch copies alone, Buffet at `6f215a1` over Alonzo's `32ff4b3` and `vla-lang`
0.13.0, which passed 107 of 107 with the describe golden unmoved and tried neither the kit nor
the page. Buffet stood at `6f215a1`, `PLATE.6`'s two commits pushed first at the owner's word,
CI's run 44474409272 green.*

**The pins.** `alonzo = "=0.4.0"` and `vla-lang = "=0.14.0"`, the version that release pins, so
that the lock holds one copy of the language; `cargo update -p alonzo -p vla-lang` moved the
lock's two packages and nothing else, `alonzo`'s checksum `ec3341ca…`, the one crates.io gave
the release's Trusted Publishing of `6e7b659`, and `vla-lang`'s `21af335e…`. The oracle compiles
with no port, as at the trial. `Cargo.toml`'s comment names what the two releases bring.

**The copies.** Each read off the tag by `git show v0.4.0:<path>` in Alonzo's clone and written
here byte for byte, the kit checked out CRLF and the page's files LF, as `.gitattributes` gives
them; its SHA-256 and length over its LF text are `check_vendored.ps1`'s baseline. The kit
30,107 bytes where 26,695 at `v0.3.0`; the template 946,703 where 757,751; the host 209,349
where 175,004; the fake module 49,732 where 48,023; the viewport 158,424 where 136,286.

**The eight calls.** Over the new kit the source fails at the pass, at its line 1625,
`vla-macro-piece-unknown`: `put` takes its pieces by name (`KIT.6`'s second slice, over
`VLA.9`). The seven Ledger `put`s of the Write sheet, `(put "A2" "Ledger" (<> Orders!$B$26 0)
(& "A" Orders!$B$30) Orders!$B$29)` and its six kin, are each `(put "A2" (sheet "Ledger") (when
(<> Orders!$B$26 0)) (address (& "A" Orders!$B$30)) (value Orders!$B$29))`, keeping its comment;
and `src/kitchen.vla`'s `strings-sheet`, whose walker `labels-from` is gone (`KIT.6`'s third
slice), calls its verb, `(labels content (address-expand 1 (+expand n 2)) ...)`, the walk's row
and column its first cell. The kit's other two moves, `twin-of` to the language's `last` and
`first-of` to `its-last`, have no call here. Measured first in Alonzo's release session and
again here: `expand --check` says `same` against the committed `src/buffet.vla`, so every row
the cartridge writes is the committed one's, byte for byte, and no expansion was written anew.
The edits were made by Edit, every line of both files still CRLF.

**What held.** 107 tests, the hand's 44 and the cartridge's 63, every frame of every session
held to the hand, the cartridge's in 9.4 to 10.2 s by cargo's own threads; and the describe golden
unmoved, its census and plan in 0.12.0's words over 0.14.0, where 0.3.0's pin moved it.
fmt and `clippy -D warnings` clean. The module, built from the registry crate under the
workspace's release profile, is 2,258,410 bytes, 0.3.0's 1,866,609: 304 bytes over the
2,258,106 CI built for Alonzo's own page of the release, as 1,866,609 stood 304 over 0.3.0's
1,866,305, the paths the module names. The page is 4,670,699 characters, `PLATE.6`'s 3,897,066:
the module's growth in base64, about 522,400, the template and the three scripts' 247,144, the
kit's 3,412, and the eight calls and the source's notes the rest.

**The offline check.** Its first run failed: "the template reaches out: 'url('". The 0.4.0
template's `personFile()` writes a save as a download by `URL.createObjectURL(` and
`URL.revokeObjectURL(`, which the check matches without case (`ENGINE.5`). Buffet's check is
Alonzo's of `v0.1.3`, adapted at `HOUSE.4`, and took none of the stores Alonzo's has held
since: `ENGINE.23`'s keymap, written to the page since 0.2.0, and 0.4.0's two, a person's file
and a run's unsaved work. So its promise, "writes nothing anywhere", had stood loose since the
pin to 0.2.0. Alonzo's three rules at the tag are carried whole, adapted to this check's shape
(no hands, no Web Audio list): `localStorage` once in the template, in `browserStore()`;
`showSaveFilePicker`, `createObjectURL` and `revokeObjectURL` once each, in `personFile()`, set
aside before `url(` is looked for, an object URL being no address on any network; `indexedDB`
and `navigator.locks` once each, in `workStore()`; none of them in any script, and no other
store named anywhere. The control has twenty-two mutants where it had fifteen: the template
naming `sessionStorage`, and `localStorage`, the save picker or `indexedDB` a second time; the
host's source naming `localStorage`, the save picker or `navigator.locks`. Each fails by its
words, and the real set and a page built from it pass. `run_checks.ps1`'s line for the control
says so; the checks and their controls ALL GREEN under Windows PowerShell 5.1, and pwsh is CI's.

**The loop oracle.** Alonzo's `tools/check_host_loop.ps1 -Page` over the page built here, from a
`git archive v0.4.0` of Alonzo in the scratchpad and never Alonzo's own tree, where a session
was scoping `ENGINE.39`; headless Chrome 157, none running before, one browser at a time
(`-Jobs 1`). The check holds Alonzo's own floors and its count of 29 cases, so on a game's page
it fails by the cases the page lacks; its page's verdicts alone are read. Every case a game's
page runs said OK: `abi`, `pass`, `replay`, `grid`, `empty`, `still`, `views`, `changes`,
`continue`, `palette`, `drawer`, `keys`, `commands`, `settle`, `widths`, `modes`, `save` and
`keep` at the first ratio, and the fifteen but `replay`, `save` and `keep` at 1.5, the plan the
check runs; no `FAIL:` line. `grid` among them, `gridMoves 6` and `gridUnbound 6` at both ratios,
where 0.3.0's page failed it on the Exam scrolled to its end from 2026-10-30, `BT1:DH33` past
the window `BT1:DG33`, and over Alonzo's `6187705` the same, re-measured earlier this day (`MISE.6`):
met by `ENGINE.43`. Run twice, over the page before the source's
notes moved and over the page as it stands, the same. Two cases pass over nothing of Buffet's:
`save` saves and opens the committed replays, `saves 0`, and `keep` holds its closure law over
them, `cartridges 0`, and Buffet has none until `HOUSE.4`'s second slice; `keep`'s drafts and its
store's double did run, `typed 1`, `draftsRound 1`. So Save, Open and a restore on Buffet are the
hand test's.

**What arrived with the page, unasked.**
- *Save and Open* (`ENGINE.5`): Save, Ctrl+S, writes the running game to a file of the person's
  as a cartridge at its frame, through the browser's save picker opening in Downloads; Open,
  Ctrl+O, opens one there, its rate and seed kept. `MISE.6`'s ask of course 7, a career worth
  saving, met.
- *Unsaved work kept* (`ENGINE.38`, `ENGINE.40`): a run's work, a keyframe and the frames since,
  and the drawer's typing kept in the browser through a closed tab or a crash and offered back on
  a line under the bar, Restore or Discard, never restored silently. On a page opened from a file,
  as Buffet's is, Autosave is off until the person's Turn on Autosave, since any page opened from
  a file can read what the browser keeps for files; and the answer is the browser's, kept under
  `alonzo-autosave` for every page from a file, so a browser that answered on Alonzo's page is
  never asked on Buffet's.
- *The bar says the last act's answer* (`ENGINE.42`): a sentence answering a person's act takes the
  bar though it repeats the one before, so a key on a view says it is read-only at every press.
- *Looks drawn*: a sheet's conditional formats, a format of groups alone over a cell's own, a
  font's family, underline and strike (`ENGINE.37`), a line feed in a wrapped text (`ENGINE.36`),
  and a text's own font after a clip (`ENGINE.41`: the Exam's options bold at every question,
  `MISE.6`'s row met). Buffet writes no conditional format yet.
- *The region shown is the region seen* (`ENGINE.43`), and the drawer's boxes the page's own
  (`CART.3`'s third slice).
- The page's version 7, a command's key with modifiers, which a `(spec 6)` cartridge does not take.

**Found, none minted.**
- *The offline check's stores*, loose since 0.2.0's pin: carried here, above.
- *Two loop cases pass over nothing of Buffet's*, `save` and `keep`, until a replay of Buffet's
  is committed (`HOUSE.4`'s second slice).
- *`ERROR.TYPE` pinned and not taken*: `vla-lang` 0.14.0 names an error's kind (`KERNEL.68`), and
  the Errors sheet's row shown stays a value, `#NUM!`'s, until a dish reads it off the error
  (`MISE.6`; the source's and the kitchen's notes say so).
- *A key on a view still answered in an author's words*: `CART.9`'s decision 49 is released, and a
  sheet both `game` and `read`, as every view is, refuses a key in the manifest's terms, now in the
  bar at every press (`MISE.6`).
- *`PLATE.6`'s paste by Control+V* is Alonzo's `ENGINE.39`, minted 2026-11-16 and being scoped
  there, not in 0.4.0 (its Found, above; `MISE.6`).

**The owner's hand test.** 2026-11-19, the page from disk: passed at every step but one, the
notice of Autosave from a file, which the steps put under the bar as the page opened. The steps
were wrong twice. The notice comes at a run's first act, and only in a browser that has not
answered Autosave for pages from a file: the answer is kept under `alonzo-autosave` beside the
keymap, which every page from a file shares (`ENGINE.38`'s decision 9), and the owner gave it at
`ENGINE.38`'s hand test of 2026-11-16. The witness is the restore after a closed tab, which
passed: from a file with Autosave unanswered, the page keeps nothing and offers nothing back.
*Measured after, over the DevTools protocol, the same page from disk in a fresh headless profile
of Chrome 157:* no line under the bar at the opening or at Buffet picked; at the first act, A on
the Exam, the sentence with Turn on Autosave and Keep Autosave off; Turn on Autosave hid it, the
bar said "Autosave is on: unsaved work is kept in this browser until you save or discard it.", and
the browser kept `on`; the same profile opened again offered the work back, Restore and Discard,
and showed no notice at an act, as the owner's browser does. The
rest as the steps said: the Exam's options bold at every question, a key on a view answered in
the bar at every press, the waiver and the Certificate's `[ Open for business ]`, a buy on
Trading landing in the Ledger's row 2 through the respelled `put`s, Save into Downloads and Open
back to the saved frame, and Restore after the tab closed.

## `HOUSE.4`, second slice — the loop oracle over Buffet's page: the owner's session, its golden and the hand

*Scoped 2026-11-19 at the owner's word, "Push it, then scope HOUSE.4's second slice", after
`HOUSE.2`'s pin to `alonzo` 0.4.0 was pushed (`bf1e6d9`, run 44499808109 green). It reads three
sources. The first is `OMAKASE.md`'s line (`COURSE.0`'s item 21). The second is course 0's tasting:
the owner's own session as the replay golden, its golden the Report sheet, the oracle crate's replay
test, and the loop oracle's `replay` case with a mutant page that fills at the new day's quote. The
third is Alonzo at `v0.4.0` (`6e7b659`), read from a `git archive` of the tag: `web/README.md`'s "A
game's page", the template's `replay`, `save` and `keep` cases, its builder, its offline check and
its loop check. Each decision carries its recommendation, for the owner to approve, amend or strike
before the build. Decisions 1 and 2 are the forks, each with its designs and their costs. The
predictions are written before any number. Approved whole by the owner the same day, each fork at
its recommendation: the session from the exam to the first quarter's card, the hand the Rust hand's
answers written into the page, and decisions 3 to 10 as written. The build waits for the owner's
word, and begins with the owner's session. Nothing is built.*

**What it stands on, read on the day.**
- *The cases on a game's page.* At `v0.4.0` a page without Alonzo's own cartridges runs eighteen
  cases and says the fifteen others are not on it. Over Buffet's page at `HOUSE.2` every one said OK,
  at a ratio of 1, and the fifteen without `replay`, `save` and `keep` at 1.5. `pass` built the source
  to its cartridge and `grid` held six unbound views. Three held nothing of Buffet's: `replay` plays
  each cartridge that has a `replay-` and a `golden-` block, and `save` and `keep` play the page's
  committed replays (`saves 0`, `cartridges 0`).
- *A replay* (Alonzo's `docs/SPEC.md`; the recorder in `host.js`). It is a header,
  `(replay "<name>" (spec N) (cartridge "<SHA-256 of the cartridge's LF text>") (seed S))`, then a
  form for each frame that has rows, and the last frame always, so that it names its length. A
  frame's rows are the Input sheet's cells that changed and each edit of a person's that landed;
  never a derived write, never a value. The page's own recording of what it plays must equal the
  committed text byte for byte, its leading comment lines aside (`round`). So a replay names the one
  cartridge it was recorded on, and a cartridge changed leaves it stale until its header names the
  new one. A replay is recorded by playing in the page: the recorder starts at each load and holds up
  to 8 MiB, and the drawer's "The replay" gives it, Copy the replay. Brickwork's, the owner's one
  game of 41,129 frames, is 3,994 frame forms and 8,056 rows, 281,623 LF bytes.
- *A golden* is the `view` grid record of one window at the replay's last frame, held whole: every
  sheet's `(sheet …)` row, the window's formats and its cells, the window the author's choice. No tool
  of Alonzo's writes one; each was written by a scratch player over the engine, the seed written to
  `Clock!B3`, each frame's rows one write, then the step.
- *The hand* is the block `hand-<name>`, the one that is a script. It registers a constructor, which
  the page makes with the replay's seed and asks, every frame from 1, `frame(n, rows)`. It answers
  `{ cells }`, `{ "Sheet!A1": value }`: any cells at any frame and none at most, each held exactly, a
  float to its last bit. The page reads one view of each sheet an answer names, at that frame. The
  first difference fails the case at its frame: "buffet's committed replay, played on a fresh host,
  is not its hand's at frame N: Sheet!A1 holds …, and the hand …". A replay with no hand is held to
  its golden alone and counted `alone`.
- *What the cases cost,* measured for Brickwork. A replay plays at 96 µs a frame, and a view read
  costs from 104 µs (the Tally) to 2.8 ms (the Wall). `save` saves once, at the first frame at rest
  at or after the middle, and compares the rest at every 64th frame. `keep` restores at frame 1, at
  every 64th frame at rest and at the last, so about one restore for each 64 frames, and grew about
  half a millisecond a frame. Brickwork's three took 58.8, 28.3 and 52.0 s. Buffet natively, measured
  today over `bf1e6d9`'s cartridge in a scratch runner: a load 8.3 ms (the median of 21, 7.7 to 9.1),
  a frame 3.55 µs.
- *Alonzo's Rust for replays is its tests' alone.* The player, the golden's writer, the save's helpers
  and SHA-256 are all `#[cfg(test)]`. The crate's public API is `Cartridge::load`, `write`, `step`,
  `view`, `settle` and `describe`. A game's oracle writes its own player over them and `vla-lang`'s
  reader, as Buffet's tests already write a person's Input rows (`input_rows`,
  `oracle/tests/cartridge.rs`) and the host's derived writes.
- *Alonzo's loop check judges a game's page only for its own,* a page of Kairos alone that it builds.
  It holds that page to floors of its own: eighteen cases, the fifteen said absent, and, with the
  module, one replay held to its golden, its round and its hand, none `alone`. It has four mutants
  there: its hand a frame late, its hand's rule wrong, its hand dropped, and the bench refusing. Its
  `-Page` judges whatever page it is given as Alonzo's own: over Buffet's it printed every case OK
  and failed, "the oracle ran 15 case(s), not the 29 pinned here".
- *Alonzo's builder and offline check carry the three blocks* (`ENGINE.3`'s second slice,
  `ENGINE.16`). They write `<name>.replay.vla` and `<name>.golden.vla`, neither without the other,
  and `<name>.hand.js`. The offline check holds a hand's source to the code tokens, with no `eval` or
  `Function` and no store's name, under five mutants. Buffet's builder and offline check, Alonzo's of
  `v0.1.3` adapted, take none of them: a `src/buffet.replay.vla` today would be built as a library,
  and `check_layout` would refuse it as a stray. `REUSE.toml` already gives `src/buffet*.vla` "its
  replay and its golden" (AGPL-3.0-or-later AND CC-BY-SA-4.0).
- *The frames,* counted by the hand's calendar at the game's pace in the same runner. A weekday is
  1,695 frames, a weekend day 720, the first day 1,411 and the first week 9,631. The first quarter's
  card turns at frame 128,611 on day 91, 71 minutes 27 seconds at the manifest's 30 frames a second,
  with the exam's wait before it. The year's card turns at frame 516,991, 4 hours 47 minutes.
- *The language's gap.* `vla-lang`'s `SIN` and `^` are Rust's `sin` and `powf`, whose last bit
  differs by platform. Over 146,000 of Buffet's inputs (2026-10-14), Windows' differed from the module
  on 2,272 and 14,183, and V8's `Math.sin` and `Math.pow` on 1,377 and 14,237; a price rounded to whole
  cents differed on none. The hand's doctrine (`oracle/src/buffet.rs`, its head) follows from it: in
  the browser nothing unrounded is held, and money is rounded once to whole cents first. The fix, the
  `libm` crate on every target, was the owner's choice and then deferred, unminted, on 2026-10-14. It
  is Frazaro's, named here and not built around.
- *Alonzo's hands against their Rust.* Naught's is 1,385 lines of JavaScript against 3,893 of Rust
  before its tests, Brickwork's 430 against 1,364; Seiyaku's is 315, Superposition's 255, Kairos's 141
  and Sand's 129. `buffet.rs` is 4,383 lines, 2,751 before its tests. The cells it is held by are
  written in `oracle/tests/cartridge.rs`: `day_held`, `market_cells`, `stocks_held`, `cash_held`,
  `trading_cells`, `performance_held`, `report_cells` and the rest.

**The decisions.**

1. **The session: the owner's own, at the game's pace, from the exam to the first quarter's card
   (rec).** As course 0's tasting has it. The owner picks Buffet on the page built from this
   commit's cartridge and takes the exam. Then they open for business, buy and sell in both pockets,
   proofread a prospectus and play the first week's days. Then they let it run, the tab in view, to
   day 91's first frame, where the Camera shows the Report and its first card, and copy the replay a
   few seconds after. That is about 135,000 frames: the owner's attention for about ten minutes, the
   exam and the first week, and about 75 minutes in all. The run may be paused and resumed at will,
   but in one load: a hidden tab steps nothing, and a reload restarts the recorder. Its golden is the
   Report sheet's window, the card, as the tasting says. Each later dish that moves `buffet.vla`
   restamps the header and writes the golden and the hand again by one command (decision 3). The
   session is played again only when a dish changes what its acts do. *The alternatives:*
   - *The first week alone,* about 14,000 frames and ten minutes of the owner's. Its golden is the
     Portfolio, both pockets' books, and the card is left to the native test of a scripted quarter
     that holds it now. Its loop cases cost about a tenth of the quarter's (220 restores against
     2,100), and the browser never sees a card's turn.
   - *The first week, then the paces typed* (`Day!B1` and `Day!B28` at 3,600, a person's sandbox
     edit) to the card, about 16,000 frames. The card reaches the browser at the week's cost, and an
     edit lands in the replay. But the course's own replay would show a person bending time, which
     the owner's rule keeps out of the game's controls and out of every player's pace.
   - *No session of the owner's,* but one written by the oracle's scripted player (`MISE.5`, not yet
     built) and written again whenever a dish moves it, as Naught's stand-in is. It costs no time of
     the owner's and every frame is reproducible, but no person's play is in it. `MISE.5` may add one
     beside the owner's.

2. **The hand: the Rust hand's answers, written into the page by the oracle (rec).** The oracle plays
   the owner's session through `buffet.rs` (decision 3) and writes `src/buffet.hand.js`. It holds a
   player of about a hundred lines, made with the seed and asked each frame, and a table of the
   rounded cells the Rust hand holds. The cells are the clock shown and the day, the four prices,
   each pocket's cash, holdings and net worth, the positions, the ledger's last row, the exam's score
   and the capital, the hustle's fees, and the card. The table holds them at each frame where a key, a
   click or an edit landed and the frame after, at each day's first frame and at every 512th frame,
   and nothing at the frames between. So the browser's session is held, frame by frame where it
   matters, to the second plate, the oracle the native tests already trust, and no rule is written a
   third time. A dish moves the hand by the same command as the golden. The page then holds the
   module in the browser against the native hand, the comparison the language's gap makes worth
   making. *The alternatives:*
   - *The whole game a third time in JavaScript* (the line's "the third plate"). By Alonzo's ratio it
     is about 900 to 1,400 lines for `buffet.rs` and as many again for the cells `cartridge.rs`
     holds, and every rule of every later dish is written three times. Its market is computed with
     V8's `Math.sin` and `Math.pow`, which differ from the module's, so a price that rounds on a half
     cent would fail a correct page.
   - *The rule alone in JavaScript* (the tasting's "the second writing of the rule"). It covers the
     order desk and the books, and what they stand on: the calendar, the market's prices, the exam's
     capital and the hustle's fees, about two thirds of the whole. The words and the looks are left to
     the native test and the golden. It carries the same V8 risk on the prices, and every money rule
     of every later dish is written three times.
   - *No hand.* The session is held in the browser to its golden and its round alone, `alone`
     allowed by Buffet's own floors, and every frame is held natively. It is the cheapest, and a
     divergence between the module and the hand is found only where it reaches the card.

3. **The oracle plays the session natively and writes what it owns: `oracle/examples/replay.rs`
   (rec).** The command is `cargo run --example replay [-- --check]`, beside `expand`. It restamps
   the replay's header with the SHA-256 of `src/buffet.vla`'s LF text, the oracle's own, about 85 lines
   as Alonzo's is, held to the standard's vectors. It plays the session over the engine as the host
   plays it: the seed, each frame's rows one write, the host's derived writes, then the step. It
   writes `src/buffet.golden.vla`, its SPDX lines and the Report's window, and `src/buffet.hand.js`.
   With `--check` it writes nothing and says `same` or names the file that moved. Beside it is the
   test the tasting asks for, the session held at every frame to `buffet.rs`: the calendar at each
   frame, the market at each tick, the books at each act, and everything at every 64th frame, by the
   functions the existing tests hold their cells with. The golden and the hand are each held to their
   file. A golden that moves is explained line by line before it is written again, as the describe
   golden is. *The alternative:* ask Alonzo to publish its player, its golden's writer and SHA-256
   (`MISE.6`), and wait.

4. **The files: `src/buffet.replay.vla`, `src/buffet.golden.vla` and `src/buffet.hand.js` (rec).**
   They sit beside the cartridge, since every block of the page is a file of `src/`, one rule for the
   builder and the offline check, and `REUSE.toml` already names the first two. The hand falls under
   `src/**`'s AGPL, its SPDX lines first, and `.gitattributes` keeps it LF. Each `.vla` opens with its
   SPDX comment lines, which the page and the oracle set aside before the header. `check_layout` takes
   three kinds, a replay, a golden and a hand, one each, and `src/`'s other `.vla` stay libraries by
   name, no longer "every other". *The alternative:* the hand beside its generator under `oracle/`,
   with the builder's and the check's rule widened to one file outside `src/`.

5. **The builder and the offline check take the three blocks by Alonzo's rules of `v0.4.0` (rec).**
   `build_web.ps1` writes `replay-buffet` and `golden-buffet` after `source-buffet`, and `hand-buffet`
   as the one script block. It refuses a replay without its golden and a golden without its replay.
   `check_web_offline.ps1` owes the three both ways and holds the hand's source as Alonzo's does: the
   code tokens, no `eval` or `Function`, no store's name. It also holds the replay's header to the
   SHA-256 of the cartridge beside it, as `check_vendored` computes its digests, so that a page built
   over a stale replay fails there before any browser runs. Its control goes from 22 mutants to 29:
   Alonzo's five of the hand (its block dropped, a `fetch`, an `eval`, `localStorage`, `indexedDB`),
   a replay block no file is, and a replay naming another cartridge.

6. **The loop check of Buffet's own: `tools/check_host_loop.ps1`, Alonzo's of `v0.4.0` adapted to
   judge Buffet's page (rec).** It is adapted as the builder and the offline check were. Its browser
   is found as Alonzo's is (`ALONZO_BROWSER`, `-Browser`, Chrome or Edge at their paths); it is skipped
   without a page or a browser, and it takes `-Jobs`. It runs Buffet's page at a ratio of 1, every
   case, and at 1.5 every case but `replay`, `save` and `keep`. Each run is judged by Buffet's own
   floors: its eighteen cases and the fifteen absent; `replay` with one hand, none `alone`, its golden
   and its round; `save` and `keep` each over the session. `-Control` builds five mutant pages:
   - the hand a frame late;
   - the hand filling the first buy at the new day's quote, at that frame (the tasting's);
   - the hand's block dropped, counted `alone`;
   - a cell of the golden changed;
   - a row of the replay cut, its round failing.

   *The alternative:* ask Alonzo for its check to judge a game's page by the game's floors
   (`MISE.6`), and run it from a tag's archive in CI.

7. **CI (rec).** In the crate job, after the page with the module and its offline check:
   `cargo run --example replay -- --check` beside `expand`'s, the native test inside `cargo test`,
   and the loop check with its control under `pwsh`, over Chrome from the image. In the ratchets job,
   `run_checks.ps1` runs the new check over its page without the module, the double's cases alone, one
   browser at a time as Alonzo's workflow has it on Windows; its control's mutants that need the
   module say they do not apply, as Alonzo's do.

8. **Held rounded, the gap named (rec).** The golden and the hand hold whole numbers, cents,
   hundredths of a percent and words, never a value of the market unrounded. A rounded cell that
   differs between the module and the hand at a frame is a finding for Frazaro's `libm` fix, never a
   hand to bend.

9. **The stale places, rewritten at the build.** `OMAKASE.md`'s line names `replay`, `grid` and
   `pass`: `grid` and `pass` pass now, and `save` and `keep` join. "Buffet's own JavaScript hand, the
   third plate" becomes decision 2's hand, and course 0's tasting names the session's shape. `MISE.6`
   takes whichever rows of the Found below the owner keeps. `README.md`'s Building names the replay's
   command beside `expand`'s, and `Delta/CLAUDE.md`'s Buffet map names the files, the check and the
   edits that trip them.

10. **The hand tests, the owner's.** At the build's start, the session, recorded on the page as
    decision 1 says, its steps given then. At its end, on the page built from the commit: the
    drawer's "The replay" holds Buffet's, and Play the replay plays it, the exam answered as it was.

**The predictions, before any number.**

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the session | 133,000 to 137,000 frames, 128,611 to the card's turn and the exam's wait before it; 40 to 150 KB of LF text, the pointer's moves the most of its rows | |
| P2 | the golden | the Report's window, under 10 KB | |
| P3 | the native test | every frame held, under 30 s | |
| P4 | the hand | a player of about 100 lines and a table of 10,000 to 20,000 cells in rows of one order, under 200 KB | |
| P5 | the page | 4,670,699 characters grown by 100,000 to 350,000 | |
| P6 | the loop check | 18 cases OK at a ratio of 1 and 15 at 1.5; `replay` with one hand, none `alone`, its golden and its round; `save` one save; `keep` about 2,100 restores; under five minutes here, `keep` the longest | |
| P7 | the controls | the loop check's 5 mutants and the offline check's 29, each failing by name | |
| P8 | CI | the crate job longer by 2 to 5 minutes | |

**Found, none minted.**
- *Alonzo's replay machinery is its tests' alone,* so a game's oracle writes its own player, golden
  writer and SHA-256 (decision 3). A row for `MISE.6`, at the owner's word.
- *Alonzo's loop check holds a game's page only for its own,* and its `-Page` holds any page to
  Alonzo's floors (decision 6). A row for `MISE.6`, at the owner's word.
- *`keep`'s watched run and live session need Kairos or Sand on the page:* both count 0 on Buffet's,
  the part of the case Buffet's page cannot hold.
- *A course's ending outgrows a sitting at the game's pace:* the first card at 71 minutes, the year's
  at 4 hours 47, and course 1's rate cycle at years. `BD-1`'s replay golden at a later course will
  want the scripted player (`MISE.5`) or another answer, at that course's scoping.


**The build, 2026-11-21.** *Built at the owner's word, "Recommendations approved; mint everything, then proceed with the second slice of HOUSE.4", the same day the owner recorded the session, from the replay the page's drawer copied. Every decision as approved, the amendments below each the owner's to strike, none struck; the owner's hand test (decision 10) passed the same day.*

- *The session* (decision 1), recorded on `FRONT.8`'s cartridge (`47ba20e`) and copied a few seconds after the card: 132,945 frames in 635 frame forms, 32,243 bytes of LF text, the clipboard having turned its line ends to CRLF. The Series 777 answered by key, all ten scored right and the fourth style, the firm's capital $10,000; the certificate's O at frame 3,736, the market's first bell, the Camera turning to the Portfolio; 98 orders in both pockets, 97 filled and one refused, at frame 37,152, a sale of Average Co. your own pocket held none of; four prospectuses proofread; every view and the Input tab looked at; then the market let run to the first quarter's card at frame 132,346, the firm's +5.80% against Average Co.'s -1.04%, an alpha of 6.84 points and an A for the quarter and the year so far, the Camera showing the Report the frame after. Between the recording and the build the owner's disclosure spelling (`73c9c26`, `MENU.1`) moved the cartridge in one cell, so the first run of the command restamped the header (decision 3's case). The owner's notes at the session were minted the same day as `COURSE.0`'s dishes 22 to 24 (`OMAKASE.md`): `POT.5`'s first slice, `PLATE.8` and `FRONT.9`.
- *The oracle* (decisions 2 and 3). `oracle/src/replay.rs`: the SHA-256 of FIPS 180-4, held to FIPS 180-2's four vectors and agreeing with `sha256sum` on the cartridge's LF text; a replay read and written back (a row a line, a frame closed by its last row's extra parenthesis, a parenthesis inside a text set aside), `acted`, the player (the derived writes the last step's Write sheet asked for, each its own write, then the frame's rows as one, then the step), the record's values by the language's own reader, and the golden's and the hand's writers. `oracle/examples/replay.rs`: `cargo run --example replay [-- --check]`, 35 s from cold in release, 12 s checking in the dev profile. The native test, `the_owners_session_holds_every_frame_to_the_hand` in `oracle/tests/cartridge.rs`: the test's host gains `play`, the replay's own rows written as the page writes them and the person's inputs read back from them, a key held where its button's cell is 1 by the code the keymap read before the frame binds it to; the exam's every frame held as the scripted exam's are, with its market and stocks; after the begin, the clock at every frame, the market at every tick, and the order desk, the books, the cash, the ledger, the commands, the views, the exam's books, views and keys, the hustle, the Performance sheet and the Report whole at every frame a key, a click or a command landed and the frame after, at each day's first frame, at every 64th and at the last, 2,417 frames; the Camera at every frame, the Report at the card and nothing between, and the sheet it turned to shown at the next frame's rows; the golden and the hand each held to its file; and the session's facts above pinned.
- *The files* (decision 4): `src/buffet.replay.vla`, its comment lines saying what the session holds; `src/buffet.golden.vla`, the Report's `A1:E17` at the last frame, 172 lines; `src/buffet.hand.js`, a constructor of thirty lines and its table, the 63 cells' names once and at each of the 685 frames held the cells that moved since the frame held before, 54,899 bytes. `check_layout.ps1` takes the three as kinds of their own, fifteen files where twelve were, its control fifteen mutants with the hand missing.
- *The page* (decision 5): `build_web.ps1` writes `replay-buffet` and `golden-buffet` after `source-buffet` and `hand-buffet` as the one script block, refusing either of the pair without the other; `check_web_offline.ps1` owes the three both ways, holds the hand's source to the code tokens, no `eval` or `Function` and no store, and the replay's header to the SHA-256 of the cartridge's LF text, its control twenty-nine mutants.
- *The loop check of Buffet's own* (decision 6), `tools/check_host_loop.ps1`: Alonzo's of `v0.4.0` adapted, its browser, its headless runs and their pool and its judgment carried whole, its floors Buffet's, the replay case's read from the session's files so that a restamp moves no number. Runs: the replay case, the save case, the rest with Alonzo's fifteen said absent, all at a ratio of 1, and the rest again at 1.5. `-Control`: five mutant pages, each one block of the built page changed. `run_checks.ps1`'s floor 5, its extras five controls.
- *CI* (decision 7): the crate job checks the command and runs the loop check and its control under `pwsh` over the image's Chrome; the ratchets job runs the check over its page without the module, the double's cases alone, one browser at a time (`BUFFET_LOOP_JOBS`), as Alonzo's Windows steps run theirs.

**Amendments at the build.**
1. *Decision 2's table is read from the native engine,* at the frames where the native test holds the same cells to `buffet.rs`, and never computed by the hand: the test fails before a table that differs from the hand's could be written, so the table is the hand's to the bit wherever the suite is green, and no holder of `cartridge.rs` moved into the library to compute it a second way.
2. *Decision 3's calendar at each frame* is the day and the time in seconds, `Day!B3:B4`, the one cell that moves every frame and of which every other cell of the calendar is a function, the whole Day sheet held at the whole frames: all thirty cells at every frame, built unoptimized as the oracle is, cost 13 s of the test's first 67.
3. *Decision 6's second mutant* is the hand's first fill a cent off, the firm's cash at the frame the first order filled (4,514): the same check, the money a fill moves at its own frame, without a second rule to find the next day's quote. Every mutant changes a block of the built page, so the control builds nothing.
4. *The keep case is withheld* from the check's runs, said on every run (the Found below), until the pin that brings Alonzo's fix.
5. *The test's helpers*: the host's reads after a step shared by `step` and `play`, the commands read by `step` alone; `plate_frame_held` split into `plate_frame`, which advances the hand, and `plate_held`, which holds, the Overview's style a parameter; `hustle_held` split into `hustle_books_held` and the waived exam's cash; and the library's `values` one pass over a record where it made three, every test's answer the same.
6. *The raw recording*, as the clipboard gave it, kept beside the build at `oracle/testdata/session/`, untracked, the committed replay its frames to the byte; removed at the commit.

**The predictions and the numbers.**

| | What | Predicted | The number |
|---|---|---|---|
| P1 | the session | 133,000 to 137,000 frames; 40 to 150 KB | 132,945 frames, the card's turn at 132,346 after the exam's 3,736; 32,243 bytes, 635 frame forms: both just under, the exam quicker and the pointer's moves fewer |
| P2 | the golden | the Report's window, under 10 KB | 7,727 bytes as the working tree holds it, 172 lines |
| P3 | the native test | every frame held, under 30 s | every frame held, 30.9 s alone and the suite's wall time 34.5 s where it was 9.9; 67 s as first written, before the commands left the replay's host, a record was read in one pass, the clock was held light between the whole frames and what the last frame showed was made only where held |
| P4 | the hand | about 100 lines and 10,000 to 20,000 cells, under 200 KB | 30 lines and 43,155 cells at 685 frames, 54,899 bytes: the cells twice the prediction, the bytes a quarter, the table keeping only what moved |
| P5 | the page | 4,670,699 characters grown by 100,000 to 350,000 | 4,692,449 to 4,788,334, by 95,885: just under |
| P6 | the loop check | 18 cases OK at 1, 15 at 1.5; `replay` one hand, none alone, its golden and its round; `save` one save; `keep` about 2,100 restores; under five minutes, `keep` the longest | 17 of 18 OK at 1 and the 15 at 1.5; `replay` one hand, none alone, its golden and its round, the session's 132,945 frames in 8.7 s and 43,155 cells held at 685 frames; `save` one save, 1,040 comparisons, 17.9 s; `keep` 2,079 restores, two not the run, 66.6 s, the longest; the check 34 s one browser at a time with `keep` withheld, 101 s with it |
| P7 | the controls | the loop check's 5 mutants and the offline check's 29, each failing by name | the 5 in 30.6 s and the 29 in 3.7 s, each by its words; the layout's 15 |
| P8 | CI | the crate job longer by 2 to 5 minutes | at the push |

**Found, none minted.**
- *The keep case fails on Buffet's page:* two of its 2,079 restores, at frames 4,544 and 36,032, are not the run there. Each follows an order filled at a frame whose end the page's host kept as a keyframe (4,514's buy, restored from at 4,544; 36,031's, at 36,032): the host keeps a keyframe at a frame's end at rest while the order's seven derived rows wait for the next frame, and a restore opens the keyframe as a load and loses them, the ledger's row of the fill never written. So a player whose tab closed then would get back the order's cash and shares and lose its ledger row. Alonzo's, in the page vendored unchanged: `ENGINE.30`'s Found (its decision 13, a save's load loses a waiting derived write), met here on a game's page for the first time. The cure is that decision's, the Write sheet read at a save's load as after a step, so that the next frame makes the writes the run made, built in `ENGINE.30`'s first slice, which waits on Frazaro's `KERNEL.46`; asked of Alonzo ahead of it at the owner's word, "Hand test passes; commit it and ask Alonzo for the fix" (`OMAKASE.md`, `MISE.6`'s table).
- *The suite's wall time is the session's now,* 34.5 s where it was 9.9, the other tests running beside it.
- *The language's gap did not bite on this session:* every held cell, the four prices among them, was the wasm module's and the native engine's to the bit at all 685 frames, where `SIN` and `^` differ by platform (decision 8).
- *The replay names the order desk's commands by their names* (`buy-10`, `rung-2`), so `PLATE.8`'s keys leave its trades naming nothing: that dish translates the replay, as its line says.

**The hand test, the owner's** (decision 10), on the page built from the build: Buffet picked, the drawer's **The replay** names `src/buffet.replay.vla` and holds it, and **Play the replay** plays the session from its first frame, the exam answered as it was. *Passed 2026-11-21, the owner's word: "Hand test passes; commit it and ask Alonzo for the fix".*

## `FRONT.8` — the night desk: every view a card

*Minted, scoped and built 2026-11-19 in one sitting at the owner's word, before `HOUSE.4`'s second
slice records the owner's session, since a replay pins its cartridge's bytes: "See, the problem
is... Buffet is boring. It looks boring, for starters, which should be an easy fix. Kairos,
Seiyaku, and Brickwork all got aesthetic passes which made them much more playable. Can we do that
to Buffet first so my eyes aren't bleeding during my playtest?" The looks are one section of the
source and every color a name there, so the pass is a door that swings both ways: built at its
recommendation, the white card rendered beside it, and the owner's look the decision.*

**What it stands on.** The three passes share a recipe: a board bound to its extent on the page's
gray (`(bound ...)`, the page's version 6, Seiyaku's Board and Brickwork's Wall), its gridlines off,
edges and fills drawn on purpose, larger type, and the family's colors carrying state (Seiyaku's
rules, Brickwork's shadow). Buffet's views were the opposite: an endless white grid with its
gridlines, every text at 11 points in column A, color on the buttons and bands alone. Four facts of
the engine at the pin, read in `vla-lang` 0.14.0 and the page at `v0.4.0` while building, shaped
what follows: a cell's format is said once, a second `style` row over it refused (`machine/load.rs`'s
`style`), so a view's background cannot be laid under its cells' own; a column's format paints only
the cells that hold nothing (`viewport.js`'s `formatOf`), and a cell holding anything with no format
draws format 0, the sheet's white; a bound sheet's extent is the rectangle of the cells that hold
anything, styles not counted; and a text spills to the bound card's edge and no further.

**The decisions, as built.**

1. *Every view a card.* The manifest's `(bound ...)` names the seven views; each `(gridlines off)`;
   each column to the card's last paints its empty cells the card, format 16, midnight plum; every
   cell that holds anything says a format of its own, the kitchen's frame verbs saying theirs; each
   card 126 characters wide, Trading's table and a margin of 5, every other view given a last column
   that makes up the width (882 pixels at 96 DPI on every tab), so that a tab turned moves no edge;
   and a row of margin under the fine print, its far corner the empty text drawn as the card, the
   kitchen's `card-corner`, since the extent is made by the cells that hold anything.
2. *The palette, named once.* Nine colors by `define` before the formats: Kairos's night `#1C1828`,
   a chip's ink; midnight plum `#2E2645`, the card; the card a shade lighter `#3B3159`, a band; the
   button's lavender `#8063D2`; lilac `#C4B0F0`, 7.31:1 on the card; the whisper `#F7F4FC`, 13.05:1;
   white, a button's text; and the owner's mint and burgundy. An author recolors every view there.
3. *The type.* A view's headline, row 3, bold lilac at 13 points (but Trading's and Gigs', sentences
   of 113 characters, at the text's size); the exam's question bold at 14; the Overview's net worth
   bold at 14 in a column of 16 characters; the certificate's title bold at 16 in Georgia; the
   proctor's signature and remark italic; the fine print lilac italic at 10; the date and a key's
   hint lilac; and the rows of buttons taller, the options and a lone button 22 points, Trading's
   grid 18.
4. *A button is lavender under white, and the lavender is what a click does.* The owner's words at
   `GUARD.1`'s hand test, "lavender background and white text", kept: the deep shade lightened toward
   Lovelace Lavender to the lightest where white still reads at 4.5:1, `#8063D2` at 4.53:1, so that a
   button stands on the card at 3.13:1, WCAG's bar for a control, where the deep shade stands at
   2.42:1. Bold but Trading's grid, as before; a button among others edged by the card on its right
   and below, two pixels, an option by the card below it, three; and no cell off a button on it, the
   bands now a shade of the card.
5. *The glyphs are chips.* A gain's ▲ in the night on the mint, 12.65:1; a loss's ▼ in the whisper
   on the burgundy, 9.96:1; each edged by the card below, so that two stay two; a flat day's = on
   the card. The stock picked on Trading, ▸, a chip of the whisper, by a look of its mark.
6. *The bell lights while the market is open.* The kitchen's `bell` writes, with its formula, a rule
   over its cell, `=Day!$B$19=1`, laying the mint's ink alone (format 24, a format of groups alone),
   so that a glance at any view says whether an order fills, 10.35:1. Seven rules, one a view; the
   page's frame budget, 876 formula cells, stands, each rule computed where the open bell moves.
7. *An error's joke ends inside its card.* The longest, #NUM!'s, 112 characters, spilled past three
   cards at the text's size ("…to contin"); at the fine print's size and slant it ends inside each,
   about 26 pixels to spare on the Overview, the tightest, once every card was widened from the 123
   characters first built to 126, where the first width left 5.
8. *What stands.* No formula, value, address or button's place moved; the model's sheets keep no
   format; the hand's rules are untouched, only its constants of the buttons' formats and the card's.

**The comparison.** The same source with the palette's names set to a white card (the card white,
the band `#EFEAFA`, the labels `#5B45A8`, the text `#1D1D1B`, the buttons the deep shade) and three
formats whose roles differ on white, built into a page in scratch and captured beside the night
desk. It reads cleanly, but its bell can only light by a fill, and a fill covers its own cell alone:
on Trading, three characters of the line. The night desk was built; the white card stays a palette's
names away.

**What it cost, measured.** The cartridge 86,501 bytes of LF text to 90,367, 1,494 rows to 1,614.
Natively, four rounds alternated between `HEAD`'s cartridge and this one in one process each: the
load 7.2 to 8.3 ms against 7.5 to 8.1, a frame 3.23 to 3.49 µs against 3.35 to 3.56; no difference
the rounds can tell. The page 4,670,699 characters to 4,682,129. The describe golden moved by what
the pass adds and nothing else: the manifest's `(bound ...)`; the plan's shapes 266 to 273, texts
261 to 262, formulas 876 to 883 and alone 616 to 623, the seven rules' answers, one formula's text
among them; the Day sheet's readers 74 to 81; and the seven shapes, `Exam[1]!A2` to `Report[1]!A2`,
`=IFERROR(IF(Day!R19C2=1,1,0),0)`, the six after the first `(same 266)`. Alonzo's loop check from
`v0.4.0` over the built page: every case a game's page runs gives the verdict it gave at `HOUSE.2`,
at both ratios; only the `grid` case's counts moved, its first sheet now bound (`gridBound` 1,
`gridCanvas` 5, `gridFonts` 190 where 64, `gridEdges` 76 where 128).

**The tests.** 108, two of them new or rewritten: `the_views_draw_the_houses_looks` holds the bands,
the button's lavender on a button's brackets and nowhere else, the chips, the buttons' edges, the
picked mark, the emptied option the card's, the codes and the headings in lilac;
`every_view_is_a_card` holds the manifest's bound, each view's gridlines off, every column to the
extent's last the card's, the width, the corner a row under the fine print, and every cell holding
anything a fill of its own at the exam and in session. `every_format_a_view_draws_reads` also holds
each rule's format laid over its cell at 4.5:1, its format reader taught italic, a size, a family
and a format of groups alone. Six tests of a view's columns read `shown 16` where `shown none`; the
Overview's net worth column is `WORTH_COLUMN`, 16.

**Found, none minted.**
- *The selection's wash on a dark card:* every view opens with `A1` selected, and on the card the
  wash blends the date's lilac to about `#D2C2F3` on about `#7F7695`, 2.6:1 (*derived*), a bright bar
  across the Exam's 100 characters until a click moves it. Alonzo's, beside `FRONT.2`'s finding of
  the wash on a button (`OMAKASE.md`'s asks table); the owner's to pass on.
- *A text stops at a bound card's edge:* a later string longer than its card, a joke or a headline,
  is cut there, and no test can measure a text's width. Its author widens the card or shortens the
  string, at its dish.

**The owner's look, 2026-11-19, and what it amended.** The owner's words, with a screenshot of
Trading at Chrome's 200%: "Wow, everything looks much better. Good job. Two notes: I zoomed the
page in to 200% and it looks more like a game this way, so is it possible to set the standard zoom
for Buffet to this size? (screenshot attached) The uncolored sheets are all back-office data tabs,
so shouldn't they be hidden?" Both built the same evening as two more decisions:

9. *The zoom is the game's own.* The page draws a grid at its cells' own sizes and never zooms one
   (Alonzo's `ENGINE.14`, decision 9), and Brickwork's answer to the owner's "a 50-100% zoom" was its
   own cells (`CART.3`'s `cell-size`); so Buffet's zoom is one number of its source, 2, which
   `the-zoom` hands to every macro that says a size, Brickwork's `wall-size` idiom, since a `define`
   reaches no expand-time primitive at 0.14.0 and a macro's parameter does: `the-looks`, the
   formats, every text's type said at 100% (11 points, the headline's 13, the question's and the
   worth's 14, the seal's 16, the fine print's and a joke's 10) and drawn at twice it; and each view's
   own sizes, `exam-sizes` to `report-sizes`, its columns (252 characters, 1,764 pixels), its buttons'
   rows (44 points, Trading's 36) and its margins, each row that holds nothing 15 points at the zoom
   by the kitchen's `margins`, since the page fits a row to the type its cells say and to nothing
   where it holds none (`viewport.js`'s `computeFits`, 20 pixels to 11 points, so that a row of
   22-point text fits 40 pixels, twice the default row). The card's own format says 11 points too,
   so its corner's row is a line tall at the zoom. At 1 the cards stand as first built; the back
   office stays at 100%. At the owner's window, 2,508 pixels wide, the card leaves about 700 to
   spare; a screen narrower than about 1,800 scrolls it, and Chrome's zoom shrinks it. The page's own
   bar, headers and tabs keep the page's size, and Chrome's zoom over the game's draws the card at
   four times. *Alternatives:* Chrome's zoom alone, which Chrome keeps for every page opened from a
   file and which no other player inherits; a zoom the record carries a sheet, as Excel's sheet
   view keeps `zoomScale`, which the language and the page do not read, `ENGINE.14`'s decision 9
   standing against it: the owner's to raise with Alonzo, these rows going back to 100% under it.
   (*Amended at the owner's second look, 2026-11-21: the zoom 1.35, so that no card scrolls
   sideways in a window 1,280 pixels wide; below.*)
10. *The back office behind the box.* Every sheet after the seven cards is hidden: the model's
    eleven by the kitchen's `hidden-sheets`, the twin of the kit's `sheets`; the Caveats, `MENU.5`'s
    one content sheet shown, a plain sheet having no place in a strip of cards until it is a card
    itself; the Keys, the Camera, the Commands and the Write sheet; and the engine's Clock and File,
    named hidden by the source, which the engine fills in a sheet the cartridge names as in one it
    makes. The strip reads the seven cards and Input, its box "hidden sheets (58)". It reverses
    `FRONT.1`'s decision 5, the model in plain sight, whose recorded alternative this was, at the
    owner's word; `AD-6` and `BD-3` stand, the box opening every sheet to anyone who ticks it, and
    the notes now say so, "Tick the hidden sheets to open the back office". The engine fills the
    Input sheet only where the cartridge holds none (`make_own_sheets`, at `alonzo` 0.4.0 and on
    Alonzo's main), so a row naming it would leave it without its labels and its zeros; the source
    copying the engine's seventy cells to hide it would be a workaround the engine's fix deletes, so
    the Input tab stays, an ask of Alonzo (`OMAKASE.md`'s asks), Seiyaku's Found of the engine's own
    sheets shown as tabs (`CART.14`) met again. (*Amended at the owner's second look: the engine's
    fix first, at the owner's word; below.*)

*What the look's amendments cost.* The cartridge 1,614 rows to 1,645, the 29 margins' rows and the
Clock's and the File's; the page 4,682,129 characters to 4,690,858; nothing natively to time, the rows
added being sizes and states, which no frame computes. The describe golden moved by one line, the
notes' last sentence, the sizes and the sheets' states being no line of it. Alonzo's loop check from
`v0.4.0` over the built page: all 40 of its case verdicts as at this item's build, at both ratios,
and one count moved, the pass's rows, 1,614 to 1,645. Captured at the owner's window, 2,508 by 1,350
at a ratio of 1, every view at the exam, in session and with a balance past the end of the numbers,
#NUM!'s joke ending about 55 pixels inside the Overview's card.

*The tests.* Still 108. `every_view_is_a_card` also holds every row of a card that holds nothing at
15 points at the zoom and every cell that holds anything a size of its own, at the exam and in
session, each caught by its mutant, Report's row 15 left out of its margins and the text's look
without a size (`Certificate!A14` at the exam); its widths, and five tests' columns, read through
`card_column`, a width said at 100% drawn at the hand's `ZOOM`; the stock names held to a view's
column at the zoom; the hand's buttons' and card's formats functions of the zoom, `button_format` and
its three kin; and `every_sheet_is_written_bare_...` holds the strip, the seven cards and Input,
every other sheet hidden.

*Found at the look, none minted.*
- *The engine's Input sheet stays a tab* (decision 10): an ask of Alonzo.
- *The page's bar and tabs at 100% beside a card at 200%:* the page draws its own controls at its
  own size whatever a game's zoom; a page whose bar and strip follow a game's zoom is Alonzo's to
  weigh, the owner's to pass on.

**The owner's second look, 2026-11-21, and what it amended.** The owner's words: "Hand tests pass.
The input tab cannot be hidden? Also, if the size now threatens scrolling on smaller monitors, maybe
we should scale it down a bit to be safe so side-scrolling doesn't ruin the UX on smaller monitors."

*What was measured before the choice.* The page's own answer, by a headless Chrome at a window's
viewport, each view after the exam: the grid's box is the window's whole width, the row numbers 32
pixels and a scrollbar 15, so at 2 a card is 1,796 pixels and scrolls sideways in any window
narrower, 1,811 where it also scrolls down; at 1,366 by 633, a 1366-by-768 laptop's viewport under
Chrome's own bars, and at 1,280 by 585, a 1080p laptop's at 150%, every view did. The candidates,
each card's width by the viewport's rule (`columnPixels` over the source's widths at the zoom) and
the windows that scroll it sideways:

| Zoom | Card, pixels | Sideways at 1,280 by 585 | at 1,366 by 633 | at 1,440 and wider |
|---|---|---|---|---|
| 200% | 1,796 | 531 | 445 | 371 at 1,440, 275 at 1,536; none at 1,920 |
| 175% | 1,575 | 310 | 224 | 150 at 1,440, 54 at 1,536 |
| 150% | 1,355 | 90 | 4 | none |
| 145% | 1,312 | 47 | none | none |
| 140% | 1,270 | 5 | none | none |
| 135% | 1,223 | none, 42 spare | none | none |
| 125% | 1,135 | none, 130 spare | none | none |

Any multiple of 5% is exact: the language takes a type's size in whole twentieths of a point
(`FontSize::parse` at `vla-lang` 0.14.0), and `*expand` spells its product to fifteen significant
digits (`str_`), so 1.35 times 11 is written 14.85. The owner chose 135% from these, and for the
Input tab the engine's fix first.

9. *(Amended.)* The zoom is 1.35, the one number `the-zoom` hands on: a card 170.1 characters, its
   text 14.85 points (the fine print's and a joke's 13.5, the headline's 17.55, the question's and
   the worth's 18.9, the seal's 21.6), its buttons' rows 29.7 points (Trading's 24.3) and its margins
   20.25. No card scrolls sideways from a window 1,280 pixels wide up, even beside a scrollbar; on
   the owner's window a card is about half its width, and Chrome's own zoom at 150% over it draws
   about the first look's 200%, the bar and the tabs with it. A zoom is a multiple of 5%, the
   source's note says why.
10. *(Amended.)* The Input tab waits for Alonzo, at the owner's word: the engine filling a sheet
    named Input that the cartridge declares and leaves empty, as it already fills a declared Clock
    and File, so that one row, `(sheet "Input" hidden)`, hides it after the pin that brings the
    fix. The owner's rule decided it: a fix that would change the rows the cartridge writes goes
    first, and the copy of the engine's seventy cells, a path the engine supports ("The cartridge's
    own Input sheet is left as it wrote it", its test), would be written twice. The playtest goes
    ahead with the tab.

*What the second look cost.* The cartridge still 1,645 rows, its sizes' values alone moving; the page
4,691,645 characters; the describe golden unmoved; Alonzo's loop check from `v0.4.0` over the built
page, all 40 case verdicts and every count as at the first look. The hand's `ZOOM` is `ZOOM_PERCENT`, 135, and
`zoomed` spells a size as the record does from `zoomed_hundredths`, in whole hundredths so that no
float enters a test; the tests read a width and a type's size in hundredths (`hundredths`). Three
checks that the zoom of 2 had loosened, a length held to twice itself, hold a text at the zoom again:
the hustle's button in its column, a band's header in its column, and the Commands sheet's words read
in hundredths. Still 108 tests, fmt and clippy clean, the checks all green. Two controls, each
restored byte for byte: the source's zoom set to 1.3 with the hand left at 135 fails eight tests;
the Gigs button's column a character narrower at 100%, its margin a character wider so that the card
keeps its width, fails the hustle's, "the button's text, 3510 hundredths of a character at the zoom,
runs past its column of 3375", which the check at 2 passed. On the built page at 1,280 by 585, 1,366
by 633, 1,440 by 790, 1,536 by 729, 1,920 by 945 and the owner's 2,508 by 1,350, no view scrolls
sideways; the Certificate, the Portfolio, Trading and the Report scroll down at 1,280, three of them
at 1,366, and the Certificate at 1,536. Captured at the owner's window and at 1,280 by 585, the
balance past the end of the numbers among them, #NUM!'s joke ending inside the Overview's card.

*Found at the second look, none minted.*
- *A tab turned moves the card's edge by up to 4 pixels at 135%.* The viewport rounds each column
  down to whole pixels by Excel's rule, so the cards are 1,219 pixels (Trading's eleven columns) to
  1,223 (the Exam, the Overview and Gigs), where at 2 every width was whole; 125% and 150% round
  within a pixel. The cure that keeps 135%: each card's margin column absorbing its own card's
  rounding, a verb of the kitchen's and a test of pixels by the viewport's rule; the owner's to weigh.
- *A card's scroll range grows as it is read on a small window.* The viewport knows a row's fitted
  height only inside the region it has read (`ENGINE.43`), so on a window that shows part of a card
  the rows below count 20 pixels until scrolled to: the Certificate measured 570 pixels tall at
  1,280 by 585 and 610 at 1,440 by 790. Alonzo's, the owner's to pass on.

**The owner's third look, 2026-11-21, and what it amended.** The owner's words, with a screenshot
of the Portfolio in a window snapped beside the editor: "Wow, half screen really is right at the width
because there's no scrollbar but it almost looks cut off. I do think Open for Business (or O) should
switch to the Portfolio tab so the player can see that the market is open and things are happening.
The Overview tab is an anticlimactic, nearly-static page that doesn't give the impression that
prices are changing. Hand tests pass."

11. *The certificate leads to the Portfolio.* `Proctor!B6` reads `Portfolio`, one value and no
    formula moved, so that the certificate's button and its key, O, land where the prices move from
    the first bell: the alternative `FRONT.3`'s decision 1 named, "the lead kept on the Portfolio,
    the Overview a tab the player finds". The Overview stays a tab beside it, third in the strip,
    whose order is unchanged. The hand's `LEAD` is the Portfolio, and the notes say the Portfolio
    first, the Overview beside it.

*What the third look cost.* The cartridge's one value; the describe golden by one line, the notes;
the page 4,692,453 characters. Still 108 tests: the Overview's own, which starts on the Overview
with the exam waived, now sees the waiver's frame end on the lead and turns back to the Overview by
a person's tab, so that every frame it holds steps with the Overview shown. A control, the source's
`B6` put back to `Overview` with the hand left at the Portfolio, fails five tests: the waiver's,
the Camera's, the scripted exam's, the Overview's and the Proctor's settings. On the built page, the
exam answered and O pressed, the tab shown is the Portfolio.

*Found at the third look, none minted.*
- *A card as wide as its window looks cut off.* The owner's window, snapped beside the editor, is
  about a card's own width by the screenshot, near 1,220 pixels, so the plum meets the window's edge
  with no gray beside it, and the card's empty margin column reads as a card that runs on. Every
  fixed width meets some window so. The cures: a rim on each card's edge, a format of its margin
  column's and of its corner's; or a zoom a step smaller, which only moves the window it happens at.
  The owner's to weigh.

**The owner's look.** Passed, 2026-11-21: "Hand tests pass". The lead was built after it, its hand
test one step: the certificate's O lands on the Portfolio; it passed too, and the item closed at the
owner's word, "Passes, commit it".

## `HOUSE.2` — the pin to `alonzo` 0.5.0: the engine at 0.5.0, the language at `vla-lang` 0.15.0, the kit and the page copied again at `v0.5.0`, and the keep case back

*Built 2026-11-24 at the owner's word, the day Alonzo released 0.5.0: "Alonzo 0.5.0 is live on
GitHub and crates.io, so pin Buffet to the new version before recommending next steps for Buffet."
No decision was asked and nothing was minted; the manifest stays `(spec 6)`. Read for it: Alonzo's
`RELEASES.md`, `## 0.5.0`, "For a game that carries the page"; Alonzo's `tools/check_web_offline.ps1`
and `tools/check_host_loop.ps1` diffed between `v0.4.0` and `v0.5.0` before anything was copied,
the 0.4.0 pin's lesson; and Alonzo's records of Buffet's `7ef30e6` over the release: the keep case
over the owner's session holding 2,077 of its 2,079 restores without decision 13's line and all
2,079 with it (`REARVIEW.md`, `ENGINE.30`), and the source expanding to its bytes under 0.15.0's
rules (`RELEASES.md`, 0.5.0). Buffet stood at `7ef30e6`, three commits ahead of its origin, none
pushed.*

**The pins.** `alonzo = "=0.5.0"` and `vla-lang = "=0.15.0"`, the version that release pins, so
that the lock holds one copy of the language; `cargo update -p alonzo -p vla-lang` moved the lock's
two packages and nothing else, `alonzo`'s checksum `336006cd…` and `vla-lang`'s `a08a78a5…`, the
one Alonzo's `check_engine_deps.ps1` pins. The oracle compiles with no port. `Cargo.toml`'s comment
names what the two releases bring.

**The copies.** Each read off the tag by `git show v0.5.0:<path>` in Alonzo's clone and written
here byte for byte, the kit checked out CRLF and the page's files LF, as `.gitattributes` gives
them; its SHA-256 and length over its LF text are `check_vendored.ps1`'s baseline. The kit 31,592
bytes where 30,107 at `v0.4.0`, its gain `file-lines`, the Format sheet's verb; the template
1,093,414 where 946,703; the host 237,358 where 209,349; the fake module and the viewport unchanged
since `v0.4.0`, their digests the same.

**No call spelled anew.** The kit spells no call anew, its notes say, and 0.15.0's rules (each
macro defined once, no parameter at a list's head, no bare word in a formula, none of the prelude's
list words defined again) find nothing in Buffet's source: `expand --check` says `same` against the
committed `src/buffet.vla`, so no row of the cartridge moved and no expansion was written anew. The
session's replay keeps its header, and `cargo run --example replay -- --check` says `same` for the
replay, the golden and the hand. The source's banner gains this pin's line, a comment the pass drops.

**What held.** 112 tests, the hand's 47 and the cartridge's 65, every frame of every session held
to the hand, the owner's 132,945 among them; the cartridge's in 45.8 s by cargo's own threads, where
they took 34.5 at `HOUSE.4`'s second slice, one run of each; and the describe golden unmoved. fmt
and `clippy -D warnings` clean. The module, built from the registry crate under the workspace's
release profile, is 2,614,394 bytes, 0.4.0's 2,258,410: 296 bytes over the 2,614,098 Alonzo built
for its own page of the release, the paths the module names, as 0.4.0's stood 304 over. The page
is 5,439,472 characters, the session's 4,788,334: the module's growth in base64, about 474,600, the
template's and the host's 174,720, the kit's 1,485 and the banner's lines the rest.

**The offline check.** Alonzo's rule at the tag names one thing more: since `ENGINE.49` a save of a
file a cartridge reads is written into the file the person opened, through the handle the
browser's open picker gave, so `showOpenFilePicker` is a way to a write and is named once, in
`personFile()`, beside the save picker and the download's two names. Carried here, the header's
stores saying so. Buffet's cartridge reads no file, so its Save writes a cartridge and asks where,
as at 0.4.0. The control's twenty-nine mutants stand and fail by their words.

**The loop check.** `tools/check_host_loop.ps1`'s case list follows Alonzo's at `v0.5.0`, whose
judgment is `v0.4.0`'s unchanged: a game's page runs twenty cases where it ran eighteen,
`clipboard` and `file` over the page's own cards, Brickwork's cut and Afterlife's reel left to
Alonzo's page (`game 0`, `gate 0`), and the fifteen others say they are not on it. The keep case is
back, a run of its own at the first ratio, its withholding taken out, and the save case grows a
second save. Under headless Chrome 158, one browser at a time, none running before, every case OK
at both ratios:
- *the replay case*: the session's 132,945 frames in 9.2 s, 43,155 cells held at 685 frames, its
  golden and its round, as at `HOUSE.4`'s second slice;
- *the save case*: two saves where it made one, the second at the first frame at rest where an
  order's derived writes wait, opened owing them as its run owes them (`owing 1`) and played on
  beside the run, every frame compared the same (`owed 1`), 1,042 comparisons over 66,537 frames,
  22.5 s;
- *the keep case*: 2,081 restores held of 2,081, the two that parted at frames 4,544 and 36,032
  among them, four owing the writes the run owed there (`owing 4`), and a keyframe taken where an
  order's writes wait, restored at the next frame at rest, held (`owed 1`); 3,593 keyframes, 75.1 s.
The four counts of decision 13 are floors, each at its first count. The check takes 113 s where it
took 34, the keep case its longest; its control's five mutant pages fail as before, each by its
words.

**What arrived with the page, unasked.**
- *Copy, cut and paste in the grid* (`ENGINE.39`): Ctrl+C on a view puts its cells on the clipboard
  as Excel's text; Ctrl+V on a view is refused, the views being read-only, in the author's words a
  key there meets (`MISE.6`), and on a model's sheet, shown from the box of hidden sheets, a paste is
  a person's edit as a typed cell is: `PLATE.6`'s hand test's paste into `Orders!B49`, met (`MISE.6`).
- *A held key's repeat* (`ENGINE.39`'s first slice): a game's command repeats at version 8 alone,
  which a `(spec 6)` cartridge does not take; the page's next and previous sheet repeat.
- *Save as* (`ENGINE.49`), Ctrl+Shift+S, beside Save; a cartridge's save asks where either way.
- *A file's lines* (`CART.16`, version 9), a file read into a cartridge's sheets and written back,
  which Buffet does not use.
- *The language at 0.15.0*: a row that empties a cell (`KERNEL.46`), dynamic arrays whole
  (`KERNEL.55`), a cell alone read in place (`KERNEL.74`), functions at expand time (`VLA.8`) and a
  source's parameters (`VLA.11`); every answer of the cartridge the same to the bit, every frame held.

**Found, none minted.**
- *The suite's time*: the cartridge's tests 45.8 s where 34.5, one run of each, not traced; the
  session's test sets it, as at `HOUSE.4`'s second slice.
- *Delete writes the empty text still*: the page's `cellClearRow` at `v0.5.0` writes `""`, which
  arithmetic reads as `#VALUE!`, though the language now empties a cell (`KERNEL.46`). Where Alonzo
  moves Delete to the empty row, which its `ENGINE.39` record says Delete waits on, a Delete over a
  model's number reads as 0, and `MISE.6`'s note of a Delete's `#VALUE!` moves with it.
- *The Input tab*: not in 0.5.0, `make_own_sheets` filling the Input sheet only where the cartridge
  holds none, so the tab stays the strip's eighth (`MISE.6`).
- *A key or a paste on a view answered in an author's words*, at 0.5.0 too (`MISE.6`).
- *`ERROR.TYPE` pinned and not taken*, at 0.15.0 too: the Errors sheet's row shown stays a value.
- *A save opened on the Exam*, the hand test's (below): Alonzo's, the page's opening.
- *The market's time watched after a load kept nowhere*, the hand test's (below): Alonzo's keeper by its design,
  the owner's to weigh.

**The owner's hand test.** 2026-11-24, the page from disk: passed at every step but two, the owner's
words, "Buffet didn't restore on closing and reopening the tab, and opening from a save file opened
on the Exam tab". Both measured after over the DevTools protocol, the same page from disk in fresh
headless profiles of Chrome 158, Autosave turned on, the browser's two pickers stubbed so that Save
and Open ran with no dialog:
- *The save opened on the Exam.* Saved on the Portfolio at frame 748, past the opening bell, and
  opened: the Exam shown. The page opens a version 4 cartridge on the sheet its Camera's third row
  names, else on its first visible sheet (Alonzo's `Host.prototype._open`), and Buffet's Camera names
  none between the cards, so every save past the bell opens on the first view, the Exam, where no
  key is the game's.
- *No restore.* The steps saved, opened the save and closed the tab, and the keeper keeps nothing of
  a run no person has acted in since its load, a record a save leaves nothing unsaved dropped
  (`WorkKeeper.prototype.tick`, Alonzo's `ENGINE.38`): with no act after the Open nothing was
  offered, and a key on the Exam the save opened on is refused and is no act. With an act after the
  Open, a game's key while the exam stood, the work was offered and restored, frame 962 from its
  keyframe at frame 624, 338 frames played again in 85 ms, and offered again after the restore with
  no act since. So the restore holds, and the step was the hand test's own: a restore follows an act
  after the last Save or Open. What stands is that the market's time watched after a load is kept
  nowhere, a game whose clock runs losing it with the tab.

The rest passed: the exam by keys, the certificate's O landing on the Portfolio, a buy on Trading and
its row on the Ledger, the Portfolio's cells copied as Excel's text and a paste on a view refused,
and Save. The pin committed and pushed at the owner's word, "commit and push the pin", the two
Founds minted after it.

