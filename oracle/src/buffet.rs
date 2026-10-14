// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Spreadsheet Company

//! Buffet by hand (`OMAKASE.md`, `MISE.2`): the game written a second time,
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

/// The frames a market day, the manifest's rate: a day a second.
pub const FRAMES_A_DAY: u64 = 30;
/// The days a year, every day a market day.
pub const DAYS_A_YEAR: u64 = 365;

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

/// The calendar, counted a frame at a time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Calendar {
    pub frames_a_day: u64,
    pub days_a_year: u64,
    /// The frame last counted: 0 before the first.
    pub frame: u64,
    /// The market's day, from 0.
    pub day: u64,
    /// The frame within the day, from 0.
    pub phase: u64,
    /// The year, from 1.
    pub year: u64,
    /// The day of the year, from 1.
    pub day_of_year: u64,
}

impl Calendar {
    /// The calendar before its first frame.
    pub fn new(frames_a_day: u64, days_a_year: u64) -> Calendar {
        Calendar {
            frames_a_day,
            days_a_year,
            frame: 0,
            day: 0,
            phase: 0,
            year: 1,
            day_of_year: 1,
        }
    }

    /// One frame on. The first frame opens the first day; each later frame
    /// is a phase on, a day's last phase rolling into the next day's first,
    /// and the year's last day into the next year's first.
    pub fn tick(&mut self) {
        self.frame += 1;
        if self.frame == 1 {
            return;
        }
        self.phase += 1;
        if self.phase < self.frames_a_day {
            return;
        }
        self.phase = 0;
        self.day += 1;
        self.day_of_year += 1;
        if self.day_of_year > self.days_a_year {
            self.day_of_year = 1;
            self.year += 1;
        }
    }

    /// Whether this frame opens a day, the one frame a day the market moves.
    pub fn boundary(&self) -> bool {
        self.phase == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    /// The calendar's turns: the first frame opens day 0, the thirty-first
    /// opens day 1, and the 10,951st opens the second year.
    #[test]
    fn the_calendar_turns_at_a_days_first_frame() {
        let mut c = Calendar::new(FRAMES_A_DAY, DAYS_A_YEAR);
        let mut at = Vec::new();
        for _ in 0..10_951 {
            c.tick();
            if [1, 2, 30, 31, 10_950, 10_951].contains(&c.frame) {
                at.push((c.frame, c.day, c.phase, c.boundary(), c.year, c.day_of_year));
            }
        }
        assert_eq!(
            at,
            vec![
                (1, 0, 0, true, 1, 1),
                (2, 0, 1, false, 1, 1),
                (30, 0, 29, false, 1, 1),
                (31, 1, 0, true, 1, 2),
                (10_950, 364, 29, false, 1, 365),
                (10_951, 365, 0, true, 2, 1),
            ]
        );
    }

    /// Seed 777's first die and draw, the numbers the Dice sheet shows on the
    /// first frame of the first game.
    #[test]
    fn seed_777_opens_on_these_dice() {
        assert_eq!(market_die(777, 0), 67_119_591);
        assert_eq!(market_draw(777, 0, 1), 650_051_262);
        assert_eq!(market_die(777, 1), 1_933_266_100);
        assert_eq!(exam_die(777), 904_580_292);
        assert_eq!(exam_draw(777, 1), 1_244_230_531);
    }
}
