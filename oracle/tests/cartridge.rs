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
//! time and the bell's line beneath it. Since `GUARD.1`: the Series 777, the
//! game's opening; its draw at every seed of a thousand, the hand's and never
//! a question twice; every frame of a scripted exam answered by clicks and by
//! keys, its proctor's books, its two views, the keys each view binds, the
//! Camera's hand-over and the bell's wait held to the hand, then the market's
//! first frames after the bell; the score funding the firm as the contract
//! says; a waiver at the first frame and in the middle; the proctor's
//! settings and buttons, every option whole inside its view's column, and the
//! game opening on the Exam. Every run that holds the market from its first
//! frame writes the waiver before it, so that its calendar stands as before.
//! Since `GUARD.1`'s hand test: ten questions scored and an eleventh, the
//! views' dollars grouped by thousands, every spelling held to the hand's
//! beside every power of ten to 2^53 cents; the Cash sheet and the
//! Portfolio's cash on hand, the firm's and the personal pocket's, held at
//! every frame those runs hold the Portfolio, the exam and the hustle; the
//! buttons' lavender look; the Certificate's caveat under its heading; and
//! the hustle's fee at $250. Since `PLATE.1`: the page's version 5; Trading,
//! the order desk's view, its buttons at the places the Orders sheet names, its
//! keys the cartridge's own commands; every frame of a scripted session of
//! orders, by clicks, keys and the palette, in both pockets, through every
//! refusal, the click beside the command and the closing bell, held to the hand:
//! the order desk's books, the positions, the cash, the commands' words and
//! holds, both views, and the ledger a frame late, the host's derived writes
//! made as the host makes them; the ledger's ring past a hundred fills; the
//! dividend paid at a day's turn to the shares each pocket held; and the desk's
//! settings, buttons and commands. The Portfolio's holding and net worth held
//! at rest in every long run. Since `PLATE.2`: the pockets' switch, a button
//! beside Trading's cash and a command of the cartridge's own on C; every frame
//! of a scripted session of switches, by the click, the key and the palette,
//! beside an order both ways and beside a pick, at a mark and with the market
//! closed, held to the hand, the views following at the switch's own frame;
//! at every switch's frame and the frame after, every balance as the last
//! frame left it (`BD-9`); and PLATE.1's sessions switching by the switch.
//! Since `POT.11`: the pace, twenty seconds a frame in session and two minutes
//! between, the time in seconds a sum over its twin; every frame of a weekday
//! and its night at that pace, where ten days stood, the long runs at their
//! own paces, both written; a price every five minutes, 78 a session, its steps
//! the day's draws 5 to 82, and the volatility factor, at the setting's 4 and
//! written as a person's edit; the clock on every view to the five minutes and
//! the hustle's due spelled to the minute, its session a frame each quarter
//! hour; the pace keyed on the session's bells, held with `B19` written open
//! at every hour as a license would widen it; and the sessions of `PLATE.1`
//! and `PLATE.2` played at the pace to their closing bell, frame 1,171. Since
//! `FRONT.3`: the Overview, where the certificate leads, standing after the
//! Certificate and binding no key: its date, the bell's line, the firm's line,
//! each pocket's net worth, the weather's line and the day's headline, held to
//! the hand at every bell and every day's turn of the long runs, at rest, and
//! at every frame of the order desk's sessions, the holdings among them; the
//! Market sheet's weather, the last close made, the change over the look back,
//! its tenths and its word, at the same frames, and with `B19` written open at
//! every hour; the look back moved to the last close made; and the Headlines
//! sheet, its rows whole, its row shown the hand's draw at every day's turn.
//! Since `BACK.2`: the firm named by the exam's last answer, its style's row of
//! the Firms sheet, the Certificate's name line and the Overview's firm line
//! held to the hand at every frame of the scripted exam, pending until the
//! eleventh answer, and to each style's firm at a waiver; the style and the
//! name read by no number of the game; and with it `MENU.3`'s first slice, the
//! four stocks' names, the hand's, held on the Stocks sheet's rows and in every
//! view, book, sentence, ledger row and command that names a stock, each name
//! inside every column that shows it. Since `CRITIC.1`: the report card, the Day
//! sheet's quarter and its turn at every frame the calendar is held; the
//! Performance sheet's books and card and the Report held to the hand's at every
//! card of the long runs, at every bell and day's turn of the hour's year, and at
//! every frame of a scripted quarter with orders in both pockets and a
//! prospectus claimed in it and paid in the next; the Camera showing the Report
//! at every card and the person's tabs their own between; the Report binding no
//! key; the Remarks sheet; and the style and your own pocket moving no figure of
//! the card. Since the pin to `alonzo` 0.3.0: the page's version 6, each view the
//! game's and read-only and every other sheet a work sheet, the Caveats read-only;
//! and every frame of every run holding, the kit's verbs in expressions expanding
//! to their bytes.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::OnceLock;

