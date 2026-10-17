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

- ✅ **`POT.10` — the market's hours: an hour a second, the opening and closing bells, and a session's prices each half hour.** *Scoped and approved 2026-10-16, the report cards amended to quarterly and annually; built the same day; amended at the owner's hand test, a price each half hour, a prospectus a week and a day's work, and every dollar a view shows in two decimals, and at the second, 2026-10-17, those dollars flush right; passed at the third: the scoping, the approval, every prediction beside its number, what the build found and the owner's three hand tests are under THE ITEMS, SCOPED.* Minted 2026-10-16 at the owner's word, who had played the game at `BACK.1`'s hand test: "one second per market day is too fast to conceptualize. The game should run at one real-world second per hour of in-game market play. I also think the market should have trading hours and non-trading hours, so the player has time between trading days to focus on other game tasks. ... these are necessary changes before anything else gets built"; so it stands before every other dish of this course. *Scoped the same day, its nineteen decisions the owner's to approve (`SPIKE.md`), as recommended there and amended at the hand test:* two minutes of the market's time a frame, thirty frames an hour at `(rate 30)`, so an hour a second, a day 24 seconds and a year about two hours and a half, every hour alike, the night and the weekend too; the market open from the opening bell, 9:30, to the closing bell, 16:00, Monday to Friday, with no holidays, 261 sessions in the first year, and the game opening on Monday's bell; the Day sheet a calendar of minutes, the day, the time to the half hour, the session and its mark, and still the one sheet that reads the frame (`BD-7`, restated); every day's close tier 1's index of that day to the bit, so that the first year stays `POT.2`'s, and between two closes the session's prices moving each half hour along a path from the last close to the day's close, drawn from the day's next thirteen draws, the exam's stream moved to columns of its own; outside the session the last close and its change, day 0 opening and closing at the starts; the views' date with its weekday and its time, a line under it saying when the market opens or closes, and their dollars in two decimals; the hustle a prospectus a week, each a day's work; the fine print a month and the dividend on its days, as they stood; an order while the market is closed refused, `PLATE.1`'s to build. *Needs:* nothing. *Oracle:* every frame of ten days, every hour of a year and two days at a frame an hour, and eight hundred days and a hundred years at a frame a day, each held to the hand, its calendar counted in minutes; `BD-7`'s scan and the census's `(every-frame 0)`; the describe golden; mutants; a frame's cost natively and in Chrome at the owner's full screen; and the owner's hand test. *Pays into:* every dish after it, each reading the calendar's day, session or mark and never the frame: `PLATE.1`'s orders, filled only while the market is open, `PLATE.3`'s timers, `CRITIC.1`'s report card, `RECIPE.1`'s cadence and `FRONT.3`'s ticker. `~days`

  *Closed 2026-10-17.* Predicted `~days`; cost one session over two days, the scoping, its approval, the build, its measurements and three hand tests, the first two amending it. What landed: the Day sheet a calendar of minutes, the Dice's seventeen market draws and the exam's stream moved to `H1:J3`, the Market's session `B27:B45` and its swing, the Stocks' price now over the index now and the change in cents, the Hustle's week and day's work, and the views' date, bell and dollars in the source and its cartridge, 569 rows and 201 formula cells; `bell` and `dollars` in `src/kitchen.vla`, and `as-of` with the weekday and the time; the caveats' rows 2, 11 and 12 rewritten and 13 and 14 added, drafts for the owner; the hand's calendar counted in minutes, its session's path, its bells, its hustle by the day and the half hour, and its spelling of dollars; 56 tests, the long runs each at the pace it needs; the describe golden blessed again; and, in `OMAKASE.md`, the contract, `BD-7` restated, and the notes on `PLATE.1`, `GUARD.1`, `MISE.4`, `MISE.5`, `PLATE.3`, `RECIPE.1`, `CRITIC.1`, `FRONT.2` and `MISE.6`, whose number format was asked of Frazaro and Alonzo at the hand test. Five of the nine predictions missed in part, each named beside its number: the describe golden's order and the waves' run (P2); a half hour's turn over its range (P4); the longest test, the hundred years' and not the hour's year (P5); the page's blocks (P6); and the dial's 120, half a frame's cost where the same was predicted, with the bell in the owner's Chrome (P8). The build found that a harness attaching after the game is picked misses its first frames; the hand tests asked a price each half hour, a prospectus a week and a day's work, two decimals on every dollar, and those dollars flush right, the record holding no number format; and a column dragged wider or narrower moves its cell's edge and not a padded text.

