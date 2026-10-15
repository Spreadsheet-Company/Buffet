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

## 🍽 THE COURSES

## 🍲 THE STOCK POT

- ✅ **`POT.1` — the calendar and the dice.** *Scoped, built and hand-tested 2026-10-14: the scoping, the owner's hand test and what they found are under THE ITEMS, SCOPED.* A Day sheet, the one sheet that reads `Clock!B1`, a label in column A beside each value: the frames a day, 30, a value the manifest's `(rate 30)` and a test hold equal, so that the teacher's dial, the Clock's rate, scales the game without touching a formula; the days a year, 365, every day a market day; the day from 0, `=INT((Clock!B1-1)/B1)`, and the phase, `=MOD(Clock!B1-1,B1)`, the frame less one so that the first frame opens the first day and every day holds its thirty; the boundary, `=IF(B4=0,1,0)`; the year and the day of the year, each from 1. A Dice sheet of two streams, each a die and its draws, each draw's uniform beside it. The market's die is the kit's `hold` over the boundary, and not the kit's `dice`, which reads the frame and rolls every frame (`BD-7`, `BD-3`): the seed on the first day, and else yesterday's die jumped 1,017 draws by one multiplication exact in a double, so that the market is a function of the seed and the day alone, and a day's draws are taken by the dishes one by one, a later draw moving no earlier one. The exam's die is the seed jumped 2^30 + 88 draws, half the period away, so that the market and the exam never share a die. Four market draws and two exam draws so far, `POT.2`'s phases and `GUARD.1`'s start and stride the first to read them, by the library's first verb, `draws`; a near-normal draw later as twelve uniforms less six (`POT.5`), each uniform a draw's cell. *Oracle, met:* the owner's hand test in Alonzo's page; every frame of two years and two days held to the hand (`MISE.2`), the calendar at every frame and the dice moving on a day's first frame alone; the same dice on the same days at one frame a day; no formula outside the Day sheet reading the frame or the phase, and the census's `(every-frame 0)` (`BD-7`, whose oracle the scoping corrected); the library's ten-thousandth draw from 1 equal to Park and Miller's published 1,043,618,065; seven mutants of the cartridge, the library and the source, each caught. *Pays into:* every dish after it reads the day and never the frame, so each costs a boundary and nothing between; the exam's die (`GUARD.1`); the phases of `POT.2`; the timers of `PLATE.3`. `~days`

  *Closed 2026-10-14.* Predicted `~days`; cost one session, which also copied the kit (`HOUSE.3`), laid the layout's first pieces (`MISE.1`) and founded the game by hand (`MISE.2`), each of those its own line. What landed: the source, `src/buffet.source.vla`, and its cartridge, `src/buffet.vla`, 41 rows; the library, `src/kitchen.vla`, and its first verb, `draws`; the hand, `oracle/src/buffet.rs`; eight tests of the cartridge in `oracle/tests/cartridge.rs` and four of the hand; the runner, `oracle/examples/expand.rs`; and the describe golden.

- ✅ **`POT.2` — tier 1: four waves and a drift, prices a function of the day.** *Scoped, built and hand-tested 2026-10-14: the scoping, its sixteen decisions approved whole by the owner, every prediction beside its number, the owner's hand test and what it raised are under THE ITEMS, SCOPED.* The Market sheet, its name plain as the model's are: the index `=g^day * (1 + A1*SIN(w1*day+p1) + ... + A4*SIN(w4*day+p4))` over its own value on the first day, so that it opens at exactly 1 and a rung at its price, with Kitchin, Juglar, Kuznets and Kondratiev periods each a prime number of days, 1,459, 3,271, 6,947 and 18,251 (about 4, 9, 19 and 50 years, the Kuznets moved off 18, where it would be two Juglars), so that no ratio is a whole number; amplitudes 0.1, 0.15, 0.1 and 0.15; the drift 1.0002 a day; the phases the market's draws 1 to 4 of the first day, held. Tier 0, one wave of a year and the drift, the same at every seed, is written beside it as the picker's first rung, and the index is `CHOOSE` on the Difficulty cell from the first day, the cell a value of 1 until `POT.6` makes it a picker. No class row at course 0, stocks being the market itself; no price, the four rungs being `STATION.1`'s, each `=INT(<its start in cents>*<its multiplier>+0.5)`, the idiosyncratic factor tier 2's. A price is a function of the day alone, so a boundary recomputes the market once and the days between recompute nothing (`BD-7`), and the index twenty days back (`Market!C25` beside today's `B25`, the same fill, recomputed and not remembered) gives `FRONT.3` its weather word's slope until `POT.4`. *Oracle* (rewritten 2026-10-14 at the owner's word, as that day's measurement leaves it: `vla-lang` 0.9.3 computes `SIN` and `^` with Rust's std, whose results differ between platforms and from the module's in the last bit): the hand, `oracle/src/buffet.rs`, calls the same std functions as the cells, in each formula's order, so it holds the index and every price exactly natively, here on Windows and in CI on Linux; the tests compare the hand with the cartridge live, and no committed golden holds an unrounded value derived from `SIN` or `^`, so whatever a replay golden holds later is rounded once to what it shows, as money is; the two-peak test, the path not repeating within a hundred years; and `BD-7`, no formula outside the Day sheet reading the frame or the phase and the census keeping `(every-frame 0)`, so the market recomputes on a boundary and nowhere else. *Pays into:* the rungs' prices (`STATION.1`), the weather word (`FRONT.3`), the benchmark (`CRITIC.1`), the picker's first rung (`POT.6`). `~days`

  *Closed 2026-10-14.* Predicted `~days`; cost one session, the scoping, the build and the owner's hand test. What landed: the Market sheet in the source and its cartridge, now 96 rows and 51 formula cells; the hand's market in `oracle/src/buffet.rs`, the same functions as the cells in each formula's order; the Market held inside the test of every frame of two years, three tests of the cartridge more (its settings, a year at difficulty 0, a hundred years at one frame a day) and three of the hand (both tiers opening at 1, the amplitudes, the two-peak test), twenty in all; the describe golden blessed again, moved by the Market alone; and, in the same commit at the owner's word, `Cargo.toml`'s comment naming the pin it holds, `alonzo` 0.1.1. Three of the eight predictions missed in part, each named beside its number: the nearest repeat's lag, seed 777's first year falling on every one of its days, and an ordinary frame 0.1 µs slower. The page held to the hand to the last bit in every one of the 70 cells the hand test showed.

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
| P2 | the hand at every frame of two years and two days | every formula cell of the Market sheet equal bit for bit at all 21,960 frames, natively here and in CI on Linux | *Measurement*: equal at all 21,960 frames here, on Windows, at the first run. CI's Linux at the first push |
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
