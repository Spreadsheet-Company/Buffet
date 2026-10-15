// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Spreadsheet Company

//! The game's cartridge, held (`docs/OMAKASE.md`, `COURSE.0`): `src/buffet.vla`
//! loaded through the engine from crates.io, as a stranger loads it, and
//! every frame of it held to the game written by hand, `buffet_oracle::buffet`.
//! Since `POT.1`: the cartridge is its source expanded; the calendar and the
//! dice at every frame of two years and two days; the same dice on the same
//! days at one frame a day; the frame read by the Day sheet alone (`BD-7`);
//! `describe` held to its golden; and the library's draws held to Park and
//! Miller's own published number. Since `POT.2`: the Market sheet's every
//! formula cell held bit for bit at those frames too, the hand calling the
//! same functions in the same order on the same platform; its settings
//! held to the hand's constants; the difficulty written 0, playing tier 0
//! through a year; and a hundred years at one frame a day, every day's index
//! held. Nothing here commits a value derived from `SIN` or `^`: each is
//! compared live, the module in the browser computing its own.

use alonzo::cartridge::{Cartridge, Seed};
use buffet_oracle::buffet::{self, Calendar, Market, DIFFICULTY, LOOK_BACK, WAVES, YEAR};
use buffet_oracle::{expand_source, lf, load, read, root, values};

const SOURCE: &str = "src/buffet.source.vla";
const CARTRIDGE: &str = "src/buffet.vla";
const DESCRIBE: &str = "oracle/testdata/describe/buffet.vla";

fn cartridge() -> Cartridge {
    load(CARTRIDGE).unwrap_or_else(|e| panic!("{CARTRIDGE} loads: {e}"))
}

fn seed(cart: &Cartridge) -> u64 {
    match cart.manifest().seed {
        Seed::Number(n) => u64::from(n),
        Seed::Host => panic!("the game's seed is a number, so every first game is the same market"),
    }
}

/// The Day, Dice and Market sheets at this frame, each cell its value as the
/// record prints it, held to the hand's calendar, dice and market for the
/// same frame.
fn held(cart: &Cartridge, cal: &Calendar, seed: u64, market: &Market) {
    let f = cal.frame;
    let day = values(cart, "Day", "B1:B7");
    let want = [
        ("B1", cal.frames_a_day),
        ("B3", cal.day),
        ("B4", cal.phase),
        ("B5", u64::from(cal.boundary())),
        ("B6", cal.year),
        ("B7", cal.day_of_year),
    ];
    for (addr, n) in want {
        assert_eq!(day.get(addr), Some(&n.to_string()), "frame {f}: Day!{addr}");
    }

    let dice = values(cart, "Dice", "B1:C9");
    let die = |addr: &str, z: u64| {
        assert_eq!(
            dice.get(addr),
            Some(&z.to_string()),
            "frame {f}, day {}: Dice!{addr}",
            cal.day
        );
    };
    let uniform = |addr: &str, z: u64| {
        let got: f64 = dice[addr]
            .parse()
            .unwrap_or_else(|e| panic!("Dice!{addr}: {e}"));
        assert_eq!(
            got.to_bits(),
            buffet::uniform(z).to_bits(),
            "frame {f}: Dice!{addr}, {got}"
        );
    };
    die("B1", buffet::market_die(seed, cal.day));
    for draw in 1..=4u64 {
        let z = buffet::market_draw(seed, cal.day, draw);
        die(&format!("B{}", draw + 1), z);
        uniform(&format!("C{}", draw + 1), z);
    }
    die("B7", buffet::exam_die(seed));
    for draw in 1..=2u64 {
        let z = buffet::exam_draw(seed, draw);
        die(&format!("B{}", draw + 7), z);
        uniform(&format!("C{}", draw + 7), z);
    }
    market_held(cart, cal, market, DIFFICULTY);
}