## 🍳 THE STATIONS

- ✅ **`STATION.1`, first slice — stocks: four rungs.** *Scoped and built 2026-10-14, hand-tested 2026-10-15: the scoping, its fifteen decisions approved whole by the owner, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The Stocks sheet, its name plain as the model's are, a rung a row, cheapest first: four fictional rungs at $1, $5, $25 and $100, labeled `rung 1` to `rung 4`, the working labels (a penny stock, a small cap, a mid cap and a blue chip) kept in the docs and never in a cell until `MENU.3` names them, since real index and fund names are others' marks; each rung's start in whole cents, its beta and its dividend yield as values, and its workings one fill a column. A price is `=INT(<start>*Market!$B$25^<beta>+0.5)`, the index the difficulty picks to the rung's beta, whole cents, one rounding, half up: a power, so that a beta holds at every level of the index and no price reaches zero at any seed, none falling below 11, 96, 833 and 4,386 cents, a bound the hand holds; the betas 2, 1.5, 1 and 0.75, the third the market itself. Yesterday's price is the same formula over yesterday's index, `Market!D25`, the Market sheet's fills reaching a column D, recomputed and never remembered. The fourth rung pays a dividend, 2% a year, 50 cents a share fixed from its start, on the year's 91st, 182nd, 273rd and 364th days, so that income exists from the first quarter, the cell holding it all that day for `PLATE.1` to pay at the day's first frame. Written as rows and not yet a ladder, a rung at a time, in the order a walk over a list of rungs writes, since the ladder is the pattern's second appearance and enters the library with the bonds (`STATION.2`). *Oracle, met:* `buffet.rs`, every price today and yesterday, each dividend and the dividend today held at every frame of two years and two days and on every day of a hundred years at one frame a day; the opening exactly the starts; each rung's least at or above the bound; the dividend on exactly four days a year, the days the hand lists; `BD-7`'s scan and the census's `(every-frame 0)` kept; the owner's hand test in Alonzo's page, every cell it showed the hand's. *Pays into:* `PLATE.1`'s rows, one a rung; the Market panel's prices and their change; the ladder hoist at course 1, whose proof is that this slice expands to its bytes unchanged, which a scratch ladder of 24 lines has already met. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: the Stocks sheet and the Market's column D in the source and its cartridge, now 131 rows and 79 formula cells; the hand's rungs, their prices, the dividend's days and the bound in `oracle/src/buffet.rs`; the Stocks sheet held inside the tests of two years, a year at difficulty 0, eight hundred days and a hundred years at one frame a day, two tests of the cartridge more (the rungs' settings, the opening) and three of the hand (the opening at both tiers, the bound, the dividend's days), 27 in all; the describe golden blessed again, moved as predicted; and, in the same commit at the owner's word, `POT.2`'s P2 cell, CI's Linux holding the market bit for bit at the first push. One of the eight predictions missed, and only at first: the suite took 11.6 s while the Stocks sheet's 56 cells were viewed at every frame and every day, and 7.4 s once the window held the four columns that compute, half again the old suite's time on the same machine under the same load. The build found that a cent is a wide step for a $1 share: the first rung's price moves on 25 of the first year's 365 days.

## 🧽 THE BACK OF HOUSE

- ✅ **`BACK.1` — the dish pit: the side hustle, the first button.** *Scoped 2026-10-16, approved the same day with the view named by the owner, built and hand-tested that day, and amended at the hand test, the key bound on its view alone: the scoping, the approval, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* Gigs, the game's second view, the owner's name for "the sheet where various clicker jobs can produce cash at varying speeds, cooldowns, and pay rates", and its one job at course 0, proofreading a prospectus: a button cell, `[ Proofread a prospectus ]` in a column as wide as its text, pressed by a click on it while Gigs shows, the pointer on the cell and the left button's rising edge, or by P while Gigs shows, the Keys sheet binding P on that view alone by a formula of the sheet shown, so that every view binds keys of its own (the owner's word at the hand test: "keybindings should be scoped to each tab to avoid limiting the entire application to 26 key commands"); each press a cell that holds 0 between presses; $25 a claim, once a market day, at most four claims a run, the owner's cap over a claim a day for good, four claims being $100, about twice a perfect trader's first year on the personal pocket's $100. A claim is judged by what the last frame showed and counts against the day shown, as an order fills at the price shown; its books stand on the Hustle sheet, the fee, the cap and the button's place as values, so that a layout of the view moves no formula; the fees are the claims times the fee, which `PLATE.2`'s personal pocket reads when it comes; the day moving reopens the hustle, and nothing reads the boundary. The views' date and fine print entered `kitchen.vla` at their second writing, `as-of` and `fine-print`, the Portfolio expanding over them to its bytes unchanged. *Oracle, met:* every frame of a scripted session of clicks and keys, written as the host writes them and the Keys sheet read after every step as the host reads it, held to the hand, the cap, the day a press counts against, the reset and the key's binding among them; Gigs at rest on every day of two years and two days and of eight hundred at one frame a day; the settings, the button at their place and the one key bound on its view; nothing reads a view, `BD-7`'s scan and the census's `(every-frame 0)` kept; the describe golden blessed again; nine mutants, each caught by the test written for it; a click's cost natively and in Chrome at full screen, a claim shown 48 to 50 ms after the click at p95, the contract's 100 ms met by half; and the owner's hand test in the page, twice. *Pays into:* `PLATE.1`'s six buttons a rung and `GUARD.1`'s four options a question, each a press's cell, its place values of the model and its keys its view's; the gigs to come, each its own speed, cooldown and pay. `~days`

  *Closed 2026-10-16.* Predicted `~days`; cost one session, the scoping, its approval, the build, its measurements, the owner's hand test and its amendment. What landed: Gigs, the Hustle sheet and the Keys sheet's one key in the source and its cartridge, 474 rows and 125 formula cells; `as-of` and `fine-print` in `src/kitchen.vla`; the hand's hustle and its test; the scripted session and the binding's test, 49 tests in all; the describe golden blessed again; and, in the same commit, the five Linux cells run 38939138517 answered, `MENU.1`'s, `MENU.2`'s, `MENU.5`'s, `FRONT.1`'s and `FRONT.2`'s. Five of the nine predictions missed in part, each named beside its number: the describe golden's shapes, the views' date and fine print being one shape over two sheets; the step after a write of the Input rows; the cartridge's block in the page; P beside a click in Chrome; and the key, bound on every sheet as decision 9 had it until the owner's word at the hand test. The build found that every record lists every sheet, so that each sheet a dish adds makes the oracle's long tests dearer; and the owner, playing the game, asked before any other dish for an hour a second and the market's trading hours, recorded with the hand test for the item that takes it.

