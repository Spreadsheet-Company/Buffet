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
//! Since `MENU.2`: each content sheet held to its file, read by the
//! language's own reader and not by the library's walk; the exam's bank, its
//! first question shown, every question whole, every field picked at every
//! row, and a question added expanding with no other edit. Since `MENU.5`:
//! each content sheet in the state its source writes, the Caveats sheet the
//! one shown; every caveat whole, the first the plain statement that the game
//! is not advice; every caveat's cells holding a value; its first row shown;
//! and its every field picked at every row. Since `FRONT.1`: the Portfolio,
//! the game's first view, its date, every stock's price and change in dollars
//! and the fine print at its foot held at every one of those frames and days,
//! its headers, and its names the Stocks sheet's; the game opening on it,
//! written to the page's version 4, with no Camera and the Screen hidden; and
//! no formula reading a view, each holding at most 1,000 formula cells. Since
//! `FRONT.2`: beside each change, which way it went, the hand's glyph at every
//! one of those frames and days. Since `BACK.1`: the side hustle, Gigs at rest
//! on each of those days, and every frame of a scripted session of clicks and
//! keys, written as the host writes them, held to the hand, the cap, the day a
//! press counts against and the day's reset among them; the hustle's settings,
//! its button at the place they name, and the one key the Keys sheet binds,
//! on its view alone.

use std::collections::{BTreeMap, BTreeSet};

use alonzo::cartridge::{Cartridge, Seed};
use buffet_oracle::buffet::{
    self, disclosure, least_index, Calendar, Hustle, Inputs, Market, DAYS_A_DISCLOSURE, DIFFICULTY,
    DIVIDEND_DAYS, LOOK_BACK, RUNGS, WAVES, YEAR,
};
use buffet_oracle::{expand_source, gather, lf, load, read, root, values};
use vla_lang::form::Form;
use vla_lang::machine::Viewed;
use vla_lang::reader::read_forms;

const SOURCE: &str = "src/buffet.source.vla";
const CARTRIDGE: &str = "src/buffet.vla";
const DESCRIBE: &str = "oracle/testdata/describe/buffet.vla";
/// The fine print's content file, and the sheet its call writes (`MENU.1`).
const DISCLOSURES: &str = "src/content/disclosures.vla";
/// The exam's bank's content file, and the sheet its call writes (`MENU.2`).
const QUESTIONS: &str = "src/content/questions.vla";
/// The caveats' content file, and the sheet its call writes (`MENU.5`).
const CAVEATS: &str = "src/content/caveats.vla";
/// The content sheets, each written by a file of `src/content/`: the sheet,
/// its file, and the state the source writes before its call, hidden as a
/// workbook keeps its reference data at the back, or visible for the one a
/// player reads whole (`MENU.5`'s decision 2).
const CONTENT: [(&str, &str, &str); 3] = [
    ("Disclosures", DISCLOSURES, "hidden"),
    ("Questions", QUESTIONS, "hidden"),
    ("Caveats", CAVEATS, "visible"),
];
/// The exam's bank's fields, in their columns from A (`MENU.2`'s decision 1,
/// the headers title-cased by the owner after the hand test): the order
/// `GUARD.1`'s view reads them in.
const QUESTION_FIELDS: [&str; 8] = [
    "Concept", "Question", "Option A", "Option B", "Option C", "Option D", "Answer", "Why",
];
/// The caveats' fields, in their columns from A (`MENU.5`'s decision 3): the
/// simplification, what the world does, the joke, and the cells it qualifies.
const CAVEAT_FIELDS: [&str; 4] = [
    "Things this game gets wrong",
    "In the real world",
    "Management's response",
    "Where",
];
/// The game's views, each a sheet of its own (`FRONT.1`), standing first
/// among the tabs: the workbook's outputs, which read the model and which no
/// formula reads. A view enters this list with the dish that fills it: Gigs
/// with `BACK.1`, the side hustle's.
const VIEWS: [&str; 2] = ["Portfolio", "Gigs"];
/// Gigs' labels down its column A from row 7 (`BACK.1`'s decision 3), beside
/// the prospectuses proofread, what remains of the run's claims and the fees.
const GIGS_LABELS: [&str; 3] = ["Proofread", "Left this run", "Fees earned ($)"];
/// The Portfolio's headers over its columns from A, in row 4 (`FRONT.1`'s
/// decision 6).
const PORTFOLIO_HEADERS: [&str; 3] = ["Security", "Price ($)", "Change ($)"];

fn cartridge() -> Cartridge {
    load(CARTRIDGE).unwrap_or_else(|e| panic!("{CARTRIDGE} loads: {e}"))
}

fn seed(cart: &Cartridge) -> u64 {
    match cart.manifest().seed {
        Seed::Number(n) => u64::from(n),
        Seed::Host => panic!("the game's seed is a number, so every first game is the same market"),
    }
}

/// A content file of `src/content/`, read by the language's own reader and
/// not by the library's walk (`MENU.2`): the sheet its call of `strings`
/// names, its fields, each a header and a width in characters, and its rows.
struct Content {
    sheet: String,
    fields: Vec<(String, u64)>,
    rows: Vec<ContentRow>,
}

/// A row of a content file's call: its fields' texts, as the file writes them.
/// A row names neither its reviewer, its review being the pull request's that
/// adds it, nor its course, a course being no freeze of the rows before it
/// (the owner's words, 2026-10-15; `BD-1`, amended).
struct ContentRow {
    words: Vec<String>,
}

/// A content file read whole: one macro, its docstring and its one call of
/// `strings`, which names the sheet, then its fields as a list of a header
/// and a width each, then the rows, each its texts. A number is a symbol to
/// the reader and a text a string.
fn content(path: &str) -> Content {
    let text = read(path);
    let forms = read_forms(&text).unwrap_or_else(|r| panic!("{path}: {} {}", r.id, r.text));
    let [Form::List(def)] = forms.as_slice() else {
        panic!("{path} holds one form");
    };
    let call = match def.items.as_slice() {
        [Form::Sym(head), Form::List(_), Form::Str(_), Form::List(call)] if head == "defmacro" => {
            call
        }
        _ => panic!("{path}: one macro, its docstring and its one call"),
    };
    let [Form::Sym(verb), Form::Str(sheet), Form::List(fields), rows @ ..] = call.items.as_slice()
    else {
        panic!("{path}: a call naming its sheet, then its fields");
    };
    assert_eq!(verb, "strings", "{path}: the call is the library's strings");
    let fields = fields
        .items
        .chunks(2)
        .map(|pair| match pair {
            [Form::Str(header), Form::Sym(width)] => (
                header.clone(),
                width
                    .parse()
                    .unwrap_or_else(|e| panic!("{path}: {header}'s width, {width}: {e}")),
            ),
            _ => panic!("{path}: a field is a header and a width"),
        })
        .collect();
    let rows = rows
        .iter()
        .map(|row| {
            let Form::List(row) = row else {
                panic!("{path}: a row is a list");
            };
            let words = row
                .items
                .iter()
                .map(|word| match word {
                    Form::Str(text) => text.clone(),
                    _ => panic!("{path}, line {}: a field is a text", row.line),
                })
                .collect();
            ContentRow { words }
        })
        .collect();
    Content {
        sheet: sheet.clone(),
        fields,
        rows,
    }
}

