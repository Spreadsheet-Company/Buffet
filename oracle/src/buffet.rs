// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Spreadsheet Company

//! Buffet by hand (`docs/OMAKASE.md`, `MISE.2`): the game written a second time,
//! in Rust, the way a person would write it, so that every frame of the
//! cartridge is held to a rule written twice. Founded with `POT.1`, the
//! calendar and the dice, and grown a dish at a time to the last course.
//!
//! The two writings share no predicate. The cartridge's calendar divides
//! the frame (`INT` and `MOD` of the frame less one); the hand counts it, a
//! phase that rolls over into the next day and a day that rolls over into
//! the next year. The cartridge's dice step: a die that jumps a day's draws
//! at each day's first frame, each jump one multiplication, and a chain of
//! single steps beneath it; the hand finds a draw by its place in the stream
//! from the seed, the multiplier raised to that place by squaring.
//!
//! The market (`POT.2`) cannot share nothing: a double's last bit is the
//! order of the operations that made it, and the dialect's `SIN` and `^` are
//! Rust's own `sin` and `powf`, whose last bit differs between platforms. So
//! the market by hand is the Market sheet's formulas written a second time,
//! each with the same functions in the same order, which holds every cell
//! exactly on the platform both run on, Windows here and Linux in CI; its
//! independence is the dice beneath it, found by place, and the day it is
//! asked about, counted. The module in the browser computes its own `sin`
//! and `powf`, so nothing unrounded is ever held against it, and money is
//! rounded once to whole cents before anything holds it.
//!
//! The stocks (`STATION.1`) share the price's formula with the cell, as the
//! waves do: the index to the rung's beta, times its start, plus a half,
//! rounded down, in the cell's order. They share nothing of the dividend:
//! the cell asks whether the day of the year is a multiple of the days
//! between two and computes a dividend from the yield; the hand lists the
//! four days and states the contract's whole cents.
//!
//! The views (`FRONT.1`) share no formatter with the record. A view draws
//! money as whole cents divided by 100 once, and the record spells the
//! quotient as its shortest round trip; the hand spells the same cents by
//! text, the dollars, a point and the cents less their trailing zeros, so
//! that the tests hold what `MISE.3` derived, that whole cents over 100 draw
//! their own two decimals at their shortest. Since `POT.10`'s hand test a
//! view draws two decimals and since `GUARD.1`'s its thousands grouped, texts
//! the cells spelled until `FRONT.2`'s second slice and a format's code draws
//! since: the code is the language's, and the hand spells the same cents by
//! counting digits back from the last, sharing no rule with it. The glyph
//! beside each change (`FRONT.2`) shares no
//! comparison with its cell: the cell reads the sign of the change in cents,
//! the hand compares the two prices.
//!
//! The side hustle (`BACK.1`) shares none with its cells either. The cells
//! judge a press by the twin of the cell that says the hustle is shut, and
//! keep the day after the last claim; the hand keeps the days its claims
//! counted against, in order, and reads the list's length and its last day.
//!
//! The market's hours (`POT.10`): the cartridge's calendar divides the time,
//! in minutes from day 0's midnight, and chooses the session's day by the
//! weekday; the hand counts minutes into hours and hours into days, keeps
//! the session whose bell it last heard and the one before, and counts the
//! session's marks as its hours turn. The session's path cannot share
//! nothing, any more than the waves can: the hand computes the bridge
//! between two closes with the cells' operations in their order, so that it
//! holds every hour of it exactly, and its independence is the calendar
//! beneath it, counted, and the draws, found by their place.
//!
//! The Series 777 (`GUARD.1`) shares none with its cells. The cells pick the
//! stride by a `CHOOSE` over ten primes written out, and compute each
//! question's row from the start, a product taken round the bank; the hand
//! finds the primes above 1,547 by trial division, and walks the draw a row
//! at a time, each the last plus the stride's remainder, wrapping. The cells
//! count the answers and the right ones over their twins, judging a press
//! by the right option the last frame showed as a number; the hand keeps
//! the options given in a list and compares each one's letter with the
//! bank's letter, as text. And the calendar's wait: the cells take the frames
//! the bell waited out of the time; the hand's calendar stands still while
//! the bell waits, and moves on from the frame after it rings. The views'
//! dollars since `GUARD.1`'s hand test share none either: the cells join the
//! leading group of the dollars and each later digit by its place, choosing
//! the padding by powers of ten; the hand counts the digits back from the
//! last, a comma before every third, and the padding from the count.
//!
//! The order desk (`PLATE.1`) shares none with its cells. The cells find a
//! click by a grid of presses summed by row and by column, and a command by
//! comparing the command run with the Commands sheet's names; the hand matches
//! the pointer to each button's place and the name to its own list. The cells
//! keep each pocket's book of each stock as state over its twin, the cost basis
//! taken down sale by sale in doubles, and derive the gain taken from three
//! sums, and the cash from the books' flows; the hand keeps every fill in a
//! list and folds it, the cost of the shares sold found in whole integers
//! rounded half up, and the cash from its start and the fills. The ledger's
//! ring is the cells' count taken round a hundred; the hand finds a row's fill
//! as the last whose number falls there.
//!
//! The pockets' switch (`PLATE.2`) shares none with its cells. The cells hold
//! the pocket over its twin, 2 your own and any other value the firm's, the
//! other at a switch, and keep one act a frame by gating each command on the
//! clicks; the hand flips the pocket it holds, and takes a frame's acts in
//! turn, a click's before a command's, stopping at the first it takes.
//!
//! The pace (`POT.11`) shares none with its cells either. The cells sum the
//! time over its twin, each frame the last frame's time and its pace, the
//! pace chosen by a cell that compares the time to the five minutes with the
//! bells; the hand moves its clock five minutes at a time by the pace where
//! it last stood, by its own reading of the hour and the weekday, hears the
//! bell at the five minutes it rings and turns the tick at every other while
//! the session is today's. The path's 78 steps share the bridge with the
//! cells as the half hours did; the hand finds each draw by its place beneath
//! the day's die, where the cells step a column down from it.
//!
//! The overview (`FRONT.3`) shares none of its rules with its cells. The cells
//! choose the last close made by whether the clock stands between the bells,
//! and the weather's word by comparing the change with each word's tenths from
//! the top; the hand asks how far its session has ticked, and counts the words
//! whose tenths the change reaches. The change itself is the market's, taken
//! in the cells' order as the waves are. The cells pick the day's headline by
//! the draw's uniform times the rows, rounded down; the hand divides the draw
//! times the rows by the modulus in whole numbers.
//!
//! The firm's name (`BACK.2`) shares none of its rule with its cells. The cells
//! name the firm by the Firms sheet's pickers, their row the style the
//! Proctor's books hold or the first before one; the hand takes the content
//! file's row at the place its own exam's last answer gives, and spells the
//! certificate's line and the Overview's itself. The stocks' names (`MENU.3`'s
//! first slice) are the hand's own, each on its rung as the Stocks sheet keeps
//! each on its rung's row, and every view, book, sentence and command that
//! names a stock is held to them, never to the Stocks sheet's.
//!
//! The report card (`CRITIC.1`) shares none of its rules with its cells. The
//! cells divide the day of the year by the year's days over four, count the
//! quarters, and mark a turn by the count against its twin; they hold the
//! worths at the last card and the one before it, at the year's start and the
//! last year's, each over its twin. The hand lists the quarters' last days,
//! counts the quarters as its calendar turns, and keeps every turn's worth in
//! a list, reading a card's starts off it by search. The cells round a return
//! by dividing two doubles and rounding down; the hand rounds the same
//! quotient in whole numbers, the two the same below 2^53. The cells grade by
//! comparing the alpha with each grade's from the top; the hand counts the
//! grades it reaches. And the percent a view draws: a format's code over the
//! hundredths divided by 10,000, where the hand spells the whole number's digits.
//!
//! The limit (`PLATE.6`'s first slice) shares none of its rules with its cells.
//! The machine turns a result no double holds into `#NUM!` and passes an error
//! through every operator; the hand reads the number a person typed as digits and
//! a power of ten, counts the capital's places against the end's and compares
//! seventeen digits, never in a double, and lists by hand the cells of the
//! firm's books the capital reaches. The cells find an error's joke by a
//! criterion spelled as its name; the hand compares the error it says landed
//! with the error the Errors sheet shows. The order desk refuses an order by
//! the error-test of its cash shown; the hand by the error it says the last
//! frame showed.

use std::collections::BTreeSet;

/// Park and Miller's minimal standard (1988): a stream of whole numbers,
/// each the last times the multiplier, modulo the modulus, a prime.
pub const MODULUS: u64 = 2_147_483_647;
pub const MULTIPLIER: u64 = 16_807;

/// The draws the market's die jumps each day, so that a day owns the draws
/// between two dice and a dish takes the next free one: the first stride
/// past a thousand whose multiplier, [`DAY_JUMP`], is below 2^22. The Dice
/// sheet's `F1`, 1,017.
pub const DAY_STRIDE: u64 = first_exact_jump(1_000);
/// The draws the exam's die stands from the seed: the first offset past
/// 2^30, about half the generator's period, whose multiplier,
/// [`EXAM_JUMP`], is below 2^22. The Dice sheet's `F3`, 1,073,741,912.
pub const EXAM_OFFSET: u64 = first_exact_jump(1 << 30);

/// 16807 to the power [`DAY_STRIDE`], modulo [`MODULUS`]: the multiplier the
/// market's die jumps a day by. The Dice sheet's `F2`, 86,383.
pub const DAY_JUMP: u64 = power(MULTIPLIER, DAY_STRIDE);
/// 16807 to the power [`EXAM_OFFSET`], modulo [`MODULUS`]: the multiplier
/// the exam's die stands from the seed by. The Dice sheet's `F4`, 1,164,196.
pub const EXAM_JUMP: u64 = power(MULTIPLIER, EXAM_OFFSET);

// Every product the cartridge's dice compute stays below 2^53, so a double
// holds it exactly: Park and Miller's own step, and each jump, whose
// multiplier is below 2^22.
const _: () = assert!((MODULUS - 1) * MULTIPLIER < 1 << 53);
const _: () = assert!((MODULUS - 1) * DAY_JUMP < 1 << 53);
const _: () = assert!((MODULUS - 1) * EXAM_JUMP < 1 << 53);
// The market and the exam never share a die: the market's die reaches the
// exam's place in the stream only after a million days, more than 2,800
// years of a game, and the exam's draws reach the market's first only after
// the rest of the period.
const _: () = assert!(EXAM_OFFSET / DAY_STRIDE > 1_000_000);
const _: () = assert!(EXAM_OFFSET / DAY_STRIDE / DAYS_A_YEAR > 2_800);
const _: () = assert!(MODULUS - 1 - EXAM_OFFSET > 1_000_000_000);

/// The game's pace (`POT.11`, the owner's words of 2026-10-19): the seconds of
/// the market's time a frame in the regular session, the Day sheet's `B28`,
/// twenty, ten minutes a second at the manifest's rate; and between sessions,
/// `B1`, a hundred and twenty, an hour a second, as every hour ran before; the
/// manifest's rate a test holds beside them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pace {
    pub session: u64,
    pub between: u64,
}

impl Pace {
    /// One pace at every hour, both cells written alike: the oracle's longer
    /// runs, a frame each quarter hour, each hour or each day.
    pub const fn every(seconds: u64) -> Pace {
        Pace {
            session: seconds,
            between: seconds,
        }
    }
}

pub const PACE: Pace = Pace {
    session: 20,
    between: 120,
};
/// The seconds of a quarter hour, an hour and a day, each one pace at every
/// hour for the oracle's longer runs: a frame each quarter hour, each hour,
/// each day.
pub const QUARTER_HOUR: u64 = 900;
pub const AN_HOUR: u64 = 3_600;
pub const A_DAY_LONG: u64 = 86_400;
/// The minutes of a day, the calendar's own unit below its time in seconds.
pub const A_DAY: u64 = 1_440;
/// The days a year, every one a day of the calendar, five in seven a session.
pub const DAYS_A_YEAR: u64 = 365;
/// The days a week, and the days of it the market trades from Monday, the
/// Day sheet's `B12`: Monday to Friday, and no holidays.
pub const DAYS_A_WEEK: u64 = 7;
pub const TRADING_DAYS: u64 = 5;
/// The opening bell and the closing bell, `B10` and `B11`, in minutes after
/// midnight: 9:30 and 16:00, New York's core session.
pub const OPENING_BELL: u64 = 570;
pub const CLOSING_BELL: u64 = 960;
/// The first frame's time, `B9`: the opening bell on day 0, a Monday, so that
/// the game opens on a market that moves.
pub const FIRST_MINUTE: u64 = 570;
/// The minutes of a tick (`POT.11`, the owner's approval of 2026-10-19): a
/// price every five minutes of the market's time, where `POT.10` priced each
/// half hour; and the ticks a half hour, six, by whose root a tick's step is
/// scaled so that the half hour's spread holds.
pub const MINUTES_A_TICK: u64 = 5;
pub const TICKS_A_HALF_HOUR: u64 = 6;
/// The closing tick, `B23`: the session's tick at the closing bell, and from
/// it to the next bell, the ticks being 0 at the opening bell and one more at
/// each five minutes of the session, 9:35 to 16:00.
pub const CLOSING_TICK: u64 = (CLOSING_BELL - OPENING_BELL) / MINUTES_A_TICK;
/// The days' names, Monday's first, as the views show them.
pub const DAY_NAMES: [&str; DAYS_A_WEEK as usize] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

// The game opens on the opening bell, and the hand's calendar counts from it;
// a session is 78 ticks; and every bell is a whole number of the pace's
// steps from midnight, so the steps between sessions land on the opening bell
// and the session's on the closing bell, 1,170 of them (`POT.11`).
const _: () = assert!(FIRST_MINUTE == OPENING_BELL);
const _: () = assert!(CLOSING_TICK == 78);
const _: () =
    assert!(OPENING_BELL * 60 % PACE.between == 0 && CLOSING_BELL * 60 % PACE.between == 0);
const _: () = assert!((CLOSING_BELL - OPENING_BELL) * 60 / PACE.session == 1_170);
const _: () = assert!((CLOSING_BELL - OPENING_BELL) * 60 % PACE.session == 0);
/// The seed of every first game, the manifest's `(seed 791886)`: one market
/// and one exam for every first player, chosen by the owner on 2026-10-14
/// for a kind first year, the index dipping 1.1% to day 91 and closing the
/// year 10.2% up (`COURSE.0`'s contract, where the draft had 777).
pub const SEED: u64 = 791_886;

/// The first jump at or past `from` draws whose multiplier, 16807 to that
/// power modulo [`MODULUS`], is below 2^22: a jump one multiplication makes,
/// exact in a double.
pub const fn first_exact_jump(from: u64) -> u64 {
    let mut n = from;
    while power(MULTIPLIER, n) >= 1 << 22 {
        n += 1;
    }
    n
}

/// `base` to the power `exp`, modulo [`MODULUS`], by repeated squaring. Every
/// factor is below 2^31, so every product fits a `u64`.
pub const fn power(base: u64, mut exp: u64) -> u64 {
    let mut out = 1u64;
    let mut square = base % MODULUS;
    while exp > 0 {
        if exp & 1 == 1 {
            out = out * square % MODULUS;
        }
        square = square * square % MODULUS;
        exp >>= 1;
    }
    out
}

/// The stream from `seed`, `place` draws on: the seed itself at place 0.
pub fn stream(seed: u64, place: u64) -> u64 {
    seed % MODULUS * power(MULTIPLIER, place) % MODULUS
}

/// The market's die on `day`, from 0: the seed jumped a day's draws for each
/// day begun, the first day's too, as if the seed were the die of the day
/// before the first, so that every day's draws are the same rule's.
pub fn market_die(seed: u64, day: u64) -> u64 {
    stream(seed, DAY_STRIDE * (day + 1))
}

/// The market's `draw`th draw on `day`, from 1: the die's next draws.
pub fn market_draw(seed: u64, day: u64, draw: u64) -> u64 {
    stream(seed, DAY_STRIDE * (day + 1) + draw)
}

/// The exam's die, a function of the seed alone.
pub fn exam_die(seed: u64) -> u64 {
    stream(seed, EXAM_OFFSET)
}

/// The exam's `draw`th draw, from 1.
pub fn exam_draw(seed: u64, draw: u64) -> u64 {
    stream(seed, EXAM_OFFSET + draw)
}

/// A draw's uniform: the draw over the modulus, between 0 and 1.
pub fn uniform(draw: u64) -> f64 {
    draw as f64 / MODULUS as f64
}

/// The calendar (`POT.1`, counted in minutes since `POT.10` and in seconds at
/// two paces since `POT.11`): a frame at a time, its seconds rolling into the
/// minute, the minute into the hour, the hour into the day, the day into the
/// week and the year; the session the hand last heard the bell for, and the
/// one before it; and the session's tick, counted as its five minutes turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Calendar {
    pub pace: Pace,
    pub days_a_year: u64,
    /// The frame last counted: 0 before the first.
    pub frame: u64,
    /// The day, from 0.
    pub day: u64,
    /// The hour of the day, from 0, the minute of the hour and the second of
    /// the minute.
    pub hour: u64,
    pub minute: u64,
    pub second: u64,
    /// The day of the week, 0 a Monday.
    pub weekday: u64,
    /// The year, from 1, and the day of the year, from 1.
    pub year: u64,
    pub day_of_year: u64,
    /// Whether this frame turned the day or began the game: the frame the
    /// dice roll in.
    pub turned: bool,
    /// The session's day: the last day whose opening bell the hand heard.
    pub session: u64,
    /// The session before it; the first session's is its own, so that the
    /// game opens at the starts.
    pub before: u64,
    /// The session's tick: 0 at the opening bell, one more as each five
    /// minutes of the session turns, [`CLOSING_TICK`] from the closing bell to
    /// the next.
    pub tick: u64,
    /// Whether the exam holds the bell at this frame (`GUARD.1`), the Day
    /// sheet's `B25`: the game not yet begun, the market closed and its time
    /// standing; and the frames before this one it held it, `B26`.
    pub waits: bool,
    pub waited: u64,
    /// Whether this frame turned the quarter (`CRITIC.1`), the Day sheet's
    /// `B35`: the quarters counted moved since the frame before, never at the
    /// game's first frame.
    pub quarter_turned: bool,
}

impl Calendar {
    /// The calendar before its first frame, at a pace.
    pub fn new(pace: Pace, days_a_year: u64) -> Calendar {
        Calendar {
            pace,
            days_a_year,
            frame: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            weekday: 0,
            year: 1,
            day_of_year: 1,
            turned: false,
            session: 0,
            before: 0,
            tick: CLOSING_TICK,
            waits: false,
            waited: 0,
            quarter_turned: false,
        }
    }

    /// One frame on, the bell never waiting: the game begun before the first
    /// frame, as a waiver written then begins it, and as every run that
    /// holds the market from its first frame plays.
    pub fn tick(&mut self) {
        self.tick_waiting(false);
    }

    /// One frame on, `waits` whether the exam holds the bell at the new
    /// frame. The first frame stands at the first frame's time on day 0, the
    /// opening bell, which rings there. Each other frame is the last frame's
    /// pace on (`POT.11`): none while the bell waited, the frames waited
    /// counted; the session's seconds if the last frame stood in session, and
    /// the seconds between sessions if it did not. The clock moves five
    /// minutes at a time, its seconds rolling into the minute, the hour and
    /// the day; at each five minutes reached, the opening bell of a trading
    /// day rings, the session today's, the one before it the last and the
    /// tick 0, and any other five minutes turns the tick while the session is
    /// today's, to the closing tick at the closing bell.
    pub fn tick_waiting(&mut self, waits: bool) {
        let quarters = self.quarters();
        self.advance(waits);
        self.quarter_turned = self.frame > 1 && self.quarters() != quarters;
    }

    /// The frame's clock moved on, `tick_waiting`'s, the quarter aside.
    fn advance(&mut self, waits: bool) {
        let step = self.pace_now();
        if self.frame >= 1 && self.waits {
            self.waited += 1;
        }
        let first = self.frame == 0;
        self.frame += 1;
        self.waits = waits;
        self.turned = first;
        if first {
            (self.hour, self.minute, self.second) = (FIRST_MINUTE / 60, FIRST_MINUTE % 60, 0);
            self.ring();
            return;
        }
        let mut left = step;
        while left > 0 {
            let into = (self.minute % MINUTES_A_TICK) * 60 + self.second;
            let to_tick = MINUTES_A_TICK * 60 - into;
            if left < to_tick {
                self.second += left;
                self.minute += self.second / 60;
                self.second %= 60;
                break;
            }
            left -= to_tick;
            let at = self.minute * 60 + self.second + to_tick;
            (self.minute, self.second) = (at / 60, 0);
            if self.minute == 60 {
                self.minute = 0;
                self.hour += 1;
                if self.hour == 24 {
                    self.hour = 0;
                    self.next_day();
                }
            }
            if self.weekday < TRADING_DAYS && self.hour * 60 + self.minute == OPENING_BELL {
                self.ring();
            } else if self.session == self.day && self.tick < CLOSING_TICK {
                self.tick += 1;
            }
        }
    }

    /// The opening bell heard: the session today's, the one before it the
    /// last session, or its own at the game's first, and the tick 0.
    fn ring(&mut self) {
        self.before = if self.frame == 1 {
            self.day
        } else {
            self.session
        };
        self.session = self.day;
        self.tick = 0;
    }

    /// The day turned: the next weekday, the next day of the year, and the
    /// next year after its last day.
    fn next_day(&mut self) {
        self.day += 1;
        self.weekday += 1;
        if self.weekday == DAYS_A_WEEK {
            self.weekday = 0;
        }
        self.day_of_year += 1;
        if self.day_of_year > self.days_a_year {
            self.day_of_year = 1;
            self.year += 1;
        }
        self.turned = true;
    }

    /// The time of day to the five minutes, in minutes after midnight, the
    /// Day sheet's `B15`; and to the half hour.
    pub fn five(&self) -> u64 {
        self.hour * 60 + self.minute - self.minute % MINUTES_A_TICK
    }
    pub fn half_hour(&self) -> u64 {
        self.hour * 60 + if self.minute >= 30 { 30 } else { 0 }
    }

