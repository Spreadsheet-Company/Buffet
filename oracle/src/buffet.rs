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

/// The minutes of the market's time a frame (`POT.10`), the Day sheet's `B1`:
/// two, so thirty frames an hour and an hour a second at the manifest's rate,
/// which a test holds; a person who writes 60 plays a frame an hour, and
/// 1,440 a frame a day.
pub const MINUTES_A_FRAME: u64 = 2;
/// The minutes a frame for a frame an hour and for a frame a day, the
/// oracle's longer runs.
pub const AN_HOUR: u64 = 60;
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
/// The closing mark, `B23`: the session's mark at the closing bell, and from
/// it to the next bell, the marks being 0 at the opening bell and one more at
/// each half hour of the session, 10:00 to 16:00 (the owner's word at the
/// hand test: "Why don't prices move on the half hour, too?").
pub const CLOSING_MARK: u64 = 13;
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

// The game opens on the opening bell, and the hand's calendar counts from it.
const _: () = assert!(FIRST_MINUTE == OPENING_BELL);
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

/// The calendar (`POT.1`, counted in minutes since `POT.10`): a frame at a
/// time, its minutes rolling into the hour, the hour into the day, the day
/// into the week and the year; the session the hand last heard the bell for,
/// and the one before it; and the session's mark, counted as its hours turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Calendar {
    pub minutes_a_frame: u64,
    pub days_a_year: u64,
    /// The frame last counted: 0 before the first.
    pub frame: u64,
    /// The day, from 0.
    pub day: u64,
    /// The hour of the day, from 0, and the minute of the hour.
    pub hour: u64,
    pub minute: u64,
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
    /// The session's mark: 0 at the opening bell, one more as each half hour
    /// of the session turns, [`CLOSING_MARK`] from the closing bell to the
    /// next.
    pub mark: u64,
}

impl Calendar {
    /// The calendar before its first frame, the minutes a frame apart.
    pub fn new(minutes_a_frame: u64, days_a_year: u64) -> Calendar {
        Calendar {
            minutes_a_frame,
            days_a_year,
            frame: 0,
            day: 0,
            hour: 0,
            minute: 0,
            weekday: 0,
            year: 1,
            day_of_year: 1,
            turned: false,
            session: 0,
            before: 0,
            mark: CLOSING_MARK,
        }
    }

