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
//! view's dollars are a text of two decimals instead, which the cell pads by
//! comparing its cents with powers of ten and the hand by counting its
//! dollars' digits. The glyph beside each change (`FRONT.2`) shares no
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
/// The look back, `B4`: how many days back the weather reads the index.
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
/// fifth: `POT.2`'s four phases take the four before it; and the day's draws,
/// the session's 78 after them.
pub const SESSION_DRAW: u64 = 5;
pub const DRAWS_A_DAY: u64 = SESSION_DRAW + CLOSING_TICK - 1;
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

/// A rung of a station's ladder (`STATION.1`): its start, a share's price on
/// day 0 in whole cents; its beta, the power of the index its price follows;
/// its dividend yield, a year; and its dividend, the whole cents a share it
/// pays on each dividend day, as the contract states it, where the Stocks
/// sheet computes it from the yield.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rung {
    pub start: u64,
    pub beta: f64,
    pub dividend_yield: f64,
    pub dividend: u64,
}

/// The stocks' four rungs, cheapest first, the Stocks sheet's rows 5 to 8:
/// $1, $5, $25 and $100 a share, the cheapest the wildest, the third the
/// market itself, and the dearest paying 2% a year, 50 cents a share on each
/// of the year's four dividend days.
pub const RUNGS: [Rung; 4] = [
    Rung {
        start: 100,
        beta: 2.0,
        dividend_yield: 0.0,
        dividend: 0,
    },
    Rung {
        start: 500,
        beta: 1.5,
        dividend_yield: 0.0,
        dividend: 0,
    },
    Rung {
        start: 2_500,
        beta: 1.0,
        dividend_yield: 0.0,
        dividend: 0,
    },
    Rung {
        start: 10_000,
        beta: 0.75,
        dividend_yield: 0.02,
        dividend: 50,
    },
];

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

/// The width in characters of every column a view's dollars stand in
/// (`POT.10`), which their padding is measured for.
pub const DOLLARS_COLUMN: u64 = 12;

/// A price's spaces, twice its dollars' digits and its commas (`POT.10`'s
/// second hand test, the commas since `GUARD.1`'s): in a column of
/// [`DOLLARS_COLUMN`] characters, the text then ends where a number ends, the
/// viewport drawing a text from its cell's left and a number to its right, a
/// space being half a digit to within a sixtieth and a comma four fifths of a
/// space, so that each comma takes a space off and leaves its text a fifth of
/// a space short of the others, never past a number's edge.
pub const PRICE_UNITS: usize = 16;

/// A change's spaces before its sign, twice its dollars' digits and its
/// commas, the sign being about a space and a digit wide.
pub const CHANGE_UNITS: usize = 14;

/// Whole dollars with their thousands grouped by commas (`GUARD.1`'s hand
/// test, the owner's word: "Thousands separators need to be implemented"):
/// 1,234,567. The cell joins the leading group and each later group's three
/// digits by their places; the hand counts the digits back from the last.
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

/// The width a spelling of whole dollars takes in units of a space: twice its
/// digits and one a comma.
fn units(dollars: u64) -> usize {
    let digits = dollars.checked_ilog10().map_or(1, |d| d as usize + 1);
    2 * digits + (digits - 1) / 3
}

/// Whole cents, never negative, as a view spells them since `POT.10`'s hand
/// test, the library's `dollars`: the dollars, their thousands grouped by
/// commas since `GUARD.1`'s, a point and the cents in two digits, after the
/// spaces that make [`PRICE_UNITS`] with twice the dollars' digits and their
/// commas, and none from seven digits. The cell chooses its padding by
/// comparing the cents with powers of ten; the hand counts the dollars'
/// digits.
pub fn two_decimals(cents: u64) -> String {
    let dollars = cents / 100;
    let pad = " ".repeat(PRICE_UNITS.saturating_sub(units(dollars)));
    format!("{pad}{}.{:02}", grouped(dollars), cents % 100)
}