    /// The time shown, in minutes from day 0's midnight, `B31`, and of the
    /// day, the clock every view shows (`POT.11`, the owner's word at its hand
    /// test: "only show the half hour while the market is closed"): the five
    /// minutes in session, where the prices turn with it, and the half hour
    /// between sessions, where an hour passes a second. The cells round the
    /// time to the five minutes down to the half hour; the hand reads its hour
    /// and its minute.
    pub fn shown(&self) -> u64 {
        self.day * A_DAY + self.clock()
    }
    pub fn clock(&self) -> u64 {
        if self.in_session() {
            self.five()
        } else {
            self.half_hour()
        }
    }

    /// The time in seconds from day 0's midnight, `B4`, and to the five
    /// minutes in minutes from day 0's midnight, `B14`.
    pub fn seconds(&self) -> u64 {
        ((self.day * A_DAY + self.hour * 60 + self.minute) * 60) + self.second
    }
    pub fn five_minutes(&self) -> u64 {
        self.day * A_DAY + self.five()
    }

    /// Whether the clock stands in the regular session, `B29`: a trading day,
    /// from its opening bell to its closing one, whether or not the bell waits
    /// and whatever a license opens (`POT.11`), which the pace is keyed on.
    pub fn in_session(&self) -> bool {
        self.weekday < TRADING_DAYS
            && (OPENING_BELL..CLOSING_BELL).contains(&(self.hour * 60 + self.minute))
    }

    /// Whether the market is open, `B19`: in session, and the bell not
    /// waiting for the exam (`GUARD.1`).
    pub fn open(&self) -> bool {
        !self.waits && self.in_session()
    }

    /// The pace now, the seconds the next frame moves the clock, `B30`: none
    /// while the bell waits, the session's in session, and the pace between
    /// sessions at every other hour.
    pub fn pace_now(&self) -> u64 {
        if self.waits {
            0
        } else if self.in_session() {
            self.pace.session
        } else {
            self.pace.between
        }
    }

    /// The day of the last close made (`FRONT.3`), which the weather reads:
    /// the session before's until the session's tick reaches its close, and
    /// the session's own from it. The cells ask whether the clock stands
    /// between the bells, `Day!B29`; the hand asks how far its session has
    /// ticked.
    pub fn last_close(&self) -> u64 {
        if self.tick < CLOSING_TICK {
            self.before
        } else {
            self.session
        }
    }
}

/// Pi, the Market sheet's `B1`, written there as `3.141592653589793`, this
/// constant's shortest spelling, since `PI()` parses in the dialect and is
/// never evaluated.
pub const PI: f64 = std::f64::consts::PI;
/// The drift, `B2`: the trend's growth a day, about 7.6% a year.
pub const DRIFT: f64 = 1.0002;
/// The difficulty, `B3`: tier 1 until `POT.6` makes the cell a picker; a
/// person who writes 0 plays tier 0.
pub const DIFFICULTY: u64 = 1;
/// The look back, `B4`: how many days before the last close made the weather
/// reads the index (`FRONT.3`).
pub const LOOK_BACK: u64 = 20;

/// A wave of the market: its period, a whole number of days, and its
/// amplitude, a fraction of the trend.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wave {
    pub days: u64,
    pub amplitude: f64,
}

impl Wave {
    /// Its speed in radians a day, a turn over its period: `=2*$B$1/B7`.
    pub fn speed(self) -> f64 {
        2.0 * PI / self.days as f64
    }

    /// Its term on `day` from `phase`, the amplitude times the sine of the
    /// angle it has turned through: `=$C7*SIN($D7*B$14+$E7)`.
    pub fn term(self, phase: f64, day: f64) -> f64 {
        self.amplitude * (self.speed() * day + phase).sin()
    }
}

/// Tier 0's one wave, a year's, its phase fixed at 0 so that the Kiddie Pool
/// is the same market at every seed: the Market sheet's row 7.
pub const YEAR: Wave = Wave {
    days: DAYS_A_YEAR,
    amplitude: 0.1,
};

/// Tier 1's four waves, rows 8 to 11: Kitchin's, Juglar's, Kuznets's and
/// Kondratiev's, each period the prime number of days nearest 4, 9, 19 and
/// 50 years (the Juglar's the lower of the two fourteen days from 9), the
/// Kuznets moved off 18 years, where it would be two Juglars to within 1%.
pub const WAVES: [Wave; 4] = [
    Wave {
        days: 1_459,
        amplitude: 0.1,
    },
    Wave {
        days: 3_271,
        amplitude: 0.15,
    },
    Wave {
        days: 6_947,
        amplitude: 0.1,
    },
    Wave {
        days: 18_251,
        amplitude: 0.15,
    },
];

/// Whether `n` is prime, by trial division.
pub const fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

// Each of tier 1's periods is a prime number of days, so no ratio between
// two of them is a whole number and none is a whole number of years.
const _: () = {
    let mut k = 0;
    while k < WAVES.len() {
        assert!(is_prime(WAVES[k].days) && WAVES[k].days > DAYS_A_YEAR);
        k += 1;
    }
};

/// The market of a seed (`POT.2`): tier 1's four phases, each a turn times
/// the uniform of the market's draw 1 to 4 on the first day, and tier 1's
/// cycle on that day; and the volatility factor within the day (`POT.11`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Market {
    /// Tier 1's phases in radians, Kitchin's first: the Market sheet's
    /// `E9:E12`, taken on day 0 and kept.
    pub phases: [f64; 4],
    /// Tier 1's cycle on day 0, `B23`, which tier 1 divides by so that it
    /// opens at 1.
    pub first_cycle: f64,
    /// The volatility factor within the day, `B6`: [`FACTOR`], unless a
    /// person's edit wrote another.
    pub factor: f64,
}

impl Market {
    /// The market of `seed`, its phases from the hand's own dice, at the
    /// setting's factor.
    pub fn new(seed: u64) -> Market {
        let phases = std::array::from_fn(|k| {
            let draw = market_draw(seed, 0, k as u64 + 1);
            2.0 * PI * uniform(draw)
        });
        let mut market = Market {
            phases,
            first_cycle: 1.0,
            factor: FACTOR,
        };
        market.first_cycle = market.cycle(0.0);
        market
    }

    /// Tier 1's four terms on `day`, Kitchin's first: `B17:B20` or `C17:C20`.
    pub fn terms(&self, day: f64) -> [f64; 4] {
        std::array::from_fn(|k| WAVES[k].term(self.phases[k], day))
    }

    /// Tier 1's cycle on `day`, one and its four waves: `=1+B17+B18+B19+B20`.
    pub fn cycle(&self, day: f64) -> f64 {
        let [kitchin, juglar, kuznets, kondratiev] = self.terms(day);
        1.0 + kitchin + juglar + kuznets + kondratiev
    }

    /// The trend on `day`, the drift raised to the day: `=$B$2^B15`.
    pub fn trend(day: f64) -> f64 {
        DRIFT.powf(day)
    }

    /// Tier 0 on `day`, the Kiddie Pool, the trend times one and the year's
    /// wave: `=B21*(1+B16)`.
    pub fn tier0(day: f64) -> f64 {
        Market::trend(day) * (1.0 + YEAR.term(0.0, day))
    }

    /// Tier 1 on `day`, the Day Trader, the trend times the cycle over the
    /// cycle on the first day, so that it opens at exactly 1:
    /// `=B21*B22/$B$23`.
    pub fn tier1(&self, day: f64) -> f64 {
        Market::trend(day) * self.cycle(day) / self.first_cycle
    }

    /// The index on `day`, as the difficulty picks, 0 tier 0 and 1 tier 1:
    /// `=CHOOSE($B$3+1,B24,B25)`. A tier not yet built is `POT.6`'s to refuse.
    pub fn index(&self, difficulty: u64, day: f64) -> f64 {
        match difficulty {
            0 => Market::tier0(day),
            1 => self.tier1(day),
            _ => panic!("tier {difficulty} is not built"),
        }
    }

    /// The five minutes' swing (`POT.11`), `B110`: the half hour's swing over
    /// the root of the ticks a half hour, six steps of it wandering as far as
    /// one of the half hour's did, times the factor, `=$B$5*$B$6/6^0.5`, the
    /// root by the power, as the dialect has no `SQRT`.
    pub fn tick_swing(&self) -> f64 {
        SWING * self.factor / (TICKS_A_HALF_HOUR as f64).powf(0.5)
    }
}

/// The half hour's swing (`POT.10`), the Market sheet's `B5`: the most one
/// half hour of a session moved the index, 0.28% of it, which wandered a
/// session as far as 0.4% an hour did; since `POT.11` the spread of a half
/// hour's six ticks at a factor of 1.
pub const SWING: f64 = 0.0028;
/// The volatility factor within the day (`POT.11`), the Market sheet's `B6`:
/// every tick's move sixteen times the half hour's spread, and no close, the
/// owner's word at the hand test, where the scoping had four; across days
/// `POT.5`'s walk takes it.
pub const FACTOR: f64 = 16.0;
/// The first of the market's draws a session's steps take each day, the
/// fifth: `POT.2`'s four phases take the four before it; the draw the day's
/// headline takes, the next free after the session's 78 (`FRONT.3`), the
/// 83rd; and the day's draws, through it.
pub const SESSION_DRAW: u64 = 5;
pub const HEADLINE_DRAW: u64 = SESSION_DRAW + CLOSING_TICK;
pub const DRAWS_A_DAY: u64 = HEADLINE_DRAW;
const _: () = assert!(HEADLINE_DRAW == 83);
/// The widest a bridge of 78 steps strays from its line, in steps: at its
/// middle tick, `2m(78-m)/78` at the 39th, every step against the rest.
pub const WIDEST: f64 = 39.0;

/// 16807 to each power up to a day's last draw, modulo [`MODULUS`]: a draw
/// found by its place beneath the day's die in one multiplication, so that a
/// session's 78 cost the hand one search of the stream a day and no more.
const BENEATH: [u64; DRAWS_A_DAY as usize + 1] = {
    let mut out = [1u64; DRAWS_A_DAY as usize + 1];
    let mut k = 1;
    while k < out.len() {
        out[k] = power(MULTIPLIER, k as u64);
        k += 1;
    }
    out
};

/// A session's path at a frame (`POT.10`, five minutes at a time since
/// `POT.11`), the Market sheet's rows 29 to 112.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Path {
    /// Each five minutes' step, `B29:B106`: the uniform of the day's market
    /// draw 5 to 82 set between -1 and 1, `2u-1`.
    pub steps: [f64; CLOSING_TICK as usize],
    /// The steps so far, `C29:C106`: the first step, then each the one before
    /// and its own.
    pub sums: [f64; CLOSING_TICK as usize],
    /// The tick's share of the session, `B109`, from 0 at the bell to 1.
    pub share: f64,
    /// The five minutes' swing, `B110`.
    pub tick_swing: f64,
    /// The swing at the tick, `B111`.
    pub swing: f64,
    /// The index now, `B112`.
    pub now: f64,
}

impl Market {
    /// The session's path at a calendar's frame (`POT.10`, `POT.11`), each value
    /// in its cell's operations and order: from the session before's close to
    /// the session's own, weighed by the tick's share, times one and the swing,
    /// `=(D26*(1-B109)+B26*B109)*(1+B111)`; the swing the five minutes' swing
    /// times the steps so far less the share of all 78,
    /// `=B110*(CHOOSE(B108+1,0,C29,...,C106)-B109*C106)`. The steps are the
    /// calendar day's draws, found by their place beneath the day's die, which
    /// a session reads in its own hours and which the swing multiplies away
    /// outside them.
    pub fn path(&self, seed: u64, difficulty: u64, cal: &Calendar) -> Path {
        let die = market_die(seed, cal.day);
        let steps: [f64; CLOSING_TICK as usize] = std::array::from_fn(|k| {
            let draw = die * BENEATH[SESSION_DRAW as usize + k] % MODULUS;
            2.0 * uniform(draw) - 1.0
        });
        let mut sums = steps;
        for k in 1..sums.len() {
            sums[k] = sums[k - 1] + steps[k];
        }
        let share = cal.tick as f64 / CLOSING_TICK as f64;
        let so_far = match cal.tick {
            0 => 0.0,
            m => sums[m as usize - 1],
        };
        let tick_swing = self.tick_swing();
        let swing = tick_swing * (so_far - share * sums[sums.len() - 1]);
        let last = self.index(difficulty, cal.before as f64);
        let close = self.index(difficulty, cal.session as f64);
        let now = (last * (1.0 - share) + close * share) * (1.0 + swing);
        Path {
            steps,
            sums,
            share,
            tick_swing,
            swing,
            now,
        }
    }
}

/// The least the index can be at any five minutes a price reads, at any seed
/// and at either tier built, at the setting's factor (`STATION.1`'s decision
/// 8, `POT.10`, `POT.11`): tier 1's cycle and its cycle on the first day are
/// each between one less and one more than the amplitudes' sum, a half, so
/// their quotient is at least a third; the trend is at least the drift to day
/// 0, the earliest day a price reads, since the first session opens from its
/// own close; tier 0 is never below nine tenths of its trend; and a tick's
/// index strays below the line between two closes by at most the five
/// minutes' swing times the widest a bridge of 78 steps goes.
pub fn least_index() -> f64 {
    let swing: f64 = WAVES.iter().map(|w| w.amplitude).sum();
    let tick = SWING * FACTOR / (TICKS_A_HALF_HOUR as f64).powf(0.5);
    Market::trend(0.0) * (1.0 - swing) / (1.0 + swing) * (1.0 - tick * WIDEST)
}

/// A rung of a station's ladder (`STATION.1`): its name, a fictional company's
/// (`MENU.3`'s first slice, with `BACK.2`); its start, a share's price on day 0
/// in whole cents; its beta, the power of the index its price follows; its
/// dividend yield, a year; and its dividend, the whole cents a share it pays on
/// each dividend day, as the contract states it, where the Stocks sheet
/// computes it from the yield.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rung {
    pub name: &'static str,
    pub start: u64,
    pub beta: f64,
    pub dividend_yield: f64,
    pub dividend: u64,
}

/// The stocks' four rungs, cheapest first, the Stocks sheet's rows 5 to 8:
/// $1, $5, $25 and $100 a share, the cheapest the wildest, a penny miner's
/// long odds, the second a small startup, the third the market itself, the
/// average company, and the dearest the textbook's widget maker, paying 2% a
/// year, 50 cents a share on each of the year's four dividend days.
pub const RUNGS: [Rung; 4] = [
    Rung {
        name: "Longshot Mining",
        start: 100,
        beta: 2.0,
        dividend_yield: 0.0,
        dividend: 0,
    },
    Rung {
        name: "Disruptr",
        start: 500,
        beta: 1.5,
        dividend_yield: 0.0,
        dividend: 0,
    },
    Rung {
        name: "Average Co.",
        start: 2_500,
        beta: 1.0,
        dividend_yield: 0.0,
        dividend: 0,
    },
    Rung {
        name: "General Widget",
        start: 10_000,
        beta: 0.75,
        dividend_yield: 0.02,
        dividend: 50,
    },
];

/// The stocks' names, cheapest first.
pub fn names() -> [&'static str; 4] {
    RUNGS.map(|r| r.name)
}

/// The dividends a year, the Stocks sheet's `B1`: one a quarter.
pub const DIVIDENDS_A_YEAR: u64 = 4;

/// The days of the year a dividend falls on, from 1: the year's 91st, 182nd,
/// 273rd and 364th, each quarter's last but the year's, listed here where the
/// Stocks sheet asks whether the day of the year is a multiple of 91.
pub const DIVIDEND_DAYS: [u64; DIVIDENDS_A_YEAR as usize] = [91, 182, 273, 364];

// The dearest rung's dividend is its yield's: four dividends of 50 cents a
// year are $2, 2% of a $100 share.
const _: () = assert!(RUNGS[3].dividend * DIVIDENDS_A_YEAR * 100 == RUNGS[3].start * 2);

impl Rung {
    /// The rung's price in whole cents at an index: its start times the
    /// index to its beta, plus a half, rounded down, `=INT(B5*<index>^C5+0.5)`,
    /// the power, the product, the sum and the floor in the cell's order.
    pub fn price_at(&self, index: f64) -> u64 {
        (self.start as f64 * index.powf(self.beta) + 0.5).floor() as u64
    }

    /// The rung's price on `day` at a difficulty: over the index today,
    /// `Stocks!E5:E8`, or over yesterday's, `F5:F8`, on the day before.
    pub fn price(&self, market: &Market, difficulty: u64, day: f64) -> u64 {
        self.price_at(market.index(difficulty, day))
    }

    /// What the rung pays a share on a day of the year, from 1, `H5:H8`: its
    /// dividend on a dividend day and nothing on any other.
    pub fn dividend_on(&self, day_of_year: u64) -> u64 {
        if DIVIDEND_DAYS.contains(&day_of_year) {
            self.dividend
        } else {
            0
        }
    }
}

/// The days a disclosure stands, the Disclosures sheet's `D5` (`MENU.1`): a
/// month of days, twelve minutes at the manifest's rate since `POT.10`.
pub const DAYS_A_DISCLOSURE: u64 = 30;

/// The row of the Disclosures sheet the fine print shows on a day, from 1
/// (`MENU.1`), when a disclosure stands `days` days among `rows`. The cell
/// divides the day and takes the remainder; the hand counts the months the
/// day has passed, each moving the row on one and the last back to the
/// first. The texts are the content's, written once, in `src/content/`.
pub fn disclosure(day: u64, rows: u64, days: u64) -> u64 {
    let mut row = 1;
    for _ in 0..day / days {
        row = if row == rows { 1 } else { row + 1 };
    }
    row
}

/// Whole cents written as their dollars (`FRONT.1`), as the Portfolio draws
/// them: the dollars, then a point and the cents less their trailing zeros,
/// no point when none is left, and a minus before a loss. Spelled by text,
/// never through a double.
pub fn dollars(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let c = cents.unsigned_abs();
    let (whole, part) = (c / 100, c % 100);
    if part == 0 {
        format!("{sign}{whole}")
    } else if part % 10 == 0 {
        format!("{sign}{whole}.{}", part / 10)
    } else {
        format!("{sign}{whole}.{part:02}")
    }
}

/// The width in characters at 100% of the columns a view's dollars stand in
/// (`POT.10`): wide enough for $9,999,999.99 under a format's code, past which
/// the viewport draws `#`s and a person drags the column wider, as in Excel.
pub const DOLLARS_COLUMN: u64 = 12;

/// Whole dollars with their thousands grouped by commas (`GUARD.1`'s hand
/// test, the owner's word: "Thousands separators need to be implemented"):
/// 1,234,567. The cell's format writes it by its code's `#,##0`; the hand counts
/// the digits back from the last.
pub fn grouped(dollars: u64) -> String {
    let digits = dollars.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (k, ch) in digits.chars().enumerate() {
        if k > 0 && (digits.len() - k) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

/// Whole cents, never negative, as a view draws them (`POT.10`'s hand test, the
/// library's `dollars`): the dollars, their thousands grouped by commas since
/// `GUARD.1`'s, a point and the cents in two digits.
pub fn two_decimals(cents: u64) -> String {
    format!("{}.{:02}", grouped(cents / 100), cents % 100)
}

/// A change in whole cents as a view draws it (`POT.10`, the library's
/// `signed-dollars`): two decimals, the thousands grouped, after a gain's or
/// nothing's `+` or a loss's `-`.
pub fn signed_two_decimals(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "+" };
    format!("{sign}{}", two_decimals(cents.unsigned_abs()))
}

/// A time of day in minutes after midnight as the views show it (`POT.10`):
/// the hour, a colon and the minutes in two digits, 9:30 and 16:00.
pub fn time_shown(minutes: u64) -> String {
    format!("{}:{:02}", minutes / 60, minutes % 60)
}

/// A view's date at a frame (`FRONT.1`, `POT.10`): the day's name, the year
/// and the day of the year, each from 1, and the time shown since `POT.11`,
/// the five minutes in session and the half hour between, as of which its
/// numbers stand, none of them audited.
pub fn as_of(cal: &Calendar) -> String {
    format!(
        "As of {}, year {}, day {}, {} (unaudited)",
        DAY_NAMES[cal.weekday as usize],
        cal.year,
        cal.day_of_year,
        time_shown(cal.clock())
    )
}

/// The bell's five lines (`POT.10`, `GUARD.1`), a view's row 2: while the
/// market is open; while the first bell waits for the exam; on a trading day
/// before its bell; after the close, Monday to Thursday; and from Friday's
/// close to Monday's bell, the last two the owner's words at the hand test.
pub const BELL_OPEN: &str = "The market is open until 16:00.";
pub const BELL_WAIT: &str = "The market opens when you leave the exam room.";
pub const BELL_BEFORE: &str = "The market opens at 9:30.";
pub const BELL_CLOSED: &str = "The market has closed. Analysts are analyzing the aftermath.";
pub const BELL_WEEKEND: &str = "The market is closed until Monday at 9:30.";

/// The bell's line at a frame. The cell compares the session's day with the
/// day; the hand asks whether the market is open, then whether the bell
/// waits, then the time and the weekday.
pub fn bell(cal: &Calendar) -> &'static str {
    if cal.open() {
        BELL_OPEN
    } else if cal.waits {
        BELL_WAIT
    } else if cal.weekday < TRADING_DAYS && cal.five() < OPENING_BELL {
        BELL_BEFORE
    } else if cal.weekday + 1 < TRADING_DAYS {
        BELL_CLOSED
    } else {
        BELL_WEEKEND
    }
}

/// Which way a stock went since yesterday, as the Portfolio shows it beside
/// its change (`FRONT.2`): up, down or flat, by its price today in whole
/// cents against yesterday's. The cell reads the sign of the change the view
/// shows; the hand compares the two prices the change was taken from.
pub fn glyph(now: i64, was: i64) -> &'static str {
    match now.cmp(&was) {
        std::cmp::Ordering::Greater => "▲",
        std::cmp::Ordering::Less => "▼",
        std::cmp::Ordering::Equal => "=",
    }
}

/// The Overview (`FRONT.3`): the firm at a glance, a tab beside the Portfolio,
/// where the certificate led until `FRONT.8`'s second look; and its firm's line
/// before the exam's last answer gives the firm its style and its name
/// (`BACK.2`).
pub const OVERVIEW: &str = "Overview";
pub const FIRM_LINE: &str = "Your firm: name pending registration.";