    /// One frame on. The first frame stands at the first frame's time on day
    /// 0; each later frame is its minutes on, a half hour at a time: each
    /// half hour reached turns the session's mark while the session is
    /// today's, the hour's second rolling into the next hour and the day's
    /// last hour into the next day. Then, if the market is open and the
    /// session is not today's, the bell has rung: the session is today's, the
    /// one before it the last, and the mark 0.
    pub fn tick(&mut self) {
        self.frame += 1;
        self.turned = self.frame == 1;
        if self.frame == 1 {
            (self.hour, self.minute) = (FIRST_MINUTE / 60, FIRST_MINUTE % 60);
        } else {
            let mut left = self.minutes_a_frame;
            while left > 0 {
                let to_half = 30 - self.minute % 30;
                if left < to_half {
                    self.minute += left;
                    break;
                }
                left -= to_half;
                self.minute += to_half;
                if self.minute == 60 {
                    self.minute = 0;
                    self.hour += 1;
                    if self.hour == 24 {
                        self.hour = 0;
                        self.next_day();
                    }
                }
                if self.session == self.day && self.mark < CLOSING_MARK {
                    self.mark += 1;
                }
            }
        }
        if self.open() && (self.frame == 1 || self.session != self.day) {
            self.before = if self.frame == 1 {
                self.day
            } else {
                self.session
            };
            self.session = self.day;
            self.mark = 0;
        }
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

    /// The time of day to the half hour, in minutes after midnight, the Day
    /// sheet's `B15`.
    pub fn half_hour(&self) -> u64 {
        self.hour * 60 + if self.minute >= 30 { 30 } else { 0 }
    }

    /// The time in minutes from day 0's midnight, `B4`, and to the half hour,
    /// `B14`.
    pub fn minutes(&self) -> u64 {
        self.day * A_DAY + self.hour * 60 + self.minute
    }
    pub fn half_hours(&self) -> u64 {
        self.day * A_DAY + self.half_hour()
    }

    /// Whether the market is open, `B19`: a trading day, from its opening
    /// bell's half hour to its closing bell's.
    pub fn open(&self) -> bool {
        self.weekday < TRADING_DAYS && (OPENING_BELL..CLOSING_BELL).contains(&self.half_hour())
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
/// cycle on that day.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Market {
    /// Tier 1's phases in radians, Kitchin's first: the Market sheet's
    /// `E8:E11`, taken on day 0 and kept.
    pub phases: [f64; 4],
    /// Tier 1's cycle on day 0, `B22`, which tier 1 divides by so that it
    /// opens at 1.
    pub first_cycle: f64,
}

impl Market {
    /// The market of `seed`, its phases from the hand's own dice.
    pub fn new(seed: u64) -> Market {
        let phases = std::array::from_fn(|k| {
            let draw = market_draw(seed, 0, k as u64 + 1);
            2.0 * PI * uniform(draw)
        });
        let mut market = Market {
            phases,
            first_cycle: 1.0,
        };
        market.first_cycle = market.cycle(0.0);
        market
    }

    /// Tier 1's four terms on `day`, Kitchin's first: `B16:B19` or `C16:C19`.
    pub fn terms(&self, day: f64) -> [f64; 4] {
        std::array::from_fn(|k| WAVES[k].term(self.phases[k], day))
    }

    /// Tier 1's cycle on `day`, one and its four waves: `=1+B16+B17+B18+B19`.
    pub fn cycle(&self, day: f64) -> f64 {
        let [kitchin, juglar, kuznets, kondratiev] = self.terms(day);
        1.0 + kitchin + juglar + kuznets + kondratiev
    }

    /// The trend on `day`, the drift raised to the day: `=$B$2^B14`.
    pub fn trend(day: f64) -> f64 {
        DRIFT.powf(day)
    }

    /// Tier 0 on `day`, the Kiddie Pool, the trend times one and the year's
    /// wave: `=B20*(1+B15)`.
    pub fn tier0(day: f64) -> f64 {
        Market::trend(day) * (1.0 + YEAR.term(0.0, day))
    }

    /// Tier 1 on `day`, the Day Trader, the trend times the cycle over the
    /// cycle on the first day, so that it opens at exactly 1:
    /// `=B20*B21/$B$22`.
    pub fn tier1(&self, day: f64) -> f64 {
        Market::trend(day) * self.cycle(day) / self.first_cycle
    }

    /// The index on `day`, as the difficulty picks, 0 tier 0 and 1 tier 1:
    /// `=CHOOSE($B$3+1,B23,B24)`. A tier not yet built is `POT.6`'s to refuse.
    pub fn index(&self, difficulty: u64, day: f64) -> f64 {
        match difficulty {
            0 => Market::tier0(day),
            1 => self.tier1(day),
            _ => panic!("tier {difficulty} is not built"),
        }
    }
}

/// The half hour's swing (`POT.10`), the Market sheet's `B5`: the most one
/// half hour of a session moves the index, 0.28% of it, which wanders a
/// session as far as 0.4% an hour did and keeps the first year above 0.98 at
/// every half hour.
pub const SWING: f64 = 0.0028;
/// The first of the market's draws a session's steps take each day, the
/// fifth: `POT.2`'s four phases take the four before it.
pub const SESSION_DRAW: u64 = 5;
/// The widest a bridge of thirteen steps strays from its line, in steps: at
/// the sixth or the seventh mark, `2m(13-m)/13`, every step against the rest.
pub const WIDEST: f64 = 84.0 / 13.0;

/// A session's path at a frame (`POT.10`), the Market sheet's rows 28 to 45.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Path {
    /// Each half hour's step, `B28:B40`: the uniform of the day's market draw
    /// 5 to 17 set between -1 and 1, `2u-1`.
    pub steps: [f64; CLOSING_MARK as usize],
    /// The steps so far, `C28:C40`: the first step, then each the one before
    /// and its own.
    pub sums: [f64; CLOSING_MARK as usize],
    /// The mark's share of the session, `B43`, from 0 at the bell to 1.
    pub share: f64,
    /// The swing at the mark, `B44`.
    pub swing: f64,
    /// The index now, `B45`.
    pub now: f64,
}

impl Market {
    /// The session's path at a calendar's frame (`POT.10`), each value in its
    /// cell's operations and order: from the session before's close to the
    /// session's own, weighed by the mark's share, times one and the swing,
    /// `=(D25*(1-B43)+B25*B43)*(1+B44)`; the swing the half hour's swing times
    /// the steps so far less the share of all thirteen,
    /// `=$B$5*(CHOOSE(B42+1,0,C28,...,C40)-B43*C40)`. The steps are the
    /// calendar day's draws, which a session reads in its own hours and which
    /// the swing multiplies away outside them.
    pub fn path(&self, seed: u64, difficulty: u64, cal: &Calendar) -> Path {
        let steps: [f64; CLOSING_MARK as usize] = std::array::from_fn(|k| {
            2.0 * uniform(market_draw(seed, cal.day, SESSION_DRAW + k as u64)) - 1.0
        });
        let mut sums = steps;
        for k in 1..sums.len() {
            sums[k] = sums[k - 1] + steps[k];
        }
        let share = cal.mark as f64 / CLOSING_MARK as f64;
        let so_far = match cal.mark {
            0 => 0.0,
            m => sums[m as usize - 1],
        };
        let swing = SWING * (so_far - share * sums[sums.len() - 1]);
        let last = self.index(difficulty, cal.before as f64);
        let close = self.index(difficulty, cal.session as f64);
        let now = (last * (1.0 - share) + close * share) * (1.0 + swing);
        Path {
            steps,
            sums,
            share,
            swing,
            now,
        }
    }
}

/// The least the index can be at any half hour a price reads, at any seed and
/// at either tier built (`STATION.1`'s decision 8, `POT.10`): tier 1's cycle
/// and its cycle on the first day are each between one less and one more than
/// the amplitudes' sum, a half, so their quotient is at least a third; the
/// trend is at least the drift to day 0, the earliest day a price reads, since
/// the first session opens from its own close; tier 0 is never below nine
/// tenths of its trend; and a half hour's index strays below the line between
/// two closes by at most the half hour's swing times the widest a bridge of
/// thirteen steps goes.
pub fn least_index() -> f64 {
    let swing: f64 = WAVES.iter().map(|w| w.amplitude).sum();
    Market::trend(0.0) * (1.0 - swing) / (1.0 + swing) * (1.0 - SWING * WIDEST)
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

/// A price's spaces and twice its dollars' digits (`POT.10`'s second hand
/// test): in a column of [`DOLLARS_COLUMN`] characters, the text then ends
/// where a number ends, the viewport drawing a text from its cell's left and a
/// number to its right, a space being half a digit to within a sixtieth.
pub const PRICE_UNITS: usize = 16;

/// A change's spaces before its sign and twice its dollars' digits, the sign
/// being about a space and a digit wide.
pub const CHANGE_UNITS: usize = 14;

/// Whole cents, never negative, as a view spells them since `POT.10`'s hand
/// test, the library's `dollars`: the dollars, a point and the cents in two
/// digits, after the spaces that make [`PRICE_UNITS`] with twice the dollars'
/// digits, and none from eight digits. The cell chooses its padding by
/// comparing the cents with powers of ten; the hand counts the dollars' digits.
pub fn two_decimals(cents: u64) -> String {
    let dollars = (cents / 100).to_string();
    let pad = " ".repeat(PRICE_UNITS.saturating_sub(2 * dollars.len()));
    format!("{pad}{dollars}.{:02}", cents % 100)
}

/// A change in whole cents as the Portfolio spells it (`POT.10`): two decimals
/// and its sign, a gain's or nothing's `+` and a loss's space and `-`, each as
/// wide as the other in the grid's font, after the spaces that make
/// [`CHANGE_UNITS`] with twice its dollars' digits, and none from seven.
pub fn signed_two_decimals(cents: i64) -> String {
    let sign = if cents < 0 { " -" } else { "+" };
    let c = cents.unsigned_abs();
    let dollars = (c / 100).to_string();
    let pad = " ".repeat(CHANGE_UNITS.saturating_sub(2 * dollars.len()));
    format!("{pad}{sign}{dollars}.{:02}", c % 100)
}

/// A time of day in minutes after midnight as the views show it (`POT.10`):
/// the hour, a colon and the minutes in two digits, 9:30 and 16:00.
pub fn time_shown(minutes: u64) -> String {
    format!("{}:{:02}", minutes / 60, minutes % 60)
}

/// A view's date at a frame (`FRONT.1`, `POT.10`): the day's name, the year
/// and the day of the year, each from 1, and the time to the half hour, as of
/// which its numbers stand, none of them audited.
pub fn as_of(cal: &Calendar) -> String {
    format!(
        "As of {}, year {}, day {}, {} (unaudited)",
        DAY_NAMES[cal.weekday as usize],
        cal.year,
        cal.day_of_year,
        time_shown(cal.half_hour())
    )
}

/// The bell's four lines (`POT.10`), a view's row 2: while the market is
/// open; on a trading day before its bell; after the close, Monday to
/// Thursday; and from Friday's close to Monday's bell, the last two the
/// owner's words at the hand test.
pub const BELL_OPEN: &str = "The market is open until 16:00.";
pub const BELL_BEFORE: &str = "The market opens at 9:30.";
pub const BELL_CLOSED: &str = "The market has closed. Analysts are analyzing the aftermath.";
pub const BELL_WEEKEND: &str = "The market is closed until Monday at 9:30.";

/// The bell's line at a frame. The cell compares the session's day with the
/// day; the hand asks whether the market is open, then the time and the
/// weekday.
pub fn bell(cal: &Calendar) -> &'static str {
    if cal.open() {
        BELL_OPEN
    } else if cal.weekday < TRADING_DAYS && cal.half_hour() < OPENING_BELL {
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
/// Hustle sheet's `B1`: $25, the contract's.
pub const FEE: u64 = 2_500;
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

/// What the host writes into the Input sheet in a frame, as far as the
/// hustle reads it (Alonzo's `SPEC.md` section 3.3, version 4): the sheet
/// shown; the pointer's column and row on it, 0 off its cells; the left
/// button held; whether the key is held; and whether it was bound when the
/// row was written, by the Keys sheet the host read after the last step.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inputs {
    pub sheet: String,
    pub column: u64,
    pub row: u64,
    pub left: bool,
    pub key: bool,
    pub bound: bool,
}

/// The side hustle by hand (`BACK.1`, a week and a day's work since
/// `POT.10`): its claims in the order they were made, each the day it counted
/// against and the half hour, in minutes from day 0's midnight, the person
/// saw when making it. The cells judge a press by `shut`'s twin and keep the
/// day the next prospectus arrives and the time the last one is done; the
/// hand keeps the list and reads its length and its last claim.
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
        let (sheet, column, row) = BUTTON;
        now.left
            && !last.left
            && now.sheet.eq_ignore_ascii_case(sheet)
            && now.column == column
            && now.row == row
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
        now.key && now.bound && !(last.key && last.bound)
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
    /// frame's, and counts against that day, its work running from the half
    /// hour the person saw. Whether a claim was made.
    pub fn frame(&mut self, now: &Inputs, last: &Inputs, day_shown: u64, half_shown: u64) -> bool {
        let pressed = Hustle::clicked(now, last) || Hustle::keyed(now, last);
        let claim = pressed && self.open(day_shown);
        if claim {
            self.claims.push((day_shown, half_shown));
        }
        claim
    }

    /// Whether the last claim's work is under way at the half hour `at`, in
    /// minutes from day 0's midnight: a day's work not yet done.
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
        let at = cal.half_hours();
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
    /// owner's second hand test): a price or a fee after the spaces that make
    /// 16 with twice its dollars' digits, a space being half a digit in the
    /// grid's font (7.008 and 3.561 px at 13 px, measured); and a change after
    /// the spaces that make 14 with twice its dollars' digits, then its sign,
    /// a gain's `+` or a loss's space and `-` (8.893 and 8.760 px), about a
    /// space and a digit. Each spells its cents back, for every count of
    /// cents to $10,000 and every change to $1,000 either way; no price is
    /// padded from eight digits of dollars, and no change from seven.
    #[test]
    fn whole_cents_spell_two_decimals_padded() {
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
            [99_999_999, 100_000_000, 9_999_999_999].map(two_decimals),
            [
                padded(4, "999999.99"),
                padded(2, "1000000.00"),
                padded(0, "99999999.99")
            ]
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
            [-99_999_999, 100_000_000].map(signed_two_decimals),
            [padded(2, " -999999.99"), padded(0, "+1000000.00")]
        );
        let digits = |dollars: u64| dollars.checked_ilog10().map_or(1, |d| d as usize + 1);
        // The cents a spelling reads back, from its dollars and its two
        // digits of cents, read as whole numbers.
        let cents = |text: &str| -> u64 {
            let (whole, part) = text.split_once('.').expect("a point");
            assert_eq!(part.len(), 2, "{text:?}: two digits of cents");
            whole.parse::<u64>().expect("dollars") * 100 + part.parse::<u64>().expect("cents")
        };
        for c in 0..1_000_000u64 {
            let s = two_decimals(c);
            let text = s.trim_start_matches(' ');
            let spaces = s.len() - text.len();
            assert_eq!(
                spaces + 2 * digits(c / 100),
                PRICE_UNITS,
                "{c} cents: {s:?}"
            );
            assert_eq!(cents(text), c, "{s:?}");
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
            assert_eq!(
                before + 2 * digits(c.unsigned_abs() / 100),
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
        let mut c = Calendar::new(MINUTES_A_FRAME, DAYS_A_YEAR);
        (c.day, c.weekday) = (day, day % DAYS_A_WEEK);
        (c.hour, c.minute) = (minutes / 60, minutes % 60);
        c
    }

    /// The hustle by hand (`BACK.1`; a week and a day's work since `POT.10`):
    /// a press claims once a week, against the day the person saw, and four
    /// times a run, its fee paid a day after the half hour the person saw. A
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
        let key = Inputs {
            key: true,
            bound: true,
            ..Inputs::default()
        };
        let unbound = Inputs {
            key: true,
            ..Inputs::default()
        };
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

    /// The calendar's turns at two minutes a frame (`POT.10`): the first frame
    /// is Monday's opening bell on day 0, the session its own and its mark 0;
    /// the sixteenth 10:00, the first half hour's mark; the 196th the closing
    /// bell, the thirteenth;
    /// the 436th midnight, Tuesday, the dice's frame, the session still
    /// Monday's; the 721st Tuesday's bell, its session Tuesday's and the one
    /// before Monday's; Saturday's midnight, Friday's session standing through
    /// the weekend; Monday's bell after it, the session before it Friday's;
    /// and the year's turn, day 365 a Tuesday.
    #[test]
    fn the_calendar_turns_at_its_bells_and_its_midnights() {
        let mut c = Calendar::new(MINUTES_A_FRAME, DAYS_A_YEAR);
        let wanted = [1, 2, 16, 195, 196, 435, 436, 721, 3_316, 5_041, 262_516];
        let mut at = Vec::new();
        while c.frame < 262_516 {
            c.tick();
            if wanted.contains(&c.frame) {
                at.push((
                    c.frame,
                    c.day,
                    c.weekday,
                    time_shown(c.hour * 60 + c.minute),
                    c.turned,
                    c.open(),
                    (c.session, c.before, c.mark),
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
                row(1, 0, 0, "9:30", true, true, (0, 0, 0), (1, 1)),
                row(2, 0, 0, "9:32", false, true, (0, 0, 0), (1, 1)),
                row(16, 0, 0, "10:00", false, true, (0, 0, 1), (1, 1)),
                row(195, 0, 0, "15:58", false, true, (0, 0, 12), (1, 1)),
                row(196, 0, 0, "16:00", false, false, (0, 0, 13), (1, 1)),
                row(435, 0, 0, "23:58", false, false, (0, 0, 13), (1, 1)),
                row(436, 1, 1, "0:00", true, false, (0, 0, 13), (1, 2)),
                row(721, 1, 1, "9:30", false, true, (1, 0, 0), (1, 2)),
                row(3_316, 5, 5, "0:00", true, false, (4, 3, 13), (1, 6)),
                row(5_041, 7, 0, "9:30", false, true, (7, 4, 0), (1, 8)),
                row(262_516, 365, 1, "0:00", true, false, (364, 361, 13), (2, 1)),
            ]
        );
    }

    /// The same days at a frame an hour and at a frame a day: the hour's
    /// frames see a session's even marks, from 9:30 to 16:30, two half hours
    /// turning in each, and the closing mark, and the day's frames every bell,
    /// a Saturday's showing Friday's session.
    #[test]
    fn an_hour_or_a_day_a_frame_counts_the_same_sessions() {
        let mut h = Calendar::new(AN_HOUR, DAYS_A_YEAR);
        let mut marks = Vec::new();
        for _ in 0..8 {
            h.tick();
            marks.push((time_shown(h.hour * 60 + h.minute), h.mark, h.open()));
        }
        assert_eq!(marks.first(), Some(&("9:30".to_string(), 0, true)));
        assert_eq!(marks.last(), Some(&("16:30".to_string(), 13, false)));
        assert_eq!(
            marks.iter().map(|m| m.1).collect::<Vec<_>>(),
            [0, 2, 4, 6, 8, 10, 12, 13]
        );
        let mut d = Calendar::new(A_DAY, DAYS_A_YEAR);
        let mut sessions = Vec::new();
        for _ in 0..9 {
            d.tick();
            assert!(d.turned && (d.hour, d.minute) == (9, 30));
            sessions.push((d.day, d.session, d.before, d.mark));
        }
        assert_eq!(
            sessions,
            [
                (0, 0, 0, 0),
                (1, 1, 0, 0),
                (2, 2, 1, 0),
                (3, 3, 2, 0),
                (4, 4, 3, 0),
                (5, 4, 3, 13),
                (6, 4, 3, 13),
                (7, 7, 4, 0),
                (8, 8, 7, 0),
            ]
        );
    }

    /// A session leaves the last close and reaches its own exactly (`POT.10`):
    /// over every session of the first game's first year, at a frame each half
    /// hour, the index at the opening bell is the session before's close to
    /// the bit, at the closing bell the session's own, and at every mark
    /// between no further off the line between them than the half hour's
    /// swing times the widest a bridge of thirteen steps strays.
    #[test]
    fn a_session_leaves_the_last_close_and_reaches_its_own() {
        let market = Market::new(SEED);
        let mut c = Calendar::new(30, DAYS_A_YEAR);
        let (mut opens, mut closes, mut closed) = (0, 0, None);
        while c.day < DAYS_A_YEAR {
            c.tick();
            if c.session != c.day {
                continue;
            }
            let p = market.path(SEED, DIFFICULTY, &c);
            let last = market.index(DIFFICULTY, c.before as f64);
            let close = market.index(DIFFICULTY, c.session as f64);
            if c.mark == 0 {
                assert_eq!(p.now.to_bits(), last.to_bits(), "day {}'s bell", c.day);
                opens += 1;
            }
            if c.mark == CLOSING_MARK && closed != Some(c.day) {
                assert_eq!(p.now.to_bits(), close.to_bits(), "day {}'s close", c.day);
                (closes, closed) = (closes + 1, Some(c.day));
            }
            let line = last * (1.0 - p.share) + close * p.share;
            assert!(
                (p.now / line - 1.0).abs() <= SWING * WIDEST,
                "day {}, mark {}: {} off its line {line}",
                c.day,
                c.mark,
                p.now
            );
        }
        assert_eq!((opens, closes), (261, 261), "the first year's sessions");
    }

    /// The widest a bridge of thirteen steps strays, `84/13` of a step: over
    /// every pattern of steps at -1 and 1, the extremes of a sum linear in
    /// each step, the most `W(m) - (m/13)W(13)` reaches at any mark.
    #[test]
    fn a_bridge_of_thirteen_steps_strays_at_most_84_13_of_a_step() {
        let k = CLOSING_MARK as usize;
        let mut most = 0.0f64;
        for pattern in 0..1u32 << k {
            let steps: Vec<f64> = (0..k)
                .map(|j| if pattern >> j & 1 == 1 { 1.0 } else { -1.0 })
                .collect();
            let all: f64 = steps.iter().sum();
            for m in 0..=k {
                let so_far: f64 = steps[..m].iter().sum();
                most = most.max((so_far - m as f64 / k as f64 * all).abs());
            }
        }
        assert!((most - WIDEST).abs() < 1e-12, "{most}");
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

    /// The first game's year is kind, as the owner chose its seed to be: the
    /// index dips no more than 2% and only in the first four months, at the
    /// close of any day and at any half hour of any session (`POT.10`), the weather
    /// turns within them, and the report card at day 365 reads at least 5% up.
    /// A change to the waves or the session's swing that spoils the first year
    /// fails here, and the seed is chosen again with it.
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
        let mut c = Calendar::new(30, DAYS_A_YEAR);
        let (mut hour_low, mut hour_day) = (f64::INFINITY, 0);
        while c.day < DAYS_A_YEAR {
            c.tick();
            let now = market.path(SEED, DIFFICULTY, &c).now;
            if now < hour_low {
                (hour_low, hour_day) = (now, c.day);
            }
        }
        assert!(
            hour_low > 0.98 && hour_day <= 120,
            "the year's lowest half hour, {hour_low}, on day {hour_day}"
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

    /// No seed prices a rung at or below zero (`STATION.1`'s decision 8):
    /// at the least the index can be at any half hour (`POT.10`), a third of
    /// the trend at day 0 less the widest a half hour strays from its line, the four
    /// rungs are 11, 94, 818 and 4,327 cents, where a day's least made them 11,
    /// 96, 833 and 4,386.
    #[test]
    fn no_seed_prices_a_rung_at_or_below_zero() {
        let least = least_index();
        assert!(
            (least - (1.0 - SWING * WIDEST) / 3.0).abs() < 1e-15,
            "{least}"
        );
        let floor: Vec<u64> = RUNGS.iter().map(|r| r.price_at(least)).collect();
        assert_eq!(floor, [11, 94, 818, 4_327]);
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
}
