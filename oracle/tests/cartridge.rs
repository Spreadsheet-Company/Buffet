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
//! held. Since `STATION.1`: the index yesterday beside it, and the Stocks
//! sheet, every rung's price today and yesterday, its dividend and the
//! dividend today, held at every one of those frames and days; the rungs'
//! settings; the opening at the starts; the least price of every rung at or
//! above the bound no seed passes; and the dividend on exactly four days a
//! year. Nothing here commits a value derived from `SIN` or `^`: each is
//! compared live, the module in the browser computing its own. Since
//! `MENU.1`: the Disclosures sheet, the fine print, its row shown and the
//! text it picks held at every one of those frames, the row the hand's; its
//! rows counted by its extent; every row picked in its turn; a row added to
//! the content file expanding with no other edit; the one picker's limit and
//! the tripwire before it; and every sheet written bare, the content hidden.

use std::collections::{BTreeMap, BTreeSet};

use alonzo::cartridge::{Cartridge, Seed};
use buffet_oracle::buffet::{
    self, disclosure, least_index, Calendar, Market, DAYS_A_DISCLOSURE, DIFFICULTY, DIVIDEND_DAYS,
    LOOK_BACK, RUNGS, WAVES, YEAR,
};
use buffet_oracle::{expand_source, gather, lf, load, read, root, values};
use vla_lang::machine::Viewed;

const SOURCE: &str = "src/buffet.source.vla";
const CARTRIDGE: &str = "src/buffet.vla";
const DESCRIBE: &str = "oracle/testdata/describe/buffet.vla";
/// The fine print's content file, and the sheet its call writes (`MENU.1`).
const DISCLOSURES: &str = "src/content/disclosures.vla";
/// The content sheets, each a file of `src/content/`, hidden among the tabs.
const CONTENT_SHEETS: [&str; 1] = ["Disclosures"];

fn cartridge() -> Cartridge {
    load(CARTRIDGE).unwrap_or_else(|e| panic!("{CARTRIDGE} loads: {e}"))
}

fn seed(cart: &Cartridge) -> u64 {
    match cart.manifest().seed {
        Seed::Number(n) => u64::from(n),
        Seed::Host => panic!("the game's seed is a number, so every first game is the same market"),
    }
}

/// The Disclosures sheet's texts, its column A below the headers, row 2 first:
/// the content's, written once, in `src/content/`.
fn disclosure_texts(cart: &Cartridge) -> Vec<String> {
    let got = values(cart, "Disclosures", "A2:A2000");
    let mut rows: Vec<(u64, String)> = got
        .into_iter()
        .map(|(addr, text)| (addr[1..].parse().expect("a row of column A"), text))
        .collect();
    rows.sort();
    for (k, (row, _)) in rows.iter().enumerate() {
        assert_eq!(
            *row,
            k as u64 + 2,
            "the Disclosures sheet's rows run on from row 2"
        );
    }
    rows.into_iter().map(|(_, text)| text).collect()
}

/// The fine print at this frame (`MENU.1`): the row shown, `F2`, the hand's
/// for the calendar's day, and the text shown, `F3`, that row's.
fn disclosures_held(cart: &Cartridge, cal: &Calendar, texts: &[String], days: u64) {
    let got = values(cart, "Disclosures", "F2:F3");
    let row = disclosure(cal.day, texts.len() as u64, days);
    assert_eq!(
        got.get("F2"),
        Some(&row.to_string()),
        "frame {}, day {}: Disclosures!F2",
        cal.frame,
        cal.day
    );
    assert_eq!(
        got.get("F3"),
        Some(&texts[row as usize - 1]),
        "frame {}, day {}: Disclosures!F3",
        cal.frame,
        cal.day
    );
}

/// The Day, Dice and Market sheets at this frame, each cell its value as the
/// record prints it, held to the hand's calendar, dice and market for the
/// same frame; and the Stocks and the Disclosures sheets with them.
fn held(cart: &Cartridge, cal: &Calendar, seed: u64, market: &Market, texts: &[String]) {
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
    stocks_held(cart, cal, market, DIFFICULTY);
    disclosures_held(cart, cal, texts, DAYS_A_DISCLOSURE);
}