/// A change in whole cents as the Portfolio spells it (`POT.10`, the
/// library's `signed-dollars` since `GUARD.1`'s hand test): two decimals, the
/// thousands grouped, and its sign, a gain's or nothing's `+` and a loss's
/// space and `-`, each as wide as the other in the grid's font, after the
/// spaces that make [`CHANGE_UNITS`] with twice its dollars' digits and their
/// commas, and none from seven digits.
pub fn signed_two_decimals(cents: i64) -> String {
    let sign = if cents < 0 { " -" } else { "+" };
    let c = cents.unsigned_abs();
    let dollars = c / 100;
    let pad = " ".repeat(CHANGE_UNITS.saturating_sub(units(dollars)));
    format!("{pad}{sign}{}.{:02}", grouped(dollars), c % 100)
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
/// Where the certificate leads, `B6`, and the exam's two views.
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
/// The Exam's width in characters, its column A, inside which every option
/// stands whole with its letter, so that every character of it is its
/// button.
pub const EXAM_COLUMN: u64 = 100;
/// The certificate's style before the eleventh answer gives one.
pub const STYLE_UNASKED: &str = "asked last";
/// The heading over the certificate's plain statement, the owner's word at
/// the hand test, so that its "You." does not stand alone.
pub const CAVEAT_HEADING: &str = "Caveat:";
/// The buttons' look (`GUARD.1`'s hand test): format 1, a fill of Lovelace
/// Lavender, the house's primary, the text the grid's own until the record
/// holds a text's color.
pub const BUTTON_FORMAT: &str = "(format 1 \"A080E0\" general nowrap)";

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

    /// The sheet the game shows, the Camera's `B3`: the exam's while a
    /// question is open, its certificate's after the eleventh, and where the
    /// certificate leads once the game has begun.
    pub fn shows(&self) -> &'static str {
        if self.begun {
            LEAD
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
        if begins {
            self.begun = true;
        }
        self.frames += 1;
        answered
    }

    /// A waiver, a person's edit of the Proctor's `B3`: the exam closed and
    /// the game begun, at the waiver's score and style.
    pub fn waive(&mut self) {
        self.waived = true;
        self.begun = true;
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
                    "The Series 777, question {ASKED} of {ASKED}, ungraded: it sets your firm's style."
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
/// The width of each button's column on Trading, in characters.
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

/// Why an order filled or did not, the Orders sheet's `B27`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Filled = 1,
    Closed = 2,
    NoStock = 3,
    NoCash = 4,
    NoShares = 5,
}

/// What the last frame showed, by which an order is judged: whether the market
/// was open, each stock's price now in whole cents, each pocket's cash, and
/// the time the person saw, as the ledger writes it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Shown {
    pub open: bool,
    pub prices: [u64; 4],
    pub cash: [u64; 2],
    pub when: String,
}

/// An order as it was taken, filled or refused: its pocket, its stock from 1,
/// 0 for none picked, and its trade from 1; what the last frame showed of the
/// market, the price, the pocket's cash and its shares of the stock; the
/// shares filled, a sale negative; and why.
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
}