/// The Market sheet at this frame (`POT.2`): each wave's speed and tier 1's
/// phases, the cycle on the first day, and every formula of the two columns,
/// today and the look back, held bit for bit to the hand's market on the
/// calendar's day at a difficulty. The hand's value has 0.0 added, which
/// turns its rare -0 into the 0 a cell holds instead and moves nothing else.
fn market_held(cart: &Cartridge, cal: &Calendar, market: &Market, difficulty: u64) {
    let f = cal.frame;
    let got = values(cart, "Market", "B7:E25");
    let cell = |addr: &str, want: f64| {
        let text = got
            .get(addr)
            .unwrap_or_else(|| panic!("frame {f}: Market!{addr} holds no value"));
        let v: f64 = text
            .parse()
            .unwrap_or_else(|e| panic!("frame {f}: Market!{addr}, {text}: {e}"));
        assert_eq!(
            v.to_bits(),
            (want + 0.0).to_bits(),
            "frame {f}, day {}: Market!{addr} is {v} and the hand's {want}",
            cal.day
        );
    };
    let mut waves = vec![(YEAR, 0.0)];
    waves.extend(WAVES.iter().copied().zip(market.phases));
    for (row, (wave, phase)) in (7..).zip(&waves) {
        cell(&format!("D{row}"), wave.speed());
        if row > 7 {
            cell(&format!("E{row}"), *phase);
        }
    }
    cell("B22", market.first_cycle);
    let today = cal.day as f64;
    for (column, day) in [("B", today), ("C", today - LOOK_BACK as f64)] {
        cell(&format!("{column}14"), day);
        for (row, (wave, phase)) in (15..).zip(&waves) {
            cell(&format!("{column}{row}"), wave.term(*phase, day));
        }
        cell(&format!("{column}20"), Market::trend(day));
        cell(&format!("{column}21"), market.cycle(day));
        cell(&format!("{column}23"), Market::tier0(day));
        cell(&format!("{column}24"), market.tier1(day));
        cell(&format!("{column}25"), market.index(difficulty, day));
    }
}

/// The cartridge is the pass's expansion of its source, byte for byte but
/// its line ends: an edit of the source not expanded again fails here.
#[test]
fn the_cartridge_is_its_source_expanded() {
    let ex = expand_source(SOURCE).unwrap_or_else(|r| {
        panic!(
            "{SOURCE}, line {}: {} {}",
            r.line, r.refusal.id, r.refusal.text
        )
    });
    assert!(
        lf(&read(CARTRIDGE)) == lf(&ex.text),
        "{CARTRIDGE} is not {SOURCE} expanded: cargo run --release --example expand"
    );
}

/// A day is the manifest's rate in frames, a second, and a year 365 days:
/// the Day sheet's two settings, the hand's two constants and the manifest
/// agree, so that the teacher's dial, the Clock's rate, scales the game.
#[test]
fn a_day_is_a_second_at_the_manifests_rate() {
    let cart = cartridge();
    let rate = u64::from(cart.manifest().rate);
    assert_eq!(rate, buffet::FRAMES_A_DAY);
    let day = values(&cart, "Day", "B1:B2");
    assert_eq!(
        day.get("B1"),
        Some(&rate.to_string()),
        "Day!B1, the frames a day"
    );
    assert_eq!(
        day.get("B2"),
        Some(&buffet::DAYS_A_YEAR.to_string()),
        "Day!B2, the days a year"
    );
}

/// Every first game plays the contract's seed, the hand's: the manifest's
/// `(seed ...)` is `buffet::SEED`, chosen by the owner for a kind first year,
/// which the hand's own test holds (`COURSE.0`'s contract, 2026-10-14).
#[test]
fn the_first_game_plays_the_contracts_seed() {
    assert_eq!(seed(&cartridge()), buffet::SEED);
}

/// The Dice sheet states each jump in draws beside the multiplier that makes
/// it, and both are the hand's, whose own test raises 16807 to each.
#[test]
fn the_dice_sheet_states_its_jumps() {
    let cart = cartridge();
    let jumps = values(&cart, "Dice", "F1:F4");
    let want = [
        ("F1", buffet::DAY_STRIDE),
        ("F2", buffet::DAY_JUMP),
        ("F3", buffet::EXAM_OFFSET),
        ("F4", buffet::EXAM_JUMP),
    ];
    for (addr, n) in want {
        assert_eq!(jumps.get(addr), Some(&n.to_string()), "Dice!{addr}");
    }
}

/// Every frame of two years and two days, 21,960 frames, held to the hand:
/// the calendar at every frame, the market's dice changing on a day's first
/// frame and on no other, the exam's never, and the market a function of the
/// day, the same at every frame of a day.
#[test]
fn every_frame_of_two_years_is_the_calendar_the_dice_and_the_market_by_hand() {
    let mut cart = cartridge();
    let seed = seed(&cart);
    let market = Market::new(seed);
    let mut cal = Calendar::new(buffet::FRAMES_A_DAY, buffet::DAYS_A_YEAR);
    let frames = (2 * buffet::DAYS_A_YEAR + 2) * buffet::FRAMES_A_DAY;
    for _ in 0..frames {
        let step = cart.step(0);
        cal.tick();
        assert!(step.done && step.frame == cal.frame, "{}", step.row());
        held(&cart, &cal, seed, &market);
    }
    assert_eq!(
        (cal.year, cal.day_of_year),
        (3, 2),
        "the run ends on the third year's second day"
    );
}