/// The Overview's firm line (`BACK.2`), its `A3`: the line pending before a
/// style is given, then the firm's name and its investment style, the
/// candidate's own words, `(style, name)`, the Firms sheet's row the style
/// picks.
pub fn firm_line(firm: Option<(&str, &str)>) -> String {
    match firm {
        None => FIRM_LINE.to_string(),
        Some((style, name)) => format!("Your firm: {name}. Investment style: {style}."),
    }
}
/// The Overview's label over each pocket's net worth, and the pockets' own, in
/// its column A from row 5.
pub const NET_WORTH_LABELS: [&str; 3] = ["Net worth ($)", "  Firm", "  Personal"];

/// The weather's table (`FRONT.3`), the Market sheet's `A115:B119`: each word
/// beside the change over the look back, in whole tenths of a percent, that
/// it begins at, from the top; and the last word, below them all. Sunny from a
/// rise of 1.0%, Calm at 0.0% and Rain from a fall of 1.0%: at the seed, a
/// century's trading days read 4.9% Rain, 23.4% Overcast, 4.5% Calm, 42.1%
/// Partly sunny and 25.0% Sunny, and 2,000 seeds' first five years alike.
pub const WEATHER: [(&str, i64); 4] = [
    ("Sunny", 10),
    ("Partly sunny", 1),
    ("Calm", 0),
    ("Overcast", -9),
];
pub const WEATHER_BELOW: &str = "Rain";

impl Market {
    /// The market's change over the look back at a calendar's frame
    /// (`FRONT.3`), the Market sheet's `B122`: the index at the last close made
    /// over its index the look back's days before, less one, in the cells'
    /// operations and order, `=B121/C26-1`; and in whole tenths of a percent,
    /// rounded half up, `B123`, `=INT(B122*1000+0.5)`.
    pub fn change(&self, difficulty: u64, cal: &Calendar) -> f64 {
        let day = cal.last_close() as f64;
        self.index(difficulty, day) / self.index(difficulty, day - LOOK_BACK as f64) - 1.0
    }
    pub fn tenths(&self, difficulty: u64, cal: &Calendar) -> i64 {
        (self.change(difficulty, cal) * 1000.0 + 0.5).floor() as i64
    }
}

/// The weather's word for a change in whole tenths of a percent, the Market
/// sheet's `B124`. The cells compare the change with each word's tenths from
/// the top and take the first it reaches; the hand counts the words whose
/// tenths it reaches and reads the table from the bottom by that count.
pub fn weather(tenths: i64) -> &'static str {
    let reached = WEATHER.iter().filter(|(_, from)| tenths >= *from).count();
    if reached == 0 {
        WEATHER_BELOW
    } else {
        WEATHER[WEATHER.len() - reached].0
    }
}

/// The weather's line on the Overview: the word, then the change it is read
/// off, up, down or flat, in tenths of a percent, over the look back's days.
/// The cell joins the whole tenths over ten and their remainder by ten; the
/// hand formats the size's digits.
pub fn weather_line(tenths: i64, days: u64) -> String {
    let size = tenths.unsigned_abs();
    let moved = match tenths.signum() {
        0 => "flat".to_string(),
        s => format!(
            "{} {}.{}%",
            if s < 0 { "down" } else { "up" },
            size / 10,
            size % 10
        ),
    };
    format!(
        "Market weather: {}, the market {moved} in the {days} days to the last close.",
        weather(tenths)
    )
}

/// The ticker's label on the Overview, before the day's headline.
pub const HEADLINE_LABEL: &str = "Today's headline: ";

/// The day's headline (`FRONT.3`): the row of the Headlines sheet, from 1,
/// that the day's 83rd market draw picks among `rows`, the sheet's `D2`. The
/// cell multiplies the draw's uniform by the rows and rounds down; the hand
/// divides the draw times the rows by the modulus, in whole numbers. The two
/// agree at every draw: the modulus is prime and the draw and the rows below
/// it, so the quotient is never a whole number and stands at least one part
/// in the modulus, about 4.7e-10, from the next, where the double's two
/// roundings move it less than 4e-13 for any sheet a picker holds.
pub fn headline(seed: u64, day: u64, rows: u64) -> u64 {
    market_draw(seed, day, HEADLINE_DRAW) * rows / MODULUS + 1
}

/// The fee a prospectus proofread pays (`BACK.1`), in whole cents, the
/// Hustle sheet's `B1`: $250, the owner's word at `GUARD.1`'s hand test,
/// where the contract had $25.
pub const FEE: u64 = 25_000;
/// The days between two prospectuses, `B6`, and a prospectus's work in
/// minutes, `B7` (`POT.10`, the owner's word at its hand test): a prospectus a
/// week, its fee paid a day after the claim.
pub const DAYS_BETWEEN: u64 = 7;
pub const WORK: u64 = A_DAY;
/// The claims a run, the Hustle sheet's `B2`: four, the contract's cap, the
/// owner's choice of 2026-10-16 over a claim a day for good.
pub const CLAIMS_A_RUN: usize = 4;
/// The view the hustle's button stands on, and the button's column and row
/// there, the Hustle sheet's `B3:B5`: Gigs' `A5`.
pub const BUTTON: (&str, u64, u64) = ("Gigs", 1, 5);
/// The button's text, in its cell.
pub const BUTTON_TEXT: &str = "[ Proofread a prospectus ]";
/// The key that holds the hustle's button while the button's view shows, as
/// the Keys sheet binds it, a `KeyboardEvent.code`; and its name beside the
/// button.
pub const KEY_CODE: &str = "KeyP";
pub const KEY_TEXT: &str = "or press P";

/// What the host writes into the Input sheet in a frame, as far as the game
/// reads it (Alonzo's `SPEC.md` section 3.3, version 4): the sheet shown; the
/// pointer's column and row on it, 0 off its cells; the left button held; the
/// keys held, each its `KeyboardEvent.code`; and the keys bound when the rows
/// were written, by the Keys sheet the host read after the last step, so that
/// a key's button is down while the key is both. The hustle's one key at
/// `BACK.1`, the exam's five more since `GUARD.1`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inputs {
    pub sheet: String,
    pub column: u64,
    pub row: u64,
    pub left: bool,
    pub held: BTreeSet<String>,
    pub bound: BTreeSet<String>,
}

impl Inputs {
    /// Whether a key's button is down this frame: the key held, and bound.
    pub fn down(&self, code: &str) -> bool {
        self.held.contains(code) && self.bound.contains(code)
    }

    /// A key's button going down this frame: down, where the last frame's
    /// was not.
    pub fn pressed(now: &Inputs, last: &Inputs, code: &str) -> bool {
        now.down(code) && !last.down(code)
    }
}

/// A view's button clicked this frame (`BACK.1`, the library's `clicked`
/// since `GUARD.1`): the left button going down over the button's cell, its
/// view's name, its column and its row, while that view shows, as the host
/// writes the pointer.
pub fn clicked(now: &Inputs, last: &Inputs, (view, column, row): (&str, u64, u64)) -> bool {
    now.left
        && !last.left
        && now.sheet.eq_ignore_ascii_case(view)
        && now.column == column
        && now.row == row
}

/// The side hustle by hand (`BACK.1`, a week and a day's work since
/// `POT.10`): its claims in the order they were made, each the day it counted
/// against and the time shown, in minutes from day 0's midnight, the person
/// saw when making it (the half hour until `POT.11`, and since, the five
/// minutes in session and the half hour between). The cells judge a press
/// by `shut`'s twin and keep the day the next prospectus arrives and the time
/// the last one is done; the hand keeps the list and reads its length and its
/// last claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hustle {
    /// The claims a run, [`CLAIMS_A_RUN`] unless a person's edit raised it.
    pub cap: usize,
    pub claims: Vec<(u64, u64)>,
}

impl Default for Hustle {
    fn default() -> Hustle {
        Hustle {
            cap: CLAIMS_A_RUN,
            claims: Vec::new(),
        }
    }
}

impl Hustle {
    /// A click this frame: the left button going down over the button's cell
    /// while its view shows, as the host writes the pointer.
    pub fn clicked(now: &Inputs, last: &Inputs) -> bool {
        clicked(now, last, BUTTON)
    }

    /// Whether the Keys sheet binds the key while `sheet` shows: on the
    /// button's view alone, so that each view binds keys of its own (the
    /// owner's word at the hand test, 2026-10-16).
    pub fn binds(sheet: &str) -> bool {
        sheet.eq_ignore_ascii_case(BUTTON.0)
    }

    /// The key's button going down this frame: the key held and bound, where
    /// the last frame's was not both.
    pub fn keyed(now: &Inputs, last: &Inputs) -> bool {
        Inputs::pressed(now, last, KEY_CODE)
    }

    /// Whether a press would claim on `day`: the run has a claim left, and no
    /// claim counted against a day of the week before it.
    pub fn open(&self, day: u64) -> bool {
        self.claims.len() < self.cap
            && match self.claims.last() {
                Some(&(d, _)) => day >= d + DAYS_BETWEEN,
                None => true,
            }
    }

    /// A frame of the hustle, its inputs beside the last frame's: a press
    /// claims when the hustle was open on the day the person saw, the last
    /// frame's, and counts against that day, its work running from the time
    /// the person saw. Whether a claim was made.
    pub fn frame(&mut self, now: &Inputs, last: &Inputs, day_shown: u64, time_seen: u64) -> bool {
        let pressed = Hustle::clicked(now, last) || Hustle::keyed(now, last);
        let claim = pressed && self.open(day_shown);
        if claim {
            self.claims.push((day_shown, time_seen));
        }
        claim
    }

    /// Whether the last claim's work is under way at the five minutes `at`,
    /// in minutes from day 0's midnight: a day's work not yet done.
    pub fn proofreading(&self, at: u64) -> bool {
        self.claims.last().is_some_and(|&(_, t)| t + WORK > at)
    }

    /// The prospectuses done at `at`, their work finished, and their fees in
    /// whole cents.
    pub fn done(&self, at: u64) -> usize {
        self.claims.len() - usize::from(self.proofreading(at))
    }
    pub fn earned(&self, at: u64) -> u64 {
        self.done(at) as u64 * FEE
    }

    /// What Gigs says of the hustle at a frame: one being proofread and when
    /// its fee is paid, the run's claims spent, this week's made and when the
    /// next arrives, or a prospectus waiting.
    pub fn status(&self, cal: &Calendar) -> String {
        let at = cal.five_minutes();
        match self.claims.last() {
            Some(&(_, t)) if self.proofreading(at) => {
                let due = t + WORK;
                let when = if due / A_DAY > cal.day {
                    "tomorrow"
                } else {
                    "today"
                };
                format!(
                    "Proofreading: the fee is paid {when} at {}.",
                    time_shown(due % A_DAY)
                )
            }
            _ if self.claims.len() >= self.cap => format!(
                "Compliance has approved {} a run, and you have proofread {}.",
                self.cap,
                self.claims.len()
            ),
            Some(&(d, _)) if !self.open(cal.day) => match d + DAYS_BETWEEN - cal.day {
                1 => "Done for the week: the next prospectus arrives tomorrow.".to_string(),
                n => format!("Done for the week: the next prospectus arrives in {n} days."),
            },
            _ => "A prospectus awaits. Nobody else has read one either.".to_string(),
        }
    }
}

/// The firm's capital before the exam, in whole cents, and what a correct
/// answer adds, the Proctor sheet's `B1` and `B2` (`GUARD.1`): $5,000 and $500,
/// the contract's.
pub const CAPITAL_BEFORE: u64 = 500_000;
pub const CAPITAL_A_RIGHT: u64 = 50_000;
/// The questions the Series 777 asks, eleven, and those it scores, the first
/// ten, drawn from the bank; the eleventh, ungraded, sets the firm's style
/// (the owner's word at the hand test: "The exam should have 11 question (10
/// of substance, the eleventh question being firm style)").
pub const ASKED: usize = 11;
pub const SCORED: usize = 10;
/// A waiver's correct answers and its style, `B4` and `B5`: five, the middle
/// of 0 to 10, and the first style.
pub const WAIVED_SCORE: u64 = 5;
pub const WAIVED_STYLE: u64 = 1;
/// The personal cash at the start, the Cash sheet's `B1` (`GUARD.1`'s hand
/// test): $100, the contract's, in whole cents. The Cash sheet keeps the
/// firm's cash, the exam's capital, and the personal, this and the hustle's
/// fees once paid, which the Portfolio shows as its cash on hand.
pub const PERSONAL_CASH: u64 = 10_000;
/// The waiver's cell, which a person's edit of 1 waives the exam by, `B3`.
pub const WAIVER: (&str, &str) = ("Proctor", "B3");
/// Where the certificate leads, `B6`: the Portfolio, its prices moving from the
/// first bell, again since `FRONT.8`'s second look (the owner's word: "The
/// Overview tab is an anticlimactic, nearly-static page that doesn't give the
/// impression that prices are changing"), where the Overview stood from
/// `FRONT.3`; and the exam's two views.
pub const LEAD: &str = "Portfolio";
pub const EXAM: &str = "Exam";
pub const CERTIFICATE: &str = "Certificate";
/// The most rows one picker holds (`MISE.3`'s probe 3), so the most a bank
/// may hold: every prime above it is coprime to every bank's size.
pub const PICKER_ROWS: u64 = 1_547;
/// The strides the exam's second draw picks among: the first ten primes
/// above [`PICKER_ROWS`], found by trial division.
pub const STRIDES: [u64; 10] = primes_above(PICKER_ROWS);
/// The exam's four options, each its letter, its row on the Exam and the key
/// that presses it there, all in [`OPTION_COLUMN`]; and the certificate's
/// button, its place and its key, the Proctor sheet's `A24:E28`.
pub const OPTIONS: [(&str, u64, &str); 4] = [
    ("A", 7, "KeyA"),
    ("B", 8, "KeyB"),
    ("C", 9, "KeyC"),
    ("D", 10, "KeyD"),
];
pub const OPTION_COLUMN: u64 = 1;
pub const OPEN_BUTTON: (&str, u64, u64) = (CERTIFICATE, 1, 11);
pub const OPEN_KEY: &str = "KeyO";
/// The texts the two views write beside their buttons, naming their keys.
pub const OPEN_TEXT: &str = "[ Open for business ]";
pub const OPEN_KEY_TEXT: &str = "or press O";
pub const ANSWER_KEYS_TEXT: &str = "Click an answer, or press its letter: A, B, C or D.";
/// The eleventh question, ungraded, the Exam view's own (`MENU.2`'s decision
/// 4).
pub const STYLE_PROMPT: &str = "Which best describes you?";
/// The Exam's width in characters at 100%, its column A, inside which every option
/// stands whole with its letter, so that every character of it is its
/// button.
pub const EXAM_COLUMN: u64 = 100;
/// The certificate's line naming the firm (`BACK.2`), where the style stood:
/// its label, and its value before the eleventh answer gives the style that
/// names the firm.
pub const NAME_LABEL: &str = "Firm name";
pub const NAME_PENDING: &str = "pending registration";
/// The heading over the certificate's plain statement, the owner's word at
/// the hand test, so that its "You." does not stand alone.
pub const CAVEAT_HEADING: &str = "Caveat:";
/// The buttons' look (`GUARD.1`'s hand test, the owner's words: "lavender
/// background and white text"; `FRONT.8`, the night desk): format 1, a button of
/// Trading's grid, white on the button's lavender, the deep shade lightened toward
/// Lovelace Lavender until white reads at 4.53:1, not bold at the owner's word at
/// `FRONT.2`'s second hand test, its text 11 points at the zoom, [`zoomed`], centered, its
/// right and bottom edges the card's own, two pixels, so that two side by side stay
/// two.
pub fn button_format() -> String {
    format!(
        "(format 1 \"8063D2\" general nowrap (font \"FFFFFF\" {}) (align center) (border (right medium \"2E2645\") (bottom medium \"2E2645\")))",
        zoomed(11)
    )
}
/// An exam's option, a button whose long text reads from its left, bold as every
/// white over lavender is (`FRONT.2`'s hand test, the owner's word), the card
/// between two, three pixels.
pub fn option_format() -> String {
    format!(
        "(format 11 \"8063D2\" general nowrap (font bold \"FFFFFF\" {}) (align left) (border (bottom thick \"2E2645\")))",
        zoomed(11)
    )
}
/// A button standing alone, on the card with no neighbor to be told from: the
/// certificate's, the switch and the hustle's.
pub fn lone_button_format() -> String {
    format!(
        "(format 13 \"8063D2\" general nowrap (font bold \"FFFFFF\" {}) (align center))",
        zoomed(11)
    )
}
/// The buttons' format's index, an option's, and a button's standing alone.
pub const BUTTON_LOOK: u64 = 1;
pub const OPTION_LOOK: u64 = 11;
pub const LONE_BUTTON_LOOK: u64 = 13;
/// The views' zoom in percent (`FRONT.8`, the owner's look: "I zoomed the page in
/// to 200% and it looks more like a game this way, so is it possible to set the
/// standard zoom for Buffet to this size?", and the second, "maybe we should scale
/// it down a bit to be safe so side-scrolling doesn't ruin the UX on smaller
/// monitors"): every size a view draws, its type, its columns and its rows'
/// heights, is said at 100% and drawn at this many hundredths of it, the source's
/// `the-zoom` handing it to each macro that says a size, since the page draws a
/// grid at its cells' own sizes and never zooms one (Alonzo's `ENGINE.14`, decision
/// 9). At 135% no card scrolls sideways in a window 1,280 pixels wide. A zoom is a
/// multiple of 5, so that each size it gives is a whole number of twentieths of a
/// point, as a type's size must be. Every width and size in this file is said at
/// 100%, in characters or points, the unit a text's length is held to; a view
/// draws it [`zoomed`].
pub const ZOOM_PERCENT: u64 = 135;

/// A size said at 100%, in hundredths as a view draws it at the zoom: a column's
/// width in hundredths of a character, a font's size or a row's height in
/// hundredths of a point; exact, so that a length is held to it with no float.
pub const fn zoomed_hundredths(at_100: u64) -> u64 {
    at_100 * ZOOM_PERCENT
}

/// A size said at 100%, as a view draws it at the zoom and the record spells it,
/// at its shortest: `135`, `35.1`, `28.35`.
pub fn zoomed(at_100: u64) -> String {
    let n = zoomed_hundredths(at_100);
    match n % 100 {
        0 => format!("{}", n / 100),
        p if p % 10 == 0 => format!("{}.{}", n / 100, p / 10),
        p => format!("{}.{p:02}", n / 100),
    }
}

/// Every view's card (`FRONT.8`): the format its columns paint their empty cells
/// with, midnight plum, a line of text tall where a cell holds the empty text, the
/// format's index; each view's width in characters at 100%, Trading's table and a
/// margin of 5, so that a tab turned moves no edge; and the Overview's net worth's
/// column, its figures larger.
pub const CARD_LOOK: u64 = 16;
pub fn card_format() -> String {
    format!(
        "(format 16 \"2E2645\" general nowrap (font {}))",
        zoomed(11)
    )
}
pub const CARD_WIDTH: u64 = 126;
pub const WORTH_COLUMN: u64 = 16;
/// The height of a card's row that holds nothing, the default row's 15 points,
/// said by the kitchen's `margins` at the zoom since the page fits a row to no type
/// where it holds none.
pub const MARGIN_ROW: u64 = 15;

/// The first ten primes above `floor`, by trial division.
pub const fn primes_above(floor: u64) -> [u64; 10] {
    let mut out = [0; 10];
    let (mut k, mut n) = (0, floor + 1);
    while k < out.len() {
        if is_prime(n) {
            out[k] = n;
            k += 1;
        }
        n += 1;
    }
    out
}

/// The exam's start, from 0, a row of a bank of `bank` questions: its first
/// draw round the bank, the Proctor sheet's `B9`.
pub fn start(seed: u64, bank: u64) -> u64 {
    exam_draw(seed, 1) % bank
}

/// The exam's stride, `B10`: one of [`STRIDES`] by its second draw.
pub fn stride(seed: u64) -> u64 {
    STRIDES[(exam_draw(seed, 2) % STRIDES.len() as u64) as usize]
}

/// The ten questions drawn, each a row of the bank from 1, `B12:B21`: from
/// the start, the stride's remainder added a question at a time and the
/// bank's size taken off once it is passed. The cells compute each row from
/// the start as a product round the bank; the hand walks.
pub fn draw(seed: u64, bank: u64) -> [u64; SCORED] {
    let step = stride(seed) % bank;
    let mut at = start(seed, bank);
    let mut rows = [0; SCORED];
    for row in rows.iter_mut() {
        *row = at + 1;
        at += step;
        if at >= bank {
            at -= bank;
        }
    }
    rows
}

/// The Series 777 by hand (`GUARD.1`): the ten rows drawn, the bank's right
/// letters, row 1 first, the options given in order, each 1 to 4, whether
/// the exam was waived and the game has begun, and the frames played. The
/// cells count the answers and the right ones over their twins and judge a
/// press by the right option the last frame showed, as a number; the hand
/// keeps the list and compares each option's letter with the bank's, as
/// text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exam {
    pub rows: [u64; SCORED],
    pub letters: Vec<String>,
    pub answers: Vec<u64>,
    pub waived: bool,
    pub begun: bool,
    pub frames: u64,
    /// The frame the game began at, its certificate's button or a waiver's,
    /// counted as `frames` counts (`CRITIC.1`: the Camera shows the lead there
    /// alone).
    pub begun_at: Option<u64>,
}

impl Exam {
    /// The exam of a seed over a bank whose right letters are `letters`.
    pub fn new(seed: u64, letters: Vec<String>) -> Exam {
        Exam {
            rows: draw(seed, letters.len() as u64),
            letters,
            answers: Vec::new(),
            waived: false,
            begun: false,
            frames: 0,
            begun_at: None,
        }
    }

    /// Whether a question is open, `B33`: fewer than eleven answered, and no
    /// waiver.
    pub fn open(&self) -> bool {
        self.answers.len() < ASKED && !self.waived
    }

    /// Whether the certificate waits for its button: all eleven answered, and
    /// the game not begun.
    pub fn pending(&self) -> bool {
        self.answers.len() == ASKED && !self.begun
    }