## 🛡 THE SNEEZE GUARD

- ✅ **`GUARD.1` — the Series 777: the opening screen and its controls tutorial.** *Scoped 2026-10-17, its twenty decisions approved whole by the owner the same day, built that day, and amended at the owner's hand test, eleven questions, the dollars grouped by thousands, the buttons in lavender, the caveat's heading, the cash on hand and $250 a prospectus; passed at the second the same day: the scoping, the approval, every prediction beside its number, what the build found and the owner's two hand tests are under THE ITEMS, SCOPED.* The game opens on the Exam, a view of its own first among the tabs: ten questions drawn from the bank by the exam's own die (`POT.1`), a seeded start and a stride among the first ten primes above 1,547, coprime to every bank a picker holds, so that none repeats; then an eleventh, ungraded, "Which best describes you?", its options the four styles of the Firms sheet, `src/content/firms.vla`, the answer the firm's style (eleven at the owner's word at the hand test, where the scoping had nine and a tenth). Each question shows with its four options, each a button in Lovelace Lavender, answered by a click, the press the library's `clicked` at its second writing, or by its key, A to D, bound on the Exam alone while a question is open (the Keys sheet's free buttons, until `ENGINE.25`'s commands are released and pinned). An answer is judged by the question the last frame showed, as an order fills at the price shown; the score is a count, the firm's capital $5,000 and $500 a right answer; the exam cannot be failed. The Proctor sheet keeps its books: the settings, the draw, the buttons' table and the workings. The Camera hands the game over: the Exam, the Certificate at the eleventh answer, then where the certificate leads, a value, the Portfolio until the Overview (`FRONT.3`), by the certificate's button or O. The Certificate: the score of ten, the capital in dollars, the style, its button, and the plain statement that the game is not advice under its heading, "Caveat:", the proctor's signature beneath. The market's first bell waits for the certificate's button, the Day sheet counting the frames it waited and taking them out of the time (`BD-7`'s clause), so that no market day passes under an exam. A second run may skip the exam by a waiver, a person's edit of `Proctor!B3`, at five correct answers, $7,500, and the first style. And at the hand test, at the owner's words: every view's dollars grouped by thousands, `dollars` and the new `signed-dollars`; the Cash sheet, the personal cash at the start, $100, the firm's cash and the personal, shown on the Portfolio as its cash on hand; and the prospectus's fee $250 where $25 (`BACK.1`). *Oracle, met:* the draw at every seed of a thousand and the contract's, the hand's and never a question twice; every frame of a scripted exam answered by clicks and by keys, the proctor's books, both views, the keys each view binds, the Camera's hand-over, the bell's wait and the cash held to the hand, then the market's first hour after the bell; the score funding the firm as the contract says at every score of the ten and at a waiver's; a waiver at the first frame and in the middle; the settings, the buttons at their places, every option whole inside its column and the buttons' looks; every amount spelled with its thousands grouped, beside every power of ten to 2^53 cents; `BD-7`'s scan and the census's `(every-frame 0)`; the describe golden blessed again; 28 mutants, each caught by the test written for it; the cost natively and in Chrome at full screen, an answer shown 49 ms after its click at p95, the contract's 100 ms met by half; and the owner's two hand tests. *Pays into:* the firm's capital and name (`BACK.2`), the pockets (`PLATE.2`), and the license ladder (`GUARD.2`), whose every exam is this one over another bank. `~days`

  *Closed 2026-10-17.* Predicted `~days`; cost one session, the scoping, its approval, the build, its measurements, the owner's hand test and its seven changes, and the second hand test. What landed: the Exam, the Certificate, the Proctor, the Cash and the Firms sheets and the Camera in the source and its cartridge, 748 rows and 271 formula cells; `clicked`, `signed-dollars`, the grouped `dollars` and the bell's fifth line in `src/kitchen.vla`; `src/content/firms.vla`, the fourth content sheet; the caveats' rows 15 and 16; the hand's exam, its draw, its keys, the calendar's wait, its grouped dollars and the cash; 67 tests, the crate's 27 and the cartridge's 40; the describe golden blessed again; the layout check's pin at four content sheets; and, in `OMAKASE.md`, the contract, `BD-5`, `BD-7`'s clause and the notes on `PLATE.1`, `PLATE.2`, `FRONT.2`, `FRONT.3`, `BACK.2` and `MISE.6`, whose asks gained the white text and the pointer over a button. Every prediction measured missed in part, each named beside its number. The build found the first frame's press refused, which the hand had taken; the long runs' waiver; a held click that a mutant first passed; and each view's date about 2 µs at every half hour. The hand test asked seven changes, made the same day, the grouping written three ways to cost the load 1.3 ms and nothing at a mark; found and unminted: the white text's deep shade, a dollar's text costing its load until the record holds a number format, and the pointer over a button, Alonzo's.