/// At one frame a day, the Day sheet's setting written as a person's edit
/// before the first frame, every frame is a day's first, and each day rolls
/// the dice the same day rolls at thirty frames and prices the same market:
/// the market is a function of the day and the seed, never of the frame
/// (`BD-3`), so a floors fixture at one frame a day plays the game's own
/// market (`MISE.4`).
#[test]
fn at_one_frame_a_day_the_same_days_roll_the_same_dice() {
    let mut cart = cartridge();
    let seed = seed(&cart);
    let market = Market::new(seed);
    cart.write("(cell \"Day\" \"B1\" 1)").unwrap_or_else(|r| {
        panic!(
            "the frames a day, written: {} {}",
            r.refusal.id, r.refusal.text
        )
    });
    let mut cal = Calendar::new(1, buffet::DAYS_A_YEAR);
    for _ in 0..800 {
        cart.step(0);
        cal.tick();
        assert!(cal.boundary());
        held(&cart, &cal, seed, &market);
    }
}

/// The Market sheet states its settings and its waves, and they are the
/// hand's constants: pi, the drift, the difficulty and the look back, each
/// wave's period and amplitude, and the Kiddie Pool's fixed phase, 0.
#[test]
fn the_market_sheet_states_its_settings() {
    let cart = cartridge();
    let got = values(&cart, "Market", "B1:E11");
    let number = |addr: &str| -> f64 {
        got.get(addr)
            .unwrap_or_else(|| panic!("Market!{addr} holds no value"))
            .parse()
            .unwrap_or_else(|e| panic!("Market!{addr}: {e}"))
    };
    let want = [
        ("B1", buffet::PI),
        ("B2", buffet::DRIFT),
        ("B3", DIFFICULTY as f64),
        ("B4", LOOK_BACK as f64),
        ("E7", 0.0),
    ];
    for (addr, n) in want {
        assert_eq!(number(addr).to_bits(), n.to_bits(), "Market!{addr}");
    }
    let waves = [YEAR, WAVES[0], WAVES[1], WAVES[2], WAVES[3]];
    for (row, wave) in (7..).zip(waves) {
        assert_eq!(
            number(&format!("B{row}")),
            wave.days as f64,
            "Market!B{row}"
        );
        assert_eq!(
            number(&format!("C{row}")).to_bits(),
            wave.amplitude.to_bits(),
            "Market!C{row}"
        );
    }
}

/// A person who writes 0 into the difficulty before the first frame plays
/// tier 0, the Kiddie Pool, and every frame of a year is its market by hand,
/// the index tier 0's and tier 1 computed beside it (`AD-6`: any cell is a
/// person's to edit; `POT.6` makes this one a picker).
#[test]
fn at_difficulty_0_the_index_is_the_kiddie_pools() {
    let mut cart = cartridge();
    let market = Market::new(seed(&cart));
    cart.write("(cell \"Market\" \"B3\" 0)")
        .unwrap_or_else(|r| {
            panic!(
                "the difficulty, written: {} {}",
                r.refusal.id, r.refusal.text
            )
        });
    let mut cal = Calendar::new(buffet::FRAMES_A_DAY, buffet::DAYS_A_YEAR);
    for _ in 0..buffet::DAYS_A_YEAR * buffet::FRAMES_A_DAY {
        cart.step(0);
        cal.tick();
        market_held(&cart, &cal, &market, 0);
    }
}

/// A hundred years at one frame a day, every frame a day's first: the index
/// today and twenty days back held to the hand on every one of the 36,500
/// days, across the whole range a long game reaches, the drift raised past
/// the 36,000th power and the Kondratiev two turns on.
#[test]
fn a_hundred_years_at_one_frame_a_day_are_the_market_by_hand() {
    let mut cart = cartridge();
    let market = Market::new(seed(&cart));
    cart.write("(cell \"Day\" \"B1\" 1)").unwrap_or_else(|r| {
        panic!(
            "the frames a day, written: {} {}",
            r.refusal.id, r.refusal.text
        )
    });
    for day in 0..100 * buffet::DAYS_A_YEAR {
        cart.step(0);
        let got = values(&cart, "Market", "B25:C25");
        let today = day as f64;
        for (addr, at) in [("B25", today), ("C25", today - LOOK_BACK as f64)] {
            let v: f64 = got[addr]
                .parse()
                .unwrap_or_else(|e| panic!("day {day}: Market!{addr}: {e}"));
            assert_eq!(
                v.to_bits(),
                (market.index(DIFFICULTY, at) + 0.0).to_bits(),
                "day {day}: Market!{addr} is {v}"
            );
        }
    }
}