/// The Market sheet at this frame (`POT.2`): each wave's speed and tier 1's
/// phases, the cycle on the first day, and every formula of the three
/// columns, today, the look back and yesterday (`STATION.1`'s), held bit for
/// bit to the hand's market on the calendar's day at a difficulty. The
/// hand's value has 0.0 added, which turns its rare -0 into the 0 a cell
/// holds instead and moves nothing else.
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
    let days = [
        ("B", today),
        ("C", today - LOOK_BACK as f64),
        ("D", today - 1.0),
    ];
    for (column, day) in days {
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

/// The Stocks sheet's workings at this frame (`STATION.1`): each rung's price
/// today and yesterday, its dividend and the dividend today, held to the
/// hand's rungs on the calendar's day at a difficulty, whole cents compared
/// as the record prints them. The window is the four columns that compute:
/// a view costs a debug build a tenth of a millisecond or more, the more the
/// more cells it holds, at every one of the frames the tests walk, and the
/// settings never move (`the_stocks_sheet_states_its_rungs` reads them).
/// The values read, for a caller that counts them.
fn stocks_held(
    cart: &Cartridge,
    cal: &Calendar,
    market: &Market,
    difficulty: u64,
) -> BTreeMap<String, String> {
    let f = cal.frame;
    let got = values(cart, "Stocks", "E5:H8");
    let cents = |addr: &str, want: u64| {
        assert_eq!(
            got.get(addr),
            Some(&want.to_string()),
            "frame {f}, day {}: Stocks!{addr}",
            cal.day
        );
    };
    let today = cal.day as f64;
    for (row, rung) in (5..).zip(&RUNGS) {
        cents(&format!("E{row}"), rung.price(market, difficulty, today));
        cents(
            &format!("F{row}"),
            rung.price(market, difficulty, today - 1.0),
        );
        cents(&format!("G{row}"), rung.dividend);
        cents(&format!("H{row}"), rung.dividend_on(cal.day_of_year));
    }
    got
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
    let texts = disclosure_texts(&cart);
    let mut cal = Calendar::new(buffet::FRAMES_A_DAY, buffet::DAYS_A_YEAR);
    let frames = (2 * buffet::DAYS_A_YEAR + 2) * buffet::FRAMES_A_DAY;
    for _ in 0..frames {
        let step = cart.step(0);
        cal.tick();
        assert!(step.done && step.frame == cal.frame, "{}", step.row());
        held(&cart, &cal, seed, &market, &texts);
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
    let texts = disclosure_texts(&cart);
    let mut cal = Calendar::new(1, buffet::DAYS_A_YEAR);
    for _ in 0..800 {
        cart.step(0);
        cal.tick();
        assert!(cal.boundary());
        held(&cart, &cal, seed, &market, &texts);
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

/// The Stocks sheet states its rungs, and they are the hand's: each rung's
/// label, its start in cents, its beta and its dividend yield; the dividends
/// a year; and, once the first frame has computed it, the days between two,
/// the hand's first dividend day. A rung's label is its place on the ladder,
/// and no name, until `MENU.3`'s first slice names the four.
#[test]
fn the_stocks_sheet_states_its_rungs() {
    let mut cart = cartridge();
    cart.step(0);
    let got = values(&cart, "Stocks", "A1:D8");
    let number = |addr: &str| -> f64 {
        got.get(addr)
            .unwrap_or_else(|| panic!("Stocks!{addr} holds no value"))
            .parse()
            .unwrap_or_else(|e| panic!("Stocks!{addr}: {e}"))
    };
    assert_eq!(number("B1"), buffet::DIVIDENDS_A_YEAR as f64, "Stocks!B1");
    assert_eq!(number("B2"), DIVIDEND_DAYS[0] as f64, "Stocks!B2");
    for (row, (k, rung)) in (5..).zip(RUNGS.iter().enumerate()) {
        assert_eq!(
            got.get(&format!("A{row}")),
            Some(&format!("rung {}", k + 1)),
            "Stocks!A{row}"
        );
        assert_eq!(
            number(&format!("B{row}")),
            rung.start as f64,
            "Stocks!B{row}"
        );
        assert_eq!(
            number(&format!("C{row}")).to_bits(),
            rung.beta.to_bits(),
            "Stocks!C{row}"
        );
        assert_eq!(
            number(&format!("D{row}")).to_bits(),
            rung.dividend_yield.to_bits(),
            "Stocks!D{row}"
        );
    }
}

/// On day 0 every rung is at its start, at every frame of the first day: the
/// index is exactly 1, and 1 to any beta is 1, here and on Linux.
#[test]
fn on_day_0_every_rung_is_at_its_start() {
    let mut cart = cartridge();
    for frame in 1..=buffet::FRAMES_A_DAY {
        cart.step(0);
        let got = values(&cart, "Stocks", "E5:E8");
        let prices: Vec<&str> = (5..=8)
            .map(|row| got[&format!("E{row}")].as_str())
            .collect();
        assert_eq!(prices, ["100", "500", "2500", "10000"], "frame {frame}");
    }
}

/// A person who writes 0 into the difficulty before the first frame plays
/// tier 0, the Kiddie Pool, and every frame of a year is its market by hand,
/// the index tier 0's and tier 1 computed beside it, and every rung priced
/// off it (`AD-6`: any cell is a person's to edit; `POT.6` makes this one a
/// picker).
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
        stocks_held(&cart, &cal, &market, 0);
    }
}

/// A hundred years at one frame a day, every frame a day's first: the index
/// today, twenty days back and yesterday, and every rung's price today and
/// yesterday and its dividend, held to the hand on every one of the 36,500
/// days, across the whole range a long game reaches, the drift raised past
/// the 36,000th power and the Kondratiev two turns on. No rung's price falls
/// below the bound no seed passes, and the dearest rung pays its dividend on
/// exactly four days a year.
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
    let mut cal = Calendar::new(1, buffet::DAYS_A_YEAR);
    let mut least = [u64::MAX; RUNGS.len()];
    let mut paid = 0;
    for day in 0..100 * buffet::DAYS_A_YEAR {
        cart.step(0);
        cal.tick();
        assert_eq!(cal.day, day);
        let got = values(&cart, "Market", "B25:D25");
        let today = day as f64;
        let days = [
            ("B25", today),
            ("C25", today - LOOK_BACK as f64),
            ("D25", today - 1.0),
        ];
        for (addr, at) in days {
            let v: f64 = got[addr]
                .parse()
                .unwrap_or_else(|e| panic!("day {day}: Market!{addr}: {e}"));
            assert_eq!(
                v.to_bits(),
                (market.index(DIFFICULTY, at) + 0.0).to_bits(),
                "day {day}: Market!{addr} is {v}"
            );
        }
        let stocks = stocks_held(&cart, &cal, &market, DIFFICULTY);
        for (k, row) in (5..).take(RUNGS.len()).enumerate() {
            for column in ["E", "F"] {
                let cents: u64 = stocks[&format!("{column}{row}")].parse().unwrap();
                least[k] = least[k].min(cents);
            }
        }
        if stocks["H8"] != "0" {
            paid += 1;
        }
    }
    let bound: Vec<u64> = RUNGS.iter().map(|r| r.price_at(least_index())).collect();
    for (k, (least, bound)) in least.iter().zip(&bound).enumerate() {
        assert!(
            least >= bound && *bound >= 1,
            "rung {}: its least, {least} cents, against the bound, {bound}",
            k + 1
        );
    }
    assert_eq!(paid, 100 * DIVIDEND_DAYS.len(), "the dividend's days");
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

/// The Disclosures sheet (`MENU.1`): its headers and labels as the library's
/// `strings` writes them, the days a disclosure stands as the hand has them,
/// and its rows counted in `F1`, the number of texts its column A holds below
/// the headers, so that the count is the sheet's extent and no number a
/// person keeps beside it.
#[test]
fn the_disclosures_sheet_counts_its_rows() {
    let cart = cartridge();
    let texts = disclosure_texts(&cart);
    let got = values(&cart, "Disclosures", "A1:F5");
    let want = [
        ("A1", "disclosure"),
        ("B1", "vintage"),
        ("C1", "approved by"),
        ("E1", "rows"),
        ("E2", "the row shown"),
        ("E3", "shown"),
        ("E5", "days a disclosure stands"),
    ];
    for (addr, text) in want {
        assert_eq!(
            got.get(addr).map(String::as_str),
            Some(text),
            "Disclosures!{addr}"
        );
    }
    assert!(!texts.is_empty());
    assert_eq!(
        got.get("F1"),
        Some(&texts.len().to_string()),
        "Disclosures!F1"
    );
    assert_eq!(
        got.get("F5"),
        Some(&DAYS_A_DISCLOSURE.to_string()),
        "Disclosures!F5"
    );
}

/// The picker picks every row in its turn (`MENU.1`): a disclosure a day at
/// one frame a day, each written as a person's edit before the first frame,
/// so that the days walk the rows from the first, twice round.
#[test]
fn the_picker_picks_every_row_in_turn() {
    let mut cart = cartridge();
    let texts = disclosure_texts(&cart);
    for edit in ["(cell \"Day\" \"B1\" 1)", "(cell \"Disclosures\" \"F5\" 1)"] {
        cart.write(edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    }
    let mut cal = Calendar::new(1, buffet::DAYS_A_YEAR);
    for _ in 0..2 * texts.len() {
        cart.step(0);
        cal.tick();
        disclosures_held(&cart, &cal, &texts, 1);
    }
}

/// A row added is a line of the content file's call and nothing else
/// (`MENU.1`): the source expanded with one row more differs from the
/// cartridge in that row's three cells, the count and the picker alone, the
/// cartridge loads, and the new row is shown in its turn. Its approver is
/// left empty, as a stranger leaves it for the reviewer (`src/content/README.md`).
#[test]
fn a_row_added_expands_with_no_other_edit() {
    let text = lf(&read(DISCLOSURES));
    let mut lines: Vec<&str> = text.lines().collect();
    let close = lines
        .iter()
        .rposition(|l| l.trim_start().starts_with(')'))
        .expect("the line that closes the call");
    let added_text = "A row added by the oracle.";
    let row = format!("  (0 \"\" \"{added_text}\")");
    lines.insert(close, &row);
    let added = lines.join("\n") + "\n";
    let source = read(SOURCE);
    let mut files = gather(&root(), SOURCE, &source);
    let at = files
        .iter()
        .position(|(path, _)| path == DISCLOSURES)
        .expect("the source includes the content file");
    files[at].1 = added;
    let ex = alonzo::pass::expand(&source, SOURCE, &files)
        .unwrap_or_else(|r| panic!("line {}: {} {}", r.line, r.refusal.id, r.refusal.text));

    let cart = cartridge();
    let n = disclosure_texts(&cart).len();
    let picker = |rows: usize| {
        let refs: Vec<String> = (2..rows + 2).map(|r| format!("A{r}")).collect();
        format!(
            "(formula \"Disclosures\" \"F3\" \"=CHOOSE(F2,{})\")",
            refs.join(",")
        )
    };
    let new_row = n + 2;
    let held = lf(&read(CARTRIDGE));
    let before: BTreeSet<&str> = held.lines().collect();
    let after: BTreeSet<&str> = ex.text.lines().collect();
    let gone: BTreeSet<String> = before.difference(&after).map(|l| l.to_string()).collect();
    let came: BTreeSet<String> = after.difference(&before).map(|l| l.to_string()).collect();
    let want_gone: BTreeSet<String> =
        [format!("(cell \"Disclosures\" \"F1\" {n})"), picker(n)].into();
    let want_came: BTreeSet<String> = [
        format!("(cell \"Disclosures\" \"A{new_row}\" \"{added_text}\")"),
        format!("(cell \"Disclosures\" \"B{new_row}\" 0)"),
        format!("(cell \"Disclosures\" \"C{new_row}\" \"\")"),
        format!("(cell \"Disclosures\" \"F1\" {})", n + 1),
        picker(n + 1),
    ]
    .into();
    assert_eq!(gone, want_gone, "the rows the added row took away");
    assert_eq!(came, want_came, "the rows the added row brought");

    let mut grown = Cartridge::load(&ex.text, "buffet.vla", ex.text.len())
        .unwrap_or_else(|r| panic!("line {}: {} {}", r.line, r.refusal.id, r.refusal.text));
    let texts = disclosure_texts(&grown);
    assert_eq!(texts.len(), n + 1);
    assert_eq!(texts[n], added_text);
    for edit in ["(cell \"Day\" \"B1\" 1)", "(cell \"Disclosures\" \"F5\" 1)"] {
        grown
            .write(edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    }
    let mut cal = Calendar::new(1, buffet::DAYS_A_YEAR);
    for _ in 0..=n {
        grown.step(0);
        cal.tick();
        disclosures_held(&grown, &cal, &texts, 1);
    }
}

/// The one picker's limit on a content sheet (`MENU.1`'s decision 5): a sheet
/// of the library's `strings` holds 1,547 rows below its headers, and 1,548
/// are refused at the picker's line, its formula past the 8,192 characters a
/// formula may hold. And the tripwire before it: every content sheet stays
/// under 1,000 rows, the day one passes being the day its dish writes the
/// pages of 500 the owner approved.
#[test]
fn one_picker_holds_1547_rows_and_no_more() {
    let source = |rows: usize| {
        let mut s = String::from(concat!(
            "; SPDX-License-Identifier: AGPL-3.0-or-later\n",
            "(cartridge \"bank\"\n",
            "  (spec 3)\n",
            "  (title \"Bank\")\n",
            "  (rate 0)\n",
            "  (screen grid 1 1)\n",
            "  (seed 1))\n",
            "(include \"common.vla\")\n",
            "(include \"kitchen.vla\")\n",
            "(sheet \"Screen\" visible)\n",
            "(strings \"Bank\" \"row\"\n",
        ));
        for k in 1..=rows {
            s.push_str(&format!("  (0 \"\" \"row {k}\")\n"));
        }
        s.push_str("  )\n");
        s
    };
    let files: Vec<(String, String)> = ["src/common.vla", "src/kitchen.vla"]
        .iter()
        .map(|p| (p.to_string(), read(p)))
        .collect();
    for (rows, loads) in [(1_547, true), (1_548, false)] {
        let ex = alonzo::pass::expand(&source(rows), "src/bank.source.vla", &files).unwrap_or_else(
            |r| {
                panic!(
                    "{rows} rows: line {}: {} {}",
                    r.line, r.refusal.id, r.refusal.text
                )
            },
        );
        let picker = ex
            .text
            .lines()
            .position(|l| l.starts_with("(formula \"Bank\" \"F3\" "))
            .expect("the picker's line");
        match Cartridge::load(&ex.text, "bank.vla", ex.text.len()) {
            Ok(_) => assert!(loads, "{rows} rows loaded"),
            Err(r) => {
                assert!(
                    !loads,
                    "{rows} rows: line {}: {} {}",
                    r.line, r.refusal.id, r.refusal.text
                );
                assert_eq!(r.refusal.id, "calc-formula-too-long", "{rows} rows");
                assert_eq!(
                    r.line as usize,
                    picker + 1,
                    "{rows} rows: refused at the picker's line"
                );
            }
        }
    }
    let cart = cartridge();
    for sheet in CONTENT_SHEETS {
        let count: usize = values(&cart, sheet, "F1:F1")["F1"]
            .parse()
            .expect("a count");
        assert!(
            count < 1_000,
            "the {sheet} sheet holds {count} rows: past 1,000 its dish writes the pages of 500 (MENU.1's decision 5)"
        );
    }
}

/// Every sheet is written bare, as a formula writes a name without quotes and
/// the kit's verbs need, and no two are the same without case; the views and
/// the model's sheets stand first, visible, then the content sheets, hidden,
/// then the devices; and the twins after every sheet (`MENU.1`'s decisions 2
/// and 3).
#[test]
fn every_sheet_is_written_bare_and_the_strings_hidden() {
    let cart = cartridge();
    let Ok(Viewed::Text(record)) = cart.view("values", "Day", Some("A1:A1")) else {
        panic!("the Day sheet's record");
    };
    let sheets: Vec<(&str, &str)> = record
        .lines()
        .filter_map(|l| {
            l.strip_prefix("(sheet \"")?
                .strip_suffix(')')?
                .split_once("\" ")
        })
        .collect();
    let mut folded = BTreeSet::new();
    for (name, _) in &sheets {
        assert_eq!(
            vla_lang::refers::quote_sheet(name),
            *name,
            "{name} is written bare"
        );
        assert!(
            folded.insert(name.to_ascii_lowercase()),
            "{name} twice, without case"
        );
    }
    let own: Vec<(&str, &str)> = sheets
        .iter()
        .copied()
        .filter(|(name, _)| !name.ends_with(".last"))
        .collect();
    let want = [
        ("Screen", "visible"),
        ("Day", "visible"),
        ("Dice", "visible"),
        ("Market", "visible"),
        ("Stocks", "visible"),
        ("Disclosures", "hidden"),
        ("Clock", "visible"),
        ("File", "visible"),
        ("Input", "visible"),
    ];
    assert_eq!(own, want, "the sheets in tab order, each with its state");
    for (name, state) in &sheets[own.len()..] {
        assert!(
            name.ends_with(".last") && *state == "hidden",
            "the twins after every sheet: {name} {state}"
        );
    }
    for sheet in CONTENT_SHEETS {
        assert!(own.contains(&(sheet, "hidden")), "{sheet} hidden");
    }
}

/// The library's verb, `draws`, ten thousand of them beneath a die of 1: the
/// last is Park and Miller's own published check, 1,043,618,065, computed by
/// the engine's step a cell at a time.
#[test]
fn the_draws_verb_meets_park_and_millers_ten_thousandth() {
    let source = concat!(
        "; SPDX-License-Identifier: AGPL-3.0-or-later\n",
        "(cartridge \"stream\"\n",
        "  (spec 3)\n",
        "  (title \"Stream\")\n",
        "  (rate 0)\n",
        "  (screen grid 1 1)\n",
        "  (seed 1))\n",
        "(include \"kitchen.vla\")\n",
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