## 🛎 THE FRONT OF HOUSE

- ✅ **`FRONT.1`, first slice — the workbook: the game's views, each a sheet of its own.** *Scoped 2026-10-15, first as panels on the Screen and then, at the owner's word, as the game's views on sheets of their own, gated on Alonzo; rescoped at the page's version 4 the same day, once `ENGINE.18` was released, its seventeen decisions approved whole by the owner on 2026-10-16, and built and hand-tested that day: both scopings, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* Buffet is played full-screen as a workbook: each of the game's views a sheet of its own, with its own widths, heights, looks and gridlines, reached by the workbook's own tabs and standing first among them, the model's sheets after them, visible (`AD-6`), every name a fund manager's and no tab numbered (`BD-2`'s amendment). The cartridge is written to the page's version 4, `(spec 4)`: the game opens on its first visible sheet with no Camera, which enters with `GUARD.1`; a click on any cell of the sheet shown is the game's, `Input!B19`, `B11` and `B12`, which the first button reads (`BACK.1`); and its Screen, holding nothing, is hidden, the manifest's screen line kept for a Screen a game may show (`FRONT.7`). The first view, the Portfolio: its date, "As of year 1, day 1 (unaudited)"; a row a stock on the Stocks sheet's own rows, each its name read from that sheet, its price today and its change since yesterday in dollars, whole cents over 100 once and the change a difference of cents; and the fine print at its foot, `Disclosures!D3`. Fourteen formula cells, reading the model and the day and never the frame (`BD-7`), and read by nothing, a view being the workbook's output; widths 22, 12 and 12 characters, the default row height, 15 points, kept by the owner's eye, gridlines on, and no background until `FRONT.2`. The caveat on number formats rewritten in the same commit, the first view to show dollars taking it (`BD-5`). Each later view enters with its dish: the Exam with `GUARD.1`, the Overview with `FRONT.3`, Work with `BACK.1`, the market's view with `PLATE.1`, the curve's at course 1. *Oracle, met:* the Portfolio held to the hand at every frame of two years and two days, whole on a day's first frame and byte for byte still through the day, and on every one of eight hundred days at one frame a day, its dollars the Stocks sheet's cents spelled by text, which a test of the hand holds to the record's own spelling from −200,000 cents to 2^52; its headers, and its names the model's; the game opening on it, written to version 4 with no Camera and the Screen hidden; no formula reading a view, each at most 1,000 formula cells; `BD-7`'s scan and the census's `(every-frame 0)`; the describe golden blessed again; eight mutants, each caught by the test written for it; `MISE.3`'s first number re-measured on the real view, in headless Chrome and in the owner's; and the owner's hand test in the page. *Pays into:* every view after it, each a sheet; every button (`BACK.1`, `PLATE.1`, `GUARD.1`), each a click on the sheet shown; the looks (`FRONT.2`), whose glyph column stands beside the change. `~days`

  *Closed 2026-10-16.* Predicted `~days`; cost two sessions, the first scoping and the rescoping, then the build, its measurements and the owner's hand test. What landed: the source's version, notes, hidden Screen, tab order and the Portfolio's section, and its cartridge, 429 rows and 107 formula cells; caveat 12 rewritten; the hand's `dollars` and `as_of` and a test of them; three tests of the cartridge more and three extended, the two years', the eight hundred days' and the tab order's, 45 in all; the describe golden blessed again. Four of the nine predictions missed in part, each named beside its number: an ordinary frame 0.1 µs dearer, five shapes more for the step's own flags; the suite 0.77 s longer at the median where half a second was predicted, a view held at every frame; the source's block 3,243 characters longer where about 2,600; and in the browser the still frame a step of the clock above its prediction and the main thread past it, a frame now about a seventh of `MISE.3`'s.

- ✅ **`FRONT.2`, first slice — the glyph: which way each change went, beside it.** *Scoped 2026-10-16 as the looks, Lovelace Lavender on the views and the glyphs in the owner's colors, a text color to be asked of Frazaro and Alonzo; approved the same day with every color deferred to an aesthetic pass at the owner's word, and so built as the glyph alone, and hand-tested that day: the scoping, the approval, every prediction beside its number and the owner's hand test are under THE ITEMS, SCOPED.* On the Portfolio, a 3-character column `D` beside each stock's change since yesterday, `▲`, `▼` or `=` by `=IF(C5>0,"▲",IF(C5<0,"▼","="))`: the direction of the number beside it, read off it, so that the two never disagree, a whole number of cents over 100 keeping its sign; a view reading its own cells and read by nothing; no header, no look and no color. *Oracle, met:* the hand's glyph, comparing whole cents and sharing no comparison with the cell, held at every frame of two years and two days and on every one of eight hundred days at one frame a day, each of the three shown on some day; `BD-7`'s scan and nothing reads a view; the describe golden blessed again, a view's first `readers` row among its changes; five mutants, each caught by the test written for it; the cost re-measured natively, in headless Chrome and in the owner's; and the owner's hand test in the page. *Pays into:* every figure a later view shows with its direction, the market's view (`PLATE.1`), the overview (`FRONT.3`), the report card (`CRITIC.1`); and the second slice, the looks, which stays in `OMAKASE.md`. `~hours`

  *Closed 2026-10-16.* Predicted `~days` for the whole dish; the first slice cost one session, the scoping, its approval, the build, its measurements and the owner's hand test, beside the pin's move to `alonzo` 0.1.5, a `HOUSE.2` commit of its own. What landed: the Portfolio's column `D` and the glyph's fill in the source and its cartridge, 431 rows and 111 formula cells; the hand's `glyph` and its test; the Portfolio held over `A1:D10`; 46 tests; the describe golden blessed again. Five of the nine predictions missed in part, each named beside its number: the cartridge's characters by three, each escaped glyph counted short; the load under its range and a day's first frame where the fine print stays a tenth under it, both inside the noise; the source's block under its range, its comments shorter than priced; and the main thread's least a hundredth under. The colors stand as the second slice, an aesthetic pass, the scoping's findings with them.

## 📜 THE MENU CARD

- ✅ **`MENU.1` — the strings sheets and the picker.** *Scoped, built and hand-tested 2026-10-15: the scoping, its fourteen decisions approved whole by the owner and amended at the hand test, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The content sheets under `src/content/`, a file a sheet holding words alone, one macro named after the file whose body is one call of the library's `strings`, each sheet one kind of string, hidden after the model's sheets, its rows of text each with the course that added it, its vintage, and the reviewer who approved it (*the vintage and the reviewer struck 2026-10-15 at the owner's word, a row's review its pull request's and no course a freeze, `BD-1` amended: `MENU.2`'s amendments after its close*); the source's manifest its first form, then every include, then the code, each content sheet's macro called where the sheet belongs (the owner's words at the hand test); the library's `picker`, `CHOOSE(k, ...)` over a sheet's rows, generated from the sheet's extent so that a row added needs no formula edited, one cell to 1,547 rows below the headers, the pages of 500 written by the dish whose sheet first passes 1,000 rows; the first sheet, the fine print as a rotating footer, the Disclosures sheet, eight rows in the owner's words, a disclosure a month from the day; and the one-page *How to add a row* in the folder, for a stranger with five minutes. Every other sheet arrives with the dish that reads it, named for it: `Questions` (`MENU.2`), `Headlines` (`FRONT.3`), `Memos` (the first of `MENU.4` and `GUARD.3`), `Firms` (`BACK.2`), `Awards` (`MENU.7`), `Errors` (`PLATE.6`) and `Exits` (`FRONT.7`). *Oracle, met:* the layout check's content rule and the source's order (`tools/check_layout.ps1`, the content sheets' pin raised from 0 to 1, its control fifteen mutants), their headers held by `check_spdx.ps1`; the page's content blocks (`tools/build_web.ps1`, `tools/check_web_offline.ps1`, its control fifteen mutants); the oracle's tests, the fine print held to the hand at every frame, the rows counted by their extent, every row picked in its turn, a row added expanding with no other edit, the one picker's limit and the tripwire before it, and every sheet written bare with the content hidden; and the owner's hand test in the page. *Pays into:* every string any view shows: the ticker (`FRONT.3`), the exam (`MENU.2`), the names (`MENU.3`), the errors (`PLATE.6`), the endings (`FRONT.7`); and the cheapest pull request there is. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build, the owner's hand test and its amendment. What landed: `src/content/disclosures.vla` and its *How to add a row*, `src/content/README.md`; `strings` and `picker` in `src/kitchen.vla`; the source's include at its top, its call of the sheet and the fine print's index, and the game's notes rewritten for `BD-2`, the back office open and the front office not; the hand's rotation and five tests of the cartridge, 33 in all; the describe golden blessed again; the layout check's content rule; the page's content blocks; `CONTRIBUTING.md`'s fifth rule; and, in the same commit, the Linux cells CI's run 38712895205 answered for `STATION.1` and `HOUSE.4`. One of the nine predictions missed: the page grew 9,587 characters where about 5,000 were predicted, the library's own block forgotten; and P6 held at the median alone, one round of five past it. After the hand test passed, the owner rewrote seven of the eight rows, grouped every include together and set the manifest first, the includes after it and the code after them; the cartridge moved in those seven rows and nowhere else.

- ✅ **`MENU.2`, first slice — the exam bank: twenty questions and their whys.** *Scoped, built and hand-tested 2026-10-15: the scoping, its fifteen decisions approved whole by the owner, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The Questions sheet, `src/content/questions.vla`, hidden after the Disclosures sheet: twenty graded questions, the nineteen drafted in `BUFFET.md` §3, checked against `BD-2` and for accuracy, and a twentieth written, *Shares*; each its concept, the question, four options, the right one's letter, the letters spread five to each over the bank, and a one-line why a concept card can quote alone (`FRONT.5`); a finance educator's read before the release (`BD-5`), the deposit insurance's figure marked to confirm. The draft's twentieth, ungraded, which sets the archetype, is no row of the bank, being asked last and always and never drawn: its options are recommended to the Firms sheet and its prompt to the Exam view (`GUARD.1`, `BACK.2`), and the contract's sentence reads nine scored questions drawn from the bank of twenty, then a tenth, ungraded. The library's `strings` grown to several fields, each a header and a width, the fine print its case of one and expanding to its bytes unchanged, `picker` unchanged, a picker a field; the row shown, `Questions!M2`, a value until `GUARD.1`'s draw drives it, with the finding for that dish that a stride picked among the primes above 1,547 is coprime to every bank a picker holds, the dialect having no `GCD`. Second slices at each license, with `GUARD.2`, toward forty to sixty. *Oracle, met:* each content sheet held to its file, read by the language's own reader; every question whole; every field picked at every row; a question added expanding with no other edit; the first question shown; the tripwire and the tab order over both sheets; the layout check's content pin raised from 1 to 2, its control fifteen mutants over seven files; the describe golden blessed again; five mutants, each caught by the test written for it; and the owner's hand test in the page. *Pays into:* `GUARD.1` and every exam after it; every content sheet of several fields after it. `~days`

  *Closed 2026-10-15.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: `src/content/questions.vla`; `strings` of several fields in `src/kitchen.vla`, the Disclosures file's one line of fields respelled; the source's include, call and row shown, and the game's notes; five tests of the cartridge more and three extended, 38 in all; the describe golden blessed again; the layout check's pin and control; the *How to add a row* for a sheet of several fields; and, in `OMAKASE.md`, the contract's sentence, `GUARD.1`'s and `BACK.2`'s notes and the hoist ledger's row. Two of the nine predictions missed: the pass took 1.7 ms more where under one was predicted, a cost paid at a build and never at a frame, and the first frame some 40 µs more where under 10 was predicted; every frame after it held. After the hand test passed, the owner rewrote eighteen of the twenty questions and title-cased the bank's headers; the cartridge moved in seventy rows of text and nowhere else. After the close, the same day and at the owner's word, the approver and then the vintage were struck from every row of both sheets, a row's review its pull request's and no course a freeze of the rows before it (`BD-1`, amended); the cartridge lost 62 rows, 347 where 409 stood (THE ITEMS, SCOPED).

- ✅ **`MENU.5` — the accuracy sheet: *Things this game gets wrong*, and *not advice*.** *Scoped, built and hand-tested 2026-10-15: the scoping, its fifteen decisions approved whole by the owner with the name the owner preferred, the amendment the approval and the moving base made, every prediction beside its number, what the build found and the owner's hand test are under THE ITEMS, SCOPED.* The Caveats sheet, `src/content/caveats.vla`, the one content sheet shown among the tabs, after the model's sheets, since a player reads it whole and no view can show a sheet's rows whole: a caveat a row, its fields what the game gets wrong, what the real world does, Management's response, the joke in the voice of a model's owner answering a finding, and where, the cells it qualifies as a formula writes them, held to a range that holds a value. Its first row is the plain statement that the game is not advice, beside its joke, *Any resemblance to financial advice, living or dead, is purely coincidental*, which the certificate reads, `Caveats!G4:G5`, at the row shown the source writes, 1. Course 0's twelve rows, three of them correcting lessons tier 1 would teach by accident, that the future can be read, that holding the four stocks spreads risk and that risk always pays; four claims of the world marked for a finance educator's read before the release (`BD-5`). The library's `strings` writes no sheet's state, the engine taking one a sheet: the source writes each content sheet's beside its call, hidden but for this one. The README and `SECURITY.md` say it plainly, in American English, and `BD-5` names the sheet and the nine caveats later dishes owe. *Oracle, met:* each content sheet held to its file and in the state its source writes; every caveat whole, the first the plain statement; every caveat's cells holding a value; the first row shown; every field picked at every row; the layout check's content pin raised from 2 to 3, its control fourteen mutants over eight files; the describe golden blessed again; seven mutants, each caught by the test written for it; and the owner's hand test in the page. *Pays into:* every later simplification has a row to go to; the certificate's plain statement (`GUARD.1`). `~hours`

  *Closed 2026-10-15.* Predicted `~hours`; cost one session, the scoping, the build and the owner's hand test, the content rows' strike committed beneath the build (`adc1bc7`), which was built on a snapshot of it in a scratch worktree and moved onto the commit unchanged. What landed: `src/content/caveats.vla`; `strings` without a sheet's state, in `src/kitchen.vla`; the source's include, the three states, the call, the row shown, the labels' width and the game's notes; three tests of the cartridge more and two extended, 41 in all; the describe golden blessed again; the layout check's pin and control; the *How to add a row* for the third sheet; the README's and `SECURITY.md`'s statements, their British spellings respelled; and, in `OMAKASE.md`, `BD-5`'s line, `GUARD.1`'s note and the hoist ledger's row, and `BUFFET.md` §10's line. Two of the nine predictions missed in part: the describe golden's `most`, 8 where 4 was predicted, the census indexing a sheet's readers by row; and the source's block in the page, 2,062 characters where 1,400. A mutant found the test of the plain statement too weak at its first writing, the word *advice* standing in the caveat's explanation too, and it was strengthened before the hand test.

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
| P5 | the suite | 49 tests green, the crate's 17 and the cartridge's 32, and in CI on Linux; within 0.4 s of `ec069e8`'s at the median | *Measurement*, here: 49 green, the crate's 17 and the cartridge's 32. Each tree's suite run alternately, five rounds, the order turned each round, each run pinned and opted out, on eight threads: 5.07 to 5.46 s where it took 4.82 to 5.11, the medians 5.13 and 4.87, 0.26 apart: inside. Each test timed alone first, two rounds alternated, one thread: the two years' test 5.03 s where 4.78, 0.25 dearer, the hundred years' 2.68 where 2.48, the eight hundred days' 0.27 where 0.23, the year at difficulty 0 1.37 where 1.34; the session 0.02 s and the binding's test 0.01, new; every other within a hundredth. The long tests' cost is the oracle's own: every record lists every sheet, 30 sheet rows where 24 stood, Gigs, Hustle, the Keys and their twins, and the oracle's `values` reads each record whole, so every held frame pays for each sheet a dish adds. Linux at the push |
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
| P5 | the suite | 52 to 56 tests green, and in CI on Linux; the suite 3.0 to 4.5 s where 5.0, its longest test the hour's year at about 3 s; every test timed alone in both trees | *Measurement*, here: 54 green, the crate's 21 and the cartridge's 33; the suite's whole 4.0 to 4.2 s on eight threads in two runs, where 5.0, inside. Each test timed alone first, two rounds alternated, one thread: the hundred years at a frame a day 4.12 s, the hour's year 3.60, the ten days at every frame 2.23, the year at difficulty 0 at a frame an hour 1.48 where a year at thirty frames a day took 1.40, the eight hundred days 0.35, the scripted session 0.30 where 0.02, its six days 4,035 frames where 180; `518d414`'s two years took 5.03 s and its hundred years 2.68 (`BACK.1`'s P5). **Missed** in one: the longest test is the hundred years', its window grown to the index now and the hand's path computed at each of its 36,500 frames, not looked into, and not the hour's year at about 3 s. Linux at the push |
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
| P5 | the suite | 62 to 66 tests green where 56, and in CI on Linux; each long test 3 to 12% dearer alone, its records listing forty sheet rows where thirty and a waiver written before it; the draw at a thousand seeds 0.2 to 0.6 s, the scripted exam 0.05 to 0.25, the ten score sessions 0.05 to 0.3; the suite's whole 4.2 to 5.2 s on eight threads where 4.0 to 4.6 | *Measurement*, here: 66 green, the crate's 27 and the cartridge's 39, inside at its edge; the suite's whole on eight threads 4.45 to 4.73 s, the median 4.55, where `b82793c`'s took 4.12 to 4.67, the median 4.19, four rounds alternated, pinned and opted out, inside. Each test timed alone, two rounds alternated, one thread, the least of each: the ten days at every frame 2.81 s where 2.53, 11% dearer; the hour's year 4.38 where 4.07, 7%; the year at difficulty 0 1.83 where 1.74, 5%; the eight hundred days 0.44 where 0.40, 9%; the hustle's session 0.17 where 0.14, its script rewritten over the new verb and its keys read as a set; the scripted exam 0.053, inside at its edge, and the ten score sessions 0.146, inside; the waiver's two sessions 0.029, the settings 0.012 and the opening 0.012; the whole 14.77 s where 13.11. A record of one cell, counted in each tree, lists 40 sheet rows where 30, 1,196 characters where 921. **Missed** in two: the hundred years at a frame a day 4.38 s where 3.84, 14% dearer, over its 3 to 12%; and the draw at a thousand seeds 0.065 s, under its 0.2 to 0.6, one load and the seed written into the Clock a step. Linux at the push |
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
