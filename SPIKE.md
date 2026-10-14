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

## 🍽 THE COURSES

## 🍲 THE STOCK POT

- ✅ **`POT.1` — the calendar and the dice.** *Scoped, built and hand-tested 2026-10-14: the scoping, the owner's hand test and what they found are under THE ITEMS, SCOPED.* A Day sheet, the one sheet that reads `Clock!B1`, a label in column A beside each value: the frames a day, 30, a value the manifest's `(rate 30)` and a test hold equal, so that the teacher's dial, the Clock's rate, scales the game without touching a formula; the days a year, 365, every day a market day; the day from 0, `=INT((Clock!B1-1)/B1)`, and the phase, `=MOD(Clock!B1-1,B1)`, the frame less one so that the first frame opens the first day and every day holds its thirty; the boundary, `=IF(B4=0,1,0)`; the year and the day of the year, each from 1. A Dice sheet of two streams, each a die and its draws, each draw's uniform beside it. The market's die is the kit's `hold` over the boundary, and not the kit's `dice`, which reads the frame and rolls every frame (`BD-7`, `BD-3`): the seed on the first day, and else yesterday's die jumped 1,017 draws by one multiplication exact in a double, so that the market is a function of the seed and the day alone, and a day's draws are taken by the dishes one by one, a later draw moving no earlier one. The exam's die is the seed jumped 2^30 + 88 draws, half the period away, so that the market and the exam never share a die. Four market draws and two exam draws so far, `POT.2`'s phases and `GUARD.1`'s start and stride the first to read them, by the library's first verb, `draws`; a near-normal draw later as twelve uniforms less six (`POT.5`), each uniform a draw's cell. *Oracle, met:* the owner's hand test in Alonzo's page; every frame of two years and two days held to the hand (`MISE.2`), the calendar at every frame and the dice moving on a day's first frame alone; the same dice on the same days at one frame a day; no formula outside the Day sheet reading the frame or the phase, and the census's `(every-frame 0)` (`BD-7`, whose oracle the scoping corrected); the library's ten-thousandth draw from 1 equal to Park and Miller's published 1,043,618,065; seven mutants of the cartridge, the library and the source, each caught. *Pays into:* every dish after it reads the day and never the frame, so each costs a boundary and nothing between; the exam's die (`GUARD.1`); the phases of `POT.2`; the timers of `PLATE.3`. `~days`

  *Closed 2026-10-14.* Predicted `~days`; cost one session, which also copied the kit (`HOUSE.3`), laid the layout's first pieces (`MISE.1`) and founded the game by hand (`MISE.2`), each of those its own line. What landed: the source, `src/buffet.source.vla`, and its cartridge, `src/buffet.vla`, 41 rows; the library, `src/kitchen.vla`, and its first verb, `draws`; the hand, `oracle/src/buffet.rs`; eight tests of the cartridge in `oracle/tests/cartridge.rs` and four of the hand; the runner, `oracle/examples/expand.rs`; and the describe golden.

## 🔪 MISE EN PLACE

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