/// `BD-7`: the Day sheet alone reads the frame, `Clock!B1`, and its phase,
/// `Day!B4`, the one cell of it that changes every frame; and `describe`'s
/// census computes no shape every frame. So an ordinary frame computes the
/// calendar's cells and what they reach, and the rest wait for a day's first
/// frame. The step row cannot say so, since it counts the plan's cells
/// passed, computed or not (`vla-lang` 0.9.3's `Machine::step`).
#[test]
fn the_day_sheet_alone_reads_the_frame() {
    let text = lf(&read(CARTRIDGE));
    let mut formulas = 0;
    for row in text.lines() {
        let Some(rest) = row.strip_prefix("(formula \"") else {
            continue;
        };
        let Some((sheet, rest)) = rest.split_once('"') else {
            continue;
        };
        formulas += 1;
        if sheet == "Day" {
            continue;
        }
        for (what, target) in [
            ("the frame", ("Clock", "B", "1")),
            ("the phase", ("Day", "B", "4")),
        ] {
            assert!(
                !reads(rest, target),
                "{row}: the {sheet} sheet reads {what}"
            );
        }
    }
    assert!(formulas >= 10, "{formulas} formula rows read");
    // The scan itself, on a reference written each way and on its near misses.
    for yes in [
        "=Clock!B1+1",
        "=Clock!$B$1",
        "=IF(Clock!B$1=1,0,1)",
        "=Clock!$B1",
    ] {
        assert!(reads(yes, ("Clock", "B", "1")), "{yes}");
    }
    for no in ["=Clock!B3", "=Clock!B10", "=Clock!$B$3*2", "=Day!B1"] {
        assert!(!reads(no, ("Clock", "B", "1")), "{no}");
    }
    let census = cartridge().describe();
    assert!(census.contains("(every-frame 0)"), "{census}");
}

/// Whether a formula's text reads one cell of a sheet, `$` marks or none.
fn reads(text: &str, (sheet, column, row): (&str, &str, &str)) -> bool {
    let head = format!("{sheet}!");
    text.match_indices(head.as_str()).any(|(at, _)| {
        let rest = text[at + head.len()..].trim_start_matches('$');
        let Some(rest) = rest.strip_prefix(column) else {
            return false;
        };
        let rest = rest.trim_start_matches('$');
        let Some(rest) = rest.strip_prefix(row) else {
            return false;
        };
        !rest.starts_with(|c: char| c.is_ascii_digit())
    })
}

/// `describe`, the manifest, the devices, the budget and the census of the
/// plan, held whole to its golden; `BUFFET_BLESS=1 cargo test describe`
/// writes the golden again, to be read in its diff before it is committed.
#[test]
fn describe_holds_its_golden() {
    let got = cartridge().describe();
    let path = root().join(DESCRIBE);
    if std::env::var_os("BUFFET_BLESS").is_some() {
        std::fs::write(&path, &got).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        return;
    }
    let held = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "{}: {e}; BUFFET_BLESS=1 cargo test describe writes it",
            path.display()
        )
    });
    assert!(
        lf(&held) == lf(&got),
        "{DESCRIBE} is not the cartridge's describe; BUFFET_BLESS=1 cargo test describe, then read the diff:\n{got}"
    );
}

/// The library's verb, `draws`, ten thousand of them beneath a die of 1: the
/// last is Park and Miller's own published check, 1,043,618,065, computed by
/// the engine's step a cell at a time.
#[test]
fn the_draws_verb_meets_park_and_millers_ten_thousandth() {
    let source = concat!(
        "; SPDX-License-Identifier: AGPL-3.0-or-later\n",
        "(include \"kitchen.vla\")\n",
        "(cartridge \"stream\"\n",
        "  (spec 3)\n",
        "  (title \"Stream\")\n",
        "  (rate 0)\n",
        "  (screen grid 1 1)\n",
        "  (seed 1))\n",
        "(sheet \"Screen\" visible)\n",
        "(cell \"Stream\" \"B1\" 1)\n",
        "(draws \"Stream\" \"B2\" 10000)\n",
    );
    let files = vec![("src/kitchen.vla".to_string(), read("src/kitchen.vla"))];
    let ex = alonzo::pass::expand(source, "src/stream.source.vla", &files)
        .unwrap_or_else(|r| panic!("line {}: {} {}", r.line, r.refusal.id, r.refusal.text));
    let mut cart = Cartridge::load(&ex.text, "stream.vla", ex.text.len())
        .unwrap_or_else(|r| panic!("line {}: {} {}", r.line, r.refusal.id, r.refusal.text));
    assert!(cart.step(0).done);
    let last = values(&cart, "Stream", "B10001:C10001");
    assert_eq!(last.get("B10001").map(String::as_str), Some("1043618065"));
    let u: f64 = last["C10001"].parse().expect("a uniform");
    assert_eq!(u.to_bits(), buffet::uniform(1_043_618_065).to_bits());
}