use alonzo::cartridge::{Cartridge, Seed};
use buffet_oracle::buffet::{
    self, disclosure, least_index, Calendar, Cards, Exam, Hustle, Inputs, Market, Order, Pace,
    Plate, Shown, AN_HOUR, A_DAY, A_DAY_LONG, CLOSING_TICK, DAYS_A_DISCLOSURE, DAYS_A_YEAR,
    DIFFICULTY, DIVIDEND_DAYS, DRAWS_A_DAY, FIRST_MINUTE, LOOK_BACK, PACE, QUARTER_HOUR, RUNGS,
    WAVES, YEAR,
};
use buffet_oracle::{derived, expand_source, gather, lf, load, read, root, values};
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
/// The firms, each its style, one of the exam's eleventh question's options
/// (`GUARD.1`), and the name the style gives the firm (`BACK.2`).
const FIRMS: &str = "src/content/firms.vla";
/// The ticker's headlines, the Overview's headline of the day (`FRONT.3`).
const HEADLINES: &str = "src/content/headlines.vla";
/// The proctor's remarks, a report card's remark on its grade (`CRITIC.1`).
const REMARKS: &str = "src/content/remarks.vla";
/// The content sheets, each written by a file of `src/content/`: the sheet,
/// its file, and the state the source writes before its call, hidden as a
/// workbook keeps its reference data at the back, or visible for the one a
/// player reads whole (`MENU.5`'s decision 2).
const CONTENT: [(&str, &str, &str); 6] = [
    ("Disclosures", DISCLOSURES, "hidden"),
    ("Questions", QUESTIONS, "hidden"),
    ("Firms", FIRMS, "hidden"),
    ("Headlines", HEADLINES, "hidden"),
    ("Remarks", REMARKS, "hidden"),
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
/// with `BACK.1`, the side hustle's; the Exam and the Certificate with
/// `GUARD.1`, standing before the Portfolio, the game opening on the Exam;
/// Trading with `PLATE.1`, beside the Portfolio; the Overview with `FRONT.3`,
/// after the Certificate, where the certificate leads; and the Report with
/// `CRITIC.1`, after Gigs, where a player meets it at the first card.
const VIEWS: [&str; 7] = [
    "Exam",
    "Certificate",
    "Overview",
    "Portfolio",
    "Trading",
    "Gigs",
    "Report",
];
/// Gigs' labels down its column A from row 7 (`BACK.1`'s decision 3), beside
/// the prospectuses proofread, what remains of the run's claims and the fees.
const GIGS_LABELS: [&str; 3] = ["Proofread", "Left this run", "Fees earned ($)"];
/// The Portfolio's headers over its columns from A, in row 4 (`FRONT.1`'s
/// decision 6); and over the holding's columns from E (`PLATE.1`).
const PORTFOLIO_HEADERS: [&str; 3] = ["Security", "Price ($)", "Change ($)"];
const HOLDING_HEADERS: [&str; 4] = ["Shares", "Cost ($)", "Value ($)", "Gain ($)"];
/// The Portfolio's cash on hand under its stocks (`GUARD.1`'s hand test, the
/// owner's word, both pockets the owner's choice): its heading and the two
/// pockets' labels, their dollars beside them in column B; and each pocket's
/// net worth under them (`PLATE.1`).
const CASH_LABELS: [(&str, &str); 6] = [
    ("A10", "Cash on hand ($)"),
    ("A11", "  Firm"),
    ("A12", "  Personal"),
    ("A14", "Net worth ($)"),
    ("A15", "  Firm"),
    ("A16", "  Personal"),
];
/// The firm's capital when the exam is waived, five right answers' (`GUARD.1`).
const WAIVED_CAPITAL: u64 = buffet::CAPITAL_BEFORE + buffet::CAPITAL_A_RIGHT * buffet::WAIVED_SCORE;
/// Trading's headers over its columns from B, in row 5 (`PLATE.1`).
const TRADING_HEADERS: [&str; 3] = ["Security", "Price ($)", "Shares"];

/// The cartridge as a stranger loads it, the exam to be sat (`GUARD.1`).
fn loaded() -> Cartridge {
    load(CARTRIDGE).unwrap_or_else(|e| panic!("{CARTRIDGE} loads: {e}"))
}

/// The cartridge with the exam waived before its first frame, a person's
/// edit of the Proctor's `B3` (`GUARD.1`'s decision 12): the game begun at the
/// first frame and the market from it, as every run that holds the market
/// plays, its calendar standing as it did before the exam came.
fn cartridge() -> Cartridge {
    let mut cart = loaded();
    let (sheet, at) = buffet::WAIVER;
    let edit = format!("(cell \"{sheet}\" \"{at}\" 1)");
    cart.write(&edit)
        .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    cart
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

/// The Day sheet at this frame (`POT.1`, `POT.10`, `POT.11`), every cell its
/// value as the record prints it, held to the hand's calendar: the seconds a
/// frame between sessions and the days a year; the day, the time in seconds,
/// the boundary, the year and the day of the year; the first frame's time, the
/// bells and the trading days; the time to the five minutes and of the day, the
/// time shown, the weekday and its name; whether the market is open; the
/// session's day, the one before it, the tick and the closing tick; whether
/// the exam holds the bell and the frames it held it (`GUARD.1`); the
/// session's seconds a frame, whether it is in session, the pace now and the
/// time shown, the five minutes in session and the half hour between; and the
/// quarter, the quarters counted and whether this frame turned one
/// (`CRITIC.1`).
fn day_held(cart: &Cartridge, cal: &Calendar) {
    let f = cal.frame;
    let got = values(cart, "Day", "B1:B35");
    let want = [
        ("B1", cal.pace.between.to_string()),
        ("B2", cal.days_a_year.to_string()),
        ("B3", cal.day.to_string()),
        ("B4", cal.seconds().to_string()),
        ("B5", u8::from(cal.turned).to_string()),
        ("B6", cal.year.to_string()),
        ("B7", cal.day_of_year.to_string()),
        ("B9", FIRST_MINUTE.to_string()),
        ("B10", buffet::OPENING_BELL.to_string()),
        ("B11", buffet::CLOSING_BELL.to_string()),
        ("B12", buffet::TRADING_DAYS.to_string()),
        ("B14", cal.five_minutes().to_string()),
        ("B15", cal.five().to_string()),
        ("B16", buffet::time_shown(cal.clock())),
        ("B17", cal.weekday.to_string()),
        ("B18", buffet::DAY_NAMES[cal.weekday as usize].to_string()),
        ("B19", u8::from(cal.open()).to_string()),
        ("B20", cal.session.to_string()),
        ("B21", cal.before.to_string()),
        ("B22", cal.tick.to_string()),
        ("B23", CLOSING_TICK.to_string()),
        ("B25", u8::from(cal.waits).to_string()),
        ("B26", cal.waited.to_string()),
        ("B28", cal.pace.session.to_string()),
        ("B29", u8::from(cal.in_session()).to_string()),
        ("B30", cal.pace_now().to_string()),
        ("B31", cal.shown().to_string()),
        ("B33", cal.quarter().to_string()),
        ("B34", cal.quarters().to_string()),
        ("B35", u8::from(cal.quarter_turned).to_string()),
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

/// What a long run holds whole when it moves and byte for byte between: the
/// Portfolio's record at each five minutes; the Dice sheet's and the session
/// table's at each day's turn, the day's die and draws and the steps they make;
/// and the waves' workings at each session and each close (`POT.11`, its 82
/// draws and 78 steps a day, spelled again at every frame, having taken the
/// hour's year from 5.8 s to 9.5; `FRONT.3`, the look back at the last close
/// made). And the session last held, so that the Overview and the weather are
/// held whole at each bell as well as at each day's turn (`FRONT.3`); and the
/// hand's report card, its books kept at every frame (`CRITIC.1`).
#[derive(Default)]
struct Held {
    portfolio: Option<(u64, String)>,
    dice: Option<(u64, String)>,
    table: Option<(u64, String)>,
    workings: Option<(u64, String)>,
    session: Option<u64>,
    cards: Cards,
}

/// The firms (`BACK.2`), its content file's rows read once by the language's own
/// reader, each its style and its name, which `each_content_sheet_is_its_file`
/// holds the Firms sheet to.
fn firm_rows() -> &'static [(String, String)] {
    static ROWS: OnceLock<Vec<(String, String)>> = OnceLock::new();
    ROWS.get_or_init(|| {
        content(FIRMS)
            .rows
            .into_iter()
            .map(|r| match r.words.as_slice() {
                [style, name] => (style.clone(), name.clone()),
                w => panic!("{FIRMS}: a firm is its style and its name: {w:?}"),
            })
            .collect()
    })
}

/// The Overview's firm line for a style (`BACK.2`), the hand's: pending at 0,
/// else the name and the style of the firm's row the style is the place of.
fn firm_line_of(style: u64) -> String {
    let rows = firm_rows();
    buffet::firm_line(match style {
        0 => None,
        s => {
            let (style, name) = &rows[s as usize - 1];
            Some((style.as_str(), name.as_str()))
        }
    })
}

/// The ticker's headlines (`FRONT.3`), its content file's rows read once by the
/// language's own reader, which `each_content_sheet_is_its_file` holds the
/// Headlines sheet to.
fn headline_texts() -> &'static [String] {
    static TEXTS: OnceLock<Vec<String>> = OnceLock::new();
    TEXTS.get_or_init(|| {
        content(HEADLINES)
            .rows
            .into_iter()
            .map(|mut r| r.words.remove(0))
            .collect()
    })
}

/// A window's record at this frame, as the `values` view prints it.
fn record(cart: &Cartridge, sheet: &str, window: &str) -> String {
    let Ok(Viewed::Text(record)) = cart.view("values", sheet, Some(window)) else {
        panic!("the {sheet} sheet's record of {window}");
    };
    record
}

/// A window held whole by `hold` when `key` moves, and at every other frame
/// its record the one held then, byte for byte.
fn still_or_held(
    cart: &Cartridge,
    cal: &Calendar,
    (sheet, window): (&str, &str),
    key: u64,
    kept: &mut Option<(u64, String)>,
    hold: impl FnOnce(),
) {
    let now = record(cart, sheet, window);
    if let Some((was, held)) = kept.as_ref() {
        if *was == key {
            assert!(
                now == *held,
                "frame {}, day {}: {sheet}!{window} moved, its key {key} standing",
                cal.frame,
                cal.day
            );
            return;
        }
    }
    hold();
    *kept = Some((key, now));
}

/// The Dice sheet at this frame (`POT.1`, `POT.10`, `POT.11`): the market's die
/// and the day's 83 draws, the headline's among them since `FRONT.3`, each draw
/// and its uniform, the hand's found by place; and the exam's die and its two
/// draws.
fn dice_held(cart: &Cartridge, cal: &Calendar, seed: u64) {
    let f = cal.frame;
    let dice = values(cart, "Dice", DICE);
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
    for draw in 1..=DRAWS_A_DAY {
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
}

/// The Dice sheet's window: the market's die and its 83 draws down B, their
/// uniforms down C, and the exam's die and its two draws in I and J.
const DICE: &str = "B1:J84";

/// What the waves' workings move with: the session, at its bell, and since
/// `FRONT.3` the last close made, at the session's close, the look back's day.
fn workings_key(cal: &Calendar) -> u64 {
    cal.session * 2 + u64::from(cal.last_close() == cal.session)
}

/// The Day, Dice and Market sheets at this frame, each cell its value as the
/// record prints it, held to the hand's calendar, dice and market for the
/// same frame; and the Stocks and the Disclosures sheets with them, and the
/// views of them all: the Portfolio, its record held whole at each five
/// minutes and kept for the frames after it, and Gigs at rest. The dice and the
/// session's table are held whole at each day's turn and the waves' workings
/// at each session and each close, byte for byte between (`Held`). Since
/// `FRONT.3`, at each bell and each day's turn, the Overview at rest and the
/// Market sheet's weather, the one reading the last close made in session and
/// the other outside it; and the day's headline at each day's turn. Since
/// `CRITIC.1`, the hand's report card kept at every frame, the firm at rest and
/// its worth its capital; the Performance sheet held at each day's turn and at
/// each card, and the Report at each bell, each day's turn and each card.
fn held(
    cart: &Cartridge,
    cal: &Calendar,
    seed: u64,
    market: &Market,
    texts: &[String],
    kept: &mut Held,
) {
    day_held(cart, cal);
    still_or_held(cart, cal, ("Dice", DICE), cal.day, &mut kept.dice, || {
        dice_held(cart, cal, seed)
    });
    still_or_held(
        cart,
        cal,
        ("Market", "B8:E26"),
        workings_key(cal),
        &mut kept.workings,
        || waves_held(cart, cal, market, DIFFICULTY),
    );
    still_or_held(
        cart,
        cal,
        ("Market", "B29:C106"),
        cal.day,
        &mut kept.table,
        || table_held(cart, cal, seed, market),
    );
    tick_held(cart, cal, seed, market, DIFFICULTY);
    let stocks = stocks_held(cart, cal, seed, market, DIFFICULTY);
    disclosures_held(cart, cal, texts, DAYS_A_DISCLOSURE);
    let turned = kept.portfolio.as_ref().map(|(five, _)| *five) != Some(cal.five_minutes());
    portfolio_held(cart, cal, &stocks, texts, &mut kept.portfolio);
    if turned {
        gigs_held(cart, cal, texts);
    }
    // Trading at rest at each day's first frame: its prices are the Portfolio's
    // own cents, held above at every half hour, and every frame of it at work is
    // the order desk's sessions' (PLATE.1); at every half hour of the year at a
    // frame an hour it made that run half again as long.
    if cal.turned {
        trading_cells(
            cart,
            cal,
            &stocks,
            texts,
            &Plate::default(),
            [WAIVED_CAPITAL, buffet::PERSONAL_CASH],
        );
        headlines_held(cart, cal, seed);
    }
    // The Overview at rest and the weather at each bell and each day's turn
    // (FRONT.3): the weather reads the session before's close at the bell and
    // the session's own outside it, and changes only at a close, which a day's
    // turn follows before the next bell; the headline moves at the day's turn.
    let rang = kept.session != Some(cal.session);
    kept.session = Some(cal.session);
    if rang || cal.turned {
        weather_held(cart, cal, market, DIFFICULTY);
        overview_cells(
            cart,
            cal,
            (seed, market),
            texts,
            &Plate::default(),
            ([WAIVED_CAPITAL, buffet::PERSONAL_CASH], prices_of(&stocks)),
            buffet::WAIVED_STYLE,
        );
    }
    // The report card (CRITIC.1): the firm at rest, its worth its capital, and
    // Average Co. at the Stocks sheet's own price now, held to the hand above.
    let price = prices_of(&stocks)[buffet::BENCHMARK];
    let card = kept
        .cards
        .frame(cal, cal.frame > 1, WAIVED_CAPITAL, price, WAIVED_CAPITAL);
    if card || cal.turned {
        performance_held(cart, cal, &kept.cards, price, WAIVED_CAPITAL);
    }
    if card || rang || cal.turned {
        report_cells(cart, cal, &kept.cards, (seed, texts), buffet::WAIVED_STYLE);
    }
}

/// The Market sheet's weather at this frame (`FRONT.3`), `B121:B124`: the index
/// at the last close made, the change over the look back and the change in
/// whole tenths of a percent, each bit for bit, and the word, the hand's.
fn weather_held(cart: &Cartridge, cal: &Calendar, market: &Market, difficulty: u64) {
    let got = values(cart, "Market", "B121:B124");
    let number = |addr: &str| -> f64 {
        got.get(addr)
            .unwrap_or_else(|| panic!("frame {}: Market!{addr} holds no value", cal.frame))
            .parse()
            .unwrap_or_else(|e| panic!("frame {}: Market!{addr}: {e}", cal.frame))
    };
    let tenths = market.tenths(difficulty, cal);
    for (addr, want) in [
        ("B121", market.index(difficulty, cal.last_close() as f64)),
        ("B122", market.change(difficulty, cal)),
        ("B123", tenths as f64),
    ] {
        assert_eq!(
            number(addr).to_bits(),
            (want + 0.0).to_bits(),
            "frame {}, day {}: Market!{addr} is {} and the hand's {want}",
            cal.frame,
            cal.day,
            number(addr)
        );
    }
    assert_eq!(
        got.get("B124").map(String::as_str),
        Some(buffet::weather(tenths)),
        "frame {}, day {}: Market!B124, the weather",
        cal.frame,
        cal.day
    );
}

/// The Headlines sheet's row shown and its text at this frame (`FRONT.3`): the
/// row the hand draws for the calendar's day, and that row's headline.
fn headlines_held(cart: &Cartridge, cal: &Calendar, seed: u64) {
    let headlines = headline_texts();
    let got = values(cart, "Headlines", "D2:D3");
    let row = buffet::headline(seed, cal.day, headlines.len() as u64);
    assert_eq!(
        got.get("D2"),
        Some(&row.to_string()),
        "frame {}, day {}: Headlines!D2",
        cal.frame,
        cal.day
    );
    assert_eq!(
        got.get("D3"),
        Some(&headlines[row as usize - 1]),
        "frame {}, day {}: Headlines!D3",
        cal.frame,
        cal.day
    );
}

/// Every cell of the Overview at this frame (`FRONT.3`), the hand's: its date
/// and the bell's line, every view's; the firm's line, by the style the exam
/// gave (`BACK.2`, `style`, a waiver's in the runs that waive it); each
/// pocket's net worth, its cash from its start at this frame and its holdings
/// at the price now (`starts`, `prices`), in dollars, under its label; the
/// weather's line, the hand's word and change at the last close made; the
/// day's headline, the row the hand draws; and the fine print.
fn overview_cells(
    cart: &Cartridge,
    cal: &Calendar,
    (seed, market): (u64, &Market),
    texts: &[String],
    plate: &Plate,
    (starts, prices): ([u64; 2], [u64; 4]),
    style: u64,
) {
    let got = values(cart, "Overview", "A1:B13");
    let text = |addr: &str, want: &str| {
        assert_eq!(
            got.get(addr).map(String::as_str),
            Some(want),
            "frame {}, day {}, {}: Overview!{addr}",
            cal.frame,
            cal.day,
            buffet::time_shown(cal.hour * 60 + cal.minute)
        );
    };
    text("A1", &buffet::as_of(cal));
    text("A2", buffet::bell(cal));
    text("A3", &firm_line_of(style));
    for (k, label) in buffet::NET_WORTH_LABELS.iter().enumerate() {
        text(&format!("A{}", 5 + k), label);
    }
    for (k, pocket) in [buffet::FIRM, buffet::PERSONAL].into_iter().enumerate() {
        let worth = plate.cash(pocket, starts[k]) + plate.holdings(pocket, prices);
        text(&format!("B{}", 6 + k), &buffet::two_decimals(worth));
    }
    text(
        "A9",
        &buffet::weather_line(market.tenths(DIFFICULTY, cal), LOOK_BACK),
    );
    let headlines = headline_texts();
    let row = buffet::headline(seed, cal.day, headlines.len() as u64);
    text(
        "A11",
        &format!("{}{}", buffet::HEADLINE_LABEL, headlines[row as usize - 1]),
    );
    let shown = disclosure(cal.day, texts.len() as u64, DAYS_A_DISCLOSURE);
    text("A13", &texts[shown as usize - 1]);
}

/// The proctor's remarks (`CRITIC.1`), its content file's rows read once by the
/// language's own reader, a row's place its grade, which
/// `each_content_sheet_is_its_file` holds the Remarks sheet to.
fn remark_texts() -> &'static [String] {
    static TEXTS: OnceLock<Vec<String>> = OnceLock::new();
    TEXTS.get_or_init(|| {
        content(REMARKS)
            .rows
            .into_iter()
            .map(|mut r| r.words.remove(0))
            .collect()
    })
}

/// A flag as a cell holds it, 1 or 0.
fn flag(b: bool) -> String {
    u8::from(b).to_string()
}

/// The Performance sheet at this frame (`CRITIC.1`), every cell the hand's: the
/// view a card is shown on; Average Co.'s price now, `price`, and the shares and
/// cents left over the capital bought at the begin; the calendar's turns, a
/// quarter's and a year's; the cards issued and the last card's quarter and
/// year, 0 before the first; each one's worth, the firm's now, `firm`, and the
/// benchmark's, then at the last card or the begin, at the one before it, at
/// the year's start or the begin, at the last year's start, 0 before each is
/// taken, and at the card's own year's start; the card's returns, alphas,
/// grades and remark, the empty text and the first remark before the first
/// card; and the grades' table.
fn performance_held(cart: &Cartridge, cal: &Calendar, cards: &Cards, price: u64, firm: u64) {
    let got = values(cart, "Performance", "A1:G29");
    let card = cards.card();
    let zero = [0, 0];
    let mut want: Vec<(String, String)> = vec![
        ("B1".into(), buffet::CARD_VIEW.into()),
        ("B3".into(), price.to_string()),
        ("B4".into(), cards.shares.to_string()),
        ("B5".into(), cards.left.to_string()),
        ("B6".into(), flag(cal.quarter_turned)),
        ("B7".into(), flag(cal.quarter_turned && cal.quarter() == 1)),
        ("B8".into(), cards.turns.len().to_string()),
        ("B9".into(), card.map_or(0, |c| c.quarter).to_string()),
        ("B10".into(), card.map_or(0, |c| c.year).to_string()),
        ("B22".into(), card.map_or(1, |c| c.remark).to_string()),
    ];
    let worths = [
        ("B", [firm, cards.benchmark(price)]),
        ("C", cards.last()),
        ("D", cards.before().unwrap_or(zero)),
        ("E", cards.year_start()),
        ("F", cards.last_year_start().unwrap_or(zero)),
        ("G", card.map_or(cards.year_start(), |c| c.year_start)),
    ];
    for (column, worth) in worths {
        for (k, w) in worth.iter().enumerate() {
            want.push((format!("{column}{}", 13 + k), w.to_string()));
        }
    }
    for (p, column) in ["B", "C"].iter().enumerate() {
        let figures: [String; 5] = match card {
            None => std::array::from_fn(|_| String::new()),
            Some(c) => [
                c.returns[p][0].to_string(),
                c.returns[p][1].to_string(),
                c.alpha[p].to_string(),
                c.grades[p].0.to_string(),
                c.grades[p].1.to_string(),
            ],
        };
        for (r, figure) in (17..).zip(figures) {
            want.push((format!("{column}{r}"), figure));
        }
    }
    for (k, (letter, from)) in buffet::GRADES.iter().enumerate() {
        want.push((format!("A{}", 25 + k), letter.to_string()));
        want.push((format!("B{}", 25 + k), from.to_string()));
    }
    want.push(("A29".into(), buffet::GRADE_BELOW.into()));
    for (addr, text) in want {
        assert_eq!(
            got.get(&addr),
            Some(&text),
            "frame {}, day {}, {}: Performance!{addr}",
            cal.frame,
            cal.day,
            buffet::time_shown(cal.hour * 60 + cal.minute)
        );
    }
    assert_eq!(got.get("B29"), None, "F below every grade, none of its own");
}

/// Every cell of the Report at this frame (`CRITIC.1`), the hand's: its date
/// and the bell's line; the card's header, by the firm's name the style gives
/// it, or pending registration, the seed and the tier; its title; its columns'
/// headers, the last the year's own at a fourth card; the firm's row by its
/// name and Average Co.'s, each one's worth at the last card or the begin in
/// dollars, its returns in percent and the alphas under them, empty before the
/// first card; the grades in a sentence; the proctor's remark, the Remarks
/// sheet's row of the card's grade; the line on your own account; and the fine
/// print.
fn report_cells(
    cart: &Cartridge,
    cal: &Calendar,
    cards: &Cards,
    (seed, texts): (u64, &[String]),
    style: u64,
) {
    let got = values(cart, buffet::CARD_VIEW, "A1:D16");
    let text = |addr: &str, want: &str| {
        assert_eq!(
            got.get(addr).map(String::as_str),
            Some(want),
            "frame {}, day {}, {}: Report!{addr}",
            cal.frame,
            cal.day,
            buffet::time_shown(cal.hour * 60 + cal.minute)
        );
    };
    let card = cards.card();
    let firm = match style {
        0 => None,
        s => Some(firm_rows()[s as usize - 1].1.as_str()),
    };
    text("A1", &buffet::as_of(cal));
    text("A2", buffet::bell(cal));
    text("A3", &buffet::card_header(firm, seed, DIFFICULTY));
    text("A5", &buffet::card_title(card.as_ref()));
    text("B6", buffet::CARD_HEADERS[0]);
    text("C6", buffet::CARD_HEADERS[1]);
    text("D6", buffet::year_header(card.as_ref()));
    text("A7", firm.unwrap_or("Your firm"));
    text(
        "A8",
        &format!("{}, bought and held", RUNGS[buffet::BENCHMARK].name),
    );
    text("A9", buffet::ALPHA_LABEL);
    for (k, worth) in cards.last().iter().enumerate() {
        text(&format!("B{}", 7 + k), &buffet::two_decimals(*worth));
    }
    for (p, column) in ["C", "D"].iter().enumerate() {
        let figures: [String; 3] = match card {
            None => std::array::from_fn(|_| String::new()),
            Some(c) => [
                buffet::percent(c.returns[p][0]),
                buffet::percent(c.returns[p][1]),
                buffet::percent(c.alpha[p]),
            ],
        };
        for (r, figure) in (7..).zip(&figures) {
            text(&format!("{column}{r}"), figure);
        }
    }
    text("A11", &buffet::grade_line(card.as_ref()));
    let remark = match card {
        None => String::new(),
        Some(c) => format!(
            "{}{}",
            buffet::REMARK_LEAD,
            remark_texts()[c.remark as usize - 1]
        ),
    };
    text("A12", &remark);
    text("A14", buffet::OWN_ACCOUNT);
    let shown = disclosure(cal.day, texts.len() as u64, DAYS_A_DISCLOSURE);
    text("A16", &texts[shown as usize - 1]);
}

/// Every cell of Trading at this frame (`PLATE.1`), the hand's: its date, the
/// bell's line and where an order's money comes from; its headers; each
/// stock's mark when picked, its name, the hand's (`MENU.3`), its price now, the Stocks sheet's cents
/// (`stocks`) spelled as dollars, the pocket's shares of it and its six
/// buttons; the pocket's cash, and beside it the switch and its key
/// (`PLATE.2`); the last order's sentence; the keys' line; and the fine print.
/// Between two half hours nothing moves it but an order, a pick or a switch,
/// so the long runs hold it at rest at each day's turn.
fn trading_cells(
    cart: &Cartridge,
    cal: &Calendar,
    stocks: &BTreeMap<String, String>,
    texts: &[String],
    plate: &Plate,
    starts: [u64; 2],
) {
    let got = values(cart, "Trading", "A1:J17");
    let text = |addr: &str, want: &str| {
        assert_eq!(
            got.get(addr).map(String::as_str),
            Some(want),
            "frame {}, day {}, {}: Trading!{addr}",
            cal.frame,
            cal.day,
            buffet::time_shown(cal.hour * 60 + cal.minute)
        );
    };
    let pocket = plate.funding();
    text("A1", &buffet::as_of(cal));
    text("A2", buffet::bell(cal));
    text("A3", buffet::trading_line(pocket));
    for (k, header) in TRADING_HEADERS.iter().enumerate() {
        text(&address(5, 2 + k), header);
    }
    for (k, name) in buffet::names().iter().enumerate() {
        let row = buffet::STOCK_ROWS[k] as usize;
        let mark = if plate.picked == k as u64 + 1 {
            "▸"
        } else {
            ""
        };
        text(&format!("A{row}"), mark);
        text(&format!("B{row}"), name);
        let now: u64 = stocks[&format!("E{}", 5 + k)].parse().expect("whole cents");
        text(&format!("C{row}"), &buffet::two_decimals(now));
        text(
            &format!("D{row}"),
            &plate.book(pocket, k as u64 + 1).shares.to_string(),
        );
        for (t, label) in buffet::TRADE_TEXTS.iter().enumerate() {
            text(&address(row, buffet::TRADE_COLUMNS[t] as usize), label);
        }
    }
    text("B11", "Cash available ($)");
    text(
        "C11",
        &buffet::two_decimals(plate.cash(pocket, starts[pocket as usize - 1])),
    );
    text("D11", buffet::SWITCH_TEXT);
    text("E11", buffet::SWITCH_KEY_TEXT);
    let said = match plate.last {
        Some(o) => o.sentence(),
        None => buffet::NO_ORDER.to_string(),
    };
    text("A13", &said);
    text("A15", buffet::TRADING_KEYS);
    let shown = disclosure(cal.day, texts.len() as u64, DAYS_A_DISCLOSURE);
    text("A17", &texts[shown as usize - 1]);
}

/// Gigs at rest (`BACK.1`), held whenever the five minutes turn (`POT.11`): its
/// date and the bell's line and its fine print the Portfolio's, the hand's; no
/// prospectus proofread, the run's claims all left, nothing earned, and a
/// prospectus waiting. Between two five minutes nothing moves it but an input,
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

/// The Portfolio at this frame (`FRONT.1`), held whole whenever the five
/// minutes turn (`POT.10`, `POT.11`): its date and the bell's line, the hand's; each
/// stock's price now and its change since the last close, the whole cents of
/// the Stocks sheet's same frame, which `stocks_held` has just held to the
/// hand, spelled as dollars by text, so that a cell spelled any other way
/// than its whole cents over 100 fails here; beside the change, which way it
/// went (`FRONT.2`), the hand's glyph of the same two prices; under them the
/// cash on hand (`GUARD.1`'s hand test), the firm's and the personal pocket's
/// in dollars, `cash`, which these runs, the exam waived and no prospectus
/// claimed, hold at $7,500 and $100; and the fine print at its foot, the row
/// the hand picks for the day.
/// On every other frame of the five minutes its record is the one held at
/// their first frame, byte for byte, as `BD-7` keeps a view still between two
/// of the calendar's five minutes, every price moving at a tick or a bell and
/// each of those on a five minutes: the same cells held at a view's cost,
/// where parsing and spelling them again at every frame took the suite's
/// longest test 0.85 s more (`FRONT.1`).
fn portfolio_held(
    cart: &Cartridge,
    cal: &Calendar,
    stocks: &BTreeMap<String, String>,
    texts: &[String],
    view: &mut Option<(u64, String)>,
) {
    let Ok(Viewed::Text(record)) = cart.view("values", "Portfolio", Some("A1:H18")) else {
        panic!("the Portfolio's record");
    };
    if let Some((five, held)) = view.as_ref() {
        if *five == cal.five_minutes() {
            assert!(
                record == *held,
                "frame {}, day {}: the Portfolio moved within the five minutes",
                cal.frame,
                cal.day
            );
            return;
        }
    }
    portfolio_cells(
        cart,
        cal,
        stocks,
        texts,
        &Plate::default(),
        [WAIVED_CAPITAL, buffet::PERSONAL_CASH],
    );
    *view = Some((cal.five_minutes(), record));
}

/// Every cell of the Portfolio at this frame (`FRONT.1`, `FRONT.2`, `GUARD.1`'s
/// hand test, `PLATE.1`), the hand's: its date, the bell's line and whose
/// holdings it shows; each stock's name (`MENU.3`), its price now and its change since the last
/// close, the Stocks sheet's own cents (`stocks`, held to the hand just before)
/// spelled as dollars, and its glyph; the pocket's holding of it, its shares,
/// their cost basis, their value at the price now and the gain not yet taken,
/// from the hand's books; each pocket's cash on hand and net worth, from its
/// start at this frame (`starts`, the firm's and your own); and the fine print.
fn portfolio_cells(
    cart: &Cartridge,
    cal: &Calendar,
    stocks: &BTreeMap<String, String>,
    texts: &[String],
    plate: &Plate,
    starts: [u64; 2],
) {
    let got = values(cart, "Portfolio", "A1:H18");
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
    let pocket = plate.funding();
    text("A3", buffet::holdings_line(pocket));
    for (k, header) in HOLDING_HEADERS.iter().enumerate() {
        text(&address(4, 5 + k), header);
    }
    let mut prices = [0; 4];
    for (k, row) in (5..5 + RUNGS.len()).enumerate() {
        let cents = |column: &str| -> i64 {
            stocks[&format!("{column}{row}")]
                .parse()
                .expect("whole cents")
        };
        let (now, was) = (cents("E"), cents("F"));
        prices[k] = now as u64;
        text(&format!("A{row}"), RUNGS[k].name);
        text(&format!("B{row}"), &buffet::two_decimals(now as u64));
        text(&format!("C{row}"), &buffet::signed_two_decimals(now - was));
        text(&format!("D{row}"), buffet::glyph(now, was));
        let book = plate.book(pocket, k as u64 + 1);
        let value = book.shares * now as u64;
        text(&format!("E{row}"), &book.shares.to_string());
        text(&format!("F{row}"), &buffet::two_decimals(book.basis));
        text(&format!("G{row}"), &buffet::two_decimals(value));
        text(
            &format!("H{row}"),
            &buffet::signed_two_decimals(value as i64 - book.basis as i64),
        );
    }
    for (addr, want) in CASH_LABELS {
        text(addr, want);
    }
    for (k, pocket) in [buffet::FIRM, buffet::PERSONAL].into_iter().enumerate() {
        let cash = plate.cash(pocket, starts[k]);
        text(&format!("B{}", 11 + k), &buffet::two_decimals(cash));
        text(
            &format!("B{}", 15 + k),
            &buffet::two_decimals(cash + plate.holdings(pocket, prices)),
        );
    }
    let shown = disclosure(cal.day, texts.len() as u64, DAYS_A_DISCLOSURE);
    text("A18", &texts[shown as usize - 1]);
}

/// The Market sheet at this frame (`POT.2`, `POT.10`, `POT.11`): each wave's
/// speed and tier 1's phases, the cycle on the first day, and every formula of
/// the three columns, the session's day, the look back and the session before,
/// and of the session's five minutes, its steps and their sums, the tick, its
/// share, the five minutes' swing at the market's factor, the swing and the
/// index now, held bit for bit to the hand's market at the calendar's frame and
/// a difficulty. The hand's value has 0.0 added, which turns its rare -0 into
/// the 0 a cell holds instead and moves nothing else.
fn market_held(cart: &Cartridge, cal: &Calendar, seed: u64, market: &Market, difficulty: u64) {
    waves_held(cart, cal, market, difficulty);
    table_held(cart, cal, seed, market);
    tick_held(cart, cal, seed, market, difficulty);
}

/// A window of the Market sheet, and a check of each of its cells against the
/// hand's value, bit for bit.
fn market_cells(cart: &Cartridge, cal: &Calendar, window: &str) -> impl Fn(&str, f64) {
    let f = cal.frame;
    let day = cal.day;
    let got = values(cart, "Market", window);
    move |addr: &str, want: f64| {
        let text = got
            .get(addr)
            .unwrap_or_else(|| panic!("frame {f}: Market!{addr} holds no value"));
        let v: f64 = text
            .parse()
            .unwrap_or_else(|e| panic!("frame {f}: Market!{addr}, {text}: {e}"));
        assert_eq!(
            v.to_bits(),
            (want + 0.0).to_bits(),
            "frame {f}, day {day}: Market!{addr} is {v} and the hand's {want}"
        );
    }
}

/// The waves and the three columns of workings, `B8:E26`: each wave's speed
/// and tier 1's phases, the cycle on the first day, and the session's day, the
/// look back, twenty days before the last close made since `FRONT.3`, and the
/// session before, each column's terms, trend, cycle, tiers and index.
fn waves_held(cart: &Cartridge, cal: &Calendar, market: &Market, difficulty: u64) {
    let cell = market_cells(cart, cal, "B8:E26");
    let mut waves = vec![(YEAR, 0.0)];
    waves.extend(WAVES.iter().copied().zip(market.phases));
    for (row, (wave, phase)) in (8..).zip(&waves) {
        cell(&format!("D{row}"), wave.speed());
        if row > 8 {
            cell(&format!("E{row}"), *phase);
        }
    }
    cell("B23", market.first_cycle);
    let session = cal.session as f64;
    let days = [
        ("B", session),
        ("C", cal.last_close() as f64 - LOOK_BACK as f64),
        ("D", cal.before as f64),
    ];
    for (column, day) in days {
        cell(&format!("{column}15"), day);
        for (row, (wave, phase)) in (16..).zip(&waves) {
            cell(&format!("{column}{row}"), wave.term(*phase, day));
        }
        cell(&format!("{column}21"), Market::trend(day));
        cell(&format!("{column}22"), market.cycle(day));
        cell(&format!("{column}24"), Market::tier0(day));
        cell(&format!("{column}25"), market.tier1(day));
        cell(&format!("{column}26"), market.index(difficulty, day));
    }
}

/// The session's table, `B29:C106`: each five minutes' step from the day's
/// draws and the steps so far.
fn table_held(cart: &Cartridge, cal: &Calendar, seed: u64, market: &Market) {
    let cell = market_cells(cart, cal, "B29:C106");
    let path = market.path(seed, DIFFICULTY, cal);
    for (row, (step, sum)) in (29..).zip(path.steps.iter().zip(&path.sums)) {
        cell(&format!("B{row}"), *step);
        cell(&format!("C{row}"), *sum);
    }
}

/// The session's tick, `B108:B112`: the tick, its share, the five minutes'
/// swing at the market's factor, the swing at the tick and the index now.
fn tick_held(cart: &Cartridge, cal: &Calendar, seed: u64, market: &Market, difficulty: u64) {
    let cell = market_cells(cart, cal, "B108:B112");
    let path = market.path(seed, difficulty, cal);
    cell("B108", cal.tick as f64);
    cell("B109", path.share);
    cell("B110", path.tick_swing);
    cell("B111", path.swing);
    cell("B112", path.now);
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

/// Ten minutes a second in session and an hour a second between sessions at
/// the manifest's rate (`POT.10`, `POT.11`, the owner's words of 2026-10-19):
/// the rate times each pace; and a year 365 days; the Day sheet's three
/// settings, the hand's constants and the manifest agree, so that the
/// teacher's dial, the Clock's rate, scales the game and moves no formula.
#[test]
fn ten_minutes_a_second_in_session_and_an_hour_between_at_the_manifests_rate() {
    let cart = cartridge();
    let rate = u64::from(cart.manifest().rate);
    assert_eq!((rate * PACE.session, rate * PACE.between), (600, AN_HOUR));
    let day = values(&cart, "Day", "B1:B28");
    for (addr, want, what) in [
        ("B1", PACE.between, "the seconds a frame between sessions"),
        ("B2", DAYS_A_YEAR, "the days a year"),
        ("B28", PACE.session, "the seconds a frame in session"),
    ] {
        assert_eq!(day.get(addr), Some(&want.to_string()), "Day!{addr}, {what}");
    }
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

/// The paces written as a person's edits before the first frame, the
/// session's seconds a frame and the seconds between sessions (`POT.11`).
fn pace_edits(pace: Pace) -> [String; 2] {
    [
        format!("(cell \"Day\" \"B28\" {})", pace.session),
        format!("(cell \"Day\" \"B1\" {})", pace.between),
    ]
}

/// The cartridge, one pace at every hour written as a person's edits before
/// the first frame, both cells, and the hand's calendar at the same pace
/// (`POT.10`, `POT.11`).
fn at_pace(seconds: u64) -> (Cartridge, Calendar) {
    let mut cart = cartridge();
    let pace = Pace::every(seconds);
    for edit in pace_edits(pace) {
        cart.write(&edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    }
    (cart, Calendar::new(pace, DAYS_A_YEAR))
}

/// Every frame of a weekday and its night at the game's pace (`POT.11`),
/// from Monday's opening bell on day 0 through Tuesday's bell to its first
/// tick, 1,711 frames, held to the hand: the calendar at every frame, the
/// session's twenty seconds landing on the closing bell and the night's two
/// minutes on the opening bell; the market's dice changing on a day's first
/// frame and on no other, the exam's never; the market's three columns and
/// its session's path, every tick of a session, the stocks and the fine print
/// at every frame; and the views, the Portfolio still between two five
/// minutes. Where ten days at every frame stood (`POT.10`), the weekend and the
/// year are the longer runs', at their own paces: the calendar's choice of a
/// pace is the one thing this run alone holds.
#[test]
fn every_frame_of_a_weekday_and_its_night_is_the_calendar_the_dice_and_the_market_by_hand() {
    let mut cart = cartridge();
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let w = words(&cart);
    let mut waived = exam_of(&cart, &w);
    waived.waive();
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    let frames = 1_711;
    let mut kept = Held::default();
    for _ in 0..frames {
        let step = cart.step(0);
        cal.tick();
        assert!(step.done && step.frame == cal.frame, "{}", step.row());
        held(&cart, &cal, seed, &market, &texts, &mut kept);
        // The exam's views at rest, waived, at the run's first frame and its
        // last: their date, bell and fine print are the Portfolio's own shapes,
        // held here at every five minutes (GUARD.1).
        if cal.frame == 1 || cal.frame == frames {
            exam_views_held(&cart, &cal, &waived, &w);
        }
    }
    assert_eq!(
        (
            cal.day,
            cal.weekday,
            cal.hour,
            cal.minute,
            cal.second,
            cal.tick
        ),
        (1, 1, 9, 35, 0, 1),
        "the run ends at Tuesday's first tick"
    );
}

/// The volatility factor (`POT.11`), a person's edit of `Market!B6` as the
/// owner tunes it: written 1 before the first frame, every tick of day 0's
/// session and its close, at a frame each five minutes, is the hand's market
/// at that factor, the half hours' spread of `POT.10`; written 10 at day 1's
/// first frame, every tick of day 1's session is the hand's at 10, a tick's
/// move wider; and at each factor the session leaves the last close and
/// reaches its own to the bit, the factor moving the hours and no close.
#[test]
fn a_factor_written_moves_the_hours_and_no_close() {
    let (mut cart, mut cal) = at_pace(buffet::MINUTES_A_TICK * 60);
    let seed = seed(&cart);
    let mut market = Market::new(seed);
    let mut closes = 0;
    for (k, factor) in [1.0, 10.0].into_iter().enumerate() {
        let edit = format!("(cell \"Market\" \"B6\" {factor})");
        cart.write(&edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
        market.factor = factor;
        loop {
            cart.step(0);
            cal.tick();
            market_held(&cart, &cal, seed, &market, DIFFICULTY);
            stocks_held(&cart, &cal, seed, &market, DIFFICULTY);
            if cal.session == cal.day && cal.five() == buffet::CLOSING_BELL {
                let now = market.path(seed, DIFFICULTY, &cal).now;
                let close = market.index(DIFFICULTY, cal.session as f64);
                assert_eq!(
                    now.to_bits(),
                    close.to_bits(),
                    "day {}'s close at {factor}",
                    cal.day
                );
                closes += 1;
            }
            if cal.day == k as u64 + 1 && cal.hour == 0 && cal.minute == 0 {
                break;
            }
        }
    }
    assert_eq!(closes, 2, "a close at each factor");
}

/// The pace keyed on the regular session's bells, never on the cell that
/// says orders fill (`POT.11`, the owner's words: "Otherwise two players could
/// play the same game at two different paces if only one opts to license for
/// extended trading hours"): `Day!B19` written open at every hour, as a
/// license's hours would widen it, and every frame of a weekday and its night
/// at the game's pace keeps the time, the session's hours, the pace now and the
/// time shown the hand's, which reads the bells alone: twenty seconds a frame
/// from 9:30 to 16:00 and two minutes a frame through the night, the clock to
/// the five minutes in session and the half hour between, the market open
/// throughout. And the weather (`FRONT.3`), keyed on the bells as the pace is:
/// the last close made the session's own from its close, the market open or
/// not, held at every frame through Tuesday's close and half an hour past it,
/// Monday's session being its own session before, so that its night reads
/// one close either way.
#[test]
fn the_pace_follows_the_bells_and_never_the_open() {
    let mut cart = cartridge();
    let edit = "(cell \"Day\" \"B19\" 1)";
    cart.write(edit)
        .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    let market = Market::new(seed(&cart));
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    for _ in 0..2_881 {
        cart.step(0);
        cal.tick();
        weather_held(&cart, &cal, &market, DIFFICULTY);
        let got = values(&cart, "Day", "B4:B31");
        for (addr, want) in [
            ("B4", cal.seconds()),
            ("B19", 1),
            ("B29", u64::from(cal.in_session())),
            ("B30", cal.pace_now()),
            ("B31", cal.shown()),
        ] {
            assert_eq!(
                got.get(addr),
                Some(&want.to_string()),
                "frame {}, day {}: Day!{addr}",
                cal.frame,
                cal.day
            );
        }
    }
    assert_eq!(
        (cal.day, cal.hour, cal.minute),
        (1, 16, 30),
        "half an hour past Tuesday's close"
    );
}

/// Every hour of a year and two days at a frame an hour (`POT.10`), both
/// paces written 3,600 as a person's edits before the first frame (`POT.11`),
/// 8,808 frames, each held to the hand as every frame is above: every twelfth
/// tick of 263 sessions, each frame half an hour after its hour, the bell's at
/// 9:30 and the close's at 16:30; the dice at each day's first frame; the
/// weekends; the year's turn, and the dividend's four days.
#[test]
fn every_hour_of_a_year_and_two_days_is_the_market_by_hand_at_a_frame_an_hour() {
    let (mut cart, mut cal) = at_pace(AN_HOUR);
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut kept = Held::default();
    let mut sessions = BTreeSet::new();
    for _ in 0..(DAYS_A_YEAR + 2) * 24 {
        cart.step(0);
        cal.tick();
        held(&cart, &cal, seed, &market, &texts, &mut kept);
        sessions.insert(cal.session);
    }
    assert_eq!(sessions.len(), 263, "the sessions held");
    assert_eq!(
        (cal.year, cal.day_of_year, cal.hour, cal.minute),
        (2, 3, 8, 30),
        "the run ends before the second year's third bell"
    );
}

/// At a frame a day, both paces written 86,400 as a person's edits before
/// the first frame, every frame turns the day at the opening bell's minute,
/// and each day rolls the dice the same day rolls at the game's pace and
/// closes the same market: the market is a function of the seed and the
/// calendar, never of the frame (`BD-3`), so a floors fixture at a frame a day
/// plays the game's own market (`MISE.4`).
#[test]
fn at_a_frame_a_day_the_same_days_roll_the_same_dice() {
    let (mut cart, mut cal) = at_pace(A_DAY_LONG);
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut kept = Held::default();
    for _ in 0..800 {
        cart.step(0);
        cal.tick();
        assert!(cal.turned);
        held(&cart, &cal, seed, &market, &texts, &mut kept);
    }
}

/// The Market sheet states its settings and its waves, and they are the
/// hand's constants: pi, the drift, the difficulty, the look back, the half
/// hour's swing (`POT.10`) and the volatility factor (`POT.11`), each wave's
/// period and amplitude, and the Kiddie Pool's fixed phase, 0.
#[test]
fn the_market_sheet_states_its_settings() {
    let cart = cartridge();
    let got = values(&cart, "Market", "B1:E12");
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
        ("B6", buffet::FACTOR),
        ("E8", 0.0),
    ];
    for (addr, n) in want {
        assert_eq!(number(addr).to_bits(), n.to_bits(), "Market!{addr}");
    }
    let waves = [YEAR, WAVES[0], WAVES[1], WAVES[2], WAVES[3]];
    for (row, wave) in (8..).zip(waves) {
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
/// name, its start in cents, its beta and its dividend yield; the dividends
/// a year; and, once the first frame has computed it, the days between two,
/// the hand's first dividend day. A rung's name stands on its own row since
/// `MENU.3`'s first slice, where its label, its place on the ladder, stood.
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
    for (row, rung) in (5..).zip(RUNGS.iter()) {
        assert_eq!(
            got.get(&format!("A{row}")).map(String::as_str),
            Some(rung.name),
            "Stocks!A{row}, the rung's name (MENU.3)"
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
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
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
        if cal.tick == 0 || cal.tick == CLOSING_TICK {
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
    let mut kept = Held::default();
    for _ in 0..DAYS_A_YEAR * 24 {
        cart.step(0);
        cal.tick();
        still_or_held(
            &cart,
            &cal,
            ("Market", "B8:E26"),
            workings_key(&cal),
            &mut kept.workings,
            || waves_held(&cart, &cal, &market, 0),
        );
        still_or_held(
            &cart,
            &cal,
            ("Market", "B29:C106"),
            cal.day,
            &mut kept.table,
            || table_held(&cart, &cal, seed, &market),
        );
        tick_held(&cart, &cal, seed, &market, 0);
        stocks_held(&cart, &cal, seed, &market, 0);
    }
}

/// A hundred years at a frame a day, both paces written 86,400, every
/// frame the opening bell's minute of the next day: the session's close, the
/// close twenty days before the last close made (`FRONT.3`) and the session
/// before's, and every rung's
/// price now, over the index now, and at the last close and its dividend, held to the
/// hand on every one of the 36,500 days, a weekend's showing Friday's
/// session, across the whole range a long game reaches, the drift raised
/// past the 36,000th power and the Kondratiev two turns on. No rung's price
/// falls below the bound no seed passes, and the dearest rung pays its
/// dividend on exactly four days a year. And every report card of the century
/// (`CRITIC.1`), 399, the firm at rest: the Performance sheet and the Report
/// the hand's at each.
#[test]
fn a_hundred_years_at_a_frame_a_day_are_the_market_by_hand() {
    let (mut cart, mut cal) = at_pace(A_DAY_LONG);
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut cards = Cards::default();
    let mut issued = 0;
    let mut least = [u64::MAX; RUNGS.len()];
    let mut paid = 0;
    for day in 0..100 * DAYS_A_YEAR {
        cart.step(0);
        cal.tick();
        assert_eq!(cal.day, day);
        let got = values(&cart, "Market", "B26:D26");
        let session = cal.session as f64;
        let back = cal.last_close() as f64 - LOOK_BACK as f64;
        let days = [
            ("B26", market.index(DIFFICULTY, session)),
            ("C26", market.index(DIFFICULTY, back)),
            ("D26", market.index(DIFFICULTY, cal.before as f64)),
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
        let price = prices_of(&stocks)[buffet::BENCHMARK];
        if cards.frame(&cal, day > 0, WAIVED_CAPITAL, price, WAIVED_CAPITAL) {
            issued += 1;
            performance_held(&cart, &cal, &cards, price, WAIVED_CAPITAL);
            report_cells(&cart, &cal, &cards, (seed, &texts), buffet::WAIVED_STYLE);
        }
    }
    assert_eq!(issued, 399, "a card at every quarter's turn of the century");
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
/// seconds, `Day!B4`, the one cell of it that changes every frame once the
/// bell has rung, or that cell's twin (`POT.10`, `POT.11`), and the frames the
/// bell waited for the exam, `Day!B26`, which change every frame of it, or
/// their twin (`GUARD.1`); and `describe`'s census computes no shape every
/// frame. So an ordinary frame computes the calendar's three cells, the time,
/// the boundary and the time to the five minutes, and the rest wait for a five
/// minutes, a tick or a day. The step row cannot say so, since it counts the
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
            ("the time in seconds", ("Day", "B", "4")),
            ("the time in seconds' twin", ("Day.last", "B", "4")),
            ("the frames the bell waited", ("Day", "B", "26")),
            ("the frames the bell waited's twin", ("Day.last", "B", "26")),
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
    let census = loaded().describe();
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
    let got = loaded().describe();
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
    let (mut cart, mut cal) = at_pace(A_DAY_LONG);
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
    let [session, between] = pace_edits(Pace::every(A_DAY_LONG));
    let (sheet, at) = buffet::WAIVER;
    let waiver = format!("(cell \"{sheet}\" \"{at}\" 1)");
    for edit in [
        session.as_str(),
        between.as_str(),
        "(cell \"Disclosures\" \"D5\" 1)",
        waiver.as_str(),
    ] {
        grown
            .write(edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    }
    let mut cal = Calendar::new(Pace::every(A_DAY_LONG), DAYS_A_YEAR);
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
/// the caveats shown, then the devices, the Keys and the Camera the game
/// writes before the engine's own; and the twins after every sheet
/// (`MENU.1`'s decisions 2 and 3, `MENU.5`'s decision 2, `FRONT.1`'s 4 and 5,
/// `BACK.1`'s 2, 4 and 9, `GUARD.1`'s 9 and 10, and its hand test's Cash
/// sheet after the books it reads; `PLATE.1`'s Trading beside the Portfolio,
/// its Orders, Positions and Ledger among the model's, and its Commands and
/// Write sheets after the Camera; `FRONT.3`'s Overview after the Certificate
/// and its Headlines hidden among the content; `CRITIC.1`'s Report after Gigs,
/// its Performance after the Ledger and its Remarks hidden among the content).
/// A state the source forgot
/// would leave its sheet visible, which the engine makes a sheet first named
/// by a cell.
#[test]
fn every_sheet_is_written_bare_and_each_content_sheet_in_its_state() {
    let cart = loaded();
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
        ("Exam", "visible"),
        ("Certificate", "visible"),
        ("Overview", "visible"),
        ("Portfolio", "visible"),
        ("Trading", "visible"),
        ("Gigs", "visible"),
        ("Report", "visible"),
        ("Day", "visible"),
        ("Dice", "visible"),
        ("Market", "visible"),
        ("Stocks", "visible"),
        ("Hustle", "visible"),
        ("Proctor", "visible"),
        ("Orders", "visible"),
        ("Positions", "visible"),
        ("Cash", "visible"),
        ("Ledger", "visible"),
        ("Performance", "visible"),
        ("Disclosures", "hidden"),
        ("Questions", "hidden"),
        ("Firms", "hidden"),
        ("Headlines", "hidden"),
        ("Remarks", "hidden"),
        ("Caveats", "visible"),
        ("Keys", "visible"),
        ("Camera", "visible"),
        ("Commands", "visible"),
        ("Write", "visible"),
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

/// The game opens on the Exam (`GUARD.1`'s decisions 9 and 10, after
/// `FRONT.1`'s 2 to 5). The cartridge is written to the page's version 6 since
/// the pin to `alonzo` 0.3.0 (version 5 since `PLATE.1` and version 4 since
/// `FRONT.1`, each kept whole by the next), whose
/// host opens a grid cartridge on the sheet its Camera's third row names
/// at the load, a value or a formula's `value` row, else on the first visible
/// sheet of its own order (Alonzo's `SPEC.md` sections 3.1 and 3.8): the
/// Camera's `B3` is a formula with no value at the load, so that rule names
/// the Exam, the first visible sheet, and the hidden Screen before it is never
/// opened on. The Camera's first two rows are whole numbers, the window's
/// place, so that its host says nothing of them.
#[test]
fn the_game_opens_on_the_exam() {
    let cart = loaded();
    assert_eq!(
        cart.manifest().spec,
        6,
        "the page's version 6: a game's views on sheets of their own, commands of its own, and the keyboard's sheets"
    );
    let Ok(Viewed::Text(camera)) = cart.view("grid", "Camera", Some("A1:B3")) else {
        panic!("the Camera's record");
    };
    let rows: Vec<&str> = camera
        .lines()
        .filter(|l| l.contains("\"Camera\" \""))
        .collect();
    for want in [
        "(cell \"Camera\" \"B1\" 1)",
        "(cell \"Camera\" \"B2\" 1)",
        "(cell \"Camera\" \"A3\" \"sheet\")",
    ] {
        assert!(rows.contains(&want), "{want} in {rows:?}");
    }
    assert!(
        rows.iter()
            .any(|l| l.starts_with("(formula \"Camera\" \"B3\" "))
            && !rows
                .iter()
                .any(|l| l.starts_with("(value \"Camera\" \"B3\" ")),
        "the Camera's B3 a formula with no value at the load, naming no sheet to open on: {rows:?}"
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

/// The keyboard's sheets (the pin to `alonzo` 0.3.0, the page's version 6, its
/// `SPEC.5`). At that page a character typed on a cell of a sheet no manifest
/// names opens the cell's edit and Delete empties it, so each view is the
/// game's and read-only, the game's keys holding there and no person's key or
/// row writing it, and the Caveats, which a player reads whole, read-only. Every
/// other sheet is named by neither, a work sheet: the model is the open book
/// (`BD-3`), and the waiver (`GUARD.1`), the paces (`POT.11`) and a sandbox edit
/// (`PLATE.6`) are a person's writes into it.
#[test]
fn the_views_are_the_games_and_no_person_writes_them() {
    let cart = loaded();
    let manifest = cart.manifest();
    assert_eq!(
        manifest.game, VIEWS,
        "the game's keys hold on the views alone"
    );
    let mut read: Vec<&str> = VIEWS.to_vec();
    read.push("Caveats");
    assert_eq!(
        manifest.read, read,
        "no person writes a view or the Caveats"
    );
    assert!(
        manifest.work.is_empty(),
        "every other sheet a work sheet by being named by neither: {:?}",
        manifest.work
    );
    let model = ["Day", "Proctor", "Cash", "Market", "Orders", "Hustle"];
    for sheet in model {
        assert!(
            !manifest.read.iter().any(|named| named == sheet),
            "{sheet} stays a person's to edit: the open book (BD-3)"
        );
    }
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

/// Every stock's name inside every column that shows it (`MENU.3`'s first
/// slice): the Stocks sheet's own, the Portfolio's and Trading's, the order
/// desk's, the books' and the ledger's, each its width by the cartridge's own
/// row, since a text is drawn into the empty cells on its right and no further
/// and each of these columns holds a value beside every name; and every
/// command's words, which name a stock, inside the Commands sheet's column of
/// words. A column with no row of its own is the default width, which cuts a
/// name, and fails here.
#[test]
fn every_stock_name_fits_every_column_that_shows_it() {
    let text = lf(&read(CARTRIDGE));
    let width_of = |sheet: &str, column: &str| -> u64 {
        let head = format!("(column \"{sheet}\" \"{column}\" ");
        let row = text
            .lines()
            .find(|l| l.starts_with(&head))
            .unwrap_or_else(|| panic!("{sheet}!{column} has no width of its own"));
        row[head.len()..]
            .split(' ')
            .next()
            .and_then(|w| w.parse().ok())
            .unwrap_or_else(|| panic!("{row}: a width"))
    };
    for (sheet, column) in [
        ("Stocks", "A"),
        ("Portfolio", "A"),
        (buffet::TRADING, "B"),
        ("Orders", "A"),
        ("Positions", "C"),
        ("Ledger", "D"),
    ] {
        let width = width_of(sheet, column);
        for name in buffet::names() {
            let n = name.chars().count() as u64;
            assert!(
                n <= width,
                "{name}: {n} characters, past {sheet}!{column}'s {width}"
            );
        }
    }
    let words = width_of("Commands", "B");
    for picked in 0..=buffet::names().len() as u64 {
        for line in buffet::command_words(picked, buffet::FIRM) {
            let n = line.chars().count() as u64;
            assert!(
                n <= words,
                "{line:?}: {n} characters, past Commands!B's {words}"
            );
        }
    }
}

/// The Overview (`FRONT.3`): at its first frames, the exam waived and the
/// Overview shown as the host shows it, every cell the hand's, the weather's
/// line and the day's headline among them; and, while it shows, no key the
/// Keys sheet binds and no command of the cartridge's own holding, the view
/// having no act of its own (keyboard-first: no act, no key). Its two columns
/// as wide as its labels and its dollars; and the Market sheet's weather table,
/// the hand's words and their tenths, Rain below them all with none of its own.
#[test]
fn the_overview_states_its_lines_and_binds_no_key() {
    let mut cart = cartridge();
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut host = Host::new(buffet::OVERVIEW);
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    for _ in 0..3 {
        host.step(&mut cart, &[]);
        cal.tick();
        let stocks = values(&cart, "Stocks", "E5:I8");
        overview_cells(
            &cart,
            &cal,
            (seed, &market),
            &texts,
            &Plate::default(),
            ([WAIVED_CAPITAL, buffet::PERSONAL_CASH], prices_of(&stocks)),
            buffet::WAIVED_STYLE,
        );
        let bound: Vec<&String> = host.keys.values().filter(|c| !c.is_empty()).collect();
        assert!(
            bound.is_empty(),
            "frame {}: keys bound while the Overview shows: {bound:?}",
            cal.frame
        );
        assert!(
            host.commands.iter().all(|(_, holds, _)| !holds),
            "frame {}: a command holds while the Overview shows",
            cal.frame
        );
    }
    assert_eq!(host.now.sheet, buffet::OVERVIEW);
    let table = values(&cart, "Market", "A114:B119");
    assert_eq!(
        table.get("A114").map(String::as_str),
        Some("the weather"),
        "Market!A114"
    );
    for (k, (word, from)) in buffet::WEATHER.iter().enumerate() {
        let r = 115 + k;
        assert_eq!(
            table.get(&format!("A{r}")).map(String::as_str),
            Some(*word),
            "Market!A{r}"
        );
        assert_eq!(
            table.get(&format!("B{r}")),
            Some(&from.to_string()),
            "Market!B{r}"
        );
    }
    assert_eq!(
        table.get("A119").map(String::as_str),
        Some(buffet::WEATHER_BELOW),
        "Market!A119"
    );
    assert!(
        !table.contains_key("B119"),
        "Rain below every word's tenths, none of its own"
    );
    let text = lf(&read(CARTRIDGE));
    for (column, width) in [("A", 22), ("B", buffet::DOLLARS_COLUMN)] {
        let row = format!("(column \"Overview\" \"{column}\" {width} shown none)");
        assert!(text.lines().any(|l| l == row), "{row}");
    }
}

/// The Headlines sheet (`FRONT.3`): its header and labels as the library's
/// `strings` writes them; its rows counted in `D1`, course 0's ten, every
/// headline one line within its column's width and none empty; and at a frame
/// a day, the exam waived, the row shown each day the hand's draw and the text
/// shown its headline, through the first forty days.
#[test]
fn the_headlines_sheet_counts_its_rows_and_shows_the_days_draw() {
    let (mut cart, mut cal) = at_pace(A_DAY_LONG);
    let seed = seed(&cart);
    let file = content(HEADLINES);
    assert_eq!(file.fields, [("Headline".to_string(), 100)]);
    let headlines = headline_texts();
    assert_eq!(headlines.len(), 10, "course 0's ten headlines");
    for (k, h) in headlines.iter().enumerate() {
        let n = h.chars().count() as u64;
        assert!(
            !h.trim().is_empty() && !h.contains('\n') && n <= file.fields[0].1,
            "headline {}: {n} characters, past its column's {}",
            k + 1,
            file.fields[0].1
        );
    }
    let got = values(&cart, "Headlines", "A1:D3");
    for (addr, text) in [
        ("A1", "Headline"),
        ("C1", "rows"),
        ("C2", "the row shown"),
        ("C3", "shown"),
    ] {
        assert_eq!(
            got.get(addr).map(String::as_str),
            Some(text),
            "Headlines!{addr}"
        );
    }
    assert_eq!(
        got.get("D1"),
        Some(&headlines.len().to_string()),
        "Headlines!D1"
    );
    for _ in 0..40 {
        cart.step(0);
        cal.tick();
        headlines_held(&cart, &cal, seed);
    }
}

/// The columns a view's dollars stand in (`POT.10`'s second hand test): each
/// [`buffet::DOLLARS_COLUMN`] characters wide, the width the library's
/// `dollars` and the Portfolio's change are padded for, so that each text
/// ends where a number ends: the Portfolio's prices and changes, Gigs' fees,
/// and since `PLATE.1` the Portfolio's cost, value and gain and Trading's
/// prices and cash, and since `FRONT.3` the Overview's net worth, which the
/// views' frames hold to the hand's spelling; and since `CRITIC.1` the Report's
/// worth, and its percents, which the library's `percent` pads for the same
/// width.
#[test]
fn every_column_of_dollars_is_the_width_its_padding_is_measured_for() {
    let text = lf(&read(CARTRIDGE));
    for (sheet, column) in [
        (buffet::CARD_VIEW, "B"),
        (buffet::CARD_VIEW, "C"),
        (buffet::CARD_VIEW, "D"),
        ("Overview", "B"),
        ("Portfolio", "B"),
        ("Portfolio", "C"),
        ("Portfolio", "F"),
        ("Portfolio", "G"),
        ("Portfolio", "H"),
        ("Trading", "C"),
        ("Gigs", "B"),
        ("Certificate", "B"),
    ] {
        let row = format!(
            "(column \"{sheet}\" \"{column}\" {} shown none)",
            buffet::DOLLARS_COLUMN
        );
        assert!(text.lines().any(|l| l == row), "{sheet}!{column}: {row}");
    }
}

/// The views' dollars with their thousands grouped (`GUARD.1`'s hand test,
/// the owner's word: "Thousands separators need to be implemented"): the
/// cells that spell whole cents, the library's `dollars` and
/// `signed-dollars`, held to the hand's spelling beside every power of ten to
/// 10^15 cents and at 2^53 cents less one, the most a double holds exactly:
/// the firm's capital, written as a person's edit of the capital before the
/// exam with no answer given, on the Certificate and the Portfolio's cash on
/// hand; and a change either way, written over the Stocks sheet's own, on the
/// Portfolio.
#[test]
fn every_amount_is_spelled_with_its_thousands_grouped() {
    let mut cart = loaded();
    let mut amounts: Vec<u64> = vec![(1 << 53) - 1, 123_456_789_012_345, 550_000];
    for p in 0..=15 {
        let ten = 10u64.pow(p);
        amounts.extend([ten - 1, ten, ten + 1]);
    }
    for c in amounts {
        let loss = -(c as i64);
        let edits = format!(
            "(cell \"Proctor\" \"B1\" {c})\n(cell \"Stocks\" \"I5\" {c})\n(cell \"Stocks\" \"I6\" {loss})"
        );
        cart.write(&edits)
            .unwrap_or_else(|r| panic!("{edits}: {} {}", r.refusal.id, r.refusal.text));
        assert!(cart.step(0).done);
        let cert = values(&cart, "Certificate", "B8");
        let view = values(&cart, "Portfolio", "B5:C12");
        for (got, want, at) in [
            (cert.get("B8"), buffet::two_decimals(c), "Certificate!B8"),
            (view.get("B11"), buffet::two_decimals(c), "Portfolio!B11"),
            (
                view.get("C5"),
                buffet::signed_two_decimals(c as i64),
                "Portfolio!C5",
            ),
            (
                view.get("C6"),
                buffet::signed_two_decimals(loss),
                "Portfolio!C6",
            ),
        ] {
            assert_eq!(got, Some(&want), "{c} cents: {at}");
        }
    }
}

/// What a scripted session sets at a frame (`BACK.1`'s decision 12), each
/// held until another act sets it again: the sheet shown, the pointer's cell
/// on it, the left button, and a key held or let go, by its code (the
/// hustle's P, and since `GUARD.1` the exam's A to D and O); and since
/// `PLATE.1` a command of the cartridge's own run by its name, as the page's
/// palette runs one, which a key of a command that holds runs too.
#[derive(Clone, Copy)]
enum Act {
    Show(&'static str),
    Point(u64, u64),
    Left(bool),
    Key(&'static str, bool),
    Run(&'static str),
}

/// The hustle's key, P.
const P: &str = buffet::KEY_CODE;

/// An act of a script on what the host holds: the sheet shown, the pointer's
/// cell, the left button, or a key held or let go.
fn act_on(now: &mut Inputs, act: Act) {
    match act {
        Act::Show(sheet) => now.sheet = sheet.to_string(),
        Act::Point(column, row) => (now.column, now.row) = (column, row),
        Act::Left(down) => now.left = down,
        Act::Key(code, true) => {
            now.held.insert(code.to_string());
        }
        Act::Key(code, false) => {
            now.held.remove(code);
        }
        Act::Run(_) => {}
    }
}

/// The key that went down this frame, as the host's last key names it: one
/// held now that was not at the last frame, the last of them by its code.
fn went_down<'a>(now: &'a Inputs, last: &Inputs) -> Option<&'a str> {
    now.held
        .iter()
        .rfind(|code| !last.held.contains(*code))
        .map(String::as_str)
}

/// The Input sheet's rows as the host writes them at version 5 (Alonzo's
/// `host.js`, `_inputRows`): ten buttons for four players, each 1 while a key
/// held is the one its cell of the Keys sheet names; the pointer's column and
/// row on the sheet shown, that sheet's own cell, 0 off its cells; its three
/// buttons; the wheel; the key that went down this frame and its code, a
/// letter's key its lower case; the sheet shown; and the command that runs at
/// this frame, or empty (`PLATE.1`).
fn input_rows(
    keys: &BTreeMap<String, String>,
    now: &Inputs,
    went_down: Option<&str>,
    command: &str,
) -> Vec<String> {
    let mut rows = Vec::new();
    for button in 1..=10 {
        for player in ["B", "C", "D", "E"] {
            let at = format!("{player}{button}");
            let held = keys.get(&at).is_some_and(|code| now.held.contains(code));
            rows.push(format!("(cell \"Input\" \"{at}\" {})", u8::from(held)));
        }
    }
    let (key, code) = match went_down {
        Some(code) => (
            code.strip_prefix("Key").unwrap_or(code).to_lowercase(),
            code,
        ),
        None => (String::new(), ""),
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
        format!("(cell \"Input\" \"B20\" \"{command}\")"),
    ]);
    rows
}

/// The cash on hand at this frame (`GUARD.1`'s hand test): the Cash sheet's
/// personal cash at the start, the firm's cash and the personal, in cents,
/// the hand's; each pocket's holdings and net worth (`PLATE.1`), which a
/// session that buys nothing holds at none and at its cash; and the
/// Portfolio's two pockets' cash and net worth in dollars, the same cents.
fn cash_held(cart: &Cartridge, frame: u64, firm: u64, personal: u64) {
    assert_eq!(
        values(cart, "Cash", "B1").get("B1"),
        Some(&buffet::PERSONAL_CASH.to_string()),
        "frame {frame}: Cash!B1"
    );
    books_cash_held(cart, frame, [firm, personal], [0, 0]);
}

/// Each pocket's cash, holdings and net worth at this frame, in cents on the
/// Cash sheet and in dollars on the Portfolio, from the hand's `cash` and
/// `holdings`, the firm's first; the personal start, `B1`, a setting a test
/// may write, is `cash_held`'s.
fn books_cash_held(cart: &Cartridge, frame: u64, cash: [u64; 2], holdings: [u64; 2]) {
    let got = values(cart, "Cash", "B3:B10");
    let view = values(cart, "Portfolio", "B11:B16");
    for (addr, want) in [
        ("B3", cash[0]),
        ("B4", cash[1]),
        ("B6", holdings[0]),
        ("B7", holdings[1]),
        ("B9", cash[0] + holdings[0]),
        ("B10", cash[1] + holdings[1]),
    ] {
        assert_eq!(
            got.get(addr),
            Some(&want.to_string()),
            "frame {frame}: Cash!{addr}"
        );
    }
    for (addr, cents) in [
        ("B11", cash[0]),
        ("B12", cash[1]),
        ("B15", cash[0] + holdings[0]),
        ("B16", cash[1] + holdings[1]),
    ] {
        assert_eq!(
            view.get(addr),
            Some(&buffet::two_decimals(cents)),
            "frame {frame}: Portfolio!{addr}"
        );
    }
}

/// The hustle at this frame (`BACK.1`; a week and a day's work since
/// `POT.10`): the Hustle sheet's workings, each a cell the hand states, the
/// press's two cells from the frame's inputs and the last frame's, the claim,
/// the claims, the day the next prospectus arrives, whether the hustle is shut
/// today, the time the last claim's work is done, whether a prospectus is
/// being proofread, those done, their fees and when the fee is paid; Gigs'
/// counts, its fees in dollars and its sentence; and since `GUARD.1`'s hand
/// test the cash on hand, the personal cash $100 and the fees once paid.
fn hustle_held(
    cart: &Cartridge,
    cal: &Calendar,
    h: &Hustle,
    now: &Inputs,
    last: &Inputs,
    claim: bool,
) {
    let flag = |b: bool| u8::from(b).to_string();
    let at = cal.five_minutes();
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
    cash_held(
        cart,
        cal.frame,
        buffet::CAPITAL_BEFORE + buffet::CAPITAL_A_RIGHT * buffet::WAIVED_SCORE,
        buffet::PERSONAL_CASH + h.earned(at),
    );
}

/// The side hustle at work, every frame of a scripted session held to the
/// hand (`BACK.1`'s decisions 5 to 9 and 12, the key bound on its view alone
/// since the owner's hand test; a prospectus a week and a day's work since
/// `POT.10`'s): four weeks and a day at a frame each quarter hour, both paces
/// written 900 seconds as a person's edits before the first frame, so that a
/// claim seen in session falls at a quarter past and its due is spelled to the
/// minute, and the claims seen between sessions at the half hour the clock
/// shows there (`POT.11`, where each half hour stood), each
/// act at its day and its frame within it, a day turning at midnight; the Input rows
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
    // each quarter hour.
    let script: &[(u64, u64, &[Act])] = &[
        (0, 0, &[Show("Portfolio")]),
        (0, 2, &[Show("Gigs")]),
        (0, 3, &[Point(1, 5)]),
        (0, 4, &[Left(true)]), // a click: the first claim, day 0's at 10:15 shown, its fee paid at day 1's 10:15
        (0, 5, &[Left(false)]),
        (0, 7, &[Left(true)]), // a second click on day 0: refused
        (0, 8, &[Left(false)]),
        (0, 9, &[Key(P, true)]), // P on Gigs, day 0: refused
        (0, 10, &[Key(P, false)]),
        (1, 0, &[Left(true)]), // day 1's first frame, day 0 shown, claimed: refused
        (1, 1, &[Left(false)]),
        (1, 2, &[Point(2, 5)]), // the pointer on B5, beside the button
        (1, 3, &[Left(true)]),  // no press
        (1, 4, &[Left(false)]),
        (1, 5, &[Show("Portfolio"), Point(1, 5)]), // the button's address, on another sheet
        (1, 6, &[Left(true)]),                     // no press
        (1, 7, &[Left(false)]),
        (1, 9, &[Key(P, true)]), // P on the Portfolio, unbound there: no press
        (1, 10, &[Key(P, false)]),
        (1, 12, &[Show("Gigs")]),
        (7, 0, &[Left(true)]), // day 7's first frame, day 6 shown, the week not out: refused
        (7, 1, &[Left(false)]),
        (7, 2, &[Left(true)]), // the second claim, day 7's
        (7, 3, &[Left(false)]),
        (7, 4, &[Show("Portfolio")]),
        (7, 5, &[Key(P, true)]), // P held on the Portfolio, unbound,
        (7, 6, &[Show("Gigs")]), // and Gigs shown under it: bound the frame after, a press, refused
        (7, 8, &[Key(P, false)]),
        (13, 40, &[Left(true)]), // day 13, the week not out: refused, and held across day 14's turn
        (13, 42, &[Key(P, true)]), // the same, by the key
        (14, 3, &[Left(false)]),
        (14, 4, &[Key(P, false)]),
        (15, 0, &[Key(P, true)]), // day 15's first frame, day 14 shown, open: the third claim, day 14's
        (15, 1, &[Key(P, false)]),
        (21, 4, &[Left(true), Key(P, true)]), // a click and P in one frame: the fourth claim, once
        (21, 5, &[Left(false), Key(P, false)]),
        (28, 4, &[Left(true)]), // day 28, the run's four spent: refused
        (28, 5, &[Left(false)]),
        (28, 9, &[Key(P, true)]), // refused
        (28, 10, &[Key(P, false)]),
        (28, 14, &[Point(0, 0)]),             // the pointer over a header
        (28, 15, &[Point(1, 5), Left(true)]), // dragged onto the button, held: a press, refused
        (28, 17, &[Left(false)]),
    ];
    // The minutes a frame, a quarter hour; and a day's first frame: the first
    // frame for day 0, its midnight after it.
    let minutes = QUARTER_HOUR / 60;
    let start = |day: u64| match day {
        0 => 1,
        d => (d * A_DAY - FIRST_MINUTE) / minutes + 1,
    };
    let script: Vec<(u64, &[Act])> = script
        .iter()
        .map(|(day, at, set)| (start(*day) + at, *set))
        .collect();
    let (mut cart, mut cal) = at_pace(QUARTER_HOUR);
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
                act_on(&mut now, *act);
            }
        }
        // P bound as the hand says, by the sheet the last frame showed, which
        // the last step's Keys sheet read; and the cartridge's keymap the same.
        now.bound = if Hustle::binds(&last.sheet) {
            BTreeSet::from([P.to_string()])
        } else {
            BTreeSet::new()
        };
        let bound: Vec<(&str, &str)> = keys
            .iter()
            .filter(|(_, code)| !code.is_empty())
            .map(|(addr, code)| (addr.as_str(), code.as_str()))
            .collect();
        let want: &[(&str, &str)] = if now.bound.contains(P) {
            &[("B5", buffet::KEY_CODE)]
        } else {
            &[]
        };
        assert_eq!(
            bound, want,
            "frame {frame}: the keys bound after {:?} showed",
            last.sheet
        );
        let rows = input_rows(&keys, &now, went_down(&now, &last), "");
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
        // The day and the time the person saw, the last frame's.
        let (shown, seen) = (cal.day, cal.shown());
        let step = cart.step(0);
        cal.tick();
        assert!(step.done && step.frame == cal.frame, "{}", step.row());
        keys = values(&cart, "Keys", "B1:E10");
        let claim = hustle.frame(&now, &last, shown, seen);
        hustle_held(&cart, &cal, &hustle, &now, &last, claim);
    }
    assert!(acts.next().is_none(), "every act of the script played");
    let days: Vec<u64> = hustle.claims.iter().map(|&(d, _)| d).collect();
    assert_eq!(
        days,
        [0, 7, 14, 21],
        "the claims, by the days they counted against"
    );
    let off_the_half_hour = hustle.claims.iter().filter(|&&(_, t)| t % 30 != 0).count();
    assert_eq!(
        off_the_half_hour, 1,
        "the first claim seen in session at 10:15, its due spelled to the minute; the three seen between sessions at the half hour shown"
    );
    assert_eq!(
        hustle.earned(cal.five_minutes()),
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
    let style = format!("(style \"{sheet}\" \"{}\" 1)", address(row, column));
    assert!(
        text.lines().any(|l| l == style) && text.lines().any(|l| l == buffet::BUTTON_FORMAT),
        "the button drawn as the buttons are, in lavender (GUARD.1's hand test): {style}"
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

/// The exam's bank shows its first question (`MENU.2`, driven by `GUARD.1`'s
/// draw): the row shown, `K2`, the Proctor's, the draw's first row at the
/// contract's seed, the hand's; and from the first frame its eight pickers,
/// `K3` to `K10`, that row's fields, each beside its label.
#[test]
fn the_questions_sheet_shows_its_first_question() {
    let mut cart = loaded();
    cart.step(0);
    let n = content(QUESTIONS).rows.len() as u64;
    let row = buffet::draw(seed(&cart), n)[0] as usize;
    let got = values(&cart, "Questions", &format!("A1:K{}", n + 1));
    assert_eq!(
        got.get("K2"),
        Some(&row.to_string()),
        "Questions!K2, the row shown"
    );
    let first: Vec<&str> = (1..=8)
        .map(|k| got[&address(row + 1, k)].as_str())
        .collect();
    let shown: Vec<&str> = (3..=10).map(|r| got[&format!("K{r}")].as_str()).collect();
    assert_eq!(shown, first, "Questions!K3:K10, the draw's first question");
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

/// A picker a field picks every row in its turn (`MENU.2`, `MENU.5`,
/// `GUARD.1`): on the exam's bank, the firms' styles and the caveats, the row
/// shown written from 1 to the sheet's last row as a person's edit, over the
/// formula the bank's and the styles' row shown hold, and at each step every
/// picker that row's field, read off the sheet's own columns. An empty text,
/// as the first caveat's `Where` is, reads empty on both sides.
#[test]
fn every_field_is_picked_at_every_row() {
    for (sheet, path) in [
        ("Questions", QUESTIONS),
        ("Firms", FIRMS),
        ("Caveats", CAVEATS),
    ] {
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

/// The words the exam shows (`GUARD.1`), read from the content files by the
/// language's own reader: the bank's rows, each its eight fields; the firms'
/// four styles; the plain statement, the first caveat's first three fields;
/// and the fine print's rows.
struct Words {
    bank: Vec<Vec<String>>,
    styles: Vec<String>,
    plain: Vec<String>,
    texts: Vec<String>,
}

fn words(cart: &Cartridge) -> Words {
    let mut caveats = content(CAVEATS).rows;
    Words {
        bank: content(QUESTIONS)
            .rows
            .into_iter()
            .map(|r| r.words)
            .collect(),
        styles: content(FIRMS)
            .rows
            .into_iter()
            .map(|mut r| r.words.remove(0))
            .collect(),
        plain: caveats.swap_remove(0).words[..3].to_vec(),
        texts: disclosure_texts(cart),
    }
}

/// The Certificate's title and signature, the scoping's drafts.
const CERTIFICATE_TITLE: &str = "The Series 777: a certificate of completion";
const SIGNATURE: &str =
    "Signed for the regulator by the proctor, whose signature is, on inspection, a doodle of a bull.";

/// The hand's exam over the cartridge's bank, at its seed.
fn exam_of(cart: &Cartridge, w: &Words) -> Exam {
    Exam::new(seed(cart), w.bank.iter().map(|r| r[6].clone()).collect())
}

/// The host of a session (`BACK.1`'s and `GUARD.1`'s scripts), as Alonzo's
/// `host.js` holds it: the inputs it holds, the keymap it read after the last
/// step, the Input rows it last wrote, and the Camera's sheet as it last read
/// it, a change of which it shows at the frame's end, so that the next frame's
/// rows name it. Since `PLATE.1`, at the page's version 5: the Commands sheet as
/// it last read it, each command's name, whether it holds and its key; the
/// commands waiting for their frames, one a frame, and the one that runs at
/// this frame; and the Write sheet's rows it read after the last step, made
/// before the next frame's inputs, each its own write (`SPEC.md` section 5).
struct Host {
    now: Inputs,
    keys: BTreeMap<String, String>,
    written: Option<Vec<String>>,
    camera: String,
    commands: Vec<(String, bool, String)>,
    waiting: VecDeque<String>,
    command: String,
    pending: Vec<String>,
}

impl Host {
    /// A host that opened the game on `sheet`, with no keymap read, the
    /// Keys sheet's formulas holding no value at the load, and the Commands
    /// sheet not yet read.
    fn new(sheet: &str) -> Host {
        Host {
            now: Inputs {
                sheet: sheet.to_string(),
                ..Inputs::default()
            },
            keys: BTreeMap::new(),
            written: None,
            camera: String::new(),
            commands: Vec::new(),
            waiting: VecDeque::new(),
            command: String::new(),
            pending: Vec::new(),
        }
    }

    /// The command a key runs (`host.js`, `_commandKey`): the first that holds
    /// with that key, as the Commands sheet last said.
    fn command_key(&self, code: &str) -> Option<String> {
        self.commands
            .iter()
            .find(|(_, holds, key)| *holds && key == code)
            .map(|(name, _, _)| name.clone())
    }

    /// One frame: the acts applied to what the host holds, a key of a command
    /// that holds running the command and holding no button, and a command run
    /// by its name waiting for its frame while it holds and refused while it
    /// does not; the keys bound as its keymap binds them; the derived writes
    /// read after the last step made, each its own write; the Input rows that
    /// changed written, the command that runs at this frame the twentieth; and
    /// the step; then the keymap, the commands, the Write sheet and the Camera
    /// read. The last frame's inputs and this frame's.
    fn step(&mut self, cart: &mut Cartridge, acts: &[Act]) -> (Inputs, Inputs) {
        let last = self.now.clone();
        for act in acts {
            match *act {
                Act::Key(code, true) => match self.command_key(code) {
                    Some(name) => self.waiting.push_back(name),
                    None => act_on(&mut self.now, *act),
                },
                Act::Run(name) => {
                    if self
                        .commands
                        .iter()
                        .any(|(n, holds, _)| n == name && *holds)
                    {
                        self.waiting.push_back(name.to_string());
                    }
                }
                _ => act_on(&mut self.now, *act),
            }
        }
        self.command = self.waiting.pop_front().unwrap_or_default();
        self.now.bound = self
            .keys
            .values()
            .filter(|code| !code.is_empty())
            .cloned()
            .collect();
        for row in std::mem::take(&mut self.pending) {
            cart.write(&row)
                .unwrap_or_else(|r| panic!("{row}: {} {}", r.refusal.id, r.refusal.text));
        }
        let rows = input_rows(
            &self.keys,
            &self.now,
            went_down(&self.now, &last),
            &self.command,
        );
        let changed: Vec<&str> = match &self.written {
            None => rows.iter().map(String::as_str).collect(),
            Some(before) => rows
                .iter()
                .zip(before)
                .filter(|(row, was)| row != was)
                .map(|(row, _)| row.as_str())
                .collect(),
        };
        if !changed.is_empty() {
            cart.write(&changed.join("\n"))
                .unwrap_or_else(|r| panic!("the Input rows: {} {}", r.refusal.id, r.refusal.text));
        }
        self.written = Some(rows);
        let step = cart.step(0);
        assert!(step.done, "{}", step.row());
        self.keys = values(cart, "Keys", "B1:E10");
        let sheet = values(cart, "Commands", "A2:D101");
        self.commands = (2..=101)
            .filter_map(|row| {
                let name = sheet.get(&format!("A{row}"))?;
                let holds = sheet.get(&format!("C{row}")).is_some_and(|h| h == "true");
                let key = sheet.get(&format!("D{row}")).cloned().unwrap_or_default();
                Some((name.clone(), holds, key))
            })
            .collect();
        self.pending = derived(cart);
        let now = self.now.clone();
        let shows = values(cart, "Camera", "B3")
            .remove("B3")
            .unwrap_or_default();
        if shows != self.camera && !shows.is_empty() {
            self.now.sheet = shows.clone();
        }
        self.camera = shows;
        (last, now)
    }
}

/// The exam's two views at this frame (`GUARD.1`), every cell the hand's: the
/// Exam's date and bell, where the exam stands, the question shown, its four
/// options each a button with its letter, the line of keys and the fine print,
/// a closed exam's question, options and keys empty; the Certificate's date
/// and bell, its title, its headline, the correct answers, the capital in
/// dollars, the firm's name, pending until the eleventh answer gives the style
/// that names it (`BACK.2`), its button and its key, the plain statement whole
/// under its heading, the signature and the fine print; and the Overview's firm
/// line, pending until then too.
fn exam_views_held(cart: &Cartridge, cal: &Calendar, exam: &Exam, w: &Words) {
    let f = cal.frame;
    let open = exam.open();
    let scored = exam.answers.len() < buffet::SCORED;
    let row = exam.row_shown() as usize - 1;
    let shown = disclosure(cal.day, w.texts.len() as u64, DAYS_A_DISCLOSURE);
    let fine = w.texts[shown as usize - 1].clone();
    let blank_or = |text: String| if open { text } else { String::new() };
    let mut want: Vec<(String, String)> = vec![
        ("A1".into(), buffet::as_of(cal)),
        ("A2".into(), buffet::bell(cal).to_string()),
        ("A3".into(), exam.status()),
        (
            "A5".into(),
            blank_or(if scored {
                w.bank[row][1].clone()
            } else {
                buffet::STYLE_PROMPT.to_string()
            }),
        ),
        ("A12".into(), blank_or(buffet::ANSWER_KEYS_TEXT.to_string())),
        ("A14".into(), fine.clone()),
    ];
    for (k, (letter, r, _)) in buffet::OPTIONS.iter().enumerate() {
        let text = if scored {
            &w.bank[row][2 + k]
        } else {
            &w.styles[k]
        };
        want.push((format!("A{r}"), blank_or(format!("[ {letter} ]  {text}"))));
    }
    let got = values(cart, "Exam", "A1:A14");
    for (addr, text) in &want {
        assert_eq!(got.get(addr), Some(text), "frame {f}: Exam!{addr}");
    }
    let style = exam.style() as usize;
    let want = [
        ("A1", buffet::as_of(cal)),
        ("A2", buffet::bell(cal).to_string()),
        ("A3", CERTIFICATE_TITLE.to_string()),
        ("A5", exam.headline().to_string()),
        ("A7", "Correct answers".to_string()),
        ("B7", format!("{} of {}", exam.score(), buffet::SCORED)),
        ("A8", "Firm capital ($)".to_string()),
        ("B8", buffet::two_decimals(exam.capital())),
        ("A9", buffet::NAME_LABEL.to_string()),
        (
            "B9",
            if style == 0 {
                buffet::NAME_PENDING.to_string()
            } else {
                firm_rows()[style - 1].1.clone()
            },
        ),
        ("A11", buffet::OPEN_TEXT.to_string()),
        ("B11", buffet::OPEN_KEY_TEXT.to_string()),
        ("A13", buffet::CAVEAT_HEADING.to_string()),
        ("A14", w.plain[0].clone()),
        ("A15", w.plain[1].clone()),
        ("A16", w.plain[2].clone()),
        ("A18", SIGNATURE.to_string()),
        ("A20", fine),
    ];
    let got = values(cart, "Certificate", "A1:B20");
    for (addr, text) in want {
        assert_eq!(got.get(addr), Some(&text), "frame {f}: Certificate!{addr}");
    }
    assert_eq!(
        values(cart, "Overview", "A3").get("A3"),
        Some(&firm_line_of(style as u64)),
        "frame {f}: Overview!A3, the firm's line"
    );
}

/// The proctor's books at this frame (`GUARD.1`), every working the hand's:
/// the draw, the bank's size, the start, the stride and the ten rows; each
/// button's click and key, read off this frame's inputs and the last's; the
/// option pressed, whether it was taken, the answers, whether a question is
/// open, the row shown, the right option, whether this frame's answer was
/// right, the right answers so far, the score, the capital, the style and
/// begun; and the bank's row shown and the Camera's sheet beside them.
fn proctor_held(
    cart: &Cartridge,
    frame: u64,
    exam: &Exam,
    io: (&Inputs, &Inputs),
    answer: (bool, bool),
) {
    let (now, last) = io;
    let (answered, right) = answer;
    let flag = |b: bool| u8::from(b).to_string();
    let bank = exam.letters.len() as u64;
    let seed = seed(cart);
    let mut want: Vec<(String, String)> = vec![
        ("B8".into(), bank.to_string()),
        ("B9".into(), buffet::start(seed, bank).to_string()),
        ("B10".into(), buffet::stride(seed).to_string()),
    ];
    for (k, row) in exam.rows.iter().enumerate() {
        want.push((format!("B{}", 12 + k), row.to_string()));
    }
    for (k, (_, row, key)) in buffet::OPTIONS.iter().enumerate() {
        let r = 24 + k;
        let place = (buffet::EXAM, buffet::OPTION_COLUMN, *row);
        want.push((format!("F{r}"), flag(buffet::clicked(now, last, place))));
        want.push((format!("G{r}"), flag(Inputs::pressed(now, last, key))));
    }
    want.push((
        "F28".into(),
        flag(buffet::clicked(now, last, buffet::OPEN_BUTTON)),
    ));
    want.push((
        "G28".into(),
        flag(Inputs::pressed(now, last, buffet::OPEN_KEY)),
    ));
    let option = Exam::option_pressed(now, last).unwrap_or(0);
    for (addr, text) in [
        ("B30", option.to_string()),
        ("B31", flag(answered)),
        ("B32", exam.answers.len().to_string()),
        ("B33", flag(exam.open())),
        ("B34", exam.row_shown().to_string()),
        ("B35", exam.right().to_string()),
        ("B36", flag(right)),
        ("B37", exam.correct().to_string()),
        ("B38", exam.score().to_string()),
        ("B39", exam.capital().to_string()),
        ("B40", exam.style().to_string()),
        ("B41", flag(exam.begun)),
    ] {
        want.push((addr.into(), text));
    }
    let got = values(cart, "Proctor", "B8:G41");
    for (addr, text) in &want {
        assert_eq!(got.get(addr), Some(text), "frame {frame}: Proctor!{addr}");
    }
    assert_eq!(
        values(cart, "Questions", "K2").get("K2"),
        Some(&exam.row_shown().to_string()),
        "frame {frame}: Questions!K2, the row shown"
    );
    assert_eq!(
        values(cart, "Camera", "B3").get("B3").map(String::as_str),
        Some(exam.shows()),
        "frame {frame}: Camera!B3, the sheet the game shows"
    );
}

/// The keys the Keys sheet binds after a step, against the hand's for the
/// sheet that frame showed and the exam's state: A to D on the Exam while a
/// question is open, O on the Certificate while it waits for its button, P on
/// Gigs, and nothing else.
fn keys_held(frame: u64, keys: &BTreeMap<String, String>, sheet: &str, exam: &Exam) {
    let bound: Vec<(&str, &str)> = keys
        .iter()
        .filter(|(_, code)| !code.is_empty())
        .map(|(addr, code)| (addr.as_str(), code.as_str()))
        .collect();
    let mut want: Vec<(String, &str)> = Vec::new();
    if Hustle::binds(sheet) {
        want.push(("B5".into(), P));
    }
    for code in exam.binds(sheet) {
        let addr = if code == buffet::OPEN_KEY {
            "B10".to_string()
        } else {
            let k = buffet::OPTIONS
                .iter()
                .position(|(_, _, c)| *c == code)
                .expect("an option's key");
            format!("B{}", 6 + k)
        };
        want.push((addr, code));
    }
    want.sort();
    let want: Vec<(&str, &str)> = want.iter().map(|(a, c)| (a.as_str(), *c)).collect();
    assert_eq!(
        bound, want,
        "frame {frame}: the keys bound while {sheet} shows"
    );
}

/// A frame of the exam by hand beside the cartridge's: the hand's exam judged
/// by this frame's inputs and the last's, the hand's calendar on a frame, the
/// bell waiting while the game has not begun; then every cell held, the cash
/// on hand among them, the firm's the exam's capital as the answers come.
fn exam_frame_held(
    cart: &Cartridge,
    host: &Host,
    io: (&Inputs, &Inputs),
    exam: &mut Exam,
    cal: &mut Calendar,
    w: &Words,
) {
    let (now, last) = io;
    let before = exam.correct();
    let answered = exam.frame(Exam::option_pressed(now, last), Exam::opened(now, last));
    let right = exam.correct() > before;
    cal.tick_waiting(!exam.begun);
    day_held(cart, cal);
    proctor_held(cart, cal.frame, exam, (now, last), (answered, right));
    exam_views_held(cart, cal, exam, w);
    keys_held(cal.frame, &host.keys, &now.sheet, exam);
    cash_held(cart, cal.frame, exam.capital(), buffet::PERSONAL_CASH);
}

/// The draw at every seed of a thousand (`GUARD.1`'s decision 4): the seed
/// written into the Clock as a person's edit, from 1 to 1,000 and the
/// contract's, and a step each; the bank's size, the start and the stride the
/// hand's, and the ten rows the hand's walk, each a row of the bank and none
/// twice.
#[test]
fn the_draw_never_repeats_a_question_at_any_seed_of_a_thousand() {
    let mut cart = loaded();
    let n = content(QUESTIONS).rows.len() as u64;
    let contract = seed(&cart);
    for s in (1..=1_000).chain([contract]) {
        let edit = format!("(cell \"Clock\" \"B3\" {s})");
        cart.write(&edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
        cart.step(0);
        let got = values(&cart, "Proctor", "B8:B21");
        let cell = |addr: &str| -> u64 {
            got.get(addr)
                .unwrap_or_else(|| panic!("seed {s}: Proctor!{addr} holds no value"))
                .parse()
                .unwrap_or_else(|e| panic!("seed {s}: Proctor!{addr}: {e}"))
        };
        assert_eq!(
            (cell("B8"), cell("B9"), cell("B10")),
            (n, buffet::start(s, n), buffet::stride(s)),
            "seed {s}: the bank, the start and the stride"
        );
        let rows: Vec<u64> = (12..=21).map(|r| cell(&format!("B{r}"))).collect();
        assert_eq!(rows, buffet::draw(s, n), "seed {s}: the ten rows");
        let distinct: BTreeSet<u64> = rows.iter().copied().collect();
        assert!(
            distinct.len() == buffet::SCORED && rows.iter().all(|r| (1..=n).contains(r)),
            "seed {s}: {rows:?}"
        );
    }
}

/// The Series 777 played (`GUARD.1`'s decisions 1 to 10), every frame of a
/// scripted session held to the hand: the Input rows written as the host
/// writes them, its keymap read after every step and its Camera followed; at
/// every frame the proctor's books, both views, the keys each view binds, the
/// Day sheet's wait and the market at its bell the hand's. Its presses: a
/// click on an option in the first frame, whose last showed no question,
/// refused; a click answering, and a key; a click on the question, beside
/// the options; a click at an option's address and a key while the Portfolio
/// shows, nothing, the key unbound there; a key held on the Portfolio as the
/// Exam is shown under it, a press the frame it is bound; a click and a key
/// on two options in one frame, the first by letter answering once; a click
/// held across two frames, answering once; the tenth by a click; the
/// certificate's button by a click and its key before the eleventh, refused,
/// the key unbound; the eleventh by its key, the style; the Camera showing the
/// Certificate, an option's key unbound there; the certificate's key, the
/// game begun and the Overview shown; then the market's first ten minutes,
/// the bell at the game's first frame and twenty seconds a frame after it, its
/// first move at 9:35 (`POT.11`).
#[test]
fn a_scripted_exam_holds_every_frame_to_the_hand() {
    use Act::{Key, Left, Point, Show};
    let mut cart = loaded();
    let w = words(&cart);
    let mut exam = exam_of(&cart, &w);
    let seed = seed(&cart);
    let market = Market::new(seed);
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    let mut host = Host::new(buffet::EXAM);
    // A key or a click on the right option of the question shown, or a wrong
    // one, chosen at its frame by the hand; a line of the script is its frame,
    // its acts, and a tap if any, by its key or by a click.
    #[derive(Clone, Copy)]
    enum Tap {
        Right,
        Wrong,
        Option(u64),
    }
    type Line = (u64, &'static [Act], Option<(Tap, bool)>);
    let script: &[Line] = &[
        (1, &[], Some((Tap::Right, false))), // a click in the first frame: refused
        (3, &[], Some((Tap::Right, false))), // question 1, right, by a click
        (5, &[], Some((Tap::Wrong, true))),  // question 2, wrong, by a key
        (7, &[Point(1, 5), Left(true)], None), // a click on the question: nothing
        (8, &[Left(false)], None),
        (9, &[Show("Portfolio")], None),
        (10, &[Point(1, 7), Left(true)], None), // an option's address on the Portfolio: nothing
        (11, &[Left(false)], None),
        (12, &[Key("KeyC", true)], None), // a key on the Portfolio, unbound: nothing
        (13, &[Key("KeyC", false)], None),
        (14, &[Key("KeyB", true)], None), // held on the Portfolio,
        (15, &[Show("Exam")], None), // the Exam shown under it: question 3, B, the frame it is bound
        (17, &[Key("KeyB", false)], None),
        (19, &[Point(1, 10), Left(true), Key("KeyB", true)], None), // D clicked and B keyed: question 4, B
        (20, &[Left(false), Key("KeyB", false)], None),
        (21, &[], Some((Tap::Right, true))),    // question 5
        (23, &[Point(1, 7), Left(true)], None), // question 6, A, by a click held across two frames: one answer
        (25, &[Left(false)], None),
        (26, &[], Some((Tap::Wrong, false))), // question 7
        (28, &[], Some((Tap::Right, true))),  // question 8
        (30, &[], Some((Tap::Wrong, true))),  // question 9
        (32, &[], Some((Tap::Right, false))), // question 10, the last scored, by a click
        (34, &[Show("Certificate")], None),
        (35, &[Point(1, 11), Left(true)], None), // the certificate's button before the eleventh: refused
        (36, &[Left(false)], None),
        (37, &[Key("KeyO", true)], None), // its key, unbound before the eleventh: nothing
        (38, &[Key("KeyO", false)], None),
        (39, &[Show("Exam")], None),
        (41, &[], Some((Tap::Option(3), true))), // the eleventh, the style, by its key; the Camera shows the Certificate
        (43, &[Key("KeyA", true)], None), // an option's key on the Certificate, unbound: nothing
        (44, &[Key("KeyA", false)], None),
        (46, &[Key("KeyO", true)], None), // the certificate's key: the game begins, the Overview shown
        (47, &[Key("KeyO", false)], None),
    ];
    let last_frame = 46 + 30;
    let mut acts = script.iter().peekable();
    let mut release: Vec<Act> = Vec::new();
    let mut begun_at = None;
    for frame in 1..=last_frame {
        let mut set: Vec<Act> = std::mem::take(&mut release);
        while let Some((_, more, tap)) = acts.next_if(|(at, _, _)| *at == frame) {
            set.extend_from_slice(more);
            if let Some((tap, by_key)) = *tap {
                let right = exam.right();
                let option = match tap {
                    Tap::Right => right,
                    Tap::Wrong => right % 4 + 1,
                    Tap::Option(o) => o,
                } as usize;
                let (_, row, code) = buffet::OPTIONS[option - 1];
                if by_key {
                    set.push(Key(code, true));
                    release.push(Key(code, false));
                } else {
                    set.extend([Point(buffet::OPTION_COLUMN, row), Left(true)]);
                    release.push(Left(false));
                }
            }
        }
        let (last, now) = host.step(&mut cart, &set);
        exam_frame_held(&cart, &host, (&now, &last), &mut exam, &mut cal, &w);
        market_held(&cart, &cal, seed, &market, DIFFICULTY);
        stocks_held(&cart, &cal, seed, &market, DIFFICULTY);
        if exam.begun && begun_at.is_none() {
            begun_at = Some(frame);
        }
        let portfolio = values(&cart, "Portfolio", "A1:A2");
        assert_eq!(
            portfolio.get("A2").map(String::as_str),
            Some(buffet::bell(&cal)),
            "frame {frame}: Portfolio!A2"
        );
    }
    assert!(acts.next().is_none(), "every act of the script played");
    assert_eq!(exam.answers.len(), buffet::ASKED, "eleven answered");
    assert_eq!(
        exam.answers[2..4],
        [2, 2],
        "the held key's B, and the first by letter of two"
    );
    assert_eq!(
        exam.answers[5], 1,
        "the click held across two frames, A once"
    );
    assert_eq!(exam.style(), 3, "the eleventh, the third style");
    assert_eq!(
        begun_at,
        Some(46),
        "the game begun at the certificate's key"
    );
    assert_eq!(
        host.now.sheet,
        buffet::LEAD,
        "the Camera showed where the certificate leads"
    );
    assert_eq!(
        (cal.waited, cal.hour, cal.minute, cal.tick),
        (45, 9, 40, 2),
        "the bell waited 45 frames, the game's first stood at 9:30, and 30 frames on it is 9:40, the second tick"
    );
}

/// The score funds the firm as the contract says (`GUARD.1`'s decision 6):
/// eleven exams sat by keys, the right answers of the ten from none to all,
/// the eleventh by the option that was right for the tenth, which scores
/// nothing, each frame's books the hand's; at its certificate the capital
/// $5,000 and $500 a right answer, in the Proctor's cents and the
/// Certificate's dollars; and a waiver's score written each of 0 to 10, the
/// same.
#[test]
fn the_score_funds_the_firm_as_the_contract_says() {
    use Act::Key;
    let w = words(&loaded());
    for k in 0..=buffet::SCORED {
        let mut cart = loaded();
        let mut exam = exam_of(&cart, &w);
        let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
        let mut host = Host::new(buffet::EXAM);
        let (last, now) = host.step(&mut cart, &[]);
        exam_frame_held(&cart, &host, (&now, &last), &mut exam, &mut cal, &w);
        for q in 0..buffet::ASKED {
            // The eleventh by the option that was right for the tenth, the
            // row still shown, which scores nothing.
            let right = exam.right();
            let option = if q < k || q >= buffet::SCORED {
                right
            } else {
                right % 4 + 1
            };
            let code = buffet::OPTIONS[option as usize - 1].2;
            for act in [Key(code, true), Key(code, false)] {
                let (last, now) = host.step(&mut cart, &[act]);
                exam_frame_held(&cart, &host, (&now, &last), &mut exam, &mut cal, &w);
            }
        }
        assert_eq!(exam.score(), k as u64);
        let got = values(&cart, "Proctor", "B38:B39");
        assert_eq!(
            (got["B38"].as_str(), got["B39"].as_str()),
            (
                k.to_string().as_str(),
                (500_000 + 50_000 * k).to_string().as_str()
            ),
            "{k} right: the score and the capital in cents"
        );
    }
    let mut cart = cartridge();
    for k in 0..=buffet::SCORED {
        let edit = format!("(cell \"Proctor\" \"B4\" {k})");
        cart.write(&edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
        cart.step(0);
        let got = values(&cart, "Proctor", "B38:B39");
        assert_eq!(
            got["B39"],
            (500_000 + 50_000 * k).to_string(),
            "a waiver at {k}"
        );
        let dollars = values(&cart, "Certificate", "B8");
        assert_eq!(
            dollars["B8"],
            buffet::two_decimals(500_000 + 50_000 * k as u64)
        );
    }
}

/// A waiver (`GUARD.1`'s decision 12), a person's edit of the Proctor's `B3`:
/// written before the first frame, the game begun at it, the exam closed, the
/// market open at its bell and moving twenty seconds a frame as with no exam,
/// the Camera at where the certificate leads, and the certificate saying the
/// exam was waived at five correct answers and the first style; written in
/// the middle of an exam, every frame of it held, the exam closed at the next
/// step, the bell rung at that frame and the market twenty seconds on at the
/// next.
#[test]
fn a_waiver_skips_the_exam_at_any_frame() {
    use Act::Key;
    let w = words(&loaded());
    // Before the first frame.
    let mut cart = cartridge();
    let mut exam = exam_of(&cart, &w);
    exam.waive();
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    let mut host = Host::new(buffet::EXAM);
    for _ in 0..3 {
        let (last, now) = host.step(&mut cart, &[]);
        exam_frame_held(&cart, &host, (&now, &last), &mut exam, &mut cal, &w);
    }
    assert_eq!(
        (cal.waited, cal.seconds()),
        (0, FIRST_MINUTE * 60 + 2 * PACE.session)
    );
    assert_eq!(host.now.sheet, buffet::LEAD);
    // In the middle of an exam: three answered by keys, then the waiver.
    let mut cart = loaded();
    let mut exam = exam_of(&cart, &w);
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    let mut host = Host::new(buffet::EXAM);
    let (last, now) = host.step(&mut cart, &[]);
    exam_frame_held(&cart, &host, (&now, &last), &mut exam, &mut cal, &w);
    for _ in 0..3 {
        let code = buffet::OPTIONS[exam.right() as usize - 1].2;
        for act in [Key(code, true), Key(code, false)] {
            let (last, now) = host.step(&mut cart, &[act]);
            exam_frame_held(&cart, &host, (&now, &last), &mut exam, &mut cal, &w);
        }
    }
    let (sheet, at) = buffet::WAIVER;
    let edit = format!("(cell \"{sheet}\" \"{at}\" 1)");
    cart.write(&edit)
        .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    let waited = cal.frame;
    for k in 0..3 {
        let (last, now) = host.step(&mut cart, &[]);
        if k == 0 {
            exam.waive();
        }
        exam_frame_held(&cart, &host, (&now, &last), &mut exam, &mut cal, &w);
    }
    assert_eq!(exam.answers.len(), 3);
    assert_eq!(
        (exam.score(), exam.style(), exam.capital()),
        (5, 1, 750_000)
    );
    assert_eq!(cal.waited, waited, "the bell rang at the waiver's frame");
    assert_eq!(cal.seconds(), FIRST_MINUTE * 60 + 2 * PACE.session);
    assert_eq!(host.now.sheet, buffet::LEAD);
}

/// Whether a formula reads a cell of another sheet, `sheet!` or its twin's
/// `sheet.last!` before the cell's letters and digits, its anchors ignored and
/// no digit after it, without case.
fn reads_cell(formula: &str, sheet: &str, cell: &str) -> bool {
    let f = formula.replace('$', "").to_ascii_lowercase();
    let cell = cell.to_ascii_lowercase();
    [format!("{sheet}!{cell}"), format!("{sheet}.last!{cell}")]
        .iter()
        .any(|read| {
            f.match_indices(read.to_ascii_lowercase().as_str())
                .any(|(at, m)| {
                    !f[at + m.len()..]
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_ascii_digit())
                })
        })
}

/// The model's sheets and the windows that hold their every cell, the devices
/// the game writes among them: what a style must leave as it found it.
const MODEL: [(&str, &str); 13] = [
    ("Day", "A1:B35"),
    ("Dice", "A1:J84"),
    ("Market", "A1:F124"),
    ("Stocks", "A1:I8"),
    ("Hustle", "A1:B19"),
    ("Proctor", "A1:G41"),
    ("Orders", "A1:I47"),
    ("Positions", "A1:M15"),
    ("Cash", "A1:B10"),
    ("Ledger", "A1:G101"),
    ("Performance", "A1:G29"),
    ("Commands", "A1:D12"),
    ("Keys", "A1:E10"),
];

/// The style names the firm and moves no number (`BACK.2`; `BD-4`, flavor and
/// never power): every formula of another sheet that reads the style, the
/// Proctor's `B40`, or reads the Firms sheet is the style's own twin, the Firms
/// sheet's row shown, the Exam's options, the Certificate's name, the
/// Overview's firm line, or since `CRITIC.1` the Report's header and its firm's
/// row, and the Performance sheet's card, which grades every style alike, is
/// among the model compared; the Firms sheet's firms, as many as the exam's
/// options, each a style and a name within its column, none empty and no name
/// twice; and a waiver's style, each of the four written as a person's edit of
/// the Proctor's `B5` before the first frame, naming its firm on the
/// Certificate and the Overview, while every cell of the model's sheets, the
/// style's two excepted, is the first style's at each of the first frames,
/// however a formula reads the style, so that every firm plays the same market
/// and the same books at the same seed (`BD-3`).
#[test]
fn the_style_names_the_firm_and_moves_no_number() {
    let text = lf(&read(CARTRIDGE));
    let mut readers = BTreeSet::new();
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
        let firms = formula.to_ascii_lowercase().contains("firms!")
            || formula.to_ascii_lowercase().contains("firms.last!");
        if firms || reads_cell(formula, "Proctor", "B40") {
            readers.insert((sheet.to_string(), range.to_string()));
        }
    }
    let want: BTreeSet<(String, String)> = [
        ("Proctor", "B40"),
        ("Firms", "E2"),
        ("Exam", "A7:A10"),
        ("Certificate", "B9"),
        (buffet::OVERVIEW, "A3"),
        (buffet::CARD_VIEW, "A3"),
        (buffet::CARD_VIEW, "A7"),
    ]
    .iter()
    .map(|(s, r)| (s.to_string(), r.to_string()))
    .collect();
    assert_eq!(
        readers, want,
        "the formulas that read the style or the Firms sheet"
    );
    let file = content(FIRMS);
    let rows = firm_rows();
    assert_eq!(
        rows.len(),
        buffet::OPTIONS.len(),
        "a firm for each of the exam's options"
    );
    let names: BTreeSet<&str> = rows.iter().map(|(_, n)| n.as_str()).collect();
    assert_eq!(names.len(), rows.len(), "no firm's name twice: {names:?}");
    for (k, (style, name)) in rows.iter().enumerate() {
        for (word, (header, width)) in [style, name].into_iter().zip(&file.fields) {
            let n = word.chars().count() as u64;
            assert!(
                !word.trim().is_empty() && n <= *width,
                "firm {}: its {header}, {word:?}, {n} characters where its column holds {width}",
                k + 1
            );
        }
    }
    let model = |cart: &Cartridge| -> BTreeMap<String, String> {
        let mut out = BTreeMap::new();
        for (sheet, window) in MODEL {
            for (addr, v) in values(cart, sheet, window) {
                if !(sheet == "Proctor" && (addr == "B5" || addr == "B40")) {
                    out.insert(format!("{sheet}!{addr}"), v);
                }
            }
        }
        out
    };
    let mut first: Vec<BTreeMap<String, String>> = Vec::new();
    for k in 1..=rows.len() as u64 {
        let mut cart = cartridge();
        let edit = format!("(cell \"Proctor\" \"B5\" {k})");
        cart.write(&edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
        for frame in 0..3 {
            assert!(cart.step(0).done);
            let now = model(&cart);
            if k == 1 {
                first.push(now);
            } else if now != first[frame] {
                let moved: Vec<&String> = now
                    .iter()
                    .filter(|(a, v)| first[frame].get(*a) != Some(*v))
                    .map(|(a, _)| a)
                    .collect();
                panic!(
                    "the style {k} moved the model at frame {}: {moved:?}",
                    frame + 1
                );
            }
        }
        assert_eq!(
            values(&cart, "Certificate", "B9").get("B9"),
            Some(&rows[k as usize - 1].1),
            "a waiver's style {k}: Certificate!B9, the firm's name"
        );
        assert_eq!(
            values(&cart, buffet::OVERVIEW, "A3").get("A3"),
            Some(&firm_line_of(k)),
            "a waiver's style {k}: Overview!A3, the firm's line"
        );
    }
}

/// The proctor's settings and buttons, and the views' buttons at their places
/// (`GUARD.1`'s decisions 3, 7 and 9): the capital, the waiver, a waiver's
/// score and style, and the lead, the hand's; the buttons' table, each
/// option's letter, view, column, row and key and the certificate's, the
/// hand's; each button's text at the place the table names, the Exam's options
/// beginning with their letters and the certificate's button no wider than
/// its column; every option of the bank and every style, with its letter,
/// inside the Exam's column, so that a click on any character of it is the
/// option's; the texts beside the buttons naming the keys bound; and no key
/// bound the viewport's, and no Alt.
#[test]
fn the_proctor_states_its_settings_and_its_buttons() {
    let mut cart = loaded();
    cart.step(0);
    let settings = values(&cart, "Proctor", "B1:B6");
    for (addr, text) in [
        ("B1", buffet::CAPITAL_BEFORE.to_string()),
        ("B2", buffet::CAPITAL_A_RIGHT.to_string()),
        ("B3", "0".to_string()),
        ("B4", buffet::WAIVED_SCORE.to_string()),
        ("B5", buffet::WAIVED_STYLE.to_string()),
        ("B6", buffet::LEAD.to_string()),
    ] {
        assert_eq!(settings.get(addr), Some(&text), "Proctor!{addr}");
    }
    assert_eq!(buffet::WAIVER, ("Proctor", "B3"));
    let table = values(&cart, "Proctor", "A24:E28");
    let mut buttons: Vec<(&str, &str, u64, u64, &str)> = buffet::OPTIONS
        .iter()
        .map(|&(letter, row, key)| (letter, buffet::EXAM, buffet::OPTION_COLUMN, row, key))
        .collect();
    let (view, column, row) = buffet::OPEN_BUTTON;
    buttons.push(("open for business", view, column, row, buffet::OPEN_KEY));
    for (k, (label, view, column, row, key)) in buttons.iter().enumerate() {
        let r = 24 + k;
        let want = [
            label.to_string(),
            view.to_string(),
            column.to_string(),
            row.to_string(),
            key.to_string(),
        ];
        for (c, text) in ["A", "B", "C", "D", "E"].iter().zip(want) {
            assert_eq!(table.get(&format!("{c}{r}")), Some(&text), "Proctor!{c}{r}");
        }
    }
    let exam = values(&cart, "Exam", "A1:A14");
    for (letter, row, _) in buffet::OPTIONS {
        let text = &exam[&format!("A{row}")];
        assert!(
            text.starts_with(&format!("[ {letter} ]  ")),
            "Exam!A{row}: {text}"
        );
    }
    let cert = values(&cart, "Certificate", "A11:B11");
    assert_eq!(cert.get("A11").map(String::as_str), Some(buffet::OPEN_TEXT));
    assert_eq!(
        cert.get("B11").map(String::as_str),
        Some(buffet::OPEN_KEY_TEXT)
    );
    let text = lf(&read(CARTRIDGE));
    for (sheet, width) in [
        ("Exam", buffet::EXAM_COLUMN),
        ("Certificate", buffet::OPEN_TEXT.chars().count() as u64),
    ] {
        let row = format!("(column \"{sheet}\" \"A\" {width} shown none)");
        assert!(text.lines().any(|l| l == row), "{sheet}!A: {row}");
    }
    // The buttons' look (the owner's word at the hand test): format 1, a fill
    // of Lovelace Lavender, on every option's cell and the certificate's
    // button; and an option emptied after the exam drawn in the default look.
    assert!(
        text.lines().any(|l| l == buffet::BUTTON_FORMAT),
        "the buttons' format: {}",
        buffet::BUTTON_FORMAT
    );
    let (view, column, row) = buffet::OPEN_BUTTON;
    let looks = buffet::OPTIONS
        .iter()
        .map(|&(_, row, _)| (buffet::EXAM, buffet::OPTION_COLUMN, row))
        .chain([(view, column, row)]);
    for (sheet, column, row) in looks {
        let style = format!(
            "(style \"{sheet}\" \"{}\" 1)",
            address(row as usize, column as usize)
        );
        assert!(text.lines().any(|l| l == style), "a button's look: {style}");
    }
    assert!(
        text.lines().any(|l| l == "(look \"Exam\" \"\" 0)"),
        "an option emptied after the exam, in the default look"
    );
    let w = words(&cart);
    for (q, row) in (1..).zip(&w.bank) {
        for (k, option) in row[2..6].iter().enumerate() {
            let shown = format!("[ {} ]  {option}", buffet::OPTIONS[k].0)
                .chars()
                .count() as u64;
            assert!(
                shown <= buffet::EXAM_COLUMN,
                "question {q}'s option {}: {shown} characters with its letter, past the Exam's {}: a click on its spill would miss",
                buffet::OPTIONS[k].0,
                buffet::EXAM_COLUMN
            );
        }
    }
    assert!(
        w.styles.len() >= buffet::OPTIONS.len(),
        "four styles at least"
    );
    for (k, style) in w.styles.iter().take(4).enumerate() {
        let shown = format!("[ {} ]  {style}", buffet::OPTIONS[k].0)
            .chars()
            .count() as u64;
        assert!(
            shown <= buffet::EXAM_COLUMN,
            "style {}: {shown} characters",
            k + 1
        );
    }
    for (letter, _, key) in buffet::OPTIONS {
        assert!(key.ends_with(letter) && buffet::ANSWER_KEYS_TEXT.contains(letter));
    }
    assert!(buffet::OPEN_KEY_TEXT.ends_with(&buffet::OPEN_KEY["Key".len()..]));
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
    let codes: Vec<&str> = buffet::OPTIONS
        .iter()
        .map(|(_, _, key)| *key)
        .chain([buffet::OPEN_KEY, P])
        .collect();
    for code in codes {
        assert!(
            !moves.contains(&code) && !code.starts_with("Alt"),
            "{code} is the viewport's or the chords'"
        );
    }
}

/// What the last frame showed the order desk (`PLATE.1`), from the hand's
/// calendar, market and books before this frame: whether the market was open,
/// each stock's price now, each pocket's cash from its start at that frame, and
/// its five minutes as the ledger writes it; and nothing at the first frame,
/// whose last is the load's, every twin empty.
fn shown(cal: &Calendar, seed: u64, market: &Market, plate: &Plate, starts: [u64; 2]) -> Shown {
    if cal.frame == 0 {
        return Shown::default();
    }
    let now = market.path(seed, DIFFICULTY, cal).now;
    Shown {
        open: cal.open(),
        prices: std::array::from_fn(|k| RUNGS[k].price_at(now)),
        cash: [
            plate.cash(buffet::FIRM, starts[0]),
            plate.cash(buffet::PERSONAL, starts[1]),
        ],
        when: format!(
            "{}, year {}, day {}, {}",
            buffet::DAY_NAMES[cal.weekday as usize],
            cal.year,
            cal.day_of_year,
            buffet::time_shown(cal.clock())
        ),
    }
}

/// Each stock's price now in whole cents at this frame, from the Stocks
/// sheet's own cells as `stocks_held` has just held them to the hand.
fn prices_of(stocks: &BTreeMap<String, String>) -> [u64; 4] {
    std::array::from_fn(|k| stocks[&format!("E{}", 5 + k)].parse().expect("whole cents"))
}

/// The order desk's books at this frame (`PLATE.1`), every working the hand's:
/// the grid's presses; the stock and the trade clicked; the command run, the
/// stock it picks and the trade it orders; the stock picked and its name; the
/// order's stock by name, its pocket, its stock and its trade; what the last
/// frame showed, the market, the price, the pocket's cash and its shares; the
/// shares filled and the outcome; the fills counted, the ledger's row and the
/// pocket's cash now; the last order as it was taken, every cell 0 before
/// the first; and since `PLATE.2` the pocket the hand holds after this frame,
/// and the switch, its place, its click, its command run and whether it
/// switched. Each stock's name is the hand's (`MENU.3`).
#[allow(clippy::too_many_arguments)]
fn orders_held(
    cart: &Cartridge,
    frame: u64,
    plate: &Plate,
    io: (&Inputs, &Inputs),
    command: &str,
    order: Option<Order>,
    shown: &Shown,
    starts: [u64; 2],
) {
    let (now, last) = io;
    let flag = |b: bool| u8::from(b).to_string();
    let name = |s: u64| {
        if s == 0 {
            String::new()
        } else {
            RUNGS[s as usize - 1].name.to_string()
        }
    };
    let press = Plate::clicked(now, last);
    let grid = values(cart, "Orders", "D6:I9");
    for (s, &row) in buffet::STOCK_ROWS.iter().enumerate() {
        for t in 0..buffet::TRADES.len() {
            let at = address(6 + s, 4 + t);
            let want = flag(press == Some((s as u64 + 1, t + 1)));
            assert_eq!(
                grid.get(&at),
                Some(&want),
                "frame {frame}: Orders!{at}, the button on Trading's row {row}"
            );
        }
    }
    let (stock_clicked, trade_clicked) = press.unwrap_or((0, 0));
    let pocket = plate.funding();
    let n = plate.fills.len() as u64;
    let o = order;
    let mut want: Vec<(&str, String)> = vec![
        ("B11", stock_clicked.to_string()),
        ("B12", trade_clicked.to_string()),
        ("B13", command.to_string()),
        ("B14", Plate::pick_of(command).to_string()),
        ("B15", Plate::trade_of(command).to_string()),
        ("B16", plate.picked.to_string()),
        (
            "B17",
            if plate.picked == 0 {
                "the stock picked, none yet".to_string()
            } else {
                name(plate.picked)
            },
        ),
        ("B18", name(o.map_or(0, |o| o.stock))),
        ("B19", pocket.to_string()),
        ("B20", o.map_or(0, |o| o.stock).to_string()),
        ("B21", o.map_or(0, |o| o.trade).to_string()),
        ("B22", flag(shown.open)),
        ("B23", o.map_or(0, |o| o.price).to_string()),
        ("B24", shown.cash[pocket as usize - 1].to_string()),
        ("B25", o.map_or(0, |o| o.held).to_string()),
        ("B26", o.map_or(0, |o| o.filled).to_string()),
        ("B27", o.map_or(0, |o| o.outcome as u8).to_string()),
        ("B29", n.to_string()),
        ("B30", ((n as i64 - 1).rem_euclid(100) + 2).to_string()),
        (
            "B31",
            plate.cash(pocket, starts[pocket as usize - 1]).to_string(),
        ),
    ];
    let held: [String; 10] = match plate.last {
        Some(l) => [
            name(l.stock),
            l.pocket.to_string(),
            l.stock.to_string(),
            l.trade.to_string(),
            flag(l.open),
            l.price.to_string(),
            l.cash.to_string(),
            l.held.to_string(),
            l.filled.to_string(),
            (l.outcome as u8).to_string(),
        ],
        None => std::array::from_fn(|_| "0".to_string()),
    };
    let addrs = [
        "B32", "B33", "B34", "B35", "B36", "B37", "B38", "B39", "B40", "B41",
    ];
    want.extend(addrs.into_iter().zip(held));
    let (_, column, row) = buffet::SWITCH;
    let switch = Plate::switch_clicked(now, last);
    want.extend([
        ("B1", plate.pocket.to_string()),
        ("B43", column.to_string()),
        ("B44", row.to_string()),
        ("B45", flag(switch)),
        ("B46", flag(command == buffet::SWITCH_COMMAND.0)),
        ("B47", flag(Plate::switches(press, switch, command))),
    ]);
    let got = values(cart, "Orders", "B1:B47");
    for (addr, text) in want {
        assert_eq!(got.get(addr), Some(&text), "frame {frame}: Orders!{addr}");
    }
}

/// The books at this frame (`PLATE.1`), every cell the hand's: a row a pocket
/// and a stock, its pocket, rung and name, the hand's (`MENU.3`), this frame's
/// fill on its row, the shares, the cost basis, the cents spent and received,
/// the dividends, the price now, the value, the gain not yet taken and the gain
/// taken; and the rows shown, the pocket an order funds.
fn positions_held(
    cart: &Cartridge,
    frame: u64,
    plate: &Plate,
    order: Option<Order>,
    prices: [u64; 4],
) {
    let got = values(cart, "Positions", "A2:M15");
    let mut want: Vec<(String, String)> = Vec::new();
    for pocket in [buffet::FIRM, buffet::PERSONAL] {
        for stock in 1..=4u64 {
            let row = 1 + (pocket - 1) * 4 + stock;
            let b = plate.book(pocket, stock);
            let filled = match order {
                Some(o) if o.pocket == pocket && o.stock == stock => o.filled,
                _ => 0,
            };
            let price = prices[stock as usize - 1];
            let value = b.shares * price;
            let cells = [
                ("A", pocket.to_string()),
                ("B", stock.to_string()),
                ("C", RUNGS[stock as usize - 1].name.to_string()),
                ("D", filled.to_string()),
                ("E", b.shares.to_string()),
                ("F", b.basis.to_string()),
                ("G", b.bought.to_string()),
                ("H", b.sold.to_string()),
                (
                    "I",
                    plate.dividends[pocket as usize - 1][stock as usize - 1].to_string(),
                ),
                ("J", price.to_string()),
                ("K", value.to_string()),
                ("L", (value as i64 - b.basis as i64).to_string()),
                (
                    "M",
                    (b.sold as i64 - b.bought as i64 + b.basis as i64).to_string(),
                ),
            ];
            want.extend(cells.map(|(c, v)| (format!("{c}{row}"), v)));
        }
    }
    let pocket = plate.funding();
    for stock in 1..=4u64 {
        let row = 11 + stock;
        let b = plate.book(pocket, stock);
        let value = b.shares * prices[stock as usize - 1];
        let cells = [
            ("A", pocket.to_string()),
            ("B", stock.to_string()),
            ("C", RUNGS[stock as usize - 1].name.to_string()),
            ("D", b.shares.to_string()),
            ("E", b.basis.to_string()),
            ("F", value.to_string()),
            ("G", (value as i64 - b.basis as i64).to_string()),
        ];
        want.extend(cells.map(|(c, v)| (format!("{c}{row}"), v)));
    }
    for (addr, text) in want {
        assert_eq!(
            got.get(&addr),
            Some(&text),
            "frame {frame}: Positions!{addr}"
        );
    }
}

/// The ledger at this frame (`PLATE.1`): exactly the hand's ring of the fills
/// landed, those of the frames before this one, the Write sheet's rows being
/// made a frame late; each row its fill's number, when, pocket, stock by the
/// hand's name (`MENU.3`), shares, price and cash after, and no other cell.
fn ledger_held(cart: &Cartridge, frame: u64, plate: &Plate, landed: usize) {
    let got = values(cart, "Ledger", "A2:G101");
    let mut want = BTreeMap::new();
    for row in 2..2 + buffet::LEDGER_ROWS {
        if let Some(f) = plate.ledger(row, landed) {
            let cells = [
                ("A", f.number.to_string()),
                ("B", f.when.clone()),
                ("C", buffet::POCKET_NAMES[f.pocket as usize - 1].to_string()),
                ("D", RUNGS[f.stock as usize - 1].name.to_string()),
                ("E", f.shares.to_string()),
                ("F", f.price.to_string()),
                ("G", f.cash_after.to_string()),
            ];
            for (c, v) in cells {
                want.insert(format!("{c}{row}"), v);
            }
        }
    }
    assert!(
        got == want,
        "frame {frame}: the Ledger\n{got:?}\nwhere the hand's is\n{want:?}"
    );
}

/// The commands at this frame (`PLATE.1`, the page's version 5): each the
/// hand's name and key, its words by the stock picked, by the hand's names
/// (`MENU.3`), and the switch's by the pocket it turns to (`PLATE.2`), and
/// whether it holds, while Trading shows, the sheet this frame's inputs named.
fn commands_held(cart: &Cartridge, frame: u64, (picked, pocket): (u64, u64), sheet: &str) {
    let got = values(cart, "Commands", "A2:D12");
    let words = buffet::command_words(picked, pocket);
    let holds = if sheet.eq_ignore_ascii_case(buffet::TRADING) {
        "true"
    } else {
        "false"
    };
    for (k, (name, key)) in buffet::PICKS
        .iter()
        .chain(buffet::TRADE_COMMANDS.iter())
        .chain([buffet::SWITCH_COMMAND].iter())
        .enumerate()
    {
        let row = k + 2;
        for (c, want) in [
            ("A", *name),
            ("B", words[k].as_str()),
            ("C", holds),
            ("D", *key),
        ] {
            assert_eq!(
                got.get(&format!("{c}{row}")).map(String::as_str),
                Some(want),
                "frame {frame}: Commands!{c}{row}"
            );
        }
    }
}

/// A frame of the order desk by hand beside the cartridge's, after the host's
/// step (`PLATE.1`): the hand's calendar on a frame, the dividends of a day's
/// first frame paid to the shares the last frame held, the order judged by what
/// the last frame showed; then every cell of the desk, the books, the cash, the
/// ledger, the commands and both views held, and the Write sheet's rows the
/// host read, seven while an order fills and none else. The order taken.
#[allow(clippy::too_many_arguments)]
fn plate_frame_held(
    cart: &Cartridge,
    host: &Host,
    io: (&Inputs, &Inputs),
    plate: &mut Plate,
    cal: &mut Calendar,
    (seed, market): (u64, &Market),
    shown: &Shown,
    starts: [u64; 2],
    texts: &[String],
) -> Option<Order> {
    let (now, last) = io;
    let landed = plate.fills.len();
    cal.tick();
    if cal.turned {
        plate.pay(std::array::from_fn(|k| {
            RUNGS[k].dividend_on(cal.day_of_year)
        }));
    }
    let order = plate.frame(
        Plate::clicked(now, last),
        Plate::switch_clicked(now, last),
        &host.command,
        shown,
        starts,
    );
    let f = cal.frame;
    let stocks = stocks_held(cart, cal, seed, market, DIFFICULTY);
    let prices = prices_of(&stocks);
    orders_held(
        cart,
        f,
        plate,
        (now, last),
        &host.command,
        order,
        shown,
        starts,
    );
    positions_held(cart, f, plate, order, prices);
    books_cash_held(
        cart,
        f,
        [
            plate.cash(buffet::FIRM, starts[0]),
            plate.cash(buffet::PERSONAL, starts[1]),
        ],
        [
            plate.holdings(buffet::FIRM, prices),
            plate.holdings(buffet::PERSONAL, prices),
        ],
    );
    ledger_held(cart, f, plate, landed);
    commands_held(cart, f, (plate.picked, plate.funding()), &now.sheet);
    trading_cells(cart, cal, &stocks, texts, plate, starts);
    portfolio_cells(cart, cal, &stocks, texts, plate, starts);
    overview_cells(
        cart,
        cal,
        (seed, market),
        texts,
        plate,
        (starts, prices),
        buffet::WAIVED_STYLE,
    );
    let wrote = plate.fills.len() > landed;
    assert_eq!(
        host.pending.len(),
        if wrote { 7 } else { 0 },
        "frame {f}: the Write sheet's rows, seven while an order fills and none else: {:?}",
        host.pending
    );
    order
}

/// The order desk at work (`PLATE.1`'s decisions 3 to 11), every frame of a
/// scripted session held to the hand at the game's pace, the exam waived:
/// the Input rows written as the host writes them, its keymap, its commands and
/// its Camera read after every step, the derived writes it read made before
/// the next frame. Its orders: a trade's key with no stock picked, refused; a
/// click on Buy 10, filling at the price shown and picking its stock; B, a
/// share of it; 1, a pick; M, as many as the cash covers; N, ten the cash no
/// longer covers, refused; a click on Sell all at 9:35, a tick; S with none
/// held, refused; 3 and D, ten
/// sold of eleven, then ten of one, refused; a click and a trade's key in one
/// frame, the click's order alone; B on the Portfolio, where no command holds,
/// a key of no button, and a palette's run there, refused by the host; a
/// palette's Sell all on Trading; your own pocket by a click on the switch
/// (`PLATE.2`, where a person's edit of `Orders!B1` stood), Buy 10 of a stock
/// its $100 covers three of, refused whole, then a click, M and a palette's
/// Sell 1 from it; the firm's again by C, and a click on Sell 1 in the
/// frame the 9:45 tick moves the price, filling at the last frame's; then at
/// the closing bell's frame, the 1,171st at twenty seconds a frame (`POT.11`),
/// a share filling at the last open frame's price, and at the next frame a
/// share refused, the market closed.
#[test]
fn a_scripted_session_holds_every_order_to_the_hand() {
    use Act::{Key, Left, Point, Run, Show};
    let script: &[(u64, &[Act])] = &[
        (1, &[Show("Trading")]),
        (3, &[Show("Trading")]), // the Camera showed the Portfolio at the begin
        (4, &[Key("KeyB", true)]), // buy-1 with no stock picked: refused
        (5, &[Key("KeyB", false)]),
        (6, &[Point(6, 8), Left(true)]), // Buy 10 of the third: filled, and picked
        (7, &[Left(false)]),
        (8, &[Key("KeyB", true)]), // buy-1 of the third
        (9, &[Key("KeyB", false)]),
        (10, &[Key("Digit1", true)]), // the first picked
        (11, &[Key("Digit1", false)]),
        (12, &[Key("KeyM", true)]), // buy-max of the first: every cent
        (13, &[Key("KeyM", false)]),
        (14, &[Key("KeyN", true)]), // buy-10: the cash spent, refused
        (15, &[Key("KeyN", false)]),
        (16, &[Point(10, 6), Left(true)]), // Sell all of the first at 9:35, a tick: the last frame's price
        (17, &[Left(false)]),
        (18, &[Key("KeyS", true)]), // sell-1 of the first, none held: refused
        (19, &[Key("KeyS", false)]),
        (20, &[Key("Digit3", true)]), // the third picked
        (21, &[Key("Digit3", false)]),
        (22, &[Key("KeyD", true)]), // sell-10 of eleven
        (23, &[Key("KeyD", false)]),
        (24, &[Key("KeyD", true)]), // sell-10 of one: refused
        (25, &[Key("KeyD", false)]),
        (26, &[Point(5, 7), Left(true), Key("KeyF", true)]), // Buy 1 of the second clicked, and sell-all keyed: the click's
        (27, &[Left(false), Key("KeyF", false)]),
        (28, &[Show("Portfolio")]),
        (29, &[Key("KeyB", true)]), // B on the Portfolio: no command holds there
        (30, &[Key("KeyB", false)]),
        (31, &[Run("buy-1")]), // the palette there: refused by the host
        (32, &[Show("Trading")]),
        (33, &[Run("sell-all")]), // the palette's: every share of the second
        (34, &[Point(4, 11), Left(true)]), // the switch: your own pocket
        (35, &[Left(false)]),
        (36, &[Point(6, 8), Left(true)]), // Buy 10 of the third, $100 covering three: refused
        (37, &[Left(false)]),
        (38, &[Point(5, 6), Left(true)]), // Buy 1 of the first
        (39, &[Left(false)]),
        (40, &[Key("KeyM", true)]), // buy-max from your own cash
        (41, &[Key("KeyM", false)]),
        (42, &[Run("sell-1")]),
        (44, &[Key("KeyC", true)]), // the switch by its key: the firm's again
        (45, &[Key("KeyC", false)]),
        (46, &[Point(8, 8), Left(true)]), // Sell 1 of the third at 9:45, a tick moving its price
        (47, &[Left(false)]),
        (1_171, &[Run("buy-1")]), // 16:00, the closing bell's frame: the last frame showed the market open
        (1_172, &[Run("buy-1")]), // 16:02: the market shown closed, refused
    ];
    let last_frame = 1_174;
    let mut cart = cartridge();
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    let mut host = Host::new(buffet::EXAM);
    let mut plate = Plate::default();
    let starts = [WAIVED_CAPITAL, buffet::PERSONAL_CASH];
    let mut acts = script.iter().peekable();
    let mut outcomes: Vec<(u64, buffet::Outcome)> = Vec::new();
    for frame in 1..=last_frame {
        let mut set: Vec<Act> = Vec::new();
        while let Some((_, more)) = acts.next_if(|(at, _)| *at == frame) {
            set.extend_from_slice(more);
        }
        let shown = shown(&cal, seed, &market, &plate, starts);
        let (last, now) = host.step(&mut cart, &set);
        let order = plate_frame_held(
            &cart,
            &host,
            (&now, &last),
            &mut plate,
            &mut cal,
            (seed, &market),
            &shown,
            starts,
            &texts,
        );
        if let Some(o) = order {
            outcomes.push((frame, o.outcome));
        }
        if frame == 46 {
            let now = RUNGS[2].price_at(market.path(seed, DIFFICULTY, &cal).now);
            assert_ne!(
                now, shown.prices[2],
                "the tick at 9:45 moves the third's price, so its sale fills at the last frame's"
            );
        }
    }
    assert!(acts.next().is_none(), "every act of the script played");
    use buffet::Outcome::{Closed, Filled, NoCash, NoShares, NoStock};
    assert_eq!(
        outcomes,
        [
            (4, NoStock),
            (6, Filled),
            (8, Filled),
            (12, Filled),
            (14, NoCash),
            (16, Filled),
            (18, NoShares),
            (22, Filled),
            (24, NoShares),
            (26, Filled),
            (33, Filled),
            (36, NoCash),
            (38, Filled),
            (40, Filled),
            (42, Filled),
            (46, Filled),
            (1_171, Filled),
            (1_172, Closed),
        ],
        "the orders taken, by frame"
    );
    let fills: Vec<(u64, u64, i64)> = plate
        .fills
        .iter()
        .map(|f| (f.pocket, f.stock, f.shares))
        .collect();
    assert_eq!(fills[0], (buffet::FIRM, 3, 10));
    assert_eq!(fills[1], (buffet::FIRM, 3, 1));
    assert_eq!(
        fills[3],
        (buffet::FIRM, 1, -fills[2].2),
        "every share of the first sold"
    );
    assert_eq!(fills[4], (buffet::FIRM, 3, -10));
    assert_eq!(fills[5..7], [(buffet::FIRM, 2, 1), (buffet::FIRM, 2, -1)]);
    assert_eq!(fills[7], (buffet::PERSONAL, 1, 1));
    assert_eq!(fills[9], (buffet::PERSONAL, 1, -1));
    assert_eq!(
        fills[10],
        (buffet::FIRM, 3, -1),
        "the third's last share at 9:45"
    );
    assert_eq!(
        fills[11],
        (buffet::FIRM, 3, 1),
        "a share of the stock picked at the closing bell's frame"
    );
    assert_eq!(host.now.sheet, buffet::TRADING);
}

/// The ledger's ring past a hundred fills (`PLATE.1`'s decision 10, `MISE.3`'s
/// decision 5): a click at the first frame, whose last is the load's, refused
/// as the market closed and picking the first stock; then a share of it bought
/// and sold by the palette's commands at every frame, 106 fills, the ledger
/// held at every frame a frame late; at the end its first row holds the 101st,
/// its next five the 102nd to the 106th, and the rest the 7th to the 100th.
#[test]
fn the_ledger_rings_past_a_hundred_fills() {
    use Act::{Left, Point, Run, Show};
    let mut cart = cartridge();
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    let mut host = Host::new(buffet::EXAM);
    let mut plate = Plate::default();
    let starts = [WAIVED_CAPITAL, buffet::PERSONAL_CASH];
    let fills = 106;
    for frame in 1..=fills + 4 {
        let set: Vec<Act> = match frame {
            1 => vec![Show("Trading"), Point(5, 6), Left(true)],
            2 => vec![Left(false)],
            3 => vec![Show("Trading")],
            f if f < fills + 4 => vec![Run(if f % 2 == 0 { "buy-1" } else { "sell-1" })],
            _ => vec![],
        };
        let shown = shown(&cal, seed, &market, &plate, starts);
        let (last, now) = host.step(&mut cart, &set);
        let order = plate_frame_held(
            &cart,
            &host,
            (&now, &last),
            &mut plate,
            &mut cal,
            (seed, &market),
            &shown,
            starts,
            &texts,
        );
        if frame == 1 {
            assert_eq!(order.map(|o| o.outcome), Some(buffet::Outcome::Closed));
        }
    }
    assert_eq!(plate.fills.len() as u64, fills);
    let rows = values(&cart, "Ledger", "A2:A101");
    let number = |row: u64| rows[&format!("A{row}")].parse::<u64>().expect("a number");
    assert_eq!(number(2), 101, "the 101st on the first row");
    for (row, n) in (3..=7).zip(102..) {
        assert_eq!(number(row), n);
    }
    for row in 8..=101 {
        assert_eq!(number(row), row - 1);
    }
}

/// The dividend (`PLATE.1`'s decision 9, `STATION.1`'s decision 11): at a frame
/// a day, every frame the opening bell's minute of the next day, ten shares of
/// the fourth stock bought for the firm and ten for your own, the switch's key
/// between them (`PLATE.2`), your own start raised to $10,000 by a person's
/// edit so that it can; then every frame held to
/// the third year's first dividend day: at each of the nine dividend days'
/// first frames, each pocket paid 50 cents a share of the shares the last frame
/// held, into its dividends and its cash, and nothing at any other frame. The
/// ninth day, the 821st, is a Tuesday, the frame before it Monday's opening
/// bell, so an order fills there: the firm's every share sold that frame, the
/// palette's switch the frame before, the dividend still paid to the ten
/// shares held as the day before closed, the record date. At the game's pace
/// a day's first frame is midnight, when no order fills, so only a pace of a
/// day a frame or more meets the two. Since `CRITIC.1` the Camera shows the
/// Report at each quarter's turn, eight of them on the way, so Trading is
/// shown again after the last before the switch.
#[test]
fn a_dividend_is_paid_to_the_shares_held_at_the_days_turn() {
    use Act::{Key, Left, Point, Run, Show};
    let (mut cart, mut cal) = at_pace(A_DAY_LONG);
    let edit = "(cell \"Cash\" \"B1\" 1000000)";
    cart.write(edit)
        .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut host = Host::new(buffet::EXAM);
    let mut plate = Plate::default();
    let starts = [WAIVED_CAPITAL, 1_000_000];
    let mut paid = Vec::new();
    for frame in 1..=822 {
        let set: Vec<Act> = match frame {
            2 => vec![Show("Trading"), Point(6, 9), Left(true)], // Buy 10 of the fourth, the firm's
            3 => vec![Left(false), Key("KeyC", true)],           // the switch by its key: your own
            4 => vec![Left(true), Key("KeyC", false)],           // Buy 10 of the fourth, your own
            5 => vec![Left(false)],
            735 => vec![Show("Trading")], // after the year's turn, the Report shown at day 730's card
            820 => vec![Run("switch-cash")], // the palette's switch: the firm's again
            821 => vec![Point(10, 9), Left(true)], // Sell all of the fourth, on a dividend's frame
            822 => vec![Left(false)],
            _ => vec![],
        };
        let before = plate.dividends;
        let shown = shown(&cal, seed, &market, &plate, starts);
        let (last, now) = host.step(&mut cart, &set);
        let order = plate_frame_held(
            &cart,
            &host,
            (&now, &last),
            &mut plate,
            &mut cal,
            (seed, &market),
            &shown,
            starts,
            &texts,
        );
        if frame == 821 {
            assert_eq!(
                order.map(|o| (o.outcome, o.filled)),
                Some((buffet::Outcome::Filled, -10)),
                "the firm's ten sold on the dividend's frame"
            );
        }
        if plate.dividends != before {
            let d: Vec<u64> = plate.dividends.iter().map(|p| p[3]).collect();
            paid.push((cal.day, d));
        }
    }
    let days = [90, 181, 272, 363, 455, 546, 637, 728, 820];
    let want: Vec<(u64, Vec<u64>)> = days
        .iter()
        .enumerate()
        .map(|(k, &day)| (day, vec![500 * (k as u64 + 1); 2]))
        .collect();
    assert_eq!(paid, want, "the nine dividends, each pocket's ten shares");
    assert_eq!(plate.book(buffet::FIRM, 4).shares, 0);
}

/// The order desk's settings, its buttons and its commands (`PLATE.1`'s
/// decisions 1 to 4): the pocket an order funds, the firm's; the buttons' view
/// and their places, an act a column and a stock a row, the hand's; each
/// button's text at its place on Trading, inside a column of 12 characters,
/// and drawn in lavender by a look of its text; the Commands sheet's headers,
/// names and keys, the hand's, the order desk reading each name from it; no
/// key the viewport moves by, none with Alt, and the keys' line naming them;
/// and every command holding while Trading shows and on no other sheet. Since
/// `PLATE.2` the switch: its place, the hand's, beside the cash available; its
/// text there inside its column, drawn in lavender by a style row, and its key
/// beside it; and its command, its name and its key, read by the order desk.
#[test]
fn the_order_desk_states_its_settings_its_buttons_and_its_commands() {
    let mut cart = cartridge();
    cart.step(0);
    let orders = values(&cart, "Orders", "A1:I9");
    assert_eq!(orders.get("B1").map(String::as_str), Some("1"));
    assert_eq!(orders.get("B2").map(String::as_str), Some(buffet::TRADING));
    for (t, column) in buffet::TRADE_COLUMNS.iter().enumerate() {
        let at = address(5, 4 + t);
        assert_eq!(orders.get(&at), Some(&column.to_string()), "Orders!{at}");
    }
    for (s, row) in buffet::STOCK_ROWS.iter().enumerate() {
        let r = 6 + s;
        assert_eq!(
            orders.get(&format!("A{r}")).map(String::as_str),
            Some(RUNGS[s].name),
            "Orders!A{r}, the stock's name, the hand's"
        );
        assert_eq!(orders.get(&format!("B{r}")), Some(&(s + 1).to_string()));
        assert_eq!(orders.get(&format!("C{r}")), Some(&row.to_string()));
    }
    let view = values(&cart, buffet::TRADING, "A1:J17");
    let text = lf(&read(CARTRIDGE));
    for (t, label) in buffet::TRADE_TEXTS.iter().enumerate() {
        let column = buffet::TRADE_COLUMNS[t] as usize;
        for &row in &buffet::STOCK_ROWS {
            let at = address(row as usize, column);
            assert_eq!(
                view.get(&at).map(String::as_str),
                Some(*label),
                "Trading!{at}"
            );
        }
        let width = format!(
            "(column \"Trading\" \"{}\" {} shown none)",
            letters(column),
            buffet::BUTTON_COLUMN
        );
        assert!(text.lines().any(|l| l == width), "{width}");
        assert!(
            label.chars().count() as u64 <= buffet::BUTTON_COLUMN,
            "{label}"
        );
        let look = format!("(look \"Trading\" \"{label}\" 1)");
        assert!(text.lines().any(|l| l == look), "a button's look: {look}");
    }
    assert!(text.lines().any(|l| l == buffet::BUTTON_FORMAT));
    let (view_name, column, row) = buffet::SWITCH;
    assert_eq!(view_name, buffet::TRADING);
    let place = values(&cart, "Orders", "B43:B44");
    assert_eq!(
        place.get("B43"),
        Some(&column.to_string()),
        "the switch's column"
    );
    assert_eq!(place.get("B44"), Some(&row.to_string()), "the switch's row");
    let at = address(row as usize, column as usize);
    assert_eq!(
        view.get(&at).map(String::as_str),
        Some(buffet::SWITCH_TEXT),
        "Trading!{at}, the switch"
    );
    assert_eq!(
        view.get(&address(row as usize, column as usize + 1))
            .map(String::as_str),
        Some(buffet::SWITCH_KEY_TEXT),
        "the switch's key beside it"
    );
    let width = format!(
        "(column \"Trading\" \"{}\" {} shown none)",
        letters(column as usize),
        buffet::BUTTON_COLUMN
    );
    assert!(text.lines().any(|l| l == width), "{width}");
    assert!(buffet::SWITCH_TEXT.chars().count() as u64 <= buffet::BUTTON_COLUMN);
    let style = format!("(style \"Trading\" \"{at}\" 1)");
    assert!(
        text.lines().any(|l| l == style),
        "the switch's look: {style}"
    );
    let commands = values(&cart, "Commands", "A1:D12");
    for (c, header) in ["A", "B", "C", "D"]
        .iter()
        .zip(["name", "words", "holds", "key"])
    {
        assert_eq!(
            commands.get(&format!("{c}1")).map(String::as_str),
            Some(header)
        );
    }
    let all: Vec<(&str, &str)> = buffet::PICKS
        .iter()
        .chain(buffet::TRADE_COMMANDS.iter())
        .copied()
        .collect();
    for (k, (name, key)) in all.iter().enumerate() {
        let row = k + 2;
        assert_eq!(
            commands.get(&format!("A{row}")).map(String::as_str),
            Some(*name)
        );
        assert_eq!(
            commands.get(&format!("D{row}")).map(String::as_str),
            Some(*key)
        );
        let read = format!("Commands!$A${row}");
        let reader = if k < 4 { "B14" } else { "B15" };
        let head = format!("(formula \"Orders\" \"{reader}\" ");
        assert!(
            text.lines()
                .any(|l| l.starts_with(&head) && l.contains(&read)),
            "Orders!{reader} reads {name}'s name from {read}"
        );
    }
    let (switch, key) = buffet::SWITCH_COMMAND;
    assert_eq!(commands.get("A12").map(String::as_str), Some(switch));
    assert_eq!(commands.get("D12").map(String::as_str), Some(key));
    assert!(
        text.lines()
            .any(|l| l.starts_with("(formula \"Orders\" \"B46\" ") && l.contains("Commands!$A$12")),
        "Orders!B46 reads the switch's name from Commands!$A$12"
    );
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
    for (_, key) in &all {
        assert!(!moves.contains(key) && !key.starts_with("Alt"), "{key}");
    }
    assert!(
        !moves.contains(&key) && !all.iter().any(|(_, k)| *k == key),
        "the switch's key, {key}, moves nothing and is no other command's"
    );
    for words in ["1 to 4", "B, N or M", "S, D or F"] {
        assert!(
            buffet::TRADING_KEYS.contains(words),
            "the keys' line: {words}"
        );
    }
    assert_eq!(
        all.iter().map(|(_, key)| *key).collect::<Vec<_>>(),
        ["Digit1", "Digit2", "Digit3", "Digit4", "KeyB", "KeyN", "KeyM", "KeyS", "KeyD", "KeyF"],
        "the keys the line names, in its order"
    );
    for (shown, holds) in [(buffet::TRADING, "true"), ("Portfolio", "false")] {
        let edit = format!("(cell \"Input\" \"B19\" \"{shown}\")");
        cart.write(&edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
        cart.step(0);
        let got = values(&cart, "Commands", "C2:C12");
        assert!(
            got.len() == all.len() + 1 && got.values().all(|h| h == holds),
            "the commands while {shown} shows: {got:?}"
        );
    }
}

/// Every balance at this frame (`BD-9`): each pocket's cash, each book's
/// shares, cost basis, cents bought and sold and dividends, the fills counted
/// and the ledger's rows, by sheet and address. A valuation, the value of the
/// shares and the net worth, moves with the price and is no balance.
fn balances(cart: &Cartridge) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for (sheet, range) in [
        ("Cash", "B3:B4"),
        ("Positions", "E2:I9"),
        ("Orders", "B29:B29"),
        ("Ledger", "A2:G101"),
    ] {
        for (addr, v) in values(cart, sheet, range) {
            out.insert(format!("{sheet}!{addr}"), v);
        }
    }
    out
}

/// The pockets' switch at work (`PLATE.2`'s decisions), every frame of a
/// scripted session held to the hand at the game's pace, the exam waived,
/// as the order desk's session is: ten shares bought for the firm; the switch
/// clicked, your own pocket, and its cash buying as many as it covers; the
/// switch by C and by the palette; the switch clicked beside buy-1's key, the
/// switch alone; a Buy 1 clicked beside C, the order alone, funded by the
/// pocket shown; the switch clicked beside a pick's key, the pick dropped; the
/// switch clicked and C in one frame, one switch; C and a click at the
/// switch's address while the Portfolio shows, neither a switch; C at 9:40, a
/// tick; and the palette's switch at 16:02, the market closed, the 1,172nd
/// frame at twenty seconds a frame in session (`POT.11`). At every
/// switch's frame and at the frame after it, every balance is the last
/// frame's (`BD-9`): a switch turns which pocket an order funds and the views
/// show, the views following at its own frame, and moves no money.
#[test]
fn a_switch_moves_no_balance_and_the_views_follow_it() {
    use Act::{Key, Left, Point, Run, Show};
    let script: &[(u64, &[Act])] = &[
        (1, &[Show("Trading")]),
        (3, &[Show("Trading")]), // the Camera showed the Portfolio at the begin
        (4, &[Point(6, 7), Left(true)]), // Buy 10 of the second, the firm's
        (5, &[Left(false)]),
        (6, &[Point(4, 11), Left(true)]), // the switch clicked: your own
        (7, &[Left(false)]),
        (8, &[Key("KeyM", true)]), // buy-max of the second from your own cash
        (9, &[Key("KeyM", false)]),
        (10, &[Key("KeyC", true)]), // the switch by its key: the firm's
        (11, &[Key("KeyC", false)]),
        (12, &[Run("switch-cash")]), // the palette's: your own
        (14, &[Point(4, 11), Left(true), Key("KeyB", true)]), // the switch clicked beside buy-1: the switch alone, the firm's
        (15, &[Left(false), Key("KeyB", false)]),
        (16, &[Point(5, 6), Left(true), Key("KeyC", true)]), // Buy 1 of the first beside C: the order alone, the firm's
        (17, &[Left(false), Key("KeyC", false)]),
        (18, &[Point(4, 11), Left(true), Key("Digit3", true)]), // the switch beside a pick: the switch alone, your own
        (19, &[Left(false), Key("Digit3", false)]),
        (20, &[Point(4, 11), Left(true), Key("KeyC", true)]), // the switch clicked and C: one switch, the firm's
        (21, &[Left(false), Key("KeyC", false)]),
        (22, &[Show("Portfolio")]),
        (23, &[Key("KeyC", true)]), // C on the Portfolio: no command holds there
        (24, &[Key("KeyC", false)]),
        (25, &[Left(true)]), // a click at the switch's address on the Portfolio: no switch
        (26, &[Left(false), Show("Trading")]),
        (31, &[Key("KeyC", true)]), // 9:40, a tick: your own
        (32, &[Key("KeyC", false)]),
        (1_172, &[Run("switch-cash")]), // 16:02, the market closed: the firm's
    ];
    let mut cart = cartridge();
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    let mut host = Host::new(buffet::EXAM);
    let mut plate = Plate::default();
    let starts = [WAIVED_CAPITAL, buffet::PERSONAL_CASH];
    use buffet::{FIRM, PERSONAL};
    // The switches the script makes, by frame, each the pocket after it.
    let switches = [
        (6, PERSONAL),
        (10, FIRM),
        (12, PERSONAL),
        (14, FIRM),
        (18, PERSONAL),
        (20, FIRM),
        (31, PERSONAL),
        (1_172, FIRM),
    ];
    let mut acts = script.iter().peekable();
    let (mut switched, mut orders) = (Vec::new(), Vec::new());
    let mut before = balances(&cart);
    let mut worth = values(&cart, "Cash", "B6:B7");
    for frame in 1..=1_174 {
        let mut set: Vec<Act> = Vec::new();
        while let Some((_, more)) = acts.next_if(|(at, _)| *at == frame) {
            set.extend_from_slice(more);
        }
        let shown = shown(&cal, seed, &market, &plate, starts);
        let (last, now) = host.step(&mut cart, &set);
        // BD-9 first, from the cells alone and before the hand holds the frame:
        // at a switch's frame and the frame after, every balance the last frame's.
        let now_balances = balances(&cart);
        if switches
            .iter()
            .any(|&(at, _)| frame == at || frame == at + 1)
        {
            assert!(
                now_balances == before,
                "frame {frame}: a switch moved a balance\n{now_balances:?}\nwhere the last frame left\n{before:?}"
            );
        }
        let now_worth = values(&cart, "Cash", "B6:B7");
        if frame == 31 {
            assert_ne!(
                now_worth, worth,
                "the tick at 9:40 moves the holdings' value, a valuation, beside the switch"
            );
        }
        before = now_balances;
        worth = now_worth;
        let was = plate.pocket;
        let order = plate_frame_held(
            &cart,
            &host,
            (&now, &last),
            &mut plate,
            &mut cal,
            (seed, &market),
            &shown,
            starts,
            &texts,
        );
        if let Some(o) = order {
            orders.push((frame, o.pocket, o.stock, o.outcome));
        }
        if plate.pocket != was {
            switched.push((frame, plate.pocket));
        }
    }
    assert!(acts.next().is_none(), "every act of the script played");
    assert_eq!(
        switched, switches,
        "the switches, by frame, each its pocket after"
    );
    use buffet::Outcome::Filled;
    assert_eq!(
        orders,
        [
            (4, FIRM, 2, Filled),
            (8, PERSONAL, 2, Filled),
            (16, FIRM, 1, Filled)
        ],
        "the orders taken, by frame, each its pocket"
    );
    assert_eq!(plate.picked, 1, "the pick beside the switch dropped");
    assert_eq!(host.now.sheet, buffet::TRADING);
}

/// The Report (`CRITIC.1`): at its first frames, the exam waived and the Report
/// shown as the host shows it, before any card, every cell the hand's, the
/// begin's worth side by side and the card's figures empty, and the Performance
/// sheet's books, its settings and its grades; while it shows, no key the Keys
/// sheet binds and no command of the cartridge's own holding, the Report having
/// no act of its own (keyboard-first: no act, no key); its columns as wide as
/// its labels, its dollars and its percents; every firm's name and Average
/// Co.'s line inside its first column; and the Remarks sheet, a remark a grade,
/// five, each one line inside its column.
#[test]
fn the_report_states_its_lines_and_binds_no_key() {
    let mut cart = cartridge();
    let seed = seed(&cart);
    let texts = disclosure_texts(&cart);
    let mut host = Host::new(buffet::EXAM);
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    let mut cards = Cards::default();
    for frame in 1..=3u64 {
        host.step(&mut cart, &[Act::Show(buffet::CARD_VIEW)]);
        cal.tick();
        let stocks = values(&cart, "Stocks", "E5:I8");
        let price = prices_of(&stocks)[buffet::BENCHMARK];
        assert!(!cards.frame(&cal, frame > 1, WAIVED_CAPITAL, price, WAIVED_CAPITAL));
        performance_held(&cart, &cal, &cards, price, WAIVED_CAPITAL);
        report_cells(&cart, &cal, &cards, (seed, &texts), buffet::WAIVED_STYLE);
        if frame > 1 {
            let bound: Vec<&String> = host.keys.values().filter(|c| !c.is_empty()).collect();
            assert!(
                bound.is_empty(),
                "frame {frame}: keys bound while the Report shows: {bound:?}"
            );
            assert!(
                host.commands.iter().all(|(_, holds, _)| !holds),
                "frame {frame}: a command holds while the Report shows"
            );
        }
    }
    assert_eq!(host.now.sheet, buffet::CARD_VIEW);
    assert_eq!((cards.shares, cards.left), (300, 0), "300 shares at $25.00");
    let text = lf(&read(CARTRIDGE));
    for (column, width) in [
        ("A", 30),
        ("B", buffet::DOLLARS_COLUMN),
        ("C", buffet::DOLLARS_COLUMN),
        ("D", buffet::DOLLARS_COLUMN),
    ] {
        let row = format!("(column \"Report\" \"{column}\" {width} shown none)");
        assert!(text.lines().any(|l| l == row), "{row}");
    }
    let line = format!("{}, bought and held", RUNGS[buffet::BENCHMARK].name);
    let names = firm_rows().iter().map(|(_, n)| n.as_str());
    for name in names.chain([line.as_str(), buffet::ALPHA_LABEL]) {
        let n = name.chars().count();
        assert!(n <= 30, "{name:?}: {n} characters, past Report!A's 30");
    }
    let file = content(REMARKS);
    assert_eq!(file.fields, [("Remark".to_string(), 100)]);
    let remarks = remark_texts();
    assert_eq!(
        remarks.len(),
        buffet::GRADES.len() + 1,
        "a remark a grade, A to D and F"
    );
    for (k, r) in remarks.iter().enumerate() {
        let n = r.chars().count() as u64;
        assert!(
            !r.trim().is_empty() && !r.contains('\n') && n <= file.fields[0].1,
            "remark {}: {n} characters, past its column's {}",
            k + 1,
            file.fields[0].1
        );
    }
    // A capital the begin's price does not divide: the capital before the exam
    // written $5,001.00 as a person's edit, the waiver's five making it
    // $7,501.00, so 300 whole shares of Average Co. at $25.00 and $1.00 left
    // over, held as cash, the two worth the capital side by side.
    let mut cart = loaded();
    for edit in [
        "(cell \"Proctor\" \"B1\" 500100)",
        "(cell \"Proctor\" \"B3\" 1)",
    ] {
        cart.write(edit)
            .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
    }
    let mut cards = Cards::default();
    let mut cal = Calendar::new(PACE, DAYS_A_YEAR);
    for frame in 1..=2u64 {
        assert!(cart.step(0).done);
        cal.tick();
        cards.frame(&cal, frame > 1, 750_100, 2_500, 750_100);
        performance_held(&cart, &cal, &cards, 2_500, 750_100);
    }
    assert_eq!(
        (cards.shares, cards.left, cards.begin),
        (300, 100, [750_100; 2])
    );
}

/// The Camera at every card (`CRITIC.1`): at a frame a day through the first
/// year and its turn, the exam waived, the host showing the Overview at the
/// game's first frame, where the certificate leads, and then the sheet each
/// tab gives it; at each quarter's turn, the Camera's `B3` the Report and the
/// host showing it at the frame's end, the second card as the first, its cells
/// the hand's; at every other frame after the first, `B3` the empty text, and
/// the person's tab kept until the next card. Every frame the order desk's and
/// the views' too: the firm in cash for three quarters, then Average Co.
/// bought with all of it on Trading at the fourth's second day, so that the
/// year's card grades the quarter C and the year F, and the proctor remarks
/// on the year's.
#[test]
fn the_camera_shows_every_card_and_the_tabs_win_between() {
    use Act::{Left, Point, Show};
    let (mut cart, mut cal) = at_pace(A_DAY_LONG);
    let seed = seed(&cart);
    let texts = disclosure_texts(&cart);
    let market = Market::new(seed);
    let mut host = Host::new(buffet::EXAM);
    let mut plate = Plate::default();
    let mut cards = Cards::default();
    let starts = [WAIVED_CAPITAL, buffet::PERSONAL_CASH];
    let mut shown_at: Vec<u64> = Vec::new();
    let tabs = ["Portfolio", "Gigs", buffet::OVERVIEW, buffet::TRADING];
    let mut tab: Option<&str> = None;
    for frame in 1..=370u64 {
        let mut set: Vec<Act> = match tab {
            Some(t) if host.now.sheet == buffet::CARD_VIEW => vec![Show(t)],
            _ => Vec::new(),
        };
        match frame {
            276 => set.extend([Point(7, 8), Left(true)]), // Buy max of Average Co., the firm's
            277 => set.push(Left(false)),
            _ => {}
        }
        let shown = shown(&cal, seed, &market, &plate, starts);
        let (last, now) = host.step(&mut cart, &set);
        plate_frame_held(
            &cart,
            &host,
            (&now, &last),
            &mut plate,
            &mut cal,
            (seed, &market),
            &shown,
            starts,
            &texts,
        );
        let index = market.path(seed, DIFFICULTY, &cal).now;
        let prices: [u64; 4] = std::array::from_fn(|k| RUNGS[k].price_at(index));
        let firm = plate.cash(buffet::FIRM, WAIVED_CAPITAL) + plate.holdings(buffet::FIRM, prices);
        let price = prices[buffet::BENCHMARK];
        let card = cards.frame(&cal, frame > 1, WAIVED_CAPITAL, price, firm);
        performance_held(&cart, &cal, &cards, price, firm);
        let camera = values(&cart, "Camera", "B3")
            .remove("B3")
            .unwrap_or_default();
        let want = if frame == 1 {
            buffet::LEAD
        } else if card {
            buffet::CARD_VIEW
        } else {
            ""
        };
        assert_eq!(camera, want, "frame {frame}: Camera!B3");
        if card {
            assert_eq!(
                host.now.sheet,
                buffet::CARD_VIEW,
                "frame {frame}: the card shown"
            );
            report_cells(&cart, &cal, &cards, (seed, &texts), buffet::WAIVED_STYLE);
            shown_at.push(frame);
            tab = Some(tabs[shown_at.len() % tabs.len()]);
        } else if let Some(t) = tab {
            if frame > shown_at.last().copied().unwrap_or(0) + 1 {
                assert_eq!(host.now.sheet, t, "frame {frame}: the person's tab kept");
            }
        } else if frame > 1 {
            assert_eq!(host.now.sheet, buffet::LEAD, "frame {frame}: the lead kept");
        }
    }
    assert_eq!(shown_at, [92, 183, 274, 366], "a card shown at each turn");
    assert_eq!(
        plate
            .fills
            .iter()
            .map(|f| (f.pocket, f.stock))
            .collect::<Vec<_>>(),
        [(buffet::FIRM, 3)],
        "Average Co. bought once, the firm's"
    );
    let year = cards.card().expect("the year's card");
    assert_eq!(
        (
            year.quarter,
            year.grades[0].1,
            year.grades[1].1,
            year.remark
        ),
        (4, "C", "F", 5),
        "the fourth quarter in line with the market, the year well behind, the remark the year's"
    );
}

/// A quarter scripted (`CRITIC.1`), every frame held to the hand: the order
/// desk, the books, the cash, the ledger, the commands, the views, the
/// Performance sheet and the Report, the exam waived. Day 0 and day 1 at a
/// frame an hour: ten shares of Longshot Mining and ten of Average Co. bought
/// for the firm, the switch by C, a share of Average Co. bought for your own,
/// and the firm's again by C; then a frame a day from day 2 to day 87, both
/// paces written 86,400 as a person's edits, and a frame an hour again from
/// day 87, written 3,600: the firm's Longshot Mining sold on Friday, day 88; a
/// prospectus claimed on Sunday, day 90, its fee paid a day later, in the next
/// quarter; the quarter's turn at Monday's first frame, the card issued and the
/// Report shown, the firm's book graded and your own pocket's orders and fee
/// moving none of its figures; and a share of General Widget bought for the
/// firm on Monday, the next quarter's.
#[test]
fn a_scripted_quarter_holds_every_card_to_the_hand() {
    use Act::{Key, Left, Point, Show};
    // Each act at its day and its hour, the frame landing on the half hour; a
    // pace written before a frame moves the clock from the frame after it.
    let script: &[(u64, u64, &[Act])] = &[
        (0, 9, &[Show("Trading")]),
        (0, 10, &[Show("Trading")]),
        (0, 11, &[Point(6, 6), Left(true)]), // Buy 10 of Longshot Mining, the firm's
        (0, 12, &[Left(false)]),
        (0, 13, &[Point(6, 8), Left(true)]), // Buy 10 of Average Co., the firm's
        (0, 14, &[Left(false)]),
        (0, 15, &[Key("KeyC", true)]), // the switch: your own
        (0, 16, &[Key("KeyC", false)]),
        (1, 10, &[Point(5, 8), Left(true)]), // Buy 1 of Average Co., your own
        (1, 11, &[Left(false)]),
        (1, 12, &[Key("KeyC", true)]), // the switch: the firm's again
        (1, 13, &[Key("KeyC", false)]),
        (88, 14, &[Point(10, 6), Left(true)]), // Sell all of Longshot Mining, the firm's, on Friday
        (88, 15, &[Left(false)]),
        (90, 18, &[Show("Gigs")]),
        (90, 20, &[Key("KeyP", true)]), // a prospectus claimed on Sunday, paid on Monday
        (90, 21, &[Key("KeyP", false)]),
        (91, 9, &[Show("Trading")]),
        (91, 10, &[Point(5, 9), Left(true)]), // Buy 1 of General Widget, the firm's, the next quarter's
        (91, 11, &[Left(false)]),
    ];
    let paces: &[(u64, u64, u64)] = &[(1, 14, A_DAY_LONG), (87, 14, AN_HOUR)];
    let (mut cart, mut cal) = at_pace(AN_HOUR);
    let seed = seed(&cart);
    let market = Market::new(seed);
    let texts = disclosure_texts(&cart);
    let mut host = Host::new(buffet::EXAM);
    let mut plate = Plate::default();
    let mut hustle = Hustle::default();
    let mut cards = Cards::default();
    let mut played = 0;
    let mut orders = Vec::new();
    let mut issued = Vec::new();
    loop {
        let mut next = cal;
        next.tick();
        if (next.day, next.hour) > (92, 1) {
            break;
        }
        let at = (next.day, next.hour);
        let lines: Vec<&(u64, u64, &[Act])> =
            script.iter().filter(|(d, h, _)| (*d, *h) == at).collect();
        played += lines.len();
        let set: Vec<Act> = lines
            .iter()
            .flat_map(|(_, _, acts)| acts.iter().copied())
            .collect();
        let pace = paces.iter().find(|(d, h, _)| (*d, *h) == at).map(|p| p.2);
        if let Some(seconds) = pace {
            for edit in pace_edits(Pace::every(seconds)) {
                cart.write(&edit)
                    .unwrap_or_else(|r| panic!("{edit}: {} {}", r.refusal.id, r.refusal.text));
            }
        }
        let starts_last = [
            WAIVED_CAPITAL,
            buffet::PERSONAL_CASH + hustle.earned(cal.five_minutes()),
        ];
        let starts = [
            WAIVED_CAPITAL,
            buffet::PERSONAL_CASH + hustle.earned(next.five_minutes()),
        ];
        let shown = shown(&cal, seed, &market, &plate, starts_last);
        let (day_shown, seen) = (cal.day, cal.shown());
        let (last, now) = host.step(&mut cart, &set);
        hustle.frame(&now, &last, day_shown, seen);
        let order = plate_frame_held(
            &cart,
            &host,
            (&now, &last),
            &mut plate,
            &mut cal,
            (seed, &market),
            &shown,
            starts,
            &texts,
        );
        if let Some(seconds) = pace {
            cal.pace = Pace::every(seconds);
        }
        if let Some(o) = order {
            orders.push(((cal.day, cal.hour), o.pocket, o.stock, o.outcome));
        }
        let now_index = market.path(seed, DIFFICULTY, &cal).now;
        let prices: [u64; 4] = std::array::from_fn(|k| RUNGS[k].price_at(now_index));
        let firm = plate.cash(buffet::FIRM, WAIVED_CAPITAL) + plate.holdings(buffet::FIRM, prices);
        let price = prices[buffet::BENCHMARK];
        if cards.frame(&cal, cal.frame > 1, WAIVED_CAPITAL, price, firm) {
            issued.push((cal.day, cal.hour, cards.card().expect("a card")));
            assert_eq!(host.now.sheet, buffet::CARD_VIEW, "the card shown");
        }
        performance_held(&cart, &cal, &cards, price, firm);
        report_cells(&cart, &cal, &cards, (seed, &texts), buffet::WAIVED_STYLE);
        assert_eq!(
            values(&cart, "Hustle", "B18").get("B18"),
            Some(&hustle.earned(cal.five_minutes()).to_string()),
            "frame {}: Hustle!B18, the fees",
            cal.frame
        );
    }
    assert_eq!(played, script.len(), "every act of the script played");
    use buffet::Outcome::Filled;
    use buffet::{FIRM, PERSONAL};
    assert_eq!(
        orders,
        [
            ((0, 11), FIRM, 1, Filled),
            ((0, 13), FIRM, 3, Filled),
            ((1, 10), PERSONAL, 3, Filled),
            ((88, 14), FIRM, 1, Filled),
            ((91, 10), FIRM, 4, Filled),
        ],
        "the orders taken, by day and hour, each its pocket"
    );
    let claimed: Vec<u64> = hustle.claims.iter().map(|&(d, _)| d).collect();
    assert_eq!(claimed, [90], "the prospectus claimed on day 90");
    assert_eq!(issued.len(), 1, "one card, the first quarter's");
    let (day, hour, card) = issued[0];
    assert_eq!((day, hour, card.quarter, card.year), (91, 0, 1, 1));
    assert_eq!(
        card.quarter_start,
        [WAIVED_CAPITAL, WAIVED_CAPITAL],
        "the begin's worth, side by side"
    );
    assert_eq!(
        card.returns[0], card.returns[1],
        "the first quarter, the year so far"
    );
    assert!(
        card.returns[0][0] != 0 && card.alpha[0] == card.returns[0][0] - card.returns[0][1],
        "the firm's book graded: {card:?}"
    );
    assert_eq!(
        hustle.earned(cal.five_minutes()),
        buffet::FEE,
        "the fee paid in the next quarter"
    );
}