    /// The bank's row shown, `B34`: the next scored question's, the tenth's
    /// from the tenth answer on.
    pub fn row_shown(&self) -> u64 {
        self.rows[self.answers.len().min(SCORED - 1)]
    }

    /// The right option of the question shown, `B35`, 1 to 4: its letter's
    /// place among the options.
    pub fn right(&self) -> u64 {
        let letter = &self.letters[self.row_shown() as usize - 1];
        OPTIONS
            .iter()
            .position(|(l, _, _)| *l == letter.as_str())
            .map_or(0, |k| k as u64 + 1)
    }

    /// The right answers of the ten, `B37`: each option's letter the bank's
    /// letter of the row asked.
    pub fn correct(&self) -> u64 {
        self.answers
            .iter()
            .zip(&self.rows)
            .filter(|(&option, &row)| {
                OPTIONS[option as usize - 1].0 == self.letters[row as usize - 1]
            })
            .count() as u64
    }

    /// The score, `B38`: a waiver's, or the right answers.
    pub fn score(&self) -> u64 {
        if self.waived {
            WAIVED_SCORE
        } else {
            self.correct()
        }
    }

    /// The firm's capital in whole cents, `B39`.
    pub fn capital(&self) -> u64 {
        CAPITAL_BEFORE + CAPITAL_A_RIGHT * self.score()
    }

    /// The firm's style, `B40`: a waiver's, else the eleventh answer, 0 until
    /// it is given.
    pub fn style(&self) -> u64 {
        if self.waived {
            WAIVED_STYLE
        } else {
            self.answers.get(SCORED).copied().unwrap_or(0)
        }
    }

    /// The sheet the game shows, the Camera's `B3`, at the frame last played:
    /// the exam's while a question is open, its certificate's after the
    /// eleventh, where the certificate leads at the frame the game began, and
    /// none, the empty text, at every frame after it (`CRITIC.1`), so that a
    /// card's Report is a change of its own; [`camera`] adds the cards.
    pub fn shows(&self) -> &'static str {
        if self.begun {
            if self.begun_at == Some(self.frames) {
                LEAD
            } else {
                ""
            }
        } else if self.open() {
            EXAM
        } else {
            CERTIFICATE
        }
    }

    /// The keys the Keys sheet binds while `sheet` shows, by the exam's state:
    /// A to D on the Exam while a question is open, and O on the Certificate
    /// while it waits for its button.
    pub fn binds(&self, sheet: &str) -> Vec<&'static str> {
        if sheet.eq_ignore_ascii_case(EXAM) && self.open() {
            OPTIONS.iter().map(|(_, _, key)| *key).collect()
        } else if sheet.eq_ignore_ascii_case(OPEN_BUTTON.0) && self.pending() {
            vec![OPEN_KEY]
        } else {
            Vec::new()
        }
    }

    /// The option pressed this frame, 1 to 4: of the four, the first by
    /// letter whose cell was clicked or whose key went down.
    pub fn option_pressed(now: &Inputs, last: &Inputs) -> Option<u64> {
        OPTIONS
            .iter()
            .position(|&(_, row, key)| {
                clicked(now, last, (EXAM, OPTION_COLUMN, row)) || Inputs::pressed(now, last, key)
            })
            .map(|k| k as u64 + 1)
    }

    /// Whether the certificate's button was pressed this frame, by its click
    /// or its key.
    pub fn opened(now: &Inputs, last: &Inputs) -> bool {
        clicked(now, last, OPEN_BUTTON) || Inputs::pressed(now, last, OPEN_KEY)
    }

    /// A frame of the exam, judged by the state the last frame showed, which
    /// is this exam's before the frame: an option pressed is an answer while a
    /// question was open, and the first frame's last showed none; the
    /// certificate's button begins the game once all eleven were answered.
    /// Whether an answer was taken.
    pub fn frame(&mut self, option: Option<u64>, opened: bool) -> bool {
        let shown = self.frames > 0;
        let begins = opened && shown && self.answers.len() == ASKED;
        let answered = option.is_some() && shown && self.open();
        if let (true, Some(o)) = (answered, option) {
            self.answers.push(o);
        }
        if begins && !self.begun {
            self.begun = true;
            self.begun_at = Some(self.frames + 1);
        }
        self.frames += 1;
        answered
    }

    /// A waiver, a person's edit of the Proctor's `B3`: the exam closed and
    /// the game begun, at the waiver's score and style, from the next frame
    /// played.
    pub fn waive(&mut self) {
        self.waived = true;
        if !self.begun {
            self.begun = true;
            self.begun_at = Some(self.frames + 1);
        }
    }

    /// Where the exam stands, the Exam's `A3`, its second sentence the
    /// owner's words at the hand test.
    pub fn status(&self) -> String {
        if self.open() {
            if self.answers.len() < SCORED {
                format!(
                    "The Series 777, question {} of {ASKED}. Correct answers earn starting capital.",
                    self.answers.len() + 1
                )
            } else {
                format!(
                    "The Series 777, question {ASKED} of {ASKED}, ungraded: it names your firm."
                )
            }
        } else if self.waived {
            "The Series 777: waived.".to_string()
        } else {
            "Pencils down. Your certificate is on its sheet.".to_string()
        }
    }

    /// What the certificate says of the exam, its `A5`.
    pub fn headline(&self) -> &'static str {
        if self.open() {
            "Pending: the proctor certifies after the eleventh question."
        } else if self.waived {
            "Waived: the regulator assumes you would have passed."
        } else {
            "The bearer has sat the Series 777 and passed, as every candidate does."
        }
    }
}

/// The view the order desk's buttons stand on (`PLATE.1`), the Orders sheet's
/// `B2`.
pub const TRADING: &str = "Trading";

/// A trade the order desk makes of a stock: a share or ten bought, as many as
/// the cash covers, a share or ten sold, or every share held.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trade {
    Buy(u64),
    BuyMax,
    Sell(u64),
    SellAll,
}

/// The six trades, in the order of each stock's buttons on Trading and of the
/// commands that make them.
pub const TRADES: [Trade; 6] = [
    Trade::Buy(1),
    Trade::Buy(10),
    Trade::BuyMax,
    Trade::Sell(1),
    Trade::Sell(10),
    Trade::SellAll,
];
/// Each trade's button, its text, and its column on Trading, E to J; and each
/// stock's row there, 6 to 9: the Orders sheet's `D5:I5` and `C6:C9`.
pub const TRADE_TEXTS: [&str; 6] = [
    "[ Buy 1 ]",
    "[ Buy 10 ]",
    "[ Buy max ]",
    "[ Sell 1 ]",
    "[ Sell 10 ]",
    "[ Sell all ]",
];
pub const TRADE_COLUMNS: [u64; 6] = [5, 6, 7, 8, 9, 10];
pub const STOCK_ROWS: [u64; 4] = [6, 7, 8, 9];
/// The width of each button's column on Trading, in characters at 100%.
pub const BUTTON_COLUMN: u64 = 12;
/// The commands that pick a stock, each its name and its key, the Commands
/// sheet's rows 2 to 5; and those that trade the stock picked, rows 6 to 11,
/// in the order of the trades (the page's version 5, `SPEC.md` section 3.10).
pub const PICKS: [(&str, &str); 4] = [
    ("rung-1", "Digit1"),
    ("rung-2", "Digit2"),
    ("rung-3", "Digit3"),
    ("rung-4", "Digit4"),
];
pub const TRADE_COMMANDS: [(&str, &str); 6] = [
    ("buy-1", "KeyB"),
    ("buy-10", "KeyN"),
    ("buy-max", "KeyM"),
    ("sell-1", "KeyS"),
    ("sell-10", "KeyD"),
    ("sell-all", "KeyF"),
];
/// A command's key as Trading says it, `KeyB` as B and `Digit1` as 1: over each
/// trade's buttons, and beside each stock not picked (`FRONT.2`'s hand test, the
/// owner's word).
pub fn key_hint(key: &str) -> &str {
    key.strip_prefix("Key")
        .or_else(|| key.strip_prefix("Digit"))
        .unwrap_or(key)
}
/// The switch (`PLATE.2`): its place on Trading, column D and row 11 beside
/// the cash available, the Orders sheet's `B43:B44`; its text and its key's
/// beside it; and its command, its name and its key, the Commands sheet's
/// row 12, C for the cash it turns.
pub const SWITCH: (&str, u64, u64) = (TRADING, 4, 11);
pub const SWITCH_TEXT: &str = "[ Switch ]";
pub const SWITCH_KEY_TEXT: &str = "or press C";
pub const SWITCH_COMMAND: (&str, &str) = ("switch-cash", "KeyC");
/// The pockets by their number, the Orders sheet's `B1`: the firm's, and
/// your own; and their names as the ledger writes them.
pub const FIRM: u64 = 1;
pub const PERSONAL: u64 = 2;
pub const POCKET_NAMES: [&str; 2] = ["Firm", "Personal"];
/// The ledger's rows, a ring, from row 2 (`MISE.3`'s probe 3).
pub const LEDGER_ROWS: u64 = 100;
/// Trading's lines: the keys, and the last order before there is one.
pub const TRADING_KEYS: &str = "Press 1 to 4 to pick a stock, marked ▸; then B, N or M buys 1, 10 or the most, and S, D or F sells 1, 10 or all.";
pub const NO_ORDER: &str = "No order yet.";
/// Trading's sentence for an order refused against cash shown as an error
/// (`PLATE.6`), plain as every refusal is, being a line a player acts on.
pub const UNCOUNTED: &str =
    "Refused: the cash shown is an error. Orders fill against cash the books can count.";
/// Trading's sentence for an order refused at the books' ceiling (`PLATE.6`'s
/// second slice), as plain.
pub const PAST_THE_CEILING: &str = "Refused: the order would take the books past their ceiling.";

/// Why an order filled or did not, the Orders sheet's `B27`; since `PLATE.6`
/// the sixth, the pocket's cash shown an error, and since its second slice the
/// seventh, the books' ceiling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Filled = 1,
    Closed = 2,
    NoStock = 3,
    NoCash = 4,
    NoShares = 5,
    Uncounted = 6,
    Ceiling = 7,
}

/// What the last frame showed, by which an order is judged: whether the market
/// was open, each stock's price now in whole cents, each pocket's cash, and
/// the time the person saw, as the ledger writes it; and since `PLATE.6` each
/// pocket's cash shown as an error instead, by its name, which no order fills
/// against.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Shown {
    pub open: bool,
    pub prices: [u64; 4],
    pub cash: [u64; 2],
    pub when: String,
    pub errors: [Option<&'static str>; 2],
}

/// An order as it was taken, filled or refused: its pocket, its stock from 1,
/// 0 for none picked, and its trade from 1; what the last frame showed of the
/// market, the price, the pocket's cash and its shares of the stock; the
/// shares filled, a sale negative; and why. Since `PLATE.6`, the cash shown's
/// error, where it showed one in place of the cash; and since its second
/// slice the shares the order asked, the fill the desk makes but for the
/// ceiling, and the pocket's cents bought and sold of the stock as the last
/// frame showed them, against which the ceiling is judged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Order {
    pub pocket: u64,
    pub stock: u64,
    pub trade: usize,
    pub open: bool,
    pub price: u64,
    pub cash: u64,
    pub held: u64,
    pub filled: i64,
    pub outcome: Outcome,
    pub error: Option<&'static str>,
    pub asked: i64,
    pub bought: u64,
    pub sold: u64,
}

impl Order {
    /// What Trading says of the order, `A13`, by the stock's name, the hand's
    /// (`MENU.3`): what was bought or sold, or why nothing was.
    pub fn sentence(&self) -> String {
        let name = match self.stock {
            0 => "",
            s => RUNGS[s as usize - 1].name,
        };
        match self.outcome {
            Outcome::Filled => {
                let n = self.filled.unsigned_abs();
                format!(
                    "{} {n} share{} of {name} at the price shown.",
                    if self.filled > 0 { "Bought" } else { "Sold" },
                    if n == 1 { "" } else { "s" }
                )
            }
            Outcome::Closed => "Refused: the market was closed. Orders fill from the opening bell to the closing bell.".to_string(),
            Outcome::NoStock => "Refused: no stock is picked. Press 1 to 4 to pick one, or click a button on its row.".to_string(),
            Outcome::NoCash => format!(
                "Refused: the cash does not cover {} of {name}.",
                if TRADES[self.trade - 1] == Trade::Buy(10) {
                    "10 shares"
                } else {
                    "a share"
                }
            ),
            Outcome::NoShares if self.held == 0 => format!("Refused: no shares of {name} to sell."),
            Outcome::NoShares => format!("Refused: fewer than 10 shares of {name} to sell."),
            Outcome::Uncounted => UNCOUNTED.to_string(),
            Outcome::Ceiling => PAST_THE_CEILING.to_string(),
        }
    }
}

/// A fill, as the ledger keeps it: its number from 1, when, the pocket, the
/// stock from 1, the shares, a sale negative, the price and the pocket's cash
/// after it, in whole cents.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fill {
    pub number: u64,
    pub when: String,
    pub pocket: u64,
    pub stock: u64,
    pub shares: i64,
    pub price: u64,
    pub cash_after: u64,
}

/// A pocket's book of one stock, folded from its fills: the shares, the cost
/// basis at the average cost, and the cents spent buying and received
/// selling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Book {
    pub shares: u64,
    pub basis: u64,
    pub bought: u64,
    pub sold: u64,
}

/// The order desk and the books by hand (`PLATE.1`): the pocket an order
/// funds, which the switch turns (`PLATE.2`), the stock picked, every fill in
/// order and the dividends paid to each pocket's each stock, and the last
/// order taken. The cells keep each book as state over its twin and derive
/// the gain taken from three sums; the hand keeps the fills and folds them, a
/// share's cost averaged sale by sale, and finds a click by matching the
/// pointer to a button's place and a command by its name. Since `PLATE.6`'s
/// second slice, the books' ceiling a person wrote into `Orders!B49`, in whole
/// cents, where they wrote one; none for the cartridge's own, 10^307, which no
/// book of whole cents below 2^64 reaches, so that the hand's books meet the
/// ceiling only where a person lowered it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plate {
    pub pocket: u64,
    pub picked: u64,
    pub fills: Vec<Fill>,
    pub dividends: [[u64; 4]; 2],
    pub last: Option<Order>,
    pub ceiling: Option<u64>,
}

impl Default for Plate {
    fn default() -> Plate {
        Plate {
            pocket: FIRM,
            picked: 0,
            fills: Vec::new(),
            dividends: [[0; 4]; 2],
            last: None,
            ceiling: None,
        }
    }
}

impl Plate {
    /// The pocket an order funds, as the Orders sheet's `B19` reads `B1`: your
    /// own at 2, and the firm's at any other value.
    pub fn funding(&self) -> u64 {
        if self.pocket == PERSONAL {
            PERSONAL
        } else {
            FIRM
        }
    }

    /// The switch clicked this frame (`PLATE.2`): the left button going down
    /// on its cell while Trading shows.
    pub fn switch_clicked(now: &Inputs, last: &Inputs) -> bool {
        clicked(now, last, SWITCH)
    }

    /// Whether this frame switches the pocket: a click on the switch, or its
    /// command while no button of an order is clicked, one act a frame and a
    /// click's first.
    pub fn switches(press: Option<(u64, usize)>, switch: bool, command: &str) -> bool {
        switch || (press.is_none() && command == SWITCH_COMMAND.0)
    }

    /// The button clicked this frame, its stock and its trade, each from 1:
    /// the left button going down on a button's cell while Trading shows.
    pub fn clicked(now: &Inputs, last: &Inputs) -> Option<(u64, usize)> {
        for (s, &row) in STOCK_ROWS.iter().enumerate() {
            for (t, &column) in TRADE_COLUMNS.iter().enumerate() {
                if clicked(now, last, (TRADING, column, row)) {
                    return Some((s as u64 + 1, t + 1));
                }
            }
        }
        None
    }

    /// The stock a command picks, from 1, and the trade a command orders,
    /// from 1, by its name; 0 for a name that does neither.
    pub fn pick_of(command: &str) -> u64 {
        PICKS
            .iter()
            .position(|(name, _)| *name == command)
            .map_or(0, |k| k as u64 + 1)
    }
    pub fn trade_of(command: &str) -> usize {
        TRADE_COMMANDS
            .iter()
            .position(|(name, _)| *name == command)
            .map_or(0, |k| k + 1)
    }

    /// A pocket's book of a stock, each from 1, folded from the fills in their
    /// order: a purchase adds its shares and its cost; a sale takes off its
    /// shares and their share of the cost, rounded half up to whole cents, or
    /// all of it with the last share.
    pub fn book(&self, pocket: u64, stock: u64) -> Book {
        let mut b = Book::default();
        for f in self
            .fills
            .iter()
            .filter(|f| f.pocket == pocket && f.stock == stock)
        {
            let n = f.shares.unsigned_abs();
            if f.shares > 0 {
                b.shares += n;
                b.basis += n * f.price;
                b.bought += n * f.price;
            } else {
                let cost = if n == b.shares {
                    b.basis
                } else {
                    let (whole, held) = (u128::from(b.basis) * u128::from(n), u128::from(b.shares));
                    ((2 * whole + held) / (2 * held)) as u64
                };
                b.shares -= n;
                b.basis -= cost;
                b.sold += n * f.price;
            }
        }
        b
    }

    /// A pocket's cash from its start, the exam's capital for the firm's or
    /// $100 and the hustle's fees for your own: less what its books spent,
    /// with what they received and its dividends.
    pub fn cash(&self, pocket: u64, start: u64) -> u64 {
        let p = (pocket - 1) as usize;
        let (mut spent, mut received) = (0, 0);
        for stock in 1..=4 {
            let b = self.book(pocket, stock);
            spent += b.bought;
            received += b.sold;
        }
        start + received + self.dividends[p].iter().sum::<u64>() - spent
    }

    /// A pocket's holdings, its shares of each stock at its price now.
    pub fn holdings(&self, pocket: u64, prices: [u64; 4]) -> u64 {
        (1..=4)
            .map(|s| self.book(pocket, s).shares * prices[s as usize - 1])
            .sum()
    }

    /// The dividends of a day's first frame, each stock's today a share, paid
    /// to the shares each pocket held at the frame before, before this
    /// frame's order.
    pub fn pay(&mut self, today: [u64; 4]) {
        for pocket in [FIRM, PERSONAL] {
            for stock in 1..=4u64 {
                let held = self.book(pocket, stock).shares;
                self.dividends[pocket as usize - 1][stock as usize - 1] +=
                    held * today[stock as usize - 1];
            }
        }
    }

    /// A frame of the order desk: a click on a button, its stock and its
    /// trade, a click on the switch, and the command run, judged by what the
    /// last frame showed, an order against cash shown as an error refused, a
    /// sale too, after the market closed and the stock unpicked and before any
    /// trade's own reasons (`PLATE.6`). One act a frame, a click's first (`PLATE.2`): a
    /// switch turns the pocket and takes nothing else, no pick and no order. A
    /// click picks its stock, as a pick command does, a click first; the order
    /// is a click's, else a trade command's on the stock picked before this
    /// frame, the one the person saw marked, and none for a frame with
    /// neither. A filled order is a fill, the pocket's cash after it found
    /// from `starts`, each pocket's start at this frame (and this frame's
    /// dividends, paid first). Since `PLATE.6`'s second slice, an order the
    /// desk would fill is refused at the books' ceiling where one is written:
    /// a purchase whose cents take the stock's cents bought past it, a sale
    /// whose cents take its cents sold or the pocket's cash shown past it, each
    /// book as the last frame showed it. The order taken, filled or refused.
    pub fn frame(
        &mut self,
        press: Option<(u64, usize)>,
        switch: bool,
        command: &str,
        shown: &Shown,
        starts: [u64; 2],
    ) -> Option<Order> {
        if Plate::switches(press, switch, command) {
            self.pocket = if self.funding() == PERSONAL {
                FIRM
            } else {
                PERSONAL
            };
            return None;
        }
        let before = self.picked;
        match press {
            Some((stock, _)) => self.picked = stock,
            None => {
                let pick = Plate::pick_of(command);
                if pick > 0 {
                    self.picked = pick;
                }
            }
        }
        let (stock, trade) = match press {
            Some(order) => order,
            None => match Plate::trade_of(command) {
                0 => return None,
                trade => (before, trade),
            },
        };
        let pocket = self.funding();
        let p = (pocket - 1) as usize;
        let price = if stock == 0 {
            0
        } else {
            shown.prices[stock as usize - 1]
        };
        let cash = shown.cash[p];
        let error = shown.errors[p];
        let book = if stock == 0 {
            Book::default()
        } else {
            self.book(pocket, stock)
        };
        let held = book.shares;
        let asked: i64 = if !shown.open || stock == 0 || error.is_some() {
            0
        } else {
            match TRADES[trade - 1] {
                Trade::Buy(n) if n * price <= cash => n as i64,
                Trade::Buy(_) => 0,
                Trade::BuyMax => (cash / price) as i64,
                Trade::Sell(n) if held >= n => -(n as i64),
                Trade::Sell(_) => 0,
                Trade::SellAll => -(held as i64),
            }
        };
        let cents = asked.unsigned_abs() * price;
        let past = asked != 0
            && self.ceiling.is_some_and(|ceiling| {
                if asked > 0 {
                    book.bought + cents > ceiling
                } else {
                    book.sold + cents > ceiling || cash + cents > ceiling
                }
            });
        let filled = if past { 0 } else { asked };
        let outcome = if !shown.open {
            Outcome::Closed
        } else if stock == 0 {
            Outcome::NoStock
        } else if error.is_some() {
            Outcome::Uncounted
        } else if past {
            Outcome::Ceiling
        } else if filled != 0 {
            Outcome::Filled
        } else if trade <= 3 {
            Outcome::NoCash
        } else {
            Outcome::NoShares
        };
        if filled != 0 {
            self.fills.push(Fill {
                number: self.fills.len() as u64 + 1,
                when: shown.when.clone(),
                pocket,
                stock,
                shares: filled,
                price,
                cash_after: 0,
            });
            let after = self.cash(pocket, starts[p]);
            if let Some(fill) = self.fills.last_mut() {
                fill.cash_after = after;
            }
        }
        let order = Order {
            pocket,
            stock,
            trade,
            open: shown.open,
            price,
            cash,
            held,
            filled,
            outcome,
            error,
            asked,
            bought: book.bought,
            sold: book.sold,
        };
        self.last = Some(order);
        Some(order)
    }