/// A column's letters from its number, 1 for A.
fn letters(mut column: usize) -> String {
    let mut out = Vec::new();
    while column > 0 {
        column -= 1;
        out.push(b'A' + (column % 26) as u8);
        column /= 26;
    }
    out.reverse();
    String::from_utf8(out).expect("capital letters")
}

/// A cell's address from its row and its column, each from 1.
fn address(row: usize, column: usize) -> String {
    format!("{}{row}", letters(column))
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

/// The fine print at this frame (`MENU.1`): the row shown, `D2`, the hand's
/// for the calendar's day, and the text shown, `D3`, that row's.
fn disclosures_held(cart: &Cartridge, cal: &Calendar, texts: &[String], days: u64) {
    let got = values(cart, "Disclosures", "D2:D3");
    let row = disclosure(cal.day, texts.len() as u64, days);
    assert_eq!(
        got.get("D2"),
        Some(&row.to_string()),
        "frame {}, day {}: Disclosures!D2",
        cal.frame,
        cal.day
    );
    assert_eq!(
        got.get("D3"),
        Some(&texts[row as usize - 1]),
        "frame {}, day {}: Disclosures!D3",
        cal.frame,
        cal.day
    );
}

/// The Day, Dice and Market sheets at this frame, each cell its value as the
/// record prints it, held to the hand's calendar, dice and market for the
/// same frame; and the Stocks and the Disclosures sheets with them, and the
/// Portfolio, the view of them all, its record held at the day's first frame
/// kept in `view` for the frames after it.
fn held(
    cart: &Cartridge,
    cal: &Calendar,
    seed: u64,
    market: &Market,
    texts: &[String],
    view: &mut Option<String>,
) {
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
    let stocks = stocks_held(cart, cal, market, DIFFICULTY);
    disclosures_held(cart, cal, texts, DAYS_A_DISCLOSURE);
    portfolio_held(cart, cal, &stocks, texts, view);
    gigs_held(cart, cal, texts);
}

/// Gigs at rest (`BACK.1`), held on each day's first frame: its date and its
/// fine print the Portfolio's, the hand's; no prospectus proofread, the run's
/// claims all left, nothing earned, and a prospectus waiting. Between two days
/// nothing moves it but an input, and these runs write none: `BD-7`'s scan
/// and the census keep the frame out of every view, and the scripted session
/// holds every frame of the hustle at work
/// (`a_scripted_session_holds_the_hustle_at_every_frame`), where holding the
/// view at every frame of two years here would cost the longest test about
/// half a second (`FRONT.1`'s P5).
fn gigs_held(cart: &Cartridge, cal: &Calendar, texts: &[String]) {
    if !cal.boundary() {
        return;
    }
    let got = values(cart, "Gigs", "A1:B13");
    let rest = Hustle::default();
    let shown = disclosure(cal.day, texts.len() as u64, DAYS_A_DISCLOSURE);
    let want = [
        ("A1", buffet::as_of(cal)),
        ("B7", "0".to_string()),
        ("B8", rest.cap.to_string()),
        ("B9", buffet::dollars(0)),
        ("A11", rest.status(cal.day)),
        ("A13", texts[shown as usize - 1].clone()),
    ];
    for (addr, text) in want {
        assert_eq!(
            got.get(addr),
            Some(&text),
            "frame {}, day {}: Gigs!{addr}",
            cal.frame,
            cal.day
        );
    }
}

/// The Portfolio at this frame (`FRONT.1`), held whole on a day's first
/// frame: its date, the hand's; each stock's price today and its change since
/// yesterday, the whole cents of the Stocks sheet's same frame, which
/// `stocks_held` has just held to the hand, spelled as dollars by text, so
/// that a cell spelled any other way than its whole cents over 100 fails
/// here; beside the change, which way it went (`FRONT.2`), the hand's glyph
/// of the same two prices; and the fine print at its foot, the row the hand
/// picks for the day.
/// On every other frame of the day its record is the one held at the day's
/// first frame, byte for byte, as `BD-7` keeps a view still between days:
/// the same cells held at a view's cost, where parsing and spelling them
/// again at every frame took the suite's longest test 0.85 s more.
fn portfolio_held(
    cart: &Cartridge,
    cal: &Calendar,
    stocks: &BTreeMap<String, String>,
    texts: &[String],
    view: &mut Option<String>,
) {
    let Ok(Viewed::Text(record)) = cart.view("values", "Portfolio", Some("A1:D10")) else {
        panic!("the Portfolio's record");
    };
    if !cal.boundary() {
        if let Some(day) = view.as_deref() {
            assert!(
                record == day,
                "frame {}, day {}: the Portfolio moved within the day",
                cal.frame,
                cal.day
            );
            return;
        }
    }
    let got = values(cart, "Portfolio", "A1:D10");
    let text = |addr: &str, want: &str| {
        assert_eq!(
            got.get(addr).map(String::as_str),
            Some(want),
            "frame {}, day {}: Portfolio!{addr}",
            cal.frame,
            cal.day
        );
    };
    text("A1", &buffet::as_of(cal));
    for row in 5..5 + RUNGS.len() {
        let cents = |column: &str| -> i64 {
            stocks[&format!("{column}{row}")]
                .parse()
                .expect("whole cents")
        };
        let (now, was) = (cents("E"), cents("F"));
        text(&format!("B{row}"), &buffet::dollars(now));
        text(&format!("C{row}"), &buffet::dollars(now - was));
        text(&format!("D{row}"), buffet::glyph(now, was));
    }
    let shown = disclosure(cal.day, texts.len() as u64, DAYS_A_DISCLOSURE);
    text("A10", &texts[shown as usize - 1]);
    *view = Some(record);
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
    let mut view = None;
    for _ in 0..frames {
        let step = cart.step(0);
        cal.tick();
        assert!(step.done && step.frame == cal.frame, "{}", step.row());
        held(&cart, &cal, seed, &market, &texts, &mut view);
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
    let mut view = None;
    for _ in 0..800 {
        cart.step(0);
        cal.tick();
        assert!(cal.boundary());
        held(&cart, &cal, seed, &market, &texts, &mut view);
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
/// and its rows counted in `D1`, the number of texts its column A holds below
/// the headers, so that the count is the sheet's extent and no number a
/// person keeps beside it.
#[test]
fn the_disclosures_sheet_counts_its_rows() {
    let cart = cartridge();
    let texts = disclosure_texts(&cart);
    let got = values(&cart, "Disclosures", "A1:D5");
    let want = [
        ("A1", "disclosure"),
        ("C1", "rows"),
        ("C2", "the row shown"),
        ("C3", "shown"),
        ("C5", "days a disclosure stands"),
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
        got.get("D1"),
        Some(&texts.len().to_string()),
        "Disclosures!D1"
    );
    assert_eq!(
        got.get("D5"),
        Some(&DAYS_A_DISCLOSURE.to_string()),
        "Disclosures!D5"
    );
}

/// The picker picks every row in its turn (`MENU.1`): a disclosure a day at
/// one frame a day, each written as a person's edit before the first frame,
/// so that the days walk the rows from the first, twice round.
#[test]
fn the_picker_picks_every_row_in_turn() {
    let mut cart = cartridge();
    let texts = disclosure_texts(&cart);
    for edit in ["(cell \"Day\" \"B1\" 1)", "(cell \"Disclosures\" \"D5\" 1)"] {
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
/// cartridge in that row's one cell, the count and the picker alone, the
/// cartridge loads, and the new row is shown in its turn, as a stranger adds
/// it (`src/content/README.md`).
#[test]
fn a_row_added_expands_with_no_other_edit() {
    let text = lf(&read(DISCLOSURES));
    let mut lines: Vec<&str> = text.lines().collect();
    let close = lines
        .iter()
        .rposition(|l| l.trim_start().starts_with(')'))
        .expect("the line that closes the call");
    let added_text = "A row added by the oracle.";
    let row = format!("  (\"{added_text}\")");
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
            "(formula \"Disclosures\" \"D3\" \"=CHOOSE(D2,{})\")",
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
        [format!("(cell \"Disclosures\" \"D1\" {n})"), picker(n)].into();
    let want_came: BTreeSet<String> = [
        format!("(cell \"Disclosures\" \"A{new_row}\" \"{added_text}\")"),
        format!("(cell \"Disclosures\" \"D1\" {})", n + 1),
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
    for edit in ["(cell \"Day\" \"B1\" 1)", "(cell \"Disclosures\" \"D5\" 1)"] {
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
            "(strings \"Bank\" (\"row\" 100)\n",
        ));
        for k in 1..=rows {
            s.push_str(&format!("  (\"row {k}\")\n"));
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
            .position(|l| l.starts_with("(formula \"Bank\" \"D3\" "))
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
    for (sheet, path, _) in CONTENT {
        let at = address(1, content(path).fields.len() + 3);
        let count: usize = values(&cart, sheet, &format!("{at}:{at}"))[&at]
            .parse()
            .expect("a count");
        assert!(
            count < 1_000,
            "the {sheet} sheet holds {count} rows: past 1,000 its dish writes the pages of 500 (MENU.1's decision 5)"
        );
    }
}

/// Every sheet of a cartridge and its state, `visible` or `hidden`, in tab
/// order, as the record's sheet rows list them: the cartridge's own, then the
/// engine's devices, then every twin.
fn tabs(cart: &Cartridge) -> Vec<(String, String)> {
    let Ok(Viewed::Text(record)) = cart.view("values", "Day", Some("A1:A1")) else {
        panic!("the Day sheet's record");
    };
    record
        .lines()
        .filter_map(|l| {
            let (name, state) = l
                .strip_prefix("(sheet \"")?
                .strip_suffix(')')?
                .split_once("\" ")?;
            Some((name.to_string(), state.to_string()))
        })
        .collect()
}

/// Every sheet is written bare, as a formula writes a name without quotes and
/// the kit's verbs need, and no two are the same without case; the Screen
/// hidden, then the views and the model's sheets, visible, then the content
/// sheets, each in the state its source writes, the reference data hidden and
/// the caveats shown, then the devices, the Keys the game writes before the
/// engine's own; and the twins after every sheet (`MENU.1`'s decisions 2 and
/// 3, `MENU.5`'s decision 2, `FRONT.1`'s 4 and 5, `BACK.1`'s 2, 4 and 9). A
/// state the source forgot would leave its sheet visible, which the engine
/// makes a sheet first named by a cell.
#[test]
fn every_sheet_is_written_bare_and_each_content_sheet_in_its_state() {
    let cart = cartridge();
    let all = tabs(&cart);
    let sheets: Vec<(&str, &str)> = all
        .iter()
        .map(|(name, state)| (name.as_str(), state.as_str()))
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
        ("Screen", "hidden"),
        ("Portfolio", "visible"),
        ("Gigs", "visible"),
        ("Day", "visible"),
        ("Dice", "visible"),
        ("Market", "visible"),
        ("Stocks", "visible"),
        ("Hustle", "visible"),
        ("Disclosures", "hidden"),
        ("Questions", "hidden"),
        ("Caveats", "visible"),
        ("Keys", "visible"),
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
    for (sheet, _, state) in CONTENT {
        assert!(own.contains(&(sheet, state)), "{sheet} {state}");
    }
}

/// The game opens on the Portfolio (`FRONT.1`'s decisions 2 to 5). The
/// cartridge is written to the page's version 4, whose host opens a grid
/// cartridge on the sheet its Camera's third row names, else on the first
/// visible sheet of its own order (Alonzo's `SPEC.md` section 3.1): it has no
/// Camera, so that rule names the Portfolio, the first visible sheet, and the
/// hidden Screen before it is never opened on.
#[test]
fn the_game_opens_on_the_portfolio() {
    let cart = cartridge();
    assert_eq!(
        cart.manifest().spec,
        4,
        "the page's version 4: a game's views on sheets of their own"
    );
    assert!(
        cart.machine().find_sheet("Camera").is_none(),
        "a Camera's third row would name the sheet the game opens on (GUARD.1 brings one)"
    );
    let all = tabs(&cart);
    let first = all.iter().find(|(_, state)| state == "visible");
    assert_eq!(
        first.map(|(name, _)| name.as_str()),
        Some(VIEWS[0]),
        "the first visible sheet, where the game opens"
    );
    assert!(
        all.iter()
            .any(|(name, state)| name == "Screen" && state == "hidden"),
        "the Screen hidden: the views are sheets of their own"
    );
}

/// The cells of a cell or a range as a cartridge's row writes it, `A5` or
/// `A5:C8`.
fn cells_in(range: &str) -> u64 {
    let corner = |a: &str| -> (u64, u64) {
        let digits = a.trim_start_matches(|ch: char| ch.is_ascii_uppercase());
        let letters = &a[..a.len() - digits.len()];
        let column = letters
            .bytes()
            .fold(0, |n, b| n * 26 + u64::from(b - b'A' + 1));
        (digits.parse().expect("a row"), column)
    };
    let (first, last) = range.split_once(':').unwrap_or((range, range));
    let ((r1, c1), (r2, c2)) = (corner(first), corner(last));
    (r2 - r1 + 1) * (c2 - c1 + 1)
}

/// A view is read by nothing (`FRONT.1`'s decision 10): no formula on a sheet
/// that is not that view names it or its twin, read without case, so the
/// views are the workbook's outputs and a dish lays one out again without
/// moving a formula; a view may read its own cells. And each view holds at
/// most 1,000 formula cells, the cap the owner approved from `MISE.3`'s
/// numbers, its frame 6.4 ms at p95 in Chrome where the bar of 8.3 falls near
/// 1,400: the day a view passes it is the day its dish measures again.
#[test]
fn nothing_reads_a_view_and_each_holds_at_most_1000_formula_cells() {
    let text = lf(&read(CARTRIDGE));
    let mut held: BTreeMap<&str, u64> = VIEWS.iter().map(|v| (*v, 0)).collect();
    for row in text.lines() {
        let Some(rest) = row.strip_prefix("(formula \"") else {
            continue;
        };
        let Some((sheet, rest)) = rest.split_once("\" \"") else {
            panic!("{row}: a formula row names its sheet, then its cells");
        };
        let Some((range, formula)) = rest.split_once('"') else {
            panic!("{row}: a formula row names its cells, then its formula");
        };
        if let Some(n) = held.get_mut(sheet) {
            *n += cells_in(range);
        }
        let formula = formula.to_ascii_lowercase();
        for view in VIEWS.iter().filter(|v| **v != sheet) {
            let name = view.to_ascii_lowercase();
            for read in [format!("{name}!"), format!("{name}.last!")] {
                assert!(
                    !formula.contains(&read),
                    "{row}: the {sheet} sheet reads the {view} view"
                );
            }
        }
    }
    for (view, n) in held {
        assert!(n > 0, "the {view} view holds no formula");
        assert!(
            n <= 1_000,
            "the {view} view holds {n} formula cells, past the 1,000 a view may hold (FRONT.1)"
        );
    }
}

/// The Portfolio's columns (`FRONT.1`'s decisions 6 and 9): its headers, and
/// each stock's name the Stocks sheet's, read by a formula and never held as
/// the view's own text, so that the day `MENU.3` names the stocks the view
/// follows with no edit.
#[test]
fn the_portfolio_heads_its_columns_and_names_its_stocks_from_the_model() {
    let mut cart = cartridge();
    cart.step(0);
    let got = values(&cart, "Portfolio", "A4:C8");
    for (k, header) in PORTFOLIO_HEADERS.iter().enumerate() {
        let at = address(4, k + 1);
        assert_eq!(
            got.get(&at).map(String::as_str),
            Some(*header),
            "Portfolio!{at}"
        );
    }
    let names = values(&cart, "Stocks", "A5:A8");
    let Ok(Viewed::Text(grid)) = cart.view("grid", "Portfolio", Some("A5:A8")) else {
        panic!("the Portfolio's record");
    };
    for row in 5..=8 {
        let at = format!("A{row}");
        assert!(
            names.contains_key(&at) && got.get(&at) == names.get(&at),
            "Portfolio!{at}, the Stocks sheet's name"
        );
        let head = format!("(formula \"Portfolio\" \"{at}\" ");
        assert!(
            grid.lines()
                .any(|l| l.starts_with(&head) && l.contains("Stocks!")),
            "Portfolio!{at} reads the name on the Stocks sheet and holds none of its own"
        );
    }
}

/// What a scripted session sets at a frame (`BACK.1`'s decision 12), each
/// held until another act sets it again: the sheet shown, the pointer's cell
/// on it, the left button, and the key the Keys sheet binds.
#[derive(Clone, Copy)]
enum Act {
    Show(&'static str),
    Point(u64, u64),
    Left(bool),
    Key(bool),
}

/// The Input sheet's rows as the host writes them at version 4 (Alonzo's
/// `host.js`, `_inputRows`): ten buttons for four players, each 1 while the
/// key its cell of the Keys sheet names is held; the pointer's column and row
/// on the sheet shown, that sheet's own cell, 0 off its cells; its three
/// buttons; the wheel; the key that went down this frame and its code; and the
/// sheet shown. The one key a session holds is the hustle's.
fn input_rows(keys: &BTreeMap<String, String>, now: &Inputs, went_down: bool) -> Vec<String> {
    let mut rows = Vec::new();
    for button in 1..=10 {
        for player in ["B", "C", "D", "E"] {
            let at = format!("{player}{button}");
            let held = now.key && keys.get(&at).map(String::as_str) == Some(buffet::KEY_CODE);
            rows.push(format!("(cell \"Input\" \"{at}\" {})", u8::from(held)));
        }
    }
    let (key, code) = if went_down {
        ("p", buffet::KEY_CODE)
    } else {
        ("", "")
    };
    rows.extend([
        format!("(cell \"Input\" \"B11\" {})", now.column),
        format!("(cell \"Input\" \"B12\" {})", now.row),
        format!("(cell \"Input\" \"B13\" {})", u8::from(now.left)),
        "(cell \"Input\" \"B14\" 0)".to_string(),
        "(cell \"Input\" \"B15\" 0)".to_string(),
        "(cell \"Input\" \"B16\" 0)".to_string(),
        format!("(cell \"Input\" \"B17\" \"{key}\")"),
        format!("(cell \"Input\" \"B18\" \"{code}\")"),
        format!("(cell \"Input\" \"B19\" \"{}\")", now.sheet),
    ]);
    rows
}

/// The hustle at this frame (`BACK.1`): the Hustle sheet's workings, each a
/// cell the hand states, the press's two cells from the frame's inputs and the
/// last frame's, the claim, the claims, the day after the last claim's day,
/// whether the hustle is shut today, and the fees; and Gigs' counts, its fees
/// in dollars and its sentence.
fn hustle_held(
    cart: &Cartridge,
    cal: &Calendar,
    h: &Hustle,
    now: &Inputs,
    last: &Inputs,
    claim: bool,
) {
    let flag = |b: bool| u8::from(b).to_string();
    let got = values(cart, "Hustle", "B7:B13");
    let want = [
        ("B7", flag(Hustle::clicked(now, last))),
        ("B8", flag(Hustle::keyed(now, last))),
        ("B9", flag(claim)),
        ("B10", h.claims.len().to_string()),
        ("B11", h.claims.last().map_or(0, |d| d + 1).to_string()),
        ("B12", flag(!h.open(cal.day))),
        ("B13", h.earned().to_string()),
    ];
    for (addr, text) in want {
        assert_eq!(
            got.get(addr),
            Some(&text),
            "frame {}, day {}: Hustle!{addr}",
            cal.frame,
            cal.day
        );
    }
    let view = values(cart, "Gigs", "A7:B11");
    let want = [
        ("B7", h.claims.len().to_string()),
        ("B8", (h.cap - h.claims.len()).to_string()),
        ("B9", buffet::dollars(h.earned() as i64)),
        ("A11", h.status(cal.day)),
    ];
    for (addr, text) in want {
        assert_eq!(
            view.get(addr),
            Some(&text),
            "frame {}, day {}: Gigs!{addr}",
            cal.frame,
            cal.day
        );
    }
}

/// The side hustle at work, every frame of a scripted session held to the
/// hand (`BACK.1`'s decisions 5 to 9 and 12, the key bound on its view alone
/// since the owner's hand test): six days at thirty frames a day, the Input
/// rows written before each step as the host writes them, the rows that
/// changed alone (`ENGINE.19`), the key through the Keys sheet as the host
/// reads it after every step, which at every frame binds P as the hand says,
/// while Gigs showed and on no other sheet; and at every frame the Hustle
/// sheet's workings and Gigs' counts and sentence the hand's. Its presses: a
/// click claiming; a second click and P the same day refused; a click in the
/// frame the next day turns, the claimed day shown, refused; a click beside the
/// button, one at the button's address while the Portfolio shows, and P there,
/// unbound, no press; P on Gigs claiming; P held on the Portfolio as Gigs is
/// shown under it, a press the frame the key is bound, refused; the button and
/// the key held on a claimed day across the turn into an open day, claiming
/// nothing; a click in the frame a day turns, the open day before shown,
/// claiming that day, and the new day open after it; a click and P in one frame
/// claiming once; then the run's four spent, a click, P and a drag onto the
/// button each a press refused, on that day and the next.
#[test]
fn a_scripted_session_holds_the_hustle_at_every_frame() {
    use Act::{Key, Left, Point, Show};
    let script: &[(u64, &[Act])] = &[
        (1, &[Show("Portfolio")]),
        (3, &[Show("Gigs")]),
        (4, &[Point(1, 5)]),
        (5, &[Left(true)]), // a click: the first claim, day 0's
        (6, &[Left(false)]),
        (8, &[Left(true)]), // a second click on day 0: refused
        (9, &[Left(false)]),
        (10, &[Key(true)]), // P on Gigs, day 0: refused
        (11, &[Key(false)]),
        (31, &[Left(true)]), // day 1's first frame, day 0 shown, claimed: refused
        (32, &[Left(false)]),
        (33, &[Point(2, 5)]), // the pointer on B5, beside the button
        (34, &[Left(true)]),  // no press
        (35, &[Left(false)]),
        (36, &[Show("Portfolio"), Point(1, 5)]), // the button's address, on another sheet
        (37, &[Left(true)]),                     // no press
        (38, &[Left(false)]),
        (40, &[Key(true)]), // P on the Portfolio, unbound there: no press
        (41, &[Key(false)]),
        (43, &[Show("Gigs")]),
        (45, &[Key(true)]), // P on Gigs: the second claim, day 1's
        (46, &[Key(false)]),
        (47, &[Show("Portfolio")]),
        (48, &[Key(true)]),    // P held on the Portfolio, unbound,
        (49, &[Show("Gigs")]), // and Gigs shown under it: bound the frame after, a press, refused
        (51, &[Key(false)]),
        (53, &[Left(true)]), // day 1 claimed: refused, and held across day 2's turn
        (55, &[Key(true)]),  // the same, by the key
        (65, &[Left(false)]),
        (66, &[Key(false)]),
        (91, &[Left(true)]), // day 3's first frame, day 2 shown, open: the third claim, day 2's
        (92, &[Left(false)]),
        (95, &[Left(true), Key(true)]), // a click and P in one frame: the fourth claim, once
        (96, &[Left(false), Key(false)]),
        (125, &[Left(true)]), // day 4, the run's four spent: refused
        (126, &[Left(false)]),
        (130, &[Key(true)]), // refused
        (131, &[Key(false)]),
        (135, &[Point(0, 0)]),             // the pointer over a header
        (136, &[Point(1, 5), Left(true)]), // dragged onto the button, held: a press, refused
        (138, &[Left(false)]),
        (151, &[Left(true)]), // day 5's first frame: refused
        (152, &[Left(false)]),
    ];
    let mut cart = cartridge();
    // The keymap the host holds: the Keys sheet as it read it after the last
    // step, and none at the load, before any formula of it holds a value.
    let mut keys: BTreeMap<String, String> = BTreeMap::new();
    let mut cal = Calendar::new(buffet::FRAMES_A_DAY, buffet::DAYS_A_YEAR);
    let mut hustle = Hustle::default();
    let mut now = Inputs::default();
    let mut written: Option<Vec<String>> = None;
    let mut acts = script.iter().peekable();
    for frame in 1..=6 * buffet::FRAMES_A_DAY {
        let last = now.clone();
        while let Some((_, set)) = acts.next_if(|(at, _)| *at == frame) {
            for act in set.iter() {
                match *act {
                    Show(sheet) => now.sheet = sheet.to_string(),
                    Point(column, row) => (now.column, now.row) = (column, row),
                    Left(down) => now.left = down,
                    Key(down) => now.key = down,
                }
            }
        }
        // P bound as the hand says, by the sheet the last frame showed, which
        // the last step's Keys sheet read; and the cartridge's keymap the same.
        now.bound = Hustle::binds(&last.sheet);
        let bound: Vec<(&str, &str)> = keys
            .iter()
            .filter(|(_, code)| !code.is_empty())
            .map(|(addr, code)| (addr.as_str(), code.as_str()))
            .collect();
        let want: &[(&str, &str)] = if now.bound {
            &[("B5", buffet::KEY_CODE)]
        } else {
            &[]
        };
        assert_eq!(
            bound, want,
            "frame {frame}: the keys bound after {:?} showed",
            last.sheet
        );
        let rows = input_rows(&keys, &now, now.key && !last.key);
        let changed: Vec<&str> = match &written {
            None => rows.iter().map(String::as_str).collect(),
            Some(before) => rows
                .iter()
                .zip(before)
                .filter(|(row, was)| row != was)
                .map(|(row, _)| row.as_str())
                .collect(),
        };
        if !changed.is_empty() {
            cart.write(&changed.join("\n")).unwrap_or_else(|r| {
                panic!(
                    "frame {frame}: the Input rows: {} {}",
                    r.refusal.id, r.refusal.text
                )
            });
        }
        written = Some(rows);
        let shown = cal.day; // the day the person saw, the last frame's
        let step = cart.step(0);
        cal.tick();
        assert!(step.done && step.frame == cal.frame, "{}", step.row());
        keys = values(&cart, "Keys", "B1:E10");
        let claim = hustle.frame(&now, &last, shown);
        hustle_held(&cart, &cal, &hustle, &now, &last, claim);
    }
    assert!(acts.next().is_none(), "every act of the script played");
    assert_eq!(
        hustle.claims,
        [0, 1, 2, 3],
        "the claims, by the days they counted against"
    );
}

/// The hustle's settings and its button (`BACK.1`'s decisions 3, 4, 5 and
/// 9): the Hustle sheet's fee, claims a run and the button's place, the
/// hand's; Gigs' cell at that place the button's text, no longer than its
/// column is wide, so that every character of it is the button, the key's
/// name beside it, and the labels below; and the Keys sheet binding one code,
/// P's, on the `a` button, the row the key's press reads, while the button's
/// view shows and nothing on any other sheet (the owner's word at the hand
/// test, 2026-10-16: each tab its own keys), no key the viewport moves by and
/// no Alt, the text beside the button naming the key bound.
#[test]
fn the_hustle_states_its_settings_and_binds_one_key() {
    let mut cart = cartridge();
    cart.step(0);
    let (sheet, column, row) = buffet::BUTTON;
    let settings = values(&cart, "Hustle", "B1:B5");
    let want = [
        ("B1", buffet::FEE.to_string()),
        ("B2", buffet::CLAIMS_A_RUN.to_string()),
        ("B3", sheet.to_string()),
        ("B4", column.to_string()),
        ("B5", row.to_string()),
    ];
    for (addr, text) in want {
        assert_eq!(settings.get(addr), Some(&text), "Hustle!{addr}");
    }
    let view = values(&cart, sheet, "A1:B13");
    let (column, row) = (column as usize, row as usize);
    assert_eq!(
        view.get(&address(row, column)).map(String::as_str),
        Some(buffet::BUTTON_TEXT),
        "{sheet}!{}: the button, at the place the Hustle sheet names",
        address(row, column)
    );
    assert_eq!(
        view.get(&address(row, column + 1)).map(String::as_str),
        Some(buffet::KEY_TEXT),
        "the key's name beside the button"
    );
    for (k, label) in GIGS_LABELS.iter().enumerate() {
        assert_eq!(
            view.get(&format!("A{}", 7 + k)).map(String::as_str),
            Some(*label),
            "{sheet}!A{}",
            7 + k
        );
    }
    let text = lf(&read(CARTRIDGE));
    let head = format!("(column \"{sheet}\" \"{}\" ", letters(column));
    let width: usize = text
        .lines()
        .find_map(|l| l.strip_prefix(head.as_str()))
        .and_then(|rest| rest.split(' ').next())
        .and_then(|w| w.parse().ok())
        .expect("the button's column's width");
    assert!(
        buffet::BUTTON_TEXT.chars().count() <= width,
        "the button's text runs past its column of {width} characters, and a click on the spill is the next cell's"
    );

    let mut codes: Vec<(String, String)> = Vec::new();
    for (shown, bound) in [(sheet, true), ("Portfolio", false)] {
        let edit = format!("(cell \"Input\" \"B19\" \"{shown}\")");
        cart.write(&edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
        cart.step(0);
        let keys = values(&cart, "Keys", "A1:E10");
        let held: Vec<(String, String)> = keys
            .iter()
            .filter(|(addr, code)| !addr.starts_with('A') && !code.is_empty())
            .map(|(addr, code)| (addr.clone(), code.clone()))
            .collect();
        let want: Vec<(String, String)> = if bound {
            vec![("B5".to_string(), buffet::KEY_CODE.to_string())]
        } else {
            Vec::new()
        };
        assert_eq!(
            held, want,
            "the keys bound while {shown} shows: P on the a button, on its view alone"
        );
        assert_eq!(
            keys.get("A5").map(String::as_str),
            Some("a"),
            "Keys!A5, its button"
        );
        codes.extend(held);
    }
    for (_, code) in &codes {
        let moves = [
            "ArrowUp",
            "ArrowDown",
            "ArrowLeft",
            "ArrowRight",
            "PageUp",
            "PageDown",
            "Home",
            "End",
            "Enter",
            "Space",
        ];
        assert!(
            !moves.contains(&code.as_str()) && !code.starts_with("Alt"),
            "{code} is the viewport's or the chords'"
        );
    }
    assert!(
        buffet::KEY_TEXT.ends_with(&buffet::KEY_CODE["Key".len()..]),
        "the text beside the button names the key bound"
    );
    let press = text
        .lines()
        .find(|l| l.starts_with("(formula \"Hustle\" \"B8\" "))
        .expect("the key's press");
    assert!(
        press.contains("Input!B5=1") && press.contains("Input.last!B5=0"),
        "the key's press reads the button the Keys sheet binds: {press}"
    );
}

/// Each content sheet is its file (`MENU.2`): the file read by the language's
/// own reader, which shares no walk with the library's `strings`, and every
/// row of its call a row of the sheet from row 2, its fields from column A, the
/// table ending at its last row; the headers its fields'; each field's width;
/// and beside the table, past one empty column, the labels, `rows`, `the row
/// shown`, `shown` and each later field's header down the column, with the
/// rows counted beside `rows`. The fine print, the exam's bank and the caveats
/// alike, one field, eight and four.
#[test]
fn each_content_sheet_is_its_file() {
    let cart = cartridge();
    let text = lf(&read(CARTRIDGE));
    for (sheet, path, _) in CONTENT {
        let file = content(path);
        assert_eq!(file.sheet, sheet, "{path} writes the {sheet} sheet");
        let n = file.fields.len();
        let rows = file.rows.len();
        let last = (rows + 2).max(n + 2);
        let got = values(&cart, sheet, &format!("A1:{}", address(last, n + 3)));
        let cell = |row: usize, column: usize| got.get(&address(row, column)).map(String::as_str);
        let headers: Vec<&str> = file.fields.iter().map(|(h, _)| h.as_str()).collect();
        for (k, header) in headers.iter().enumerate() {
            assert_eq!(
                cell(1, k + 1),
                Some(*header),
                "{sheet}!{}",
                address(1, k + 1)
            );
        }
        let widths: Vec<(usize, u64)> = (1..).zip(file.fields.iter().map(|f| f.1)).collect();
        for (column, width) in widths {
            let row = format!(
                "(column \"{sheet}\" \"{}\" {width} shown none)",
                letters(column)
            );
            assert!(text.lines().any(|l| l == row), "{sheet}: {row}");
        }
        for (r, row) in (2..).zip(&file.rows) {
            assert_eq!(
                row.words.len(),
                n,
                "{path}: the row for {sheet}'s row {r} holds {} fields where the sheet has {n}",
                row.words.len()
            );
            for (k, word) in row.words.iter().enumerate() {
                assert_eq!(
                    cell(r, k + 1),
                    Some(word.as_str()),
                    "{sheet}!{}",
                    address(r, k + 1)
                );
            }
        }
        for column in 1..=n {
            assert_eq!(
                cell(rows + 2, column),
                None,
                "{sheet}!{}: the table ends at its last row",
                address(rows + 2, column)
            );
        }
        let mut labels = vec!["rows", "the row shown", "shown"];
        labels.extend(file.fields.iter().skip(1).map(|(h, _)| h.as_str()));
        for (k, label) in labels.iter().enumerate() {
            assert_eq!(
                cell(k + 1, n + 2),
                Some(*label),
                "{sheet}!{}",
                address(k + 1, n + 2)
            );
        }
        assert_eq!(
            cell(1, n + 3),
            Some(rows.to_string().as_str()),
            "{sheet}!{}, the rows counted",
            address(1, n + 3)
        );
    }
}

/// The exam's bank shows its first question (`MENU.2`): the row shown, `K2`,
/// a value the source writes, 1, until `GUARD.1`'s draw drives it; and from
/// the first frame its eight pickers, `K3` to `K10`, the first row's fields,
/// each beside its label.
#[test]
fn the_questions_sheet_shows_its_first_question() {
    let mut cart = cartridge();
    cart.step(0);
    let got = values(&cart, "Questions", "A1:K10");
    assert_eq!(
        got.get("K2").map(String::as_str),
        Some("1"),
        "Questions!K2, the row shown"
    );
    let first: Vec<&str> = (1..=8).map(|k| got[&address(2, k)].as_str()).collect();
    let shown: Vec<&str> = (3..=10).map(|r| got[&format!("K{r}")].as_str()).collect();
    assert_eq!(shown, first, "Questions!K3:K10, the first question");
}

/// Every question is whole (`MENU.2`): the bank's eight fields in their
/// order, and each question eight texts, none empty; its four options
/// distinct; its answer the right option's letter, a capital from A to D; its
/// why one line of at most 140 characters, which a concept card quotes alone.
/// And no letter answers more than a third of the bank, so a player who
/// always answers one letter learns only the letter: the draft answered B
/// twelve times in nineteen (decision 3).
#[test]
fn every_question_is_whole() {
    let bank = content(QUESTIONS);
    let headers: Vec<&str> = bank.fields.iter().map(|(h, _)| h.as_str()).collect();
    assert_eq!(
        headers, QUESTION_FIELDS,
        "the bank's fields, in their order"
    );
    let mut answers: BTreeMap<&str, usize> = BTreeMap::new();
    for (q, row) in (1..).zip(&bank.rows) {
        assert_eq!(row.words.len(), 8, "question {q}: eight fields");
        for (field, word) in QUESTION_FIELDS.iter().zip(&row.words) {
            assert!(
                !word.trim().is_empty(),
                "question {q}: its {field} is empty"
            );
        }
        let options: BTreeSet<&str> = row.words[2..6].iter().map(String::as_str).collect();
        assert_eq!(options.len(), 4, "question {q}: four options, each its own");
        let answer = row.words[6].as_str();
        assert!(
            ["A", "B", "C", "D"].contains(&answer),
            "question {q}: its answer is {answer:?}, where a capital from A to D names the right option"
        );
        let why = row.words[7].chars().count();
        assert!(
            why <= 140,
            "question {q}: its why runs {why} characters, past one line of 140"
        );
        *answers.entry(answer).or_insert(0) += 1;
    }
    let n = bank.rows.len();
    for (letter, count) in answers {
        assert!(
            3 * count <= n,
            "{letter} answers {count} of the {n} questions, more than a third: the reviewer spreads the letters (MENU.2's decision 3)"
        );
    }
}

/// A picker a field picks every row in its turn (`MENU.2`, `MENU.5`): on each
/// sheet whose row shown is a value the source writes, the exam's bank and the
/// caveats, the row shown written from 1 to the sheet's last row as a person's
/// edit, and at each step every picker that row's field, read off the sheet's
/// own columns. An empty text, as the first caveat's `Where` is, reads empty on
/// both sides.
#[test]
fn every_field_is_picked_at_every_row() {
    for (sheet, path) in [("Questions", QUESTIONS), ("Caveats", CAVEATS)] {
        let mut cart = cartridge();
        let file = content(path);
        let (n, rows) = (file.fields.len(), file.rows.len());
        let at = n + 3;
        let table = values(&cart, sheet, &format!("A2:{}", address(rows + 1, n)));
        for k in 1..=rows {
            let edit = format!("(cell \"{sheet}\" \"{}\" {k})", address(2, at));
            cart.write(&edit)
                .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
            cart.step(0);
            let window = format!("{}:{}", address(3, at), address(n + 2, at));
            let got = values(&cart, sheet, &window);
            for field in 1..=n {
                let shown = address(field + 2, at);
                assert_eq!(
                    got.get(&shown).map_or("", String::as_str),
                    table.get(&address(k + 1, field)).map_or("", String::as_str),
                    "the row shown {k}: {sheet}!{shown}"
                );
            }
        }
    }
}

/// A question added is a row of the content file's call and nothing else
/// (`MENU.2`): the source expanded with one question more differs from the
/// cartridge in its eight cells, the count and the eight pickers alone, loads,
/// and shows the new question at its row, as a stranger adds it
/// (`src/content/README.md`).
#[test]
fn a_question_added_expands_with_no_other_edit() {
    let added = [
        "A concept added by the oracle",
        "A question the oracle added. Its answer is:",
        "the first option",
        "the second option",
        "the third option",
        "the fourth option",
        "D",
        "A why the oracle added, in one line.",
    ];
    let text = lf(&read(QUESTIONS));
    let lines: Vec<&str> = text.lines().collect();
    let close = lines
        .iter()
        .rposition(|l| l.trim_start().starts_with(')'))
        .expect("the line that closes the call");
    let mut grown: Vec<String> = lines[..close].iter().map(|l| l.to_string()).collect();
    for (k, word) in added.iter().enumerate() {
        let open = if k == 0 { "    (" } else { "     " };
        let end = if k + 1 == added.len() { ")" } else { "" };
        grown.push(format!("{open}\"{word}\"{end}"));
    }
    grown.extend(lines[close..].iter().map(|l| l.to_string()));
    let source = read(SOURCE);
    let mut files = gather(&root(), SOURCE, &source);
    let at = files
        .iter()
        .position(|(path, _)| path == QUESTIONS)
        .expect("the source includes the bank");
    files[at].1 = grown.join("\n") + "\n";
    let ex = alonzo::pass::expand(&source, SOURCE, &files)
        .unwrap_or_else(|r| panic!("line {}: {} {}", r.line, r.refusal.id, r.refusal.text));

    let n = content(QUESTIONS).rows.len();
    let picker = |field: usize, rows: usize| {
        let refs: Vec<String> = (2..rows + 2).map(|r| address(r, field)).collect();
        format!(
            "(formula \"Questions\" \"K{}\" \"=CHOOSE(K2,{})\")",
            field + 2,
            refs.join(",")
        )
    };
    let held = lf(&read(CARTRIDGE));
    let before: BTreeSet<&str> = held.lines().collect();
    let after: BTreeSet<&str> = ex.text.lines().collect();
    let gone: BTreeSet<String> = before.difference(&after).map(|l| l.to_string()).collect();
    let came: BTreeSet<String> = after.difference(&before).map(|l| l.to_string()).collect();
    let mut want_gone: BTreeSet<String> = (1..=8).map(|f| picker(f, n)).collect();
    want_gone.insert(format!("(cell \"Questions\" \"K1\" {n})"));
    let new_row = n + 2;
    let mut want_came: BTreeSet<String> = (1..=8).map(|f| picker(f, n + 1)).collect();
    want_came.insert(format!("(cell \"Questions\" \"K1\" {})", n + 1));
    for (k, word) in added.iter().enumerate() {
        want_came.insert(format!(
            "(cell \"Questions\" \"{}\" \"{word}\")",
            address(new_row, k + 1)
        ));
    }
    assert_eq!(gone, want_gone, "the rows the added question took away");
    assert_eq!(came, want_came, "the rows the added question brought");

    let mut cart = Cartridge::load(&ex.text, "buffet.vla", ex.text.len())
        .unwrap_or_else(|r| panic!("line {}: {} {}", r.line, r.refusal.id, r.refusal.text));
    let edit = format!("(cell \"Questions\" \"K2\" {})", n + 1);
    cart.write(&edit)
        .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    cart.step(0);
    let got = values(&cart, "Questions", "K3:K10");
    let shown: Vec<&str> = (3..=10).map(|r| got[&format!("K{r}")].as_str()).collect();
    assert_eq!(shown, added, "the question added, shown at its row");
}

/// The caveats show their first row (`MENU.5`'s decisions 4 and 5): the row
/// shown, `G2`, a value the source writes, 1; and from the first frame the four
/// pickers, `G3` to `G6`, the first row's fields, the plain statement that the
/// game is not advice and its joke among them, which the certificate reads
/// through a reference (`GUARD.1`).
#[test]
fn the_caveats_sheet_shows_its_first_row() {
    let mut cart = cartridge();
    cart.step(0);
    let got = values(&cart, "Caveats", "A1:G6");
    assert_eq!(
        got.get("G2").map(String::as_str),
        Some("1"),
        "Caveats!G2, the row shown"
    );
    let first: Vec<&str> = (1..=4)
        .map(|k| got.get(&address(2, k)).map_or("", String::as_str))
        .collect();
    let shown: Vec<&str> = (3..=6)
        .map(|r| got.get(&format!("G{r}")).map_or("", String::as_str))
        .collect();
    assert_eq!(shown, first, "Caveats!G3:G6, the first row");
}

/// Every caveat is whole (`MENU.5`'s decisions 3 and 4): the sheet's four
/// fields in their order; each caveat four texts, the first three never
/// empty; every text one line of at most 140 characters, as a why is, and no
/// wider than its field's column, so that it reads whole on a sheet a player
/// reads; and the first caveat the plain statement that the game is not
/// advice, which the certificate reads, so that a caveat added goes after it:
/// a sentence of it that denies advice, a `not`, `no` or `never` before the
/// word, since the word alone stands in the caveat's explanation too.
#[test]
fn every_caveat_is_whole() {
    let caveats = content(CAVEATS);
    let headers: Vec<&str> = caveats.fields.iter().map(|(h, _)| h.as_str()).collect();
    assert_eq!(
        headers, CAVEAT_FIELDS,
        "the caveats' fields, in their order"
    );
    for (c, row) in (1..).zip(&caveats.rows) {
        assert_eq!(row.words.len(), 4, "caveat {c}: four fields");
        for (k, ((field, width), word)) in caveats.fields.iter().zip(&row.words).enumerate() {
            let n = word.chars().count();
            assert!(
                !word.contains('\n') && n <= 140,
                "caveat {c}: its {field} is no line of at most 140 characters, but {n}"
            );
            assert!(
                n as u64 <= *width,
                "caveat {c}: its {field} runs {n} characters, past its column's {width}: widen the field at the top of its file"
            );
            assert!(
                k == 3 || !word.trim().is_empty(),
                "caveat {c}: its {field} is empty"
            );
        }
    }
    let first = caveats.rows.first().expect("a caveat").words.join(" ");
    let denies = first.to_lowercase().split('.').any(|sentence| {
        let words: Vec<&str> = sentence
            .split(|ch: char| !ch.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .collect();
        words
            .iter()
            .position(|w| ["not", "no", "never"].contains(w))
            .is_some_and(|at| words[at..].iter().any(|w| w.starts_with("advice")))
    });
    assert!(
        denies,
        "the first caveat is the plain statement that the game is not advice, which the certificate reads: {first}"
    );
}

/// Every caveat's `Where` holds a value (`MENU.5`'s decision 3): each that is
/// not empty is a sheet of the cartridge, `!`, and a cell or a range, as a
/// formula writes a reference, which after the first frame holds a value; so
/// a caveat cannot outlive the cells it qualifies when a model's sheet moves.
#[test]
fn every_caveat_points_at_cells_that_hold_a_value() {
    let mut cart = cartridge();
    cart.step(0);
    let cell = |text: &str| {
        let digits = text.trim_start_matches(|ch: char| ch.is_ascii_uppercase());
        digits.len() < text.len()
            && !digits.is_empty()
            && digits.chars().all(|ch| ch.is_ascii_digit())
    };
    for (c, row) in (1..).zip(&content(CAVEATS).rows) {
        let place = row.words[3].as_str();
        if place.is_empty() {
            continue;
        }
        let Some((sheet, range)) = place.split_once('!') else {
            panic!("caveat {c}: its Where, {place:?}, is no reference, a sheet then a cell");
        };
        let ends: Vec<&str> = range.split(':').collect();
        assert!(
            ends.len() <= 2 && ends.iter().all(|e| cell(e)),
            "caveat {c}: its Where, {place:?}, names no cell or range"
        );
        let Ok(Viewed::Text(record)) = cart.view("values", sheet, Some(range)) else {
            panic!("caveat {c}: its Where, {place:?}, is no range of a sheet of the cartridge");
        };
        let held = [
            format!("(value \"{sheet}\" "),
            format!("(cell \"{sheet}\" "),
        ];
        assert!(
            record
                .lines()
                .any(|l| held.iter().any(|h| l.starts_with(h.as_str()))),
            "caveat {c}: {place} holds no value"
        );
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
