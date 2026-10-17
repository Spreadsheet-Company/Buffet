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
//! on its view alone. Since `POT.10`: the calendar counted in minutes, two a
//! frame, and the market's hours, held at the pace each run needs: every
//! frame of ten days at the manifest's pace, the calendar, the dice, the
//! market and its session's path, the stocks, the fine print and both views;
//! every hour of a year and two days at a frame an hour; and eight hundred
//! days, and a hundred years, at a frame a day, the minutes a frame written
//! as a person's edit. Every close is its day's index to the bit, every hour
//! of a session the hand's path, the views' date with its weekday and its
//! time and the bell's line beneath it.

use std::collections::{BTreeMap, BTreeSet};

use alonzo::cartridge::{Cartridge, Seed};
use buffet_oracle::buffet::{
    self, disclosure, least_index, Calendar, Hustle, Inputs, Market, AN_HOUR, A_DAY, CLOSING_MARK,
    DAYS_A_DISCLOSURE, DAYS_A_YEAR, DIFFICULTY, DIVIDEND_DAYS, FIRST_MINUTE, LOOK_BACK,
    MINUTES_A_FRAME, RUNGS, SESSION_DRAW, WAVES, YEAR,
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

/// The market's draws a day (`POT.1`, `POT.10`): `POT.2`'s four phases, then
/// the session's thirteen steps.
const MARKET_DRAWS: u64 = SESSION_DRAW + CLOSING_MARK - 1;

/// The Day sheet at this frame (`POT.1`, `POT.10`), every cell its value as
/// the record prints it, held to the hand's calendar: the minutes a frame and
/// the days a year; the day, the time in minutes, the boundary, the year and
/// the day of the year; the first frame's time, the bells and the trading
/// days; the time to the half hour and of the day, the time shown, the
/// weekday and its name; whether the market is open; the session's day, the
/// one before it, the mark and the closing mark.
fn day_held(cart: &Cartridge, cal: &Calendar) {
    let f = cal.frame;
    let got = values(cart, "Day", "B1:B23");
    let want = [
        ("B1", cal.minutes_a_frame.to_string()),
        ("B2", cal.days_a_year.to_string()),
        ("B3", cal.day.to_string()),
        ("B4", cal.minutes().to_string()),
        ("B5", u8::from(cal.turned).to_string()),
        ("B6", cal.year.to_string()),
        ("B7", cal.day_of_year.to_string()),
        ("B9", FIRST_MINUTE.to_string()),
        ("B10", buffet::OPENING_BELL.to_string()),
        ("B11", buffet::CLOSING_BELL.to_string()),
        ("B12", buffet::TRADING_DAYS.to_string()),
        ("B14", cal.half_hours().to_string()),
        ("B15", cal.half_hour().to_string()),
        ("B16", buffet::time_shown(cal.half_hour())),
        ("B17", cal.weekday.to_string()),
        ("B18", buffet::DAY_NAMES[cal.weekday as usize].to_string()),
        ("B19", u8::from(cal.open()).to_string()),
        ("B20", cal.session.to_string()),
        ("B21", cal.before.to_string()),
        ("B22", cal.mark.to_string()),
        ("B23", CLOSING_MARK.to_string()),
    ];
    for (addr, text) in want {
        assert_eq!(
            got.get(addr),
            Some(&text),
            "frame {f}, day {}, {}: Day!{addr}",
            cal.day,
            buffet::time_shown(cal.hour * 60 + cal.minute)
        );
    }
}

/// The Day, Dice and Market sheets at this frame, each cell its value as the
/// record prints it, held to the hand's calendar, dice and market for the
/// same frame; and the Stocks and the Disclosures sheets with them, and the
/// views of them all: the Portfolio, its record held whole at each half hour
/// and kept in `view` for the frames after it, and Gigs at rest.
fn held(
    cart: &Cartridge,
    cal: &Calendar,
    seed: u64,
    market: &Market,
    texts: &[String],
    view: &mut Option<(u64, String)>,
) {
    let f = cal.frame;
    day_held(cart, cal);
    let dice = values(cart, "Dice", "B1:J18");
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
    for draw in 1..=MARKET_DRAWS {
        let z = buffet::market_draw(seed, cal.day, draw);
        die(&format!("B{}", draw + 1), z);
        uniform(&format!("C{}", draw + 1), z);
    }
    die("I1", buffet::exam_die(seed));
    for draw in 1..=2u64 {
        let z = buffet::exam_draw(seed, draw);
        die(&format!("I{}", draw + 1), z);
        uniform(&format!("J{}", draw + 1), z);
    }
    market_held(cart, cal, seed, market, DIFFICULTY);
    let stocks = stocks_held(cart, cal, seed, market, DIFFICULTY);
    disclosures_held(cart, cal, texts, DAYS_A_DISCLOSURE);
    let turned = view.as_ref().map(|(half, _)| *half) != Some(cal.half_hours());
    portfolio_held(cart, cal, &stocks, texts, view);
    if turned {
        gigs_held(cart, cal, texts);
    }
}

/// Gigs at rest (`BACK.1`), held whenever the half hour turns (`POT.10`): its
/// date and the bell's line and its fine print the Portfolio's, the hand's; no
/// prospectus proofread, the run's claims all left, nothing earned, and a
/// prospectus waiting. Between two half hours nothing moves it but an input,
/// and these runs write none: `BD-7`'s scan and the census keep the frame out
/// of every view, and the scripted session holds every frame of the hustle at
/// work (`a_scripted_session_holds_the_hustle_at_every_frame`).
fn gigs_held(cart: &Cartridge, cal: &Calendar, texts: &[String]) {
    let got = values(cart, "Gigs", "A1:B13");
    let rest = Hustle::default();
    let shown = disclosure(cal.day, texts.len() as u64, DAYS_A_DISCLOSURE);
    let want = [
        ("A1", buffet::as_of(cal)),
        ("A2", buffet::bell(cal).to_string()),
        ("B7", "0".to_string()),
        ("B8", rest.cap.to_string()),
        ("B9", buffet::two_decimals(0)),
        ("A11", rest.status(cal)),
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

/// The Portfolio at this frame (`FRONT.1`), held whole whenever the half
/// hour turns (`POT.10`): its date and the bell's line, the hand's; each
/// stock's price now and its change since the last close, the whole cents of
/// the Stocks sheet's same frame, which `stocks_held` has just held to the
/// hand, spelled as dollars by text, so that a cell spelled any other way
/// than its whole cents over 100 fails here; beside the change, which way it
/// went (`FRONT.2`), the hand's glyph of the same two prices; and the fine
/// print at its foot, the row the hand picks for the day.
/// On every other frame of the half hour its record is the one held at the
/// half hour's first frame, byte for byte, as `BD-7` keeps a view still
/// between two of the calendar's half hours, every price moving at a mark or
/// a bell and each of those on a half hour: the same cells held at a view's
/// cost, where parsing and spelling them again at every frame took the
/// suite's longest test 0.85 s more (`FRONT.1`).
fn portfolio_held(
    cart: &Cartridge,
    cal: &Calendar,
    stocks: &BTreeMap<String, String>,
    texts: &[String],
    view: &mut Option<(u64, String)>,
) {
    let Ok(Viewed::Text(record)) = cart.view("values", "Portfolio", Some("A1:D10")) else {
        panic!("the Portfolio's record");
    };
    if let Some((half, held)) = view.as_ref() {
        if *half == cal.half_hours() {
            assert!(
                record == *held,
                "frame {}, day {}: the Portfolio moved within the half hour",
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
            "frame {}, day {}, {}: Portfolio!{addr}",
            cal.frame,
            cal.day,
            buffet::time_shown(cal.hour * 60 + cal.minute)
        );
    };
    text("A1", &buffet::as_of(cal));
    text("A2", buffet::bell(cal));
    for row in 5..5 + RUNGS.len() {
        let cents = |column: &str| -> i64 {
            stocks[&format!("{column}{row}")]
                .parse()
                .expect("whole cents")
        };
        let (now, was) = (cents("E"), cents("F"));
        text(&format!("B{row}"), &buffet::two_decimals(now as u64));
        text(&format!("C{row}"), &buffet::signed_two_decimals(now - was));
        text(&format!("D{row}"), buffet::glyph(now, was));
    }
    let shown = disclosure(cal.day, texts.len() as u64, DAYS_A_DISCLOSURE);
    text("A10", &texts[shown as usize - 1]);
    *view = Some((cal.half_hours(), record));
}

/// The Market sheet at this frame (`POT.2`, `POT.10`): each wave's speed and
/// tier 1's phases, the cycle on the first day, and every formula of the
/// three columns, the session's day, the look back and the session before,
/// and of the session's hours, its steps and their sums, the mark, its share,
/// the swing and the index now, held bit for bit to the hand's market at the
/// calendar's frame and a difficulty. The hand's value has 0.0 added, which
/// turns its rare -0 into the 0 a cell holds instead and moves nothing else.
fn market_held(cart: &Cartridge, cal: &Calendar, seed: u64, market: &Market, difficulty: u64) {
    let f = cal.frame;
    let got = values(cart, "Market", "B7:E45");
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
    let session = cal.session as f64;
    let days = [
        ("B", session),
        ("C", session - LOOK_BACK as f64),
        ("D", cal.before as f64),
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
    let path = market.path(seed, difficulty, cal);
    for (row, (step, sum)) in (28..).zip(path.steps.iter().zip(&path.sums)) {
        cell(&format!("B{row}"), *step);
        cell(&format!("C{row}"), *sum);
    }
    cell("B42", cal.mark as f64);
    cell("B43", path.share);
    cell("B44", path.swing);
    cell("B45", path.now);
}

/// The Stocks sheet's workings at this frame (`STATION.1`, `POT.10`): each
/// rung's price now, over the hand's index now, and at the last close, over
/// the session before's index, its dividend and the dividend today, held to
/// the hand's rungs at the calendar's frame and a difficulty, whole cents
/// compared as the record prints them. The window is the four columns that
/// compute: a view costs a debug build a tenth of a millisecond or more, the
/// more the more cells it holds, at every one of the frames the tests walk,
/// and the settings never move (`the_stocks_sheet_states_its_rungs` reads
/// them). The values read, for a caller that counts them.
fn stocks_held(
    cart: &Cartridge,
    cal: &Calendar,
    seed: u64,
    market: &Market,
    difficulty: u64,
) -> BTreeMap<String, String> {
    let f = cal.frame;
    let got = values(cart, "Stocks", "E5:I8");
    let cents = |addr: &str, want: u64| {
        assert_eq!(
            got.get(addr),
            Some(&want.to_string()),
            "frame {f}, day {}: Stocks!{addr}",
            cal.day
        );
    };
    let now = market.path(seed, difficulty, cal).now;
    for (row, rung) in (5..).zip(&RUNGS) {
        cents(&format!("E{row}"), rung.price_at(now));
        cents(
            &format!("F{row}"),
            rung.price(market, difficulty, cal.before as f64),
        );
        cents(&format!("G{row}"), rung.dividend);
        cents(&format!("H{row}"), rung.dividend_on(cal.day_of_year));
        let change =
            rung.price_at(now) as i64 - rung.price(market, difficulty, cal.before as f64) as i64;
        assert_eq!(
            got.get(&format!("I{row}")),
            Some(&change.to_string()),
            "frame {f}, day {}: Stocks!I{row}",
            cal.day
        );
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

/// An hour is a second at the manifest's rate (`POT.10`): sixty over the
/// minutes a frame is the rate, and a year 365 days; the Day sheet's two
/// settings, the hand's constants and the manifest agree, so that the
/// teacher's dial, the Clock's rate, scales the game and moves no formula.
#[test]
fn an_hour_is_a_second_at_the_manifests_rate() {
    let cart = cartridge();
    let rate = u64::from(cart.manifest().rate);
    assert_eq!(rate * MINUTES_A_FRAME, AN_HOUR);
    let day = values(&cart, "Day", "B1:B2");
    assert_eq!(
        day.get("B1"),
        Some(&MINUTES_A_FRAME.to_string()),
        "Day!B1, the minutes a frame"
    );
    assert_eq!(
        day.get("B2"),
        Some(&DAYS_A_YEAR.to_string()),
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

/// The cartridge, its minutes a frame written as a person's edit before the
/// first frame, and the hand's calendar at the same pace (`POT.10`).
fn at_pace(minutes_a_frame: u64) -> (Cartridge, Calendar) {
    let mut cart = cartridge();
    let edit = format!("(cell \"Day\" \"B1\" {minutes_a_frame})");
    cart.write(&edit)
        .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    (cart, Calendar::new(minutes_a_frame, DAYS_A_YEAR))
}

/// Every frame of ten days at the manifest's pace (`POT.10`), from Monday's
/// opening bell on day 0 to the second Wednesday's last frame, 6,915 frames,
/// held to the hand: the calendar at every frame; the market's dice changing
/// on a day's first frame and on no other, the exam's never; the market's
/// three columns and its session's path, the stocks and the fine print at
/// every frame; and the views, the Portfolio still between two half hours.
/// Its days are every kind of day: the first, its bell at the first frame;
/// four more sessions, each its bell, its hours and its close; the first
/// weekend; Monday's bell after it, the session before it Friday's.
#[test]
fn every_frame_of_ten_days_is_the_calendar_the_dice_and_the_market_by_hand() {
    let mut cart = cartridge();
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut cal = Calendar::new(MINUTES_A_FRAME, DAYS_A_YEAR);
    let frames = (10 * A_DAY - FIRST_MINUTE) / MINUTES_A_FRAME;
    let mut view = None;
    for _ in 0..frames {
        let step = cart.step(0);
        cal.tick();
        assert!(step.done && step.frame == cal.frame, "{}", step.row());
        held(&cart, &cal, seed, &market, &texts, &mut view);
    }
    assert_eq!(
        (frames, cal.day, cal.weekday, cal.hour, cal.minute),
        (6_915, 9, 2, 23, 58),
        "the run ends at the second Wednesday's last frame"
    );
}

/// Every hour of a year and two days at a frame an hour (`POT.10`), the
/// minutes a frame written 60 as a person's edit before the first frame,
/// 8,808 frames, each held to the hand as every frame is above: every mark of
/// 263 sessions, each frame half an hour after its hour, the bell's at 9:30
/// and the close's at 16:30; the dice at each day's first frame; the year's
/// turn, and the dividend's four days.
#[test]
fn every_hour_of_a_year_and_two_days_is_the_market_by_hand_at_a_frame_an_hour() {
    let (mut cart, mut cal) = at_pace(AN_HOUR);
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut view = None;
    let mut sessions = BTreeSet::new();
    for _ in 0..(DAYS_A_YEAR + 2) * 24 {
        cart.step(0);
        cal.tick();
        held(&cart, &cal, seed, &market, &texts, &mut view);
        sessions.insert(cal.session);
    }
    assert_eq!(sessions.len(), 263, "the sessions held");
    assert_eq!(
        (cal.year, cal.day_of_year, cal.hour, cal.minute),
        (2, 3, 8, 30),
        "the run ends before the second year's third bell"
    );
}

/// At a frame a day, the minutes a frame written 1,440 as a person's edit
/// before the first frame, every frame turns the day at the opening bell's
/// minute, and each day rolls the dice the same day rolls at two minutes a
/// frame and closes the same market: the market is a function of the seed
/// and the calendar, never of the frame (`BD-3`), so a floors fixture at a
/// frame a day plays the game's own market (`MISE.4`).
#[test]
fn at_a_frame_a_day_the_same_days_roll_the_same_dice() {
    let (mut cart, mut cal) = at_pace(A_DAY);
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut view = None;
    for _ in 0..800 {
        cart.step(0);
        cal.tick();
        assert!(cal.turned);
        held(&cart, &cal, seed, &market, &texts, &mut view);
    }
}

/// The Market sheet states its settings and its waves, and they are the
/// hand's constants: pi, the drift, the difficulty, the look back and the
/// hour's swing (`POT.10`), each wave's period and amplitude, and the Kiddie
/// Pool's fixed phase, 0.
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
        ("B5", buffet::SWING),
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

/// On day 0 every rung opens and closes at its start (`STATION.1`, `POT.10`):
/// the first session opens from its own close, the index on day 0 exactly 1,
/// and 1 to any beta is 1, here and on Linux; so at the opening bell, the
/// first frame, and at every frame from the closing bell to midnight, each
/// price now is its start, and at every frame of the day the last close is,
/// the hours between wandering by the session's path.
#[test]
fn on_day_0_every_rung_opens_and_closes_at_its_start() {
    let mut cart = cartridge();
    let mut cal = Calendar::new(MINUTES_A_FRAME, DAYS_A_YEAR);
    let starts = ["100", "500", "2500", "10000"];
    let mut wandered = false;
    while cal.day == 0 {
        cart.step(0);
        cal.tick();
        if cal.day > 0 {
            break;
        }
        let got = values(&cart, "Stocks", "E5:F8");
        let column = |c: &str| -> Vec<&str> {
            (5..=8)
                .map(|row| got[&format!("{c}{row}")].as_str())
                .collect()
        };
        assert_eq!(column("F"), starts, "frame {}: the last close", cal.frame);
        if cal.mark == 0 || cal.mark == CLOSING_MARK {
            assert_eq!(column("E"), starts, "frame {}: the price now", cal.frame);
        } else {
            wandered |= column("E") != starts;
        }
    }
    assert!(wandered, "the first session's hours moved no price");
}

/// A person who writes 0 into the difficulty before the first frame plays
/// tier 0, the Kiddie Pool, and every hour of a year at a frame an hour is
/// its market by hand, the index tier 0's and tier 1 computed beside it, the
/// session's path over tier 0's closes, and every rung priced off it (`AD-6`:
/// any cell is a person's to edit; `POT.6` makes this one a picker).
#[test]
fn at_difficulty_0_the_index_is_the_kiddie_pools() {
    let (mut cart, mut cal) = at_pace(AN_HOUR);
    let seed = seed(&cart);
    let market = Market::new(seed);
    cart.write("(cell \"Market\" \"B3\" 0)")
        .unwrap_or_else(|r| {
            panic!(
                "the difficulty, written: {} {}",
                r.refusal.id, r.refusal.text
            )
        });
    for _ in 0..DAYS_A_YEAR * 24 {
        cart.step(0);
        cal.tick();
        market_held(&cart, &cal, seed, &market, 0);
        stocks_held(&cart, &cal, seed, &market, 0);
    }
}

/// A hundred years at a frame a day, the minutes a frame written 1,440, every
/// frame the opening bell's minute of the next day: the session's close, the
/// close twenty days before it and the session before's, and every rung's
/// price now, over the index now, and at the last close and its dividend, held to the
/// hand on every one of the 36,500 days, a weekend's showing Friday's
/// session, across the whole range a long game reaches, the drift raised
/// past the 36,000th power and the Kondratiev two turns on. No rung's price
/// falls below the bound no seed passes, and the dearest rung pays its
/// dividend on exactly four days a year.
#[test]
fn a_hundred_years_at_a_frame_a_day_are_the_market_by_hand() {
    let (mut cart, mut cal) = at_pace(A_DAY);
    let seed = seed(&cart);
    let market = Market::new(seed);
    let mut least = [u64::MAX; RUNGS.len()];
    let mut paid = 0;
    for day in 0..100 * DAYS_A_YEAR {
        cart.step(0);
        cal.tick();
        assert_eq!(cal.day, day);
        let got = values(&cart, "Market", "B25:D25");
        let session = cal.session as f64;
        let days = [
            ("B25", market.index(DIFFICULTY, session)),
            ("C25", market.index(DIFFICULTY, session - LOOK_BACK as f64)),
            ("D25", market.index(DIFFICULTY, cal.before as f64)),
        ];
        for (addr, want) in days {
            let v: f64 = got[addr]
                .parse()
                .unwrap_or_else(|e| panic!("day {day}: Market!{addr}: {e}"));
            assert_eq!(
                v.to_bits(),
                (want + 0.0).to_bits(),
                "day {day}: Market!{addr} is {v}"
            );
        }
        let stocks = stocks_held(&cart, &cal, seed, &market, DIFFICULTY);
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

/// `BD-7`: the Day sheet alone reads the frame, `Clock!B1`, and its time in
/// minutes, `Day!B4`, the one cell of it that changes every frame, or that
/// cell's twin (`POT.10`); and `describe`'s census computes no shape every
/// frame. So an ordinary frame computes the calendar's three cells, the time,
/// the boundary and the time to the half hour, and the rest wait for a half
/// hour, a mark or a day. The step row cannot say so, since it counts the
/// plan's cells passed, computed or not (`vla-lang` 0.9.3's `Machine::step`).
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
            ("the time to the minute", ("Day", "B", "4")),
            ("the time to the minute's twin", ("Day.last", "B", "4")),
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
/// a frame a day, each written as a person's edit before the first frame, so
/// that the days walk the rows from the first, twice round.
#[test]
fn the_picker_picks_every_row_in_turn() {
    let (mut cart, mut cal) = at_pace(A_DAY);
    let texts = disclosure_texts(&cart);
    let edit = "(cell \"Disclosures\" \"D5\" 1)";
    cart.write(edit)
        .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
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
    let pace = format!("(cell \"Day\" \"B1\" {A_DAY})");
    for edit in [pace.as_str(), "(cell \"Disclosures\" \"D5\" 1)"] {
        grown
            .write(edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    }
    let mut cal = Calendar::new(A_DAY, DAYS_A_YEAR);
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

/// The columns a view's dollars stand in (`POT.10`'s second hand test): each
/// [`buffet::DOLLARS_COLUMN`] characters wide, the width the library's
/// `dollars` and the Portfolio's change are padded for, so that each text
/// ends where a number ends: the Portfolio's prices and changes, and Gigs'
/// fees, which the views' frames hold to the hand's spelling.
#[test]
fn every_column_of_dollars_is_the_width_its_padding_is_measured_for() {
    let text = lf(&read(CARTRIDGE));
    for (sheet, column) in [("Portfolio", "B"), ("Portfolio", "C"), ("Gigs", "B")] {
        let row = format!(
            "(column \"{sheet}\" \"{column}\" {} shown none)",
            buffet::DOLLARS_COLUMN
        );
        assert!(text.lines().any(|l| l == row), "{sheet}!{column}: {row}");
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

/// The hustle at this frame (`BACK.1`; a week and a day's work since
/// `POT.10`): the Hustle sheet's workings, each a cell the hand states, the
/// press's two cells from the frame's inputs and the last frame's, the claim,
/// the claims, the day the next prospectus arrives, whether the hustle is shut
/// today, the time the last claim's work is done, whether a prospectus is
/// being proofread, those done, their fees and when the fee is paid; and Gigs'
/// counts, its fees in dollars and its sentence.
fn hustle_held(
    cart: &Cartridge,
    cal: &Calendar,
    h: &Hustle,
    now: &Inputs,
    last: &Inputs,
    claim: bool,
) {
    let flag = |b: bool| u8::from(b).to_string();
    let at = cal.half_hours();
    let due = h.claims.last().map_or(0, |&(_, t)| t + buffet::WORK);
    let when = if due / A_DAY > cal.day {
        "tomorrow"
    } else {
        "today"
    };
    let got = values(cart, "Hustle", "B9:B19");
    let want = [
        ("B9", flag(Hustle::clicked(now, last))),
        ("B10", flag(Hustle::keyed(now, last))),
        ("B11", flag(claim)),
        ("B12", h.claims.len().to_string()),
        (
            "B13",
            h.claims
                .last()
                .map_or(0, |&(d, _)| d + buffet::DAYS_BETWEEN)
                .to_string(),
        ),
        ("B14", flag(!h.open(cal.day))),
        ("B15", due.to_string()),
        ("B16", flag(h.proofreading(at))),
        ("B17", h.done(at).to_string()),
        ("B18", h.earned(at).to_string()),
        (
            "B19",
            format!("{when} at {}", buffet::time_shown(due % A_DAY)),
        ),
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
        ("B7", h.done(at).to_string()),
        ("B8", (h.cap - h.claims.len()).to_string()),
        ("B9", buffet::two_decimals(h.earned(at))),
        ("A11", h.status(cal)),
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
/// since the owner's hand test; a prospectus a week and a day's work since
/// `POT.10`'s): four weeks and a day at a frame each half hour, the minutes a
/// frame written 30 as a person's edit before the first frame, each act at its
/// day and its frame within it, a day turning at midnight; the Input rows
/// written before each step as the host writes them, the rows that changed
/// alone (`ENGINE.19`), the key through the Keys sheet as the host reads it
/// after every step, which at every frame binds P as the hand says, while
/// Gigs showed and on no other sheet; and at every frame the Hustle sheet's
/// workings and Gigs' counts, fees and sentence the hand's. Its presses: a
/// click claiming, its fee paid a day later; a second click and P the same
/// day refused; a click in the frame the next day turns, the claimed day
/// shown, refused; a click beside the button, one at the button's address
/// while the Portfolio shows, and P there, unbound, no press; a click in the
/// frame the week turns, the last day of the week before shown, refused, and
/// the next frame's claiming; P held on the Portfolio as Gigs is shown under
/// it, a press the frame the key is bound, refused; the button and the key
/// held across the turn into an open day, claiming nothing; P in the frame a
/// day turns, the open day before shown, claiming that day, and the week
/// counted from it; a click and P in one frame claiming once; then the run's
/// four spent, a click, P and a drag onto the button each a press refused.
#[test]
fn a_scripted_session_holds_the_hustle_at_every_frame() {
    use Act::{Key, Left, Point, Show};
    // Each act at its day and its frame within the day: day 0 from its first
    // frame, the opening bell, and every later day from its midnight, a frame
    // each half hour.
    let script: &[(u64, u64, &[Act])] = &[
        (0, 0, &[Show("Portfolio")]),
        (0, 2, &[Show("Gigs")]),
        (0, 3, &[Point(1, 5)]),
        (0, 4, &[Left(true)]), // a click: the first claim, day 0's, its fee paid at day 1's 11:00
        (0, 5, &[Left(false)]),
        (0, 7, &[Left(true)]), // a second click on day 0: refused
        (0, 8, &[Left(false)]),
        (0, 9, &[Key(true)]), // P on Gigs, day 0: refused
        (0, 10, &[Key(false)]),
        (1, 0, &[Left(true)]), // day 1's first frame, day 0 shown, claimed: refused
        (1, 1, &[Left(false)]),
        (1, 2, &[Point(2, 5)]), // the pointer on B5, beside the button
        (1, 3, &[Left(true)]),  // no press
        (1, 4, &[Left(false)]),
        (1, 5, &[Show("Portfolio"), Point(1, 5)]), // the button's address, on another sheet
        (1, 6, &[Left(true)]),                     // no press
        (1, 7, &[Left(false)]),
        (1, 9, &[Key(true)]), // P on the Portfolio, unbound there: no press
        (1, 10, &[Key(false)]),
        (1, 12, &[Show("Gigs")]),
        (7, 0, &[Left(true)]), // day 7's first frame, day 6 shown, the week not out: refused
        (7, 1, &[Left(false)]),
        (7, 2, &[Left(true)]), // the second claim, day 7's
        (7, 3, &[Left(false)]),
        (7, 4, &[Show("Portfolio")]),
        (7, 5, &[Key(true)]),    // P held on the Portfolio, unbound,
        (7, 6, &[Show("Gigs")]), // and Gigs shown under it: bound the frame after, a press, refused
        (7, 8, &[Key(false)]),
        (13, 40, &[Left(true)]), // day 13, the week not out: refused, and held across day 14's turn
        (13, 42, &[Key(true)]),  // the same, by the key
        (14, 3, &[Left(false)]),
        (14, 4, &[Key(false)]),
        (15, 0, &[Key(true)]), // day 15's first frame, day 14 shown, open: the third claim, day 14's
        (15, 1, &[Key(false)]),
        (21, 4, &[Left(true), Key(true)]), // a click and P in one frame: the fourth claim, once
        (21, 5, &[Left(false), Key(false)]),
        (28, 4, &[Left(true)]), // day 28, the run's four spent: refused
        (28, 5, &[Left(false)]),
        (28, 9, &[Key(true)]), // refused
        (28, 10, &[Key(false)]),
        (28, 14, &[Point(0, 0)]),             // the pointer over a header
        (28, 15, &[Point(1, 5), Left(true)]), // dragged onto the button, held: a press, refused
        (28, 17, &[Left(false)]),
    ];
    const PACE: u64 = 30;
    // A day's first frame: the first frame for day 0, its midnight after it.
    let start = |day: u64| match day {
        0 => 1,
        d => (d * A_DAY - FIRST_MINUTE) / PACE + 1,
    };
    let script: Vec<(u64, &[Act])> = script
        .iter()
        .map(|(day, at, set)| (start(*day) + at, *set))
        .collect();
    let (mut cart, mut cal) = at_pace(PACE);
    // The keymap the host holds: the Keys sheet as it read it after the last
    // step, and none at the load, before any formula of it holds a value.
    let mut keys: BTreeMap<String, String> = BTreeMap::new();
    let mut hustle = Hustle::default();
    let mut now = Inputs::default();
    let mut written: Option<Vec<String>> = None;
    let mut acts = script.iter().peekable();
    for frame in 1..start(29) {
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
        // The day and the half hour the person saw, the last frame's.
        let (shown, half) = (cal.day, cal.half_hours());
        let step = cart.step(0);
        cal.tick();
        assert!(step.done && step.frame == cal.frame, "{}", step.row());
        keys = values(&cart, "Keys", "B1:E10");
        let claim = hustle.frame(&now, &last, shown, half);
        hustle_held(&cart, &cal, &hustle, &now, &last, claim);
    }
    assert!(acts.next().is_none(), "every act of the script played");
    let days: Vec<u64> = hustle.claims.iter().map(|&(d, _)| d).collect();
    assert_eq!(
        days,
        [0, 7, 14, 21],
        "the claims, by the days they counted against"
    );
    assert_eq!(
        hustle.earned(cal.half_hours()),
        4 * buffet::FEE,
        "every fee paid by the run's end"
    );
}

/// The hustle's settings and its button (`BACK.1`'s decisions 3, 4, 5 and
/// 9): the Hustle sheet's fee, claims a run, the button's place, and since
/// `POT.10` the days between two prospectuses and a prospectus's work, the
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
    let settings = values(&cart, "Hustle", "B1:B7");
    let want = [
        ("B1", buffet::FEE.to_string()),
        ("B2", buffet::CLAIMS_A_RUN.to_string()),
        ("B3", sheet.to_string()),
        ("B4", column.to_string()),
        ("B5", row.to_string()),
        ("B6", buffet::DAYS_BETWEEN.to_string()),
        ("B7", buffet::WORK.to_string()),
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
        .find(|l| l.starts_with("(formula \"Hustle\" \"B10\" "))
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