    /// The fill a ledger's row holds, of the first `fills` fills: the last
    /// whose number falls on that row of the ring, `MOD(n-1,100)+2`.
    pub fn ledger(&self, row: u64, fills: usize) -> Option<&Fill> {
        self.fills[..fills]
            .iter()
            .rev()
            .find(|f| (f.number - 1) % LEDGER_ROWS + 2 == row)
    }
}

/// Trading's line on where an order's money comes from, `A3`; and the
/// Portfolio's on whose holdings it shows, its `A3`: the pocket an order funds.
pub fn trading_line(pocket: u64) -> &'static str {
    if pocket == PERSONAL {
        "Orders fill at the price shown, while the market is open, from your own cash."
    } else {
        "Orders fill at the price shown, while the market is open, from the firm's cash, which is to say other people's money."
    }
}
pub fn holdings_line(pocket: u64) -> &'static str {
    if pocket == PERSONAL {
        "Your own holdings, at the price now."
    } else {
        "The firm's holdings, at the price now."
    }
}

/// The words of the Commands sheet's rows from 2, a command's sentence the
/// palette shows: each pick's by its stock's name, the hand's (`MENU.3`), each
/// trade's by the stock picked's, or none yet, and the switch's by the pocket
/// it turns to, the other than `pocket`'s (`PLATE.2`).
pub fn command_words(picked: u64, pocket: u64) -> Vec<String> {
    let stock = match picked {
        0 => "the stock picked, none yet",
        p => RUNGS[p as usize - 1].name,
    };
    let mut out: Vec<String> = RUNGS
        .iter()
        .map(|r| format!("Pick {}: the buy and sell commands trade it", r.name))
        .collect();
    out.extend([
        format!("Buy 1 share of {stock}"),
        format!("Buy 10 shares of {stock}"),
        format!("Buy as many shares of {stock} as the cash covers"),
        format!("Sell 1 share of {stock}"),
        format!("Sell 10 shares of {stock}"),
        format!("Sell every share of {stock}"),
        format!(
            "Switch to {} cash: the buy and sell commands spend it",
            if pocket == PERSONAL {
                "the firm's"
            } else {
                "your own"
            }
        ),
    ]);
    out
}

/// The view a card is shown on (`CRITIC.1`), the Performance sheet's `B1`,
/// which the Camera names at each quarter's turn.
pub const CARD_VIEW: &str = "Report";

/// The last day of each quarter of the year, from 1 (`CRITIC.1`): 91, 182,
/// 273 and 365, so quarters of 91, 91, 91 and 92 days, listed here where the
/// Day sheet divides the day of the year by the year's days over four.
pub const QUARTER_ENDS: [u64; 4] = [91, 182, 273, 365];

/// The quarter of a day of the year, from 1: the first whose last day it does
/// not pass.
pub fn quarter_of(day_of_year: u64) -> u64 {
    QUARTER_ENDS
        .iter()
        .position(|&end| day_of_year <= end)
        .map_or(QUARTER_ENDS.len() as u64, |k| k as u64 + 1)
}

impl Calendar {
    /// The quarter of the calendar's day, `Day!B33`, and the quarters counted
    /// from the game's first, `B34`: the years before this one's four each,
    /// and this year's.
    pub fn quarter(&self) -> u64 {
        quarter_of(self.day_of_year)
    }
    pub fn quarters(&self) -> u64 {
        (self.year - 1) * QUARTER_ENDS.len() as u64 + self.quarter()
    }
}

/// The broad rung the card grades against (`CRITIC.1`): Average Co., the
/// third, the market itself.
pub const BENCHMARK: usize = 2;

/// A return in whole hundredths of a percent, from a worth at the start to one
/// at the end, each in whole cents: the gain times 10,000 over the start,
/// rounded half up, in whole numbers. The cell divides two doubles and rounds
/// down, `INT(((e-s)*20000+s)/(2*s))`; the two agree at every worth whose
/// numerator stays below 2^53 (*derived*: both are whole, so a double holds
/// each exactly; and the quotient of two whole numbers, if not whole, stands
/// at least one over the divisor below the next whole number, while a
/// double's rounding of it moves it less than the quotient over 2^53, so its
/// floor is the true one).
pub fn points(start: u64, end: u64) -> i64 {
    let (s, e) = (i128::from(start), i128::from(end));
    ((e - s) * 20_000 + s).div_euclid(2 * s) as i64
}

/// The grades (`CRITIC.1`), the Performance sheet's `A25:B29`: each letter
/// beside the alpha it begins at, in whole hundredths of a point, from the top,
/// and F below them all: A from two points ahead of the market, B from half a
/// point, C within half a point either way, D to two points behind.
pub const GRADES: [(&str, i64); 4] = [("A", 200), ("B", 50), ("C", -50), ("D", -200)];
pub const GRADE_BELOW: &str = "F";

/// A grade for an alpha in hundredths of a point: its place, 1 for A to 5 for
/// F, and its letter. The cells compare the alpha with each grade's from the
/// top and take the first it reaches; the hand counts the grades it reaches
/// and reads the table from the bottom by that count.
pub fn grade(alpha: i64) -> (u64, &'static str) {
    let reached = GRADES.iter().filter(|(_, from)| alpha >= *from).count();
    if reached == 0 {
        (GRADES.len() as u64 + 1, GRADE_BELOW)
    } else {
        let k = GRADES.len() - reached;
        (k as u64 + 1, GRADES[k].0)
    }
}

/// Whole hundredths of a percent as a view draws them, the library's `percent`:
/// a gain's or nothing's `+`, a loss's `-`, the whole percent, a point, two
/// digits and the percent sign. The cell divides by 10,000 under a format's
/// code; the hand spells the whole number's digits.
pub fn percent(points: i64) -> String {
    let sign = if points < 0 { "-" } else { "+" };
    let p = points.unsigned_abs();
    format!("{sign}{}.{:02}%", p / 100, p % 100)
}

/// A turn of the quarter, as the hand keeps it: the quarter that ended and its
/// year, each from 1, and the worth of the firm and of the benchmark bought and
/// held at the turn, in whole cents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Turn {
    pub quarter: u64,
    pub year: u64,
    pub worth: [u64; 2],
}

/// The report card by hand (`CRITIC.1`): the benchmark bought at the begin, and
/// every worth the card reads, the begin's and each turn's, kept in a list. The
/// cells hold the last two of each kind over their twins, at the last card and
/// the one before, at the year's start and the last year's, keyed on the
/// calendar's turn; the hand keeps every turn in order and reads a card off the
/// list, a quarter's start the turn before it and a year's start the turn that
/// closed the year before, by search.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cards {
    /// Average Co.'s shares the firm's capital buys at the begin's price, and
    /// the cents left over, kept as cash.
    pub shares: u64,
    pub left: u64,
    /// The worth of the firm and of the benchmark at the begin.
    pub begin: [u64; 2],
    pub turns: Vec<Turn>,
    /// The quarters counted at the frame last played, none before the first.
    pub counted: Option<u64>,
}

/// A card, read off the hand's list: its quarter and year; the worth at the
/// card, at its quarter's start and at its year's start, the firm's first; each
/// return in whole hundredths of a percent, over the quarter and over the year
/// so far, the firm's and the benchmark's; each period's alpha and grade; and
/// the grade the proctor remarks on, the year's at a fourth quarter's card.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub quarter: u64,
    pub year: u64,
    pub worth: [u64; 2],
    pub quarter_start: [u64; 2],
    pub year_start: [u64; 2],
    /// `[period][who]`: the quarter's and the year so far's, the firm's and
    /// the benchmark's.
    pub returns: [[i64; 2]; 2],
    pub alpha: [i64; 2],
    pub grades: [(u64, &'static str); 2],
    pub remark: u64,
}

impl Cards {
    /// The benchmark's worth at a price of Average Co., in whole cents: its
    /// shares at the price and the cents left over.
    pub fn benchmark(&self, price: u64) -> u64 {
        self.shares * price + self.left
    }

    /// A frame of the card's books after it is played: until the frame the game
    /// begins, the benchmark bought again with the capital so far at the price
    /// now, and the begin's worth the frame's; then, when the quarters counted
    /// moved since the frame before, the game's first frame excepted, a turn,
    /// the quarter that ended the one the frame before stood in, at the firm's
    /// worth now and the benchmark's. `begun_before`, whether the game had begun
    /// by the frame before; `capital`, the exam's; `price`, Average Co.'s now;
    /// `firm`, the firm's net worth now. Whether a card was issued.
    pub fn frame(
        &mut self,
        cal: &Calendar,
        begun_before: bool,
        capital: u64,
        price: u64,
        firm: u64,
    ) -> bool {
        if !begun_before {
            self.shares = capital / price;
            self.left = capital - self.shares * price;
            self.begin = [firm, self.benchmark(price)];
        }
        let now = cal.quarters();
        let turned = matches!(self.counted, Some(was) if was != now);
        if turned {
            let was = self.counted.unwrap_or(1);
            let per_year = QUARTER_ENDS.len() as u64;
            self.turns.push(Turn {
                quarter: (was - 1) % per_year + 1,
                year: (was - 1) / per_year + 1,
                worth: [firm, self.benchmark(price)],
            });
        }
        self.counted = Some(now);
        turned
    }

    /// The worth at the last turn, or at the begin before the first; and at the
    /// turn before it, or at the begin at the first card, or none before it.
    pub fn last(&self) -> [u64; 2] {
        self.turns.last().map_or(self.begin, |t| t.worth)
    }
    pub fn before(&self) -> Option<[u64; 2]> {
        match self.turns.len() {
            0 => None,
            1 => Some(self.begin),
            n => Some(self.turns[n - 2].worth),
        }
    }

    /// The worth at the year's start, the turn that closed the last fourth
    /// quarter or the begin; and at the year's start before it, or none before
    /// a year has closed.
    pub fn year_start(&self) -> [u64; 2] {
        self.turns
            .iter()
            .rev()
            .find(|t| t.quarter == 4)
            .map_or(self.begin, |t| t.worth)
    }
    pub fn last_year_start(&self) -> Option<[u64; 2]> {
        let mut years = self.turns.iter().rev().filter(|t| t.quarter == 4);
        years.next()?;
        Some(years.next().map_or(self.begin, |t| t.worth))
    }

    /// The card issued last, none before the first turn.
    pub fn card(&self) -> Option<Card> {
        let n = self.turns.len();
        let t = self.turns.last()?;
        let quarter_start = if n == 1 {
            self.begin
        } else {
            self.turns[n - 2].worth
        };
        let year_start = self.turns[..n - 1]
            .iter()
            .rev()
            .find(|u| u.quarter == 4 && u.year + 1 == t.year)
            .map_or(self.begin, |u| u.worth);
        let returns = [quarter_start, year_start]
            .map(|start| [points(start[0], t.worth[0]), points(start[1], t.worth[1])]);
        let alpha = returns.map(|r| r[0] - r[1]);
        let grades = alpha.map(grade);
        let remark = if t.quarter == 4 {
            grades[1].0
        } else {
            grades[0].0
        };
        Some(Card {
            quarter: t.quarter,
            year: t.year,
            worth: t.worth,
            quarter_start,
            year_start,
            returns,
            alpha,
            grades,
            remark,
        })
    }
}

/// The sheet the game shows at the frame last played, the Camera's `B3`
/// (`CRITIC.1`): the Report at a card's turn once the game has begun, and
/// otherwise the exam's choice, [`Exam::shows`].
pub fn camera(exam: &Exam, card: bool) -> &'static str {
    if exam.begun && card {
        CARD_VIEW
    } else {
        exam.shows()
    }
}

/// The Report's lines (`CRITIC.1`): the card's header for a firm, by its name
/// or pending registration, and the market it was graded on; its title, which
/// quarter of which year, or when the first card comes; the header of its last
/// column; the grades in a sentence; the remark's lead; and the line on your
/// own account.
pub const NO_CARD: &str =
    "No card yet: the first is issued when the first quarter ends, after its 91st day.";
pub const QUARTER_NAMES: [&str; 4] = ["first", "second", "third", "fourth"];
pub const OWN_ACCOUNT: &str = "Your own account is not graded: the proctor grades the firm's book, and your own money is your own business.";
pub const ALPHA_LABEL: &str = "Alpha, the difference";
pub const CARD_HEADERS: [&str; 2] = ["Worth ($)", "The quarter"];
pub const REMARK_LEAD: &str = "The proctor remarks: ";

pub fn card_header(firm: Option<&str>, seed: u64, tier: u64) -> String {
    format!(
        "Report card for {}, graded by the proctor at seed {seed} and tier {tier}.",
        firm.unwrap_or("a firm pending registration")
    )
}
pub fn card_title(card: Option<&Card>) -> String {
    match card {
        None => NO_CARD.to_string(),
        Some(c) => format!(
            "The {} quarter of year {}, to its last close.",
            QUARTER_NAMES[c.quarter as usize - 1],
            c.year
        ),
    }
}
pub fn year_header(card: Option<&Card>) -> &'static str {
    if card.is_some_and(|c| c.quarter == 4) {
        "The year"
    } else {
        "The year so far"
    }
}
pub fn grade_line(card: Option<&Card>) -> String {
    match card {
        None => String::new(),
        Some(c) => format!(
            "Grade: {} for the quarter, and {} for the {}.",
            c.grades[0].1,
            c.grades[1].1,
            if c.quarter == 4 {
                "year"
            } else {
                "year so far"
            }
        ),
    }
}

// ---- PLATE.6: the limit ----------------------------------------------------------

/// The error a sum past what a double holds becomes (`PLATE.6`), and the error
/// a text where a number goes becomes, by their names as a cell draws them.
pub const NUM: &str = "#NUM!";
pub const VALUE: &str = "#VALUE!";

/// The largest number a double holds, 1.7976931348623157E+308, as its
/// seventeen significant digits and their power of ten: the end of the numbers
/// (`BUFFET.md`, "Numbers"). A number is past it where it is past the half of
/// the last place above it, where a double rounds to no number at all, which
/// in seventeen digits is past 1.7976931348623158E+308.
pub const END: (u64, u32) = (17_976_931_348_623_157, 292);

/// What a person writes into a number's cell of the model (`PLATE.6`): a
/// number, the digits of its significand and its power of ten, `1E+308` being
/// 1 and 308, at most seventeen digits, the most a double's shortest spelling
/// needs; or the empty text, which Delete writes, the page's language writing
/// no empty cell (Alonzo's `cellClearRow`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Typed {
    Number { digits: u64, power: u32 },
    Emptied,
}

impl Typed {
    /// What the person types for it: the digits, then an E and the power where
    /// there is one, or the empty text in quotes.
    pub fn typed(self) -> String {
        match self {
            Typed::Number { digits, power: 0 } => digits.to_string(),
            Typed::Number { digits, power } => format!("{digits}E+{power}"),
            Typed::Emptied => "\"\"".to_string(),
        }
    }

    /// The row the page writes for it into a cell (Alonzo's `cellEditRow`).
    pub fn row(self, (sheet, addr): (&str, &str)) -> String {
        format!("(cell \"{sheet}\" \"{addr}\" {})", self.typed())
    }

    /// The cell as the record spells it: a number in full, its digits and
    /// then a zero a power, as the shortest round trip of a whole number
    /// past ten to the fifteenth prints; the empty text as nothing.
    pub fn spelled(self) -> String {
        match self {
            Typed::Number { digits, power } => format!("{digits}{}", "0".repeat(power as usize)),
            Typed::Emptied => String::new(),
        }
    }
}

/// A number's places before the point against the end's, and where it has as
/// many, its first seventeen digits: digits times ten to a power, counted digit
/// by digit and never in a double.
fn beside_the_end(digits: u64, power: u32) -> (std::cmp::Ordering, Option<u64>) {
    let places = |d: u64| d.to_string().len() as u32;
    assert!(places(digits) <= 17, "{digits}: at most seventeen digits");
    let (end, end_power) = END;
    if digits == 0 {
        return (std::cmp::Ordering::Less, None);
    }
    match (places(digits) + power).cmp(&(places(end) + end_power)) {
        std::cmp::Ordering::Equal => (
            std::cmp::Ordering::Equal,
            Some(digits * 10u64.pow(17 - places(digits))),
        ),
        other => (other, None),
    }
}

/// Whether digits times ten to a power are past the end of the numbers: more
/// places before the point than the end has, or as many and its seventeen
/// digits past the end's half place, the end's seventeen and one more being
/// the last a double reads back as the end.
pub fn past_the_end(digits: u64, power: u32) -> bool {
    match beside_the_end(digits, power) {
        (std::cmp::Ordering::Equal, Some(seventeen)) => seventeen > END.0 + 1,
        (places, _) => places == std::cmp::Ordering::Greater,
    }
}

/// The firm's capital, `Proctor!B39`, with a correct answer's worth a person
/// wrote into `Proctor!B2` (`PLATE.6`): the capital before the exam and the
/// score's answers at that worth, in whole cents below 2^64; past the end of
/// the numbers, `#NUM!`; and the empty text where a number goes, `#VALUE!`.
/// The cell multiplies the double nearest the worth, so the hand judges no
/// capital within eight of the end's seventeenth digits of it, where that
/// double's own rounding and the product's would decide.
pub fn capital_with(a_right: Typed, score: u64) -> Result<u64, &'static str> {
    match a_right {
        Typed::Emptied => Err(VALUE),
        Typed::Number { digits, power } => {
            let answers = digits
                .checked_mul(score)
                .expect("a worth's digits times the score, below 2^64");
            if answers == 0 {
                return Ok(CAPITAL_BEFORE);
            }
            if let (_, Some(seventeen)) = beside_the_end(answers, power) {
                assert!(
                    seventeen.abs_diff(END.0 + 1) > 8,
                    "{answers}E+{power}: within eight of the end's seventeenth digit, which the hand does not judge"
                );
            }
            if past_the_end(answers, power) {
                return Err(NUM);
            }
            let cents = 10u64
                .checked_pow(power)
                .and_then(|ten| answers.checked_mul(ten))
                .and_then(|cents| cents.checked_add(CAPITAL_BEFORE))
                .expect("the hand counts a capital below 2^64 cents, or past the end");
            Ok(cents)
        }
    }
}

/// Where the firm's capital is the cell a person wrote (`PLATE.6`): the
/// Proctor's correct answer's worth, `B2`, which the capital multiplies by the
/// score.
pub const A_RIGHT: (&str, &str) = ("Proctor", "B2");

/// The model's cells an error of the firm's capital reaches, by the hand's
/// reading of the books: the capital; the firm's cash, the capital and its
/// books' flows; its net worth, the cash and the holdings; and the report
/// card's worth now, the net worth. The holds of the card take it only at a
/// turn, and the benchmark's only before the begin, neither of which an edit
/// after the begin and undone within the quarter meets.
pub const FIRM_BOOKS: [(&str, &str); 4] = [
    ("Proctor", "B39"),
    ("Cash", "B3"),
    ("Cash", "B9"),
    ("Performance", "B13"),
];
/// The views' figures of them, each beside the cell of its joke: the
/// Certificate's capital, the Overview's net worth, and the Portfolio's cash
/// on hand and net worth, the firm's rows.
pub const FIRM_FIGURES: [(&str, &str, &str); 4] = [
    (CERTIFICATE, "B8", "C8"),
    (OVERVIEW, "B6", "C6"),
    ("Portfolio", "B11", "C11"),
    ("Portfolio", "B15", "C15"),
];
/// Where it lands while the firm's pocket funds the order desk: the pocket's
/// cash now and the Write sheet's value of a fill's cash after, which writes
/// nothing while no order fills; and Trading's cash available, its joke under
/// it, the switch standing beside it.
pub const FUNDING: [(&str, &str); 2] = [("Orders", "B31"), ("Write", "C8")];
pub const FUNDING_FIGURE: (&str, &str, &str) = (TRADING, "C11", "C12");
/// And a frame later, while the firm's pocket funds it, the cash shown, the
/// last frame's, by which an order is judged.
pub const CASH_SHOWN: (&str, &str) = ("Orders", "B24");
/// The error whose joke a view shows, the Errors sheet's row shown, its first:
/// the error past the end of the numbers.
pub const ERROR_SHOWN: &str = NUM;

/// Every cell an error of the firm's capital holds at a frame (`PLATE.6`), each
/// with what it holds as the record spells it: `error`, the capital's error at
/// this frame; `shown`, its error at the last frame; `funding`, the pocket an
/// order funds at this frame; and `joke`, the Errors sheet's joke of the error
/// it shows. A figure holds the error by its name, and the cell beside it the
/// joke where the error is the one the Errors sheet shows, else the empty text.
/// Every other cell of the game holds what it holds with no edit at all.
pub fn landed(
    error: Option<&'static str>,
    shown: Option<&'static str>,
    funding: u64,
    joke: &str,
) -> Vec<(&'static str, &'static str, String)> {
    let spelled = |e: &str| format!("(error \"{e}\")");
    let joke_of = |e: &str| {
        if e == ERROR_SHOWN {
            joke.to_string()
        } else {
            String::new()
        }
    };
    let mut out = Vec::new();
    if let Some(e) = error {
        out.extend(FIRM_BOOKS.map(|(sheet, addr)| (sheet, addr, spelled(e))));
        let mut figures = FIRM_FIGURES.to_vec();
        if funding == FIRM {
            out.extend(FUNDING.map(|(sheet, addr)| (sheet, addr, spelled(e))));
            figures.push(FUNDING_FIGURE);
        }
        for (sheet, figure, beside) in figures {
            out.push((sheet, figure, spelled(e)));
            out.push((sheet, beside, joke_of(e)));
        }
    }
    if let (Some(e), FIRM) = (shown, funding) {
        out.push((CASH_SHOWN.0, CASH_SHOWN.1, spelled(e)));
    }
    out
}

// ---- PLATE.6's second slice: the books' ceiling ------------------------------------