impl Order {
    /// What Trading says of the order, `A13`, by the stock's name: what was
    /// bought or sold, or why nothing was.
    pub fn sentence(&self, name: &str) -> String {
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
/// pointer to a button's place and a command by its name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plate {
    pub pocket: u64,
    pub picked: u64,
    pub fills: Vec<Fill>,
    pub dividends: [[u64; 4]; 2],
    pub last: Option<Order>,
}

impl Default for Plate {
    fn default() -> Plate {
        Plate {
            pocket: FIRM,
            picked: 0,
            fills: Vec::new(),
            dividends: [[0; 4]; 2],
            last: None,
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
    /// last frame showed. One act a frame, a click's first (`PLATE.2`): a
    /// switch turns the pocket and takes nothing else, no pick and no order. A
    /// click picks its stock, as a pick command does, a click first; the order
    /// is a click's, else a trade command's on the stock picked before this
    /// frame, the one the person saw marked, and none for a frame with
    /// neither. A filled order is a fill, the pocket's cash after it found
    /// from `starts`, each pocket's start at this frame (and this frame's
    /// dividends, paid first). The order taken, filled or refused.
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
        let held = if stock == 0 {
            0
        } else {
            self.book(pocket, stock).shares
        };
        let filled: i64 = if !shown.open || stock == 0 {
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
        let outcome = if !shown.open {
            Outcome::Closed
        } else if stock == 0 {
            Outcome::NoStock
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
/// palette shows: each pick's by its stock's name, each trade's by the stock
/// picked's, or none yet, and the switch's by the pocket it turns to, the
/// other than `pocket`'s (`PLATE.2`).
pub fn command_words(names: &[String; 4], picked: u64, pocket: u64) -> Vec<String> {
    let stock = if picked == 0 {
        "the stock picked, none yet".to_string()
    } else {
        names[picked as usize - 1].clone()
    };
    let mut out: Vec<String> = names
        .iter()
        .map(|name| format!("Pick {name}: the buy and sell commands trade it"))
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

#[cfg(test)]
mod tests {
    use super::*;

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

    /// The views' dollars since `POT.10`'s hand test, two decimals as a text,
    /// each ending where a number ends in its column of 12 characters (the
    /// owner's second hand test), the thousands grouped by commas since
    /// `GUARD.1`'s: a price or a fee after the spaces that make 16 with twice
    /// its dollars' digits and its commas, a space being half a digit in the
    /// grid's font and a comma four fifths of a space (7.008, 3.561 and 2.818
    /// px at 13 px, measured); and a change after the spaces that make 14 with
    /// the same, then its sign, a gain's `+` or a loss's space and `-` (8.893
    /// and 8.760 px), about a space and a digit. Each spells its cents back,
    /// its groups three digits after the first's one to three, for every
    /// count of cents to $10,000, every change to $1,000 either way, and
    /// beside every power of ten to 10^15 cents; no price is padded from
    /// seven digits of dollars, and no change from seven.
    #[test]
    fn whole_cents_spell_two_decimals_padded() {
        assert_eq!(
            [0, 7, 999, 1_000, 65_536, 1_234_567].map(grouped),
            ["0", "7", "999", "1,000", "65,536", "1,234,567"]
        );
        let padded = |spaces: usize, text: &str| format!("{}{text}", " ".repeat(spaces));
        assert_eq!(
            [100, 499, 2_498, 9_993, 10_000, 0, 2_500].map(two_decimals),
            [
                padded(14, "1.00"),
                padded(14, "4.99"),
                padded(12, "24.98"),
                padded(12, "99.93"),
                padded(10, "100.00"),
                padded(14, "0.00"),
                padded(12, "25.00")
            ]
        );
        assert_eq!(
            [100_000, 550_000, 99_999_999, 100_000_000, 9_999_999_999].map(two_decimals),
            [
                padded(7, "1,000.00"),
                padded(7, "5,500.00"),
                padded(3, "999,999.99"),
                padded(0, "1,000,000.00"),
                padded(0, "99,999,999.99")
            ]
        );
        assert_eq!(
            two_decimals((1 << 53) - 1),
            "90,071,992,547,409.91",
            "2^53 cents less one, the most a double holds exactly"
        );
        assert_eq!(
            [17, -2, 0, -1_234, 1_234].map(signed_two_decimals),
            [
                padded(12, "+0.17"),
                padded(12, " -0.02"),
                padded(12, "+0.00"),
                padded(10, " -12.34"),
                padded(10, "+12.34")
            ]
        );
        assert_eq!(
            [-123_456, -99_999_999, 100_000_000].map(signed_two_decimals),
            [
                padded(5, " -1,234.56"),
                padded(1, " -999,999.99"),
                padded(0, "+1,000,000.00")
            ]
        );
        let digits = |dollars: u64| dollars.checked_ilog10().map_or(1, |d| d as usize + 1);
        let commas = |dollars: u64| (digits(dollars) - 1) / 3;
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
            let s = two_decimals(c);
            let text = s.trim_start_matches(' ');
            let spaces = s.len() - text.len();
            assert_eq!(
                spaces + 2 * digits(c / 100) + commas(c / 100),
                PRICE_UNITS,
                "{c} cents: {s:?}"
            );
            assert_eq!(cents(text), c, "{s:?}");
        }
        for p in 0..=15 {
            let ten = 10u64.pow(p);
            for c in [ten - 1, ten, ten + 1] {
                let s = two_decimals(c);
                let text = s.trim_start_matches(' ');
                assert_eq!(cents(text), c, "{s:?}");
                assert_eq!(
                    s.len() - text.len(),
                    PRICE_UNITS.saturating_sub(2 * digits(c / 100) + commas(c / 100)),
                    "{c} cents: {s:?}"
                );
            }
        }
        for c in -100_000..=100_000i64 {
            let s = signed_two_decimals(c);
            let text = s.trim_start_matches(' ');
            let (minus, rest) = match text.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, text.strip_prefix('+').expect("a sign")),
            };
            assert_eq!(minus, c < 0, "{s:?}");
            assert_eq!(cents(rest), c.unsigned_abs(), "{s:?}");
            let before = s.len() - text.len() - usize::from(minus);
            let dollars = c.unsigned_abs() / 100;
            assert_eq!(
                before + 2 * digits(dollars) + commas(dollars),
                CHANGE_UNITS,
                "{c} cents: {s:?}"
            );
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
            "The Series 777, question 11 of 11, ungraded: it sets your firm's style."
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
    /// waiver's five, $7,500, its style the first, at any answer.
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
        assert!(!w.open() && w.begun && w.shows() == LEAD);
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
}