/// The books' ceiling (`PLATE.6`'s second slice): the Orders sheet's `B49`, a
/// setting, in cents, 10^307, below the end of the numbers by a factor of
/// nearly eighteen. Every book of the order desk stays under it, so that each
/// sum the cash takes of four books is a number, and so is ten times a cost
/// basis, the share of it ten shares sold take before its division.
pub const CEILING: Typed = Typed::Number {
    digits: 1,
    power: 307,
};
pub const CEILING_CELL: (&str, &str) = ("Orders", "B49");
/// The books a sum of the cash takes of one kind, a pocket's four stocks; and
/// the most shares a sale takes a share of the basis for, the rest being all
/// of it with the last share.
pub const BOOKS_SUMMED: u64 = 4;
pub const SHARES_OF_A_BASIS: u64 = 10;

#[cfg(test)]
mod tests {
    use super::*;

    /// The end of the numbers is a double's (`PLATE.6`): the hand's count of
    /// places and digits says past exactly where the platform's own reading of
    /// the same decimal is no number, at the end, a place either side of its
    /// seventeenth digit's half, at the powers of ten around it, and at the
    /// capital's own products.
    #[test]
    fn the_end_of_the_numbers_is_where_a_double_ends() {
        for (digits, power) in [
            (17_976_931_348_623_157, 292),
            (17_976_931_348_623_156, 292),
            (17_976_931_348_623_160, 292),
            (17_976_931_348_623_199, 292),
            (1, 308),
            (5, 308),
            (17, 307),
            (18, 307),
            (1, 309),
            (9_999_999_999_999_999, 292),
            (1, 307),
            (0, 400),
            (250_000, 0),
        ] {
            let read: f64 = format!("{digits}E{power}").parse().expect("a decimal");
            assert_eq!(
                past_the_end(digits, power),
                read.is_infinite(),
                "{digits}E+{power}, read as {read}"
            );
        }
        assert_eq!(
            f64::MAX,
            1.797_693_134_862_315_7e308,
            "the end, as END spells it"
        );
    }

    /// The firm's capital at a correct answer's worth a person wrote: the
    /// contract's at 50,000 cents; past the end at 1E+308 and any score from
    /// two, and at one already, 1E+308 itself being counted; the capital before
    /// the exam at a score of none whatever the worth; and the error of a text
    /// where Delete leaves the empty one.
    #[test]
    fn a_worth_past_the_end_makes_the_capital_num() {
        let worth = |digits, power| Typed::Number { digits, power };
        assert_eq!(capital_with(worth(50_000, 0), 5), Ok(750_000));
        assert_eq!(capital_with(worth(5, 4), 10), Ok(1_000_000));
        assert_eq!(capital_with(worth(1, 308), WAIVED_SCORE), Err(NUM));
        assert_eq!(capital_with(worth(1, 308), 2), Err(NUM));
        assert_eq!(capital_with(worth(1, 308), 0), Ok(CAPITAL_BEFORE));
        assert_eq!(capital_with(Typed::Emptied, WAIVED_SCORE), Err(VALUE));
        assert!(
            !past_the_end(1, 308),
            "1E+308 itself a number: only the sum passes the end"
        );
        assert_eq!(worth(1, 308).typed(), "1E+308");
        assert_eq!(worth(1, 308).spelled().len(), 309);
        assert_eq!(worth(50_000, 0).spelled(), "50000");
        assert_eq!(
            Typed::Emptied.row(A_RIGHT),
            "(cell \"Proctor\" \"B2\" \"\")"
        );
    }

    /// The error's landing by hand: nothing with no error; the books, the
    /// figures and their jokes with one, and the order desk's and Trading's
    /// while the firm's pocket funds; the cash shown a frame behind; and the
    /// jokes empty beside an error the Errors sheet does not show.
    #[test]
    fn an_error_lands_on_the_firms_books_and_beside_its_figures() {
        assert!(landed(None, None, FIRM, "a joke").is_empty());
        let at = |l: &[(&str, &str, String)], sheet: &str, addr: &str| {
            l.iter()
                .find(|(s, a, _)| *s == sheet && *a == addr)
                .map(|(_, _, v)| v.clone())
        };
        let firm = landed(Some(NUM), Some(NUM), FIRM, "a joke");
        assert_eq!(firm.len(), 4 + 2 + 2 * 5 + 1);
        assert_eq!(at(&firm, "Cash", "B9"), Some("(error \"#NUM!\")".into()));
        assert_eq!(at(&firm, TRADING, "C12"), Some("a joke".into()));
        assert_eq!(at(&firm, "Orders", "B24"), Some("(error \"#NUM!\")".into()));
        let own = landed(Some(NUM), Some(NUM), PERSONAL, "a joke");
        assert_eq!(own.len(), 4 + 2 * 4);
        assert_eq!(at(&own, TRADING, "C11"), None);
        assert_eq!(at(&own, "Orders", "B24"), None);
        let late = landed(None, Some(NUM), FIRM, "a joke");
        assert_eq!(late, [("Orders", "B24", "(error \"#NUM!\")".to_string())]);
        let text = landed(Some(VALUE), None, FIRM, "a joke");
        assert_eq!(
            at(&text, OVERVIEW, "B6"),
            Some("(error \"#VALUE!\")".into())
        );
        assert_eq!(at(&text, OVERVIEW, "C6"), Some(String::new()));
    }

    /// An order against cash shown as an error, by hand (`PLATE.6`): a buy and
    /// a sale alike refused, the sixth outcome, its sentence, no fill and every
    /// balance as it was; the market closed and no stock picked still their
    /// own reasons first; your own pocket's cash, a number, filling; and the
    /// same buy filling once the cash shows a number again.
    #[test]
    fn an_order_against_an_error_is_refused_and_moves_no_balance() {
        let mut shown = Shown {
            open: true,
            prices: [100, 500, 2_500, 10_000],
            cash: [750_000, 10_000],
            when: "Monday, year 1, day 1, 9:30".to_string(),
            errors: [None, None],
        };
        let starts = [750_000, 10_000];
        let mut p = Plate::default();
        assert_eq!(
            p.frame(Some((1, 1)), false, "", &shown, starts)
                .map(|o| o.filled),
            Some(1)
        );
        shown.errors = [Some(NUM), None];
        let before = balances(&p, starts);
        for (press, command) in [(Some((1, 1)), ""), (None, "sell-all"), (None, "buy-max")] {
            let o = p
                .frame(press, false, command, &shown, starts)
                .expect("an order");
            assert_eq!(
                (o.outcome, o.filled, o.error),
                (Outcome::Uncounted, 0, Some(NUM))
            );
            assert_eq!(o.sentence(), UNCOUNTED);
            assert_eq!(balances(&p, starts), before, "{command:?} moved a balance");
        }
        shown.open = false;
        assert_eq!(
            p.frame(None, false, "buy-1", &shown, starts)
                .map(|o| o.outcome),
            Some(Outcome::Closed)
        );
        shown.open = true;
        let mut unpicked = Plate::default();
        assert_eq!(
            unpicked
                .frame(None, false, "buy-1", &shown, starts)
                .map(|o| o.outcome),
            Some(Outcome::NoStock)
        );
        p.pocket = PERSONAL;
        assert_eq!(
            p.frame(None, false, "buy-1", &shown, starts)
                .map(|o| (o.outcome, o.error)),
            Some((Outcome::Filled, None))
        );
        p.pocket = FIRM;
        shown.errors = [None, None];
        assert_eq!(
            p.frame(None, false, "buy-1", &shown, starts)
                .map(|o| o.outcome),
            Some(Outcome::Filled)
        );
    }

    /// The books' ceiling where it stands (`PLATE.6`'s second slice), counted
    /// by digits: 10^307, the page's row for it, and the record's spelling;
    /// under the end of the numbers four books of it summed and ten times a
    /// basis of it, and past the end eighteen of it.
    #[test]
    fn the_ceiling_leaves_every_sum_of_the_books_a_number() {
        assert_eq!(
            CEILING.row(CEILING_CELL),
            "(cell \"Orders\" \"B49\" 1E+307)"
        );
        assert_eq!(CEILING.spelled(), format!("1{}", "0".repeat(307)));
        let Typed::Number { digits, power } = CEILING else {
            panic!("a number")
        };
        for times in [BOOKS_SUMMED, SHARES_OF_A_BASIS, 17] {
            assert!(
                !past_the_end(digits * times, power),
                "{times} times the ceiling"
            );
        }
        assert!(
            past_the_end(digits * 18, power),
            "eighteen times the ceiling"
        );
    }

    /// An order at the books' ceiling, by hand (`PLATE.6`'s second slice): with
    /// none written, the cartridge's own, nothing refused; then a ceiling of a
    /// person's: a purchase taking the stock's cents bought to it filling, and
    /// one share more refused, the seventh outcome, its sentence, no fill and
    /// every balance as it was; a sale whose cents and the cash shown pass it
    /// refused, the cash alone deciding; a sale under it filling; and one whose
    /// cents take the cents sold past it refused, the sold alone deciding.
    #[test]
    fn an_order_past_the_ceiling_is_refused_and_moves_no_balance() {
        let shown = Shown {
            open: true,
            prices: [100, 500, 2_500, 10_000],
            cash: [750_000, 10_000],
            when: "Monday, year 1, day 1, 9:30".to_string(),
            errors: [None, None],
        };
        let starts = [750_000, 10_000];
        let mut p = Plate::default();
        assert_eq!(
            p.frame(Some((1, 3)), false, "", &shown, starts)
                .map(|o| (o.outcome, o.filled)),
            Some((Outcome::Filled, 7_500)),
            "no ceiling written: the cartridge's own, which no book reaches"
        );
        p.frame(None, false, "sell-all", &shown, starts);
        let bought = p.book(FIRM, 1).bought;
        p.ceiling = Some(bought + 1_000);
        let o = p
            .frame(None, false, "buy-10", &shown, starts)
            .expect("an order");
        assert_eq!(
            (o.outcome, o.asked, o.filled, o.bought),
            (Outcome::Filled, 10, 10, bought),
            "to the ceiling and no further: filled"
        );
        let before = balances(&p, starts);
        let o = p
            .frame(None, false, "buy-1", &shown, starts)
            .expect("an order");
        assert_eq!(
            (o.outcome, o.asked, o.filled, o.bought),
            (Outcome::Ceiling, 1, 0, bought + 1_000)
        );
        assert_eq!(o.sentence(), PAST_THE_CEILING);
        assert_eq!(
            balances(&p, starts),
            before,
            "a purchase refused moved a balance"
        );
        // The second stock, never sold: ten of it under the ceiling; then the
        // firm's cash shown, $7,500, past a ceiling of $1,000, so no sale.
        assert_eq!(
            p.frame(Some((2, 2)), false, "", &shown, starts)
                .map(|o| o.outcome),
            Some(Outcome::Filled)
        );
        let before = balances(&p, starts);
        p.ceiling = Some(100_000);
        let o = p
            .frame(None, false, "sell-1", &shown, starts)
            .expect("an order");
        assert_eq!((o.outcome, o.asked, o.filled), (Outcome::Ceiling, -1, 0));
        assert!(o.sold + 500 <= 100_000, "the cash alone decides");
        assert_eq!(
            balances(&p, starts),
            before,
            "a sale refused moved a balance"
        );
        // Your own, the second stock at $5: twenty bought, sold and bought
        // again, then a ceiling at the cents sold and ten shares more.
        p.pocket = PERSONAL;
        let mut mine = shown.clone();
        for command in ["buy-max", "sell-all", "buy-max"] {
            mine.cash[1] = p.cash(PERSONAL, starts[1]);
            p.frame(None, false, command, &mine, starts);
        }
        let sold = p.book(PERSONAL, 2).sold;
        assert_eq!(sold, 10_000);
        p.ceiling = Some(sold + 5_000);
        mine.cash[1] = p.cash(PERSONAL, starts[1]);
        let o = p
            .frame(None, false, "sell-10", &mine, starts)
            .expect("an order");
        assert_eq!(
            (o.outcome, o.filled),
            (Outcome::Filled, -10),
            "the cents sold to the ceiling: filled"
        );
        mine.cash[1] = p.cash(PERSONAL, starts[1]);
        let o = p
            .frame(None, false, "sell-10", &mine, starts)
            .expect("an order");
        assert_eq!(
            (o.outcome, o.filled, o.sold),
            (Outcome::Ceiling, 0, sold + 5_000)
        );
        assert!(
            mine.cash[1] + 5_000 <= sold + 5_000,
            "the cents sold alone decide"
        );
    }

    /// The record's spelling of a view's money is the hand's (`FRONT.1`): a
    /// number as its shortest round trip, Rust's own `{}` (Alonzo's `SPEC.md`
    /// section 3.1), of whole cents over 100 is their dollars by text, for
    /// every count of cents from -200,000 to 2,000,000 and beside every power
    /// of two to 2^52 cents, past $45 trillion: below 2^46 dollars two doubles
    /// stand closer than a cent, so no shorter spelling names the same double
    /// (`MISE.3`, *derived*).
    #[test]
    fn whole_cents_over_100_spell_their_own_dollars() {
        let spelled = |k: i64| {
            assert_eq!(format!("{}", k as f64 / 100.0), dollars(k), "{k} cents");
        };
        for k in -200_000..=2_000_000 {
            spelled(k);
        }
        for p in 21..=52 {
            for d in -101..=101 {
                spelled((1i64 << p) + d);
                spelled(-(1i64 << p) - d);
            }
        }
        assert_eq!(
            [dollars(1230), dollars(2500), dollars(-3), dollars(0)],
            ["12.3", "25", "-0.03", "0"]
        );
    }

    /// The views' dollars since `POT.10`'s hand test, two decimals, their
    /// thousands grouped by commas since `GUARD.1`'s, and a change's sign. Each
    /// spells its cents back, its groups three digits after the first's one to
    /// three, for every count of cents to $10,000, every change to $1,000 either
    /// way, and beside every power of ten to 10^15 cents.
    #[test]
    fn whole_cents_spell_two_decimals() {
        assert_eq!(
            [0, 7, 999, 1_000, 65_536, 1_234_567].map(grouped),
            ["0", "7", "999", "1,000", "65,536", "1,234,567"]
        );
        assert_eq!(
            [100, 499, 2_498, 10_000, 0, 100_000, 99_999_999, 100_000_000].map(two_decimals),
            [
                "1.00",
                "4.99",
                "24.98",
                "100.00",
                "0.00",
                "1,000.00",
                "999,999.99",
                "1,000,000.00"
            ]
        );
        assert_eq!(
            two_decimals((1 << 53) - 1),
            "90,071,992,547,409.91",
            "2^53 cents less one, the most a double holds exactly"
        );
        assert_eq!(
            [17, -2, 0, -1_234, -123_456, 100_000_000].map(signed_two_decimals),
            [
                "+0.17",
                "-0.02",
                "+0.00",
                "-12.34",
                "-1,234.56",
                "+1,000,000.00"
            ]
        );
        // The cents a spelling reads back, from its dollars, each comma three
        // digits before the point or the next comma and the first group one
        // to three, and its two digits of cents, read as whole numbers.
        let cents = |text: &str| -> u64 {
            let (whole, part) = text.split_once('.').expect("a point");
            assert_eq!(part.len(), 2, "{text:?}: two digits of cents");
            let groups: Vec<&str> = whole.split(',').collect();
            assert!(
                (1..=3).contains(&groups[0].len()) && groups[1..].iter().all(|g| g.len() == 3),
                "{text:?}: the thousands grouped"
            );
            groups.concat().parse::<u64>().expect("dollars") * 100
                + part.parse::<u64>().expect("cents")
        };
        for c in 0..1_000_000u64 {
            assert_eq!(cents(&two_decimals(c)), c, "{c} cents");
        }
        for p in 0..=15 {
            let ten = 10u64.pow(p);
            for c in [ten - 1, ten, ten + 1] {
                assert_eq!(cents(&two_decimals(c)), c, "{c} cents");
            }
        }
        for c in -100_000..=100_000i64 {
            let s = signed_two_decimals(c);
            let (minus, rest) = match s.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, s.strip_prefix('+').expect("a sign")),
            };
            assert_eq!(minus, c < 0, "{s:?}");
            assert_eq!(cents(rest), c.unsigned_abs(), "{s:?}");
        }
    }

    /// Which way a stock went (`FRONT.2`): up, down and flat each; and over
    /// the first game's two years and two days, each of the three shown on
    /// some day by some stock, so that the Portfolio held to the hand at
    /// every frame holds every branch of the cell's formula too.
    #[test]
    fn a_glyph_says_which_way_the_cents_went() {
        assert_eq!(
            [glyph(1_230, 1_229), glyph(1_229, 1_230), glyph(25, 25)],
            ["▲", "▼", "="]
        );
        let market = Market::new(SEED);
        let mut shown = std::collections::BTreeSet::new();
        for day in 0..2 * DAYS_A_YEAR + 2 {
            for rung in &RUNGS {
                let today = day as f64;
                let now = rung.price(&market, DIFFICULTY, today) as i64;
                let was = rung.price(&market, DIFFICULTY, today - 1.0) as i64;
                shown.insert(glyph(now, was));
            }
        }
        assert_eq!(shown.len(), 3, "the glyphs shown: {shown:?}");
    }

    /// A calendar standing at a day and a time of day in minutes, for the
    /// hustle's sentences.
    fn when(day: u64, minutes: u64) -> Calendar {
        let mut c = Calendar::new(PACE, DAYS_A_YEAR);
        (c.day, c.weekday) = (day, day % DAYS_A_WEEK);
        (c.hour, c.minute) = (minutes / 60, minutes % 60);
        c
    }

    /// A frame's inputs with these keys held and these bound, nothing else.
    fn keys(held: &[&str], bound: &[&str]) -> Inputs {
        Inputs {
            held: held.iter().map(|k| k.to_string()).collect(),
            bound: bound.iter().map(|k| k.to_string()).collect(),
            ..Inputs::default()
        }
    }

    /// The hustle by hand (`BACK.1`; a week and a day's work since `POT.10`):
    /// a press claims once a week, against the day the person saw, and four
    /// times a run, its fee paid a day after the time the person saw. A
    /// key's press on day 0 at 10:00 claims, and the prospectus is being
    /// proofread, its fee due tomorrow and then today at 10:00, and paid at
    /// day 1's 10:00; the week counts down to the next; a press within it is
    /// refused; a key held is no press; days 7, 14 and 21 claim, the fifth
    /// press is refused, and the run's sentence waits for the last fee; a
    /// claim counted against the day before, as in the frame a day turns,
    /// opens the next week from that day; the key is bound on the button's
    /// view alone, and a key held while unbound is no press; and the button's
    /// click is a press only on its own cell of its own view, the button going
    /// down.
    #[test]
    fn the_hustle_claims_once_a_week_and_four_a_run() {
        let up = Inputs::default();
        let key = keys(&[KEY_CODE], &[KEY_CODE]);
        let unbound = keys(&[KEY_CODE], &[]);
        assert!(Hustle::binds(BUTTON.0) && Hustle::binds("gigs") && !Hustle::binds("Portfolio"));
        assert!(!Hustle::keyed(&unbound, &up), "a key held while unbound");
        assert!(
            Hustle::keyed(&key, &unbound),
            "a key held as it is bound presses"
        );
        let mut h = Hustle::default();
        assert_eq!(
            h.status(&when(0, 600)),
            "A prospectus awaits. Nobody else has read one either."
        );
        assert!(h.frame(&key, &up, 0, 600), "day 0's first press claims");
        assert!(h.proofreading(600) && h.earned(600) == 0, "the day's work");
        assert_eq!(
            h.status(&when(0, 600)),
            "Proofreading: the fee is paid tomorrow at 10:00."
        );
        assert_eq!(
            h.status(&when(1, 570)),
            "Proofreading: the fee is paid today at 10:00."
        );
        assert!(
            !h.proofreading(A_DAY + 600) && h.earned(A_DAY + 600) == FEE,
            "the fee paid a day later"
        );
        assert_eq!(
            h.status(&when(1, 600)),
            "Done for the week: the next prospectus arrives in 6 days."
        );
        assert_eq!(
            h.status(&when(6, 600)),
            "Done for the week: the next prospectus arrives tomorrow."
        );
        assert!(
            !h.frame(&key, &up, 6, 6 * A_DAY + 600),
            "a press on day 6 is refused, the week not out"
        );
        assert!(
            !h.frame(&key, &key, 7, 7 * A_DAY + 600),
            "a key held is no press"
        );
        for day in [7, 14, 21] {
            assert!(
                h.frame(&key, &up, day, day * A_DAY + 600),
                "day {day} claims"
            );
        }
        let days: Vec<u64> = h.claims.iter().map(|&(d, _)| d).collect();
        assert_eq!(days, [0, 7, 14, 21]);
        assert!(
            !h.frame(&key, &up, 28, 28 * A_DAY + 600),
            "the fifth press is refused"
        );
        assert_eq!(
            h.status(&when(21, 900)),
            "Proofreading: the fee is paid tomorrow at 10:00."
        );
        assert_eq!(h.earned(22 * A_DAY + 600), 4 * FEE);
        assert_eq!(
            h.status(&when(28, 600)),
            "Compliance has approved 4 a run, and you have proofread 4."
        );

        let mut turning = Hustle::default();
        assert!(
            turning.frame(&key, &up, 6, 6 * A_DAY + 1410),
            "a press shown day 6, at 23:30"
        );
        assert!(
            !turning.open(12) && turning.open(13),
            "day 13 open after day 6's claim"
        );

        let (sheet, column, row) = BUTTON;
        let at = |sheet: &str, column: u64, row: u64, left: bool| Inputs {
            sheet: sheet.to_string(),
            column,
            row,
            left,
            ..Inputs::default()
        };
        assert!(Hustle::clicked(
            &at(sheet, column, row, true),
            &at(sheet, column, row, false)
        ));
        assert!(Hustle::clicked(
            &at("gigs", column, row, true),
            &at("gigs", column, row, false)
        ));
        for (now, last, why) in [
            (
                at(sheet, column + 1, row, true),
                at(sheet, column + 1, row, false),
                "beside the button",
            ),
            (
                at("Portfolio", column, row, true),
                at("Portfolio", column, row, false),
                "on another view",
            ),
            (
                at(sheet, column, row, true),
                at(sheet, column, row, true),
                "the button held",
            ),
        ] {
            assert!(!Hustle::clicked(&now, &last), "{why}");
        }
    }

    /// The fine print's round: the first row for the first month, the next
    /// from day 30, and the first again once every row has stood its month.
    #[test]
    fn the_fine_print_moves_on_a_row_a_month() {
        let at = |day| disclosure(day, 8, DAYS_A_DISCLOSURE);
        assert_eq!([at(0), at(29), at(30), at(59), at(60)], [1, 1, 2, 2, 3]);
        assert_eq!([at(7 * 30 + 29), at(8 * 30), at(365)], [8, 1, 5]);
        assert_eq!(disclosure(3, 2, 1), 2, "a day a disclosure, two rows");
    }

    /// Park and Miller's own check, as their paper gives it: from 1, the
    /// 10,000th number of the stream is 1,043,618,065.
    #[test]
    fn the_stream_from_one_meets_park_and_millers_ten_thousandth() {
        assert_eq!(stream(1, 10_000), 1_043_618_065);
        let mut z = 1u64;
        for _ in 0..10_000 {
            z = z * MULTIPLIER % MODULUS;
        }
        assert_eq!(z, 1_043_618_065, "the stream stepped one draw at a time");
    }

    /// The jumps the searches find are the numbers the source writes in the
    /// Dice sheet, and each multiplier is its stride's power found the slow
    /// way too, a step at a time.
    #[test]
    fn the_jumps_are_the_numbers_the_source_writes() {
        assert_eq!([DAY_STRIDE, DAY_JUMP], [1_017, 86_383]);
        assert_eq!([EXAM_OFFSET, EXAM_JUMP], [1_073_741_912, 1_164_196]);
        let mut z = 1u64;
        for _ in 0..DAY_STRIDE {
            z = z * MULTIPLIER % MODULUS;
        }
        assert_eq!(z, DAY_JUMP);
    }

    /// The calendar's turns at the game's pace (`POT.11`): the first frame is
    /// Monday's opening bell on day 0, the session its own and its tick 0; the
    /// second twenty seconds on; the sixteenth 9:35, the first tick; the
    /// 1,170th 15:59:40, the 77th; the 1,171st the closing bell, the 78th, the
    /// last of the session's twenty-second steps landing on it; then two
    /// minutes a frame, 16:02 and 23:58; the 1,411th midnight, Tuesday, the
    /// dice's frame, the session still Monday's; the 1,695th 9:28 and the
    /// 1,696th Tuesday's bell, the night's steps landing on it, its session
    /// Tuesday's and the one before Monday's; Saturday's midnight, Friday's
    /// session standing through the weekend; Monday's bell after it, the
    /// session before it Friday's; and the year's turn, day 365 a Tuesday.
    #[test]
    fn the_calendar_turns_at_its_bells_and_its_midnights() {
        let mut c = Calendar::new(PACE, DAYS_A_YEAR);
        let wanted = [
            1, 2, 16, 1_170, 1_171, 1_172, 1_410, 1_411, 1_695, 1_696, 1_697, 8_191, 9_916, 516_991,
        ];
        let mut at = Vec::new();
        while c.frame < 516_991 {
            c.tick();
            if wanted.contains(&c.frame) {
                at.push((
                    c.frame,
                    c.day,
                    c.weekday,
                    format!("{}:{:02}", time_shown(c.hour * 60 + c.minute), c.second),
                    c.turned,
                    c.open(),
                    (c.session, c.before, c.tick),
                    (c.year, c.day_of_year),
                ));
            }
        }
        let row = |frame, day, weekday, time: &str, turned, open, s, y| {
            (frame, day, weekday, time.to_string(), turned, open, s, y)
        };
        assert_eq!(
            at,
            vec![
                row(1, 0, 0, "9:30:00", true, true, (0, 0, 0), (1, 1)),
                row(2, 0, 0, "9:30:20", false, true, (0, 0, 0), (1, 1)),
                row(16, 0, 0, "9:35:00", false, true, (0, 0, 1), (1, 1)),
                row(1_170, 0, 0, "15:59:40", false, true, (0, 0, 77), (1, 1)),
                row(1_171, 0, 0, "16:00:00", false, false, (0, 0, 78), (1, 1)),
                row(1_172, 0, 0, "16:02:00", false, false, (0, 0, 78), (1, 1)),
                row(1_410, 0, 0, "23:58:00", false, false, (0, 0, 78), (1, 1)),
                row(1_411, 1, 1, "0:00:00", true, false, (0, 0, 78), (1, 2)),
                row(1_695, 1, 1, "9:28:00", false, false, (0, 0, 78), (1, 2)),
                row(1_696, 1, 1, "9:30:00", false, true, (1, 0, 0), (1, 2)),
                row(1_697, 1, 1, "9:30:20", false, true, (1, 0, 0), (1, 2)),
                row(8_191, 5, 5, "0:00:00", true, false, (4, 3, 78), (1, 6)),
                row(9_916, 7, 0, "9:30:00", false, true, (7, 4, 0), (1, 8)),
                row(
                    516_991,
                    365,
                    1,
                    "0:00:00",
                    true,
                    false,
                    (364, 361, 78),
                    (2, 1)
                ),
            ]
        );
    }

    /// The same days at a frame an hour and at a frame a day, one pace at
    /// every hour: the hour's frames see a session's ticks twelve apart, from
    /// 9:30 to 16:30, and the closing tick, and the day's frames every bell, a
    /// Saturday's showing Friday's session.
    #[test]
    fn an_hour_or_a_day_a_frame_counts_the_same_sessions() {
        let mut h = Calendar::new(Pace::every(AN_HOUR), DAYS_A_YEAR);
        let mut ticks = Vec::new();
        for _ in 0..8 {
            h.tick();
            ticks.push((time_shown(h.hour * 60 + h.minute), h.tick, h.open()));
        }
        assert_eq!(ticks.first(), Some(&("9:30".to_string(), 0, true)));
        assert_eq!(ticks.last(), Some(&("16:30".to_string(), 78, false)));
        assert_eq!(
            ticks.iter().map(|m| m.1).collect::<Vec<_>>(),
            [0, 12, 24, 36, 48, 60, 72, 78]
        );
        let mut d = Calendar::new(Pace::every(A_DAY_LONG), DAYS_A_YEAR);
        let mut sessions = Vec::new();
        for _ in 0..9 {
            d.tick();
            assert!(d.turned && (d.hour, d.minute) == (9, 30));
            sessions.push((d.day, d.session, d.before, d.tick));
        }
        assert_eq!(
            sessions,
            [
                (0, 0, 0, 0),
                (1, 1, 0, 0),
                (2, 2, 1, 0),
                (3, 3, 2, 0),
                (4, 4, 3, 0),
                (5, 4, 3, 78),
                (6, 4, 3, 78),
                (7, 7, 4, 0),
                (8, 8, 7, 0),
            ]
        );
    }

    /// A step that crosses the opening bell rings it there and counts the
    /// ticks after it (`POT.11`): from 9:00 a frame an hour, the bell heard at
    /// 9:30 inside the step and 10:00 the sixth tick, as the cells' tick reads
    /// the time; and a step of a whole session at 9:30 lands on the close.
    #[test]
    fn a_step_across_the_bell_rings_it_inside() {
        let mut c = Calendar::new(Pace::every(AN_HOUR), DAYS_A_YEAR);
        c.tick();
        (c.day, c.weekday, c.hour, c.minute, c.session, c.tick) = (1, 1, 9, 0, 0, CLOSING_TICK);
        c.tick();
        assert_eq!(
            (c.hour, c.minute, c.session, c.before, c.tick),
            (10, 0, 1, 0, 6)
        );
        let mut s = Calendar::new(Pace::every((CLOSING_BELL - OPENING_BELL) * 60), DAYS_A_YEAR);
        s.tick();
        s.tick();
        assert_eq!(
            (s.hour, s.minute, s.tick, s.open()),
            (16, 0, CLOSING_TICK, false)
        );
    }

    /// A session leaves the last close and reaches its own exactly (`POT.10`,
    /// `POT.11`): over every session of the first game's first year, at a frame
    /// each five minutes, the index at the opening bell is the session
    /// before's close to the bit, at the closing bell the session's own, and at
    /// every tick between no further off the line between them than the five
    /// minutes' swing times the widest a bridge of 78 steps strays.
    #[test]
    fn a_session_leaves_the_last_close_and_reaches_its_own() {
        let market = Market::new(SEED);
        let mut c = Calendar::new(Pace::every(MINUTES_A_TICK * 60), DAYS_A_YEAR);
        let (mut opens, mut closes, mut closed) = (0, 0, None);
        while c.day < DAYS_A_YEAR {
            c.tick();
            if c.session != c.day {
                continue;
            }
            let p = market.path(SEED, DIFFICULTY, &c);
            let last = market.index(DIFFICULTY, c.before as f64);
            let close = market.index(DIFFICULTY, c.session as f64);
            if c.tick == 0 {
                assert_eq!(p.now.to_bits(), last.to_bits(), "day {}'s bell", c.day);
                opens += 1;
            }
            if c.tick == CLOSING_TICK && closed != Some(c.day) {
                assert_eq!(p.now.to_bits(), close.to_bits(), "day {}'s close", c.day);
                (closes, closed) = (closes + 1, Some(c.day));
            }
            let line = last * (1.0 - p.share) + close * p.share;
            assert!(
                (p.now / line - 1.0).abs() <= market.tick_swing() * WIDEST,
                "day {}, tick {}: {} off its line {line}",
                c.day,
                c.tick,
                p.now
            );
        }
        assert_eq!((opens, closes), (261, 261), "the first year's sessions");
    }

    /// The widest a bridge of `n` steps strays at its `m`th, over every pattern
    /// of steps at -1 and 1: the sum `W(m) - (m/n)W(n)` is linear in each step,
    /// its coefficient `1 - m/n` for the first `m` and `-m/n` for the rest, so
    /// its extreme takes each step at the sign of its coefficient,
    /// `2m(n-m)/n`. Held against every pattern of a bridge of thirteen,
    /// `POT.10`'s, whose widest is `84/13`; then a bridge of 78, `POT.11`'s,
    /// strays at most 39 steps, at its 39th tick.
    #[test]
    fn a_bridge_of_78_steps_strays_at_most_39_steps() {
        let widest = |n: usize, m: usize| 2.0 * m as f64 * (n - m) as f64 / n as f64;
        let k = 13;
        for m in 0..=k {
            let mut most = 0.0f64;
            for pattern in 0..1u32 << k {
                let steps: Vec<f64> = (0..k)
                    .map(|j| if pattern >> j & 1 == 1 { 1.0 } else { -1.0 })
                    .collect();
                let all: f64 = steps.iter().sum();
                let so_far: f64 = steps[..m].iter().sum();
                most = most.max((so_far - m as f64 / k as f64 * all).abs());
            }
            assert!(
                (most - widest(k, m)).abs() < 1e-12,
                "thirteen at {m}: {most}"
            );
        }
        let n = CLOSING_TICK as usize;
        let at: Vec<f64> = (0..=n).map(|m| widest(n, m)).collect();
        let most = at.iter().copied().fold(0.0, f64::max);
        assert_eq!((most, at[n / 2]), (WIDEST, WIDEST));
        assert_eq!(widest(13, 6), 84.0 / 13.0);
    }

    /// The bell's lines name the bells (`POT.10`): the open line the closing
    /// bell's time, the lines before the bell and over the weekend the opening
    /// bell's, the weekend's Monday, the first trading day.
    #[test]
    fn the_bells_lines_name_the_bells() {
        let (open, close) = (time_shown(OPENING_BELL), time_shown(CLOSING_BELL));
        assert_eq!((open.as_str(), close.as_str()), ("9:30", "16:00"));
        assert!(BELL_OPEN.contains(&format!("until {close}.")));
        assert!(BELL_BEFORE.contains(&format!("at {open}.")));
        assert!(BELL_WEEKEND.contains(&format!("until {} at {open}.", DAY_NAMES[0])));
        assert!(!BELL_CLOSED.contains(':'), "the night's line names no time");
    }

    /// Seed 777's first die and draw, the numbers `POT.1`'s hand test read on
    /// the Dice sheet, 777 being the contract's seed until 2026-10-14.
    #[test]
    fn seed_777_opens_on_these_dice() {
        assert_eq!(market_die(777, 0), 67_119_591);
        assert_eq!(market_draw(777, 0, 1), 650_051_262);
        assert_eq!(market_die(777, 1), 1_933_266_100);
        assert_eq!(exam_die(777), 904_580_292);
        assert_eq!(exam_draw(777, 1), 1_244_230_531);
    }

    /// Seed 777's market on its first day: both tiers open at exactly 1, and
    /// the Kitchin's phase is a turn times the first draw's uniform.
    #[test]
    fn seed_777s_market_opens_at_one() {
        let market = Market::new(777);
        assert_eq!(market.tier1(0.0).to_bits(), 1f64.to_bits());
        assert_eq!(Market::tier0(0.0).to_bits(), 1f64.to_bits());
        assert_eq!(market.phases[0], 2.0 * PI * uniform(650_051_262));
    }

    /// The four amplitudes sum to a half, so tier 1's cycle stays between a
    /// half and three halves, never at or below zero, at any seed and day.
    #[test]
    fn the_amplitudes_keep_the_cycle_above_zero() {
        let sum: f64 = WAVES.iter().map(|w| w.amplitude).sum();
        assert!((sum - 0.5).abs() < 1e-15, "{sum}");
    }

    /// The two-peak test: two peaks of a sine give its period and every peak
    /// after them, which makes a market of one wave arithmetic. No lag from
    /// two years to a hundred carries the first game's tier 1 onto itself: at
    /// every such lag, the index over its trend that lag later differs from
    /// the index over its trend on some day of the first Kitchin wave by at
    /// least 0.02, where a path that repeated would differ by nothing.
    #[test]
    fn the_path_does_not_repeat_within_a_hundred_years() {
        let market = Market::new(SEED);
        let window = WAVES[0].days as usize;
        let longest = (100 * DAYS_A_YEAR) as usize;
        let level: Vec<f64> = (0..window + longest)
            .map(|day| market.cycle(day as f64) / market.first_cycle)
            .collect();
        let (mut least, mut at) = (f64::INFINITY, 0);
        for lag in 2 * DAYS_A_YEAR as usize..=longest {
            let mut most = 0.0f64;
            for (later, now) in level[lag..lag + window].iter().zip(&level[..window]) {
                most = most.max((later - now).abs());
                if most >= least {
                    break;
                }
            }
            if most < least {
                (least, at) = (most, lag);
            }
        }
        assert!(
            least >= 0.02,
            "{at} days later the path differs by {least} at most"
        );
    }

    /// The least the index reaches at any tick of the first year at the
    /// factor of 16 (`POT.11`), the five minutes wandering sixteen times the
    /// half hours' spread: a floor under the year's lowest tick, 0.8210 on day
    /// 39 at 12:25 (0.9501 there at a factor of 4 and 0.98233 at 1).
    const KIND_TICK: f64 = 0.82;

    /// The first game's year is kind, as the owner chose its seed to be: the
    /// index dips no more than 2% and only in the first four months at the
    /// close of any day, the weather turns within them, and the report card at
    /// day 365 reads at least 5% up; and at any five minutes of any session,
    /// the hours wandering sixteen times wider since `POT.11`, the index stays
    /// above [`KIND_TICK`], its lowest within the first four months too. A
    /// change to the waves, the session's swing or the factor that spoils the
    /// first year fails here, and the seed is chosen again with it.
    #[test]
    fn the_first_game_opens_on_a_kind_year() {
        let market = Market::new(SEED);
        let index = |day: i64| market.tier1(day as f64);
        let (mut low, mut low_day) = (f64::INFINITY, 0);
        for day in 0..=365 {
            if index(day) < low {
                (low, low_day) = (index(day), day);
            }
        }
        assert!(
            low > 0.98 && low_day <= 120,
            "the year's low, {low}, on day {low_day}"
        );
        let mut c = Calendar::new(Pace::every(MINUTES_A_TICK * 60), DAYS_A_YEAR);
        let (mut tick_low, mut tick_day) = (f64::INFINITY, 0);
        while c.day < DAYS_A_YEAR {
            c.tick();
            let now = market.path(SEED, DIFFICULTY, &c).now;
            if now < tick_low {
                (tick_low, tick_day) = (now, c.day);
            }
        }
        assert!(
            tick_low > KIND_TICK && tick_day <= 120,
            "the year's lowest tick, {tick_low}, on day {tick_day}"
        );
        let turns = |day: i64| index(day) >= index(day - 20);
        let turn = (1..=365).find(|&day| turns(day) != turns(day - 1));
        assert!(
            turn.is_some_and(|day| day <= 130),
            "the weather first turns on day {turn:?}"
        );
        assert!(index(365) >= 1.05, "the report card reads {}", index(365));
    }

    /// Every rung opens at its start exactly, at both tiers: the index is 1
    /// on day 0, and 1 to any power is 1.
    #[test]
    fn every_rung_opens_at_its_start() {
        let market = Market::new(SEED);
        for difficulty in [0, 1] {
            let opening: Vec<u64> = RUNGS
                .iter()
                .map(|r| r.price(&market, difficulty, 0.0))
                .collect();
            assert_eq!(opening, [100, 500, 2_500, 10_000], "tier {difficulty}");
        }
    }

    /// The rungs' least prices no seed passes at the factor of 16, in cents.
    const FLOOR_CENTS: [u64; 4] = [1, 15, 239, 1_719];

    /// No seed prices a rung at or below zero (`STATION.1`'s decision 8):
    /// at the least the index can be at any five minutes (`POT.11`), a third of
    /// the trend at day 0 less the widest a tick strays from its line at the
    /// factor of 16, the four rungs are 1, 15, 239 and 1,719 cents, where the
    /// factor of 4 made them 8, 72, 685 and 3,786, a half hour's least 11, 94,
    /// 818 and 4,327 (`POT.10`) and a day's 11, 96, 833 and 4,386; a factor
    /// past 22 would let a price reach zero.
    #[test]
    fn no_seed_prices_a_rung_at_or_below_zero() {
        let least = least_index();
        let tick = SWING * FACTOR / 6f64.sqrt();
        assert!(
            (least - (1.0 - tick * WIDEST) / 3.0).abs() < 1e-15,
            "{least}"
        );
        let floor: Vec<u64> = RUNGS.iter().map(|r| r.price_at(least)).collect();
        assert_eq!(floor, FLOOR_CENTS);
    }

    /// The dividend falls four times a year, 91 days apart, its last inside
    /// the year, and only the dearest rung pays it.
    #[test]
    fn the_dividend_falls_four_times_a_year() {
        for pair in DIVIDEND_DAYS.windows(2) {
            assert_eq!(pair[1] - pair[0], 91, "{pair:?}");
        }
        assert!(DIVIDEND_DAYS[3] <= DAYS_A_YEAR);
        let paid: Vec<u64> = RUNGS
            .iter()
            .map(|r| (1..=DAYS_A_YEAR).map(|day| r.dividend_on(day)).sum())
            .collect();
        assert_eq!(paid, [0, 0, 0, 200], "cents a share a year");
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    /// The exam's strides (`GUARD.1`): the first ten primes above the most
    /// rows a picker holds, each prime, in order, none skipped between them,
    /// and so coprime to every bank's size from 1 to 1,547; and among them
    /// every stride coprime to the first bank's twenty, so that its twenty
    /// starts make 160 exams.
    #[test]
    fn the_strides_are_the_first_ten_primes_above_the_most_a_picker_holds() {
        let between: Vec<u64> = (PICKER_ROWS + 1..=STRIDES[9])
            .filter(|&n| is_prime(n))
            .collect();
        assert_eq!(
            between, STRIDES,
            "the primes from 1,548 to the tenth, every one"
        );
        assert_eq!(STRIDES[0], 1_549);
        for bank in 1..=PICKER_ROWS {
            for s in STRIDES {
                assert_eq!(gcd(s, bank), 1, "{s} and a bank of {bank}");
            }
        }
        let classes: BTreeSet<u64> = STRIDES.iter().map(|s| s % 20).collect();
        let coprime: BTreeSet<u64> = (1..20).filter(|&r| gcd(r, 20) == 1).collect();
        assert_eq!(classes, coprime, "every stride a bank of twenty can take");
    }

    /// Every bank a picker holds, ten to 1,547 rows, with every stride and
    /// from its first row and its last: ten rows, each a row of the bank, and
    /// no row twice; a bank of ten meets all of its rows. And the first game's
    /// exam, at the contract's seed over the bank's twenty, the rows the
    /// scoping derived and the tenth since the owner's eleven questions: 9,
    /// 18, 7, 16, 5, 14, 3, 12, 1 and 10.
    #[test]
    fn every_bank_a_picker_holds_draws_ten_rows_and_no_repeat() {
        for bank in SCORED as u64..=PICKER_ROWS {
            for s in STRIDES {
                for first in [0, bank - 1] {
                    let mut rows = BTreeSet::new();
                    let mut at = first;
                    for _ in 0..SCORED {
                        assert!(at < bank);
                        rows.insert(at);
                        at = (at + s % bank) % bank;
                    }
                    assert_eq!(rows.len(), SCORED, "a bank of {bank}, the stride {s}");
                }
            }
        }
        assert_eq!(draw(SEED, 20), [9, 18, 7, 16, 5, 14, 3, 12, 1, 10]);
        assert_eq!((start(SEED, 20), stride(SEED)), (8, 1_549));
        for seed in 1..=1_000 {
            let rows = draw(seed, 20);
            let distinct: BTreeSet<u64> = rows.iter().copied().collect();
            assert!(
                distinct.len() == SCORED && rows.iter().all(|r| (1..=20).contains(r)),
                "seed {seed}"
            );
        }
    }

    /// A bank of twenty whose right letters run A, B, C, D and round again.
    fn letters() -> Vec<String> {
        (0..20).map(|k| OPTIONS[k % 4].0.to_string()).collect()
    }

    /// The exam by hand (`GUARD.1`): a question open until eleven are
    /// answered, each press judged by the state before it; the right answers
    /// of the ten counted by letter and the eleventh the style; the
    /// certificate waiting for its button once all eleven are in, and the game
    /// begun by it and by nothing before; the sheet shown and the keys bound
    /// by the state; the option pressed the first by letter, by its cell's
    /// click on the Exam or its key.
    #[test]
    fn the_exam_takes_eleven_answers_and_begins_at_its_certificate() {
        let mut e = Exam::new(SEED, letters());
        assert!(e.open() && !e.pending() && !e.begun);
        assert_eq!(
            (e.shows(), e.status().as_str()),
            (
                EXAM,
                "The Series 777, question 1 of 11. Correct answers earn starting capital."
            )
        );
        assert_eq!(e.binds("Exam"), ["KeyA", "KeyB", "KeyC", "KeyD"]);
        assert!(e.binds("Portfolio").is_empty() && e.binds("Certificate").is_empty());
        assert_eq!(e.row_shown(), 9, "the draw's first row");
        assert!(
            !e.frame(None, true),
            "the certificate's button, with a question open, begins nothing"
        );
        assert!(!e.begun);
        for k in 0..SCORED {
            let right = e.right();
            let option = if k % 2 == 0 { right } else { right % 4 + 1 };
            assert!(e.frame(Some(option), false), "answer {}", k + 1);
        }
        assert_eq!(
            e.correct(),
            5,
            "the first, the third, the fifth, the seventh and the ninth"
        );
        assert_eq!(
            e.status(),
            "The Series 777, question 11 of 11, ungraded: it names your firm."
        );
        assert_eq!(e.style(), 0, "no style before the eleventh");
        assert!(e.frame(Some(3), false), "the eleventh");
        assert!(!e.open() && e.pending());
        assert_eq!((e.style(), e.score(), e.capital()), (3, 5, 750_000));
        assert_eq!(e.shows(), CERTIFICATE);
        assert_eq!(
            e.status(),
            "Pencils down. Your certificate is on its sheet."
        );
        assert!(!e.frame(Some(1), false), "a twelfth is no answer");
        assert_eq!(e.answers.len(), ASKED);
        assert!(e.binds("Exam").is_empty());
        assert_eq!(e.binds("Certificate"), [OPEN_KEY]);
        assert!(
            !e.frame(None, true) && e.begun,
            "the certificate's button begins the game"
        );
        assert_eq!(e.shows(), LEAD);
        assert!(e.binds("Certificate").is_empty() && e.binds("Exam").is_empty());

        // The option pressed: a click on its cell of the Exam, or its key.
        let at = |sheet: &str, row: u64, left: bool| Inputs {
            sheet: sheet.to_string(),
            column: OPTION_COLUMN,
            row,
            left,
            ..Inputs::default()
        };
        assert_eq!(
            Exam::option_pressed(&at(EXAM, 9, true), &at(EXAM, 9, false)),
            Some(3)
        );
        assert_eq!(
            Exam::option_pressed(&at(EXAM, 9, true), &at(EXAM, 9, true)),
            None,
            "held"
        );
        assert_eq!(
            Exam::option_pressed(&at(EXAM, 5, true), &at(EXAM, 5, false)),
            None,
            "the question"
        );
        assert_eq!(
            Exam::option_pressed(&at("Portfolio", 9, true), &at("Portfolio", 9, false)),
            None
        );
        let mut both = at(EXAM, 10, true);
        both.held.insert("KeyB".to_string());
        both.bound.insert("KeyB".to_string());
        assert_eq!(
            Exam::option_pressed(&both, &at(EXAM, 10, false)),
            Some(2),
            "the first by letter of two"
        );
        assert_eq!(
            Exam::option_pressed(&keys(&["KeyC"], &[]), &Inputs::default()),
            None,
            "unbound"
        );
        let (sheet, column, row) = OPEN_BUTTON;
        let open = Inputs {
            sheet: sheet.to_string(),
            column,
            row,
            left: true,
            ..Inputs::default()
        };
        assert!(Exam::opened(&open, &Inputs::default()));
        assert!(Exam::opened(
            &keys(&[OPEN_KEY], &[OPEN_KEY]),
            &Inputs::default()
        ));
    }

    /// The score funds the firm as the contract says (`GUARD.1`): $5,000 and
    /// $500 a right answer, from $5,000 to $10,000 as the right answers of
    /// the ten run from none to all, the eleventh counting nothing; and a
    /// waiver's five, $7,500, its style the first, at any answer, the Camera
    /// naming the lead at the waiver's frame and nothing at the next
    /// (`CRITIC.1`).
    #[test]
    fn every_score_funds_the_contracts_capital() {
        let mut first = Exam::new(SEED, letters());
        assert!(
            !first.frame(Some(first.right()), false),
            "a press in the first frame, whose last showed no question"
        );
        for k in 0..=SCORED {
            let mut e = Exam::new(SEED, letters());
            e.frame(None, false);
            for q in 0..SCORED {
                let right = e.right();
                e.frame(Some(if q < k { right } else { right % 4 + 1 }), false);
            }
            e.frame(Some(4), false);
            assert_eq!(e.score(), k as u64);
            assert_eq!(e.capital(), 500_000 + 50_000 * k as u64, "{k} right");
        }
        let mut w = Exam::new(SEED, letters());
        w.frame(None, false);
        assert!(w.frame(Some(w.right()), false));
        w.waive();
        assert!(!w.open() && w.begun);
        w.frame(None, false);
        assert_eq!(w.shows(), LEAD, "the lead at the waiver's frame");
        assert_eq!(
            camera(&w, true),
            CARD_VIEW,
            "a card's turn names the Report"
        );
        w.frame(None, false);
        assert_eq!(w.shows(), "", "and nothing at the frame after it");
        assert_eq!(
            (w.score(), w.style(), w.capital()),
            (WAIVED_SCORE, WAIVED_STYLE, 750_000)
        );
        assert_eq!(w.status(), "The Series 777: waived.");
    }

    /// The calendar waits at the first bell (`GUARD.1`): while the exam holds
    /// it, every frame stands at the first frame's time, day 0's session at
    /// its tick 0, the market closed and the bell's line the wait's, the pace
    /// none (`POT.11`) and the frames counted; the game's first frame stands
    /// there too, the market open; and the frame after it is the session's
    /// twenty seconds on.
    #[test]
    fn the_calendar_waits_at_the_first_bell_until_the_game_begins() {
        let mut c = Calendar::new(PACE, DAYS_A_YEAR);
        for f in 1..=5 {
            c.tick_waiting(true);
            assert_eq!(
                (c.seconds(), c.session, c.tick, c.waited, c.pace_now()),
                (FIRST_MINUTE * 60, 0, 0, f - 1, 0)
            );
            assert!(c.waits && c.in_session() && !c.open() && c.turned == (f == 1));
            assert_eq!(bell(&c), BELL_WAIT);
        }
        c.tick_waiting(false);
        assert_eq!(
            (c.seconds(), c.waited, c.pace_now()),
            (FIRST_MINUTE * 60, 5, PACE.session),
            "the game's first frame stands at the bell"
        );
        assert!(c.open() && bell(&c) == BELL_OPEN);
        c.tick_waiting(false);
        assert_eq!(
            (c.seconds(), c.waited),
            (FIRST_MINUTE * 60 + PACE.session, 5)
        );
    }

    /// Every book of both pockets, each pocket's cash and the fills: what a
    /// switch must leave as it found it (`BD-9`).
    fn balances(p: &Plate, starts: [u64; 2]) -> (Vec<Book>, [u64; 2], usize) {
        let books = [FIRM, PERSONAL]
            .iter()
            .flat_map(|&pocket| (1..=4).map(move |stock| (pocket, stock)))
            .map(|(pocket, stock)| p.book(pocket, stock))
            .collect();
        let cash = [p.cash(FIRM, starts[0]), p.cash(PERSONAL, starts[1])];
        (books, cash, p.fills.len())
    }

    /// The switch by hand (`PLATE.2`): a click on it, or its command, turns the
    /// pocket and moves no balance, takes no order and picks no stock; and one
    /// act a frame, a click's first: a trade clicked beside the switch's
    /// command fills from the pocket shown and switches nothing, and the switch
    /// clicked beside a trade's command or a pick's takes neither.
    #[test]
    fn the_switch_turns_the_pocket_and_moves_no_balance() {
        let shown = Shown {
            open: true,
            prices: [100, 500, 2_500, 10_000],
            cash: [750_000, 10_000],
            when: "Monday, year 1, day 1, 9:30".to_string(),
            errors: [None, None],
        };
        let starts = [750_000, 10_000];
        let (switch, _) = SWITCH_COMMAND;
        let mut p = Plate::default();
        let bought = p.frame(Some((2, 2)), false, "", &shown, starts);
        assert_eq!(bought.map(|o| (o.pocket, o.filled)), Some((FIRM, 10)));
        for (press, clicked, command, pocket) in [
            (None, true, "", PERSONAL),
            (None, false, switch, FIRM),
            (None, true, "buy-1", PERSONAL),
            (None, true, "rung-3", FIRM),
            (None, true, switch, PERSONAL),
        ] {
            let before = balances(&p, starts);
            assert_eq!(p.frame(press, clicked, command, &shown, starts), None);
            assert_eq!(p.pocket, pocket, "the switch beside {command:?}");
            assert_eq!(balances(&p, starts), before, "no balance moved");
            assert_eq!(p.picked, 2, "no stock picked beside the switch");
        }
        let order = p.frame(Some((1, 1)), false, switch, &shown, starts);
        assert_eq!(
            order.map(|o| (o.pocket, o.stock, o.filled)),
            Some((PERSONAL, 1, 1)),
            "a trade clicked beside the switch's command fills from the pocket shown"
        );
        assert_eq!(p.pocket, PERSONAL, "and switches nothing");
    }

    /// The weather's word (`FRONT.3`) is the first of the table's whose tenths
    /// a change reaches, from the top, and Rain below them all: at each word's
    /// own tenths and a tenth under it, and far past both ends; the table
    /// descending, each word once, and the lines spelled up, down and flat.
    #[test]
    fn the_weather_reads_the_first_word_its_tenths_reach() {
        for pair in WEATHER.windows(2) {
            assert!(pair[0].1 > pair[1].1, "{pair:?}: descending");
        }
        for (k, (word, from)) in WEATHER.iter().enumerate() {
            assert_eq!(weather(*from), *word, "at {from} tenths");
            let below = WEATHER.get(k + 1).map_or(WEATHER_BELOW, |(w, _)| *w);
            assert_eq!(weather(from - 1), below, "a tenth under {from}");
        }
        assert_eq!(weather(10_000), "Sunny");
        assert_eq!(weather(-10_000), WEATHER_BELOW);
        assert_eq!(
            weather_line(8, 20),
            "Market weather: Partly sunny, the market up 0.8% in the 20 days to the last close."
        );
        assert_eq!(
            weather_line(-15, 20),
            "Market weather: Rain, the market down 1.5% in the 20 days to the last close."
        );
        assert_eq!(
            weather_line(0, 5),
            "Market weather: Calm, the market flat in the 5 days to the last close."
        );
        assert_eq!(
            weather_line(123, 20),
            "Market weather: Sunny, the market up 12.3% in the 20 days to the last close."
        );
    }

    /// The first game's weather (`FRONT.3`), at a frame each five minutes of
    /// its first year: Overcast at the first bell, the market easing into the
    /// year's dip, then Calm from day 93's close, Partly sunny from day 112's
    /// and Sunny from day 263's, each change at a closing bell and none at any
    /// other five minutes, the word reading the last close made.
    #[test]
    fn the_first_years_weather_turns_three_times_each_at_a_close() {
        let market = Market::new(SEED);
        let mut c = Calendar::new(Pace::every(MINUTES_A_TICK * 60), DAYS_A_YEAR);
        c.tick();
        let mut was = weather(market.tenths(DIFFICULTY, &c));
        assert_eq!(was, "Overcast", "the first bell's weather");
        let mut turns = Vec::new();
        while c.day < DAYS_A_YEAR {
            c.tick();
            let now = weather(market.tenths(DIFFICULTY, &c));
            if now != was {
                turns.push((c.day, c.hour * 60 + c.minute, now));
                was = now;
            }
        }
        assert_eq!(
            turns,
            [
                (93, CLOSING_BELL, "Calm"),
                (112, CLOSING_BELL, "Partly sunny"),
                (263, CLOSING_BELL, "Sunny"),
            ]
        );
    }

    /// The day's headline (`FRONT.3`): the hand's whole-number rule is the
    /// cell's double one, the draw's uniform times the rows rounded down, at
    /// every day of the first game's century and at each count of rows from
    /// one to twenty and at the most a picker holds; and in the first year each
    /// of the ten rows is the day's headline on some day.
    #[test]
    fn the_headline_rule_is_the_cells_at_every_day_of_a_century() {
        let mut seen = [false; 10];
        for day in 0..100 * DAYS_A_YEAR {
            let z = market_draw(SEED, day, HEADLINE_DRAW);
            for rows in (1..=20).chain([PICKER_ROWS]) {
                let cell = (uniform(z) * rows as f64).floor() as u64 + 1;
                assert_eq!(headline(SEED, day, rows), cell, "day {day}, {rows} rows");
            }
            if day < DAYS_A_YEAR {
                seen[headline(SEED, day, 10) as usize - 1] = true;
            }
        }
        assert!(
            seen.iter().all(|s| *s),
            "every headline of ten in the first year: {seen:?}"
        );
    }

    /// The names (`BACK.2`, `MENU.3`'s first slice): four stocks, each a name of
    /// its own, none a working label, each one line; the order's sentence and
    /// the commands' words naming the stock by the hand's name; and the firm's
    /// line pending until a style is given, then the name and the style.
    #[test]
    fn the_stocks_and_the_firm_are_named() {
        let names = names();
        let distinct: BTreeSet<&str> = names.iter().copied().collect();
        assert_eq!(distinct.len(), names.len(), "four names: {names:?}");
        for (k, name) in names.iter().enumerate() {
            assert!(
                !name.trim().is_empty()
                    && !name.contains('\n')
                    && !name.to_lowercase().contains("rung"),
                "stock {}: {name:?}",
                k + 1
            );
        }
        let order = Order {
            pocket: FIRM,
            stock: 1,
            trade: 2,
            open: true,
            price: 100,
            cash: 750_000,
            held: 0,
            filled: 10,
            outcome: Outcome::Filled,
            error: None,
            asked: 10,
            bought: 0,
            sold: 0,
        };
        assert_eq!(
            order.sentence(),
            "Bought 10 shares of Longshot Mining at the price shown."
        );
        assert_eq!(
            Order {
                stock: 0,
                filled: 0,
                outcome: Outcome::NoStock,
                ..order
            }
            .sentence(),
            "Refused: no stock is picked. Press 1 to 4 to pick one, or click a button on its row."
        );
        let words = command_words(3, FIRM);
        assert_eq!(
            words[2],
            "Pick Average Co.: the buy and sell commands trade it"
        );
        assert_eq!(words[4], "Buy 1 share of Average Co.");
        assert_eq!(
            command_words(0, FIRM)[9],
            "Sell every share of the stock picked, none yet"
        );
        assert_eq!(firm_line(None), FIRM_LINE);
        assert_eq!(
            firm_line(Some(("I feel the market", "Vibes LLC"))),
            "Your firm: Vibes LLC. Investment style: I feel the market."
        );
    }

    /// The quarters (`CRITIC.1`): 91, 91, 91 and 92 days, the year's days, each
    /// day of the year in the quarter whose last day it does not pass; and at a
    /// frame a day over three years, the count moving at the first frame of the
    /// year's 92nd, 183rd and 274th days and of each next year's first, twelve
    /// turns, never at the game's first frame.
    #[test]
    fn the_quarters_turn_at_their_days() {
        let lengths: Vec<u64> = QUARTER_ENDS
            .iter()
            .scan(0, |was, &end| {
                let n = end - *was;
                *was = end;
                Some(n)
            })
            .collect();
        assert_eq!(lengths, [91, 91, 91, 92]);
        assert_eq!(QUARTER_ENDS[3], DAYS_A_YEAR);
        for (day, q) in [
            (1, 1),
            (91, 1),
            (92, 2),
            (182, 2),
            (183, 3),
            (273, 3),
            (274, 4),
            (365, 4),
        ] {
            assert_eq!(quarter_of(day), q, "day {day} of the year");
        }
        let mut c = Calendar::new(Pace::every(A_DAY_LONG), DAYS_A_YEAR);
        let mut turns = Vec::new();
        let mut was: Option<u64> = None;
        while c.year < 4 {
            c.tick();
            let now = c.quarters();
            if was.is_some_and(|w| w != now) {
                turns.push((c.year, c.day_of_year));
            }
            was = Some(now);
        }
        let want: Vec<(u64, u64)> = (1..=3)
            .flat_map(|y| [(y, 92), (y, 183), (y, 274), (y + 1, 1)])
            .collect();
        assert_eq!(turns, want, "the turns, by year and day of the year");
    }

    /// A return in whole hundredths of a percent (`CRITIC.1`): rounded half up,
    /// a half's both ways at once; and the cell's spelling of it, two doubles
    /// divided and rounded down, `INT(((e-s)*20000+s)/(2*s))`, the hand's at
    /// every pair of worths from 1 to 1,000 cents and their next thousand, and
    /// beside every power of two to 2^40 cents, where the numerator stays below
    /// 2^53.
    #[test]
    fn a_return_is_rounded_half_up_in_whole_numbers() {
        assert_eq!(points(20_000, 20_001), 1, "half a hundredth up rounds up");
        assert_eq!(
            points(20_000, 19_999),
            0,
            "half a hundredth down rounds up to 0"
        );
        assert_eq!(points(750_000, 742_200), -104);
        assert_eq!(points(750_000, 825_600), 1_008);
        assert_eq!(points(100, 0), -10_000, "all of it lost");
        let cell = |s: u64, e: u64| -> i64 {
            let (s, e) = (s as f64, e as f64);
            (((e - s) * 20_000.0 + s) / (2.0 * s)).floor() as i64
        };
        for s in 1..=1_000u64 {
            for e in 0..=2_000u64 {
                assert_eq!(points(s, e), cell(s, e), "{s} to {e} cents");
            }
        }
        for k in 10..=40u32 {
            let s = 1u64 << k;
            for e in [
                s - 1,
                s,
                s + 1,
                s / 2,
                s * 3 / 2,
                s + s / 20_000,
                s - s / 20_000,
            ] {
                assert_eq!(points(s, e), cell(s, e), "{s} to {e} cents");
            }
        }
    }

    /// A grade (`CRITIC.1`) is the first whose alpha the card's reaches, from the
    /// top, and F below them all: at each grade's own alpha and a hundredth
    /// under it, and far past both ends; the table descending.
    #[test]
    fn a_grade_is_the_first_its_alpha_reaches() {
        for pair in GRADES.windows(2) {
            assert!(pair[0].1 > pair[1].1, "{pair:?}: descending");
        }
        for (k, (letter, from)) in GRADES.iter().enumerate() {
            assert_eq!(grade(*from), (k as u64 + 1, *letter), "at {from}");
            let below = GRADES.get(k + 1).map_or(GRADE_BELOW, |(l, _)| *l);
            assert_eq!(grade(from - 1).1, below, "a hundredth under {from}");
        }
        assert_eq!(grade(i64::MAX), (1, "A"));
        assert_eq!(grade(i64::MIN), (5, GRADE_BELOW));
        assert_eq!(grade(0).1, "C", "the market's own return, bought and held");
    }

    /// The percents a view draws (`CRITIC.1`): their sign, two decimals and the
    /// percent sign, from one whole digit to five.
    #[test]
    fn a_percent_spells_its_sign_and_two_decimals() {
        for (p, text) in [
            (0, "+0.00%"),
            (-104, "-1.04%"),
            (1_008, "+10.08%"),
            (-1_008, "-10.08%"),
            (5, "+0.05%"),
            (12_345, "+123.45%"),
            (-9_999_999, "-99999.99%"),
        ] {
            assert_eq!(percent(p), text, "{p} hundredths");
        }
    }

    /// The first year's cards at the seed (`CRITIC.1`), the firm at rest, its
    /// cash its worth: each turn at the game's pace at its frame, the first
    /// 1:11:27 of play; and at a frame a day, the four cards' returns, Average
    /// Co. bought with $7,500 at the begin's $25.00, 300 shares and no cents
    /// left: the firm's 0.00% at each, the benchmark's -1.04%, +1.09%, +3.64% and
    /// +6.17%, the year +10.08%; so the alphas their negation and the grades B,
    /// D, F and F, the year's F, the remark at the fourth card the year's.
    #[test]
    fn the_first_years_cards_at_the_seed() {
        let mut c = Calendar::new(PACE, DAYS_A_YEAR);
        let mut was = None;
        let mut frames = Vec::new();
        while frames.len() < 4 {
            c.tick();
            let now = c.quarters();
            if was.is_some_and(|w| w != now) {
                frames.push(c.frame);
            }
            was = Some(now);
        }
        assert_eq!(frames, [128_611, 257_506, 386_401, 516_991]);
        let play = |f: u64| (f - 1) as f64 / 30.0;
        assert_eq!(
            play(frames[0]),
            4_287.0,
            "the first card at 1:11:27 of play"
        );
        assert_eq!(play(frames[3]), 17_233.0, "the year's at 4:47:13");

        let market = Market::new(SEED);
        let capital = CAPITAL_BEFORE + CAPITAL_A_RIGHT * WAIVED_SCORE;
        let mut cal = Calendar::new(Pace::every(A_DAY_LONG), DAYS_A_YEAR);
        let mut cards = Cards::default();
        let mut issued = Vec::new();
        for frame in 1..=366u64 {
            cal.tick();
            let price = RUNGS[BENCHMARK].price_at(market.path(SEED, DIFFICULTY, &cal).now);
            if cards.frame(&cal, frame > 1, capital, price, capital) {
                issued.push(cards.card().expect("a card"));
            }
        }
        assert_eq!((cards.shares, cards.left), (300, 0));
        let got: Vec<(u64, i64, i64, &str)> = issued
            .iter()
            .map(|c| (c.quarter, c.returns[0][1], c.alpha[0], c.grades[0].1))
            .collect();
        assert_eq!(
            got,
            [
                (1, -104, 104, "B"),
                (2, 109, -109, "D"),
                (3, 364, -364, "F"),
                (4, 617, -617, "F"),
            ]
        );
        let year = issued[3];
        assert_eq!(
            (year.returns[1], year.alpha[1], year.grades[1], year.remark),
            ([0, 1_008], -1_008, (5, "F"), 5)
        );
        assert_eq!(
            issued[1].returns[1],
            [0, 4],
            "the year so far at the second card"
        );
    }
}
