// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Spreadsheet Company

//! The owner's session as the page records it, and what the oracle writes
//! from it (`docs/SPIKE.md`, `HOUSE.4`'s second slice). A replay is the page's
//! own text (Alonzo's `docs/SPEC.md` section 7.6): a header naming the
//! cartridge by the SHA-256 of its LF text and the seed, then a form for each
//! frame that has rows, and the last frame always, a frame's rows the Input
//! sheet's cells that changed and each edit of a person's that landed. The
//! page plays it on a fresh host and holds its end to the golden, its own
//! recording to the replay byte for byte, and every frame to the hand.
//!
//! What is here: the digest, the oracle's own (FIPS 180-4), since Alonzo's is
//! its tests' alone; the replay read into its frames and restamped for a
//! cartridge changed; a player that plays a replay over the engine as the
//! host plays it, the derived writes the last step's Write sheet made first,
//! each its own write, then the frame's rows as one, then the step; the
//! golden, the `grid` record of the Report's window at the last frame, the
//! record the page's `viewRecord` asks the module for; and the hand, a
//! script the page runs, holding the cells of the session the native test
//! holds to `buffet.rs` (`oracle/tests/cartridge.rs`), at the frames where a
//! key, a click or a command landed and the frame after, at each day's first
//! frame and at every 512th, each value as the record spells it, so that the
//! module in the browser is held to the native engine where the language's
//! `SIN` and `^` differ by platform (decision 8), and no rule is written a
//! third time (decision 2).

use std::collections::BTreeMap;

use alonzo::cartridge::Cartridge;
use vla_lang::form::Form;
use vla_lang::machine::Viewed;
use vla_lang::reader::read_forms;

use crate::{derived, lf};

/// The cartridge, and the three files the session leaves beside it in `src/`
/// (decision 4): the replay, its golden and the hand.
pub const CARTRIDGE: &str = "src/buffet.vla";
pub const REPLAY: &str = "src/buffet.replay.vla";
pub const GOLDEN: &str = "src/buffet.golden.vla";
pub const HAND: &str = "src/buffet.hand.js";

/// The golden's window: the Report, the first quarter's card, its bound
/// card's extent (`FRONT.8`) from `A1` to its far corner.
pub const GOLDEN_WINDOW: (&str, &str) = ("Report", "A1:E17");

/// Between the frames a key, a click, a command or a day's turn lands at, the
/// hand holds every 512th frame (decision 2).
pub const HAND_EVERY: u64 = 512;

/// The cells the hand holds, a sheet and its cells (decision 2): the clock
/// shown and the day, the four prices, each pocket's cash, holdings and net
/// worth, the positions' shares and cost, the last order and the ledger's row
/// it wrote, the exam's score, capital, style and begin, the hustle's fees,
/// and the card: every one a whole number of cents, shares or hundredths of
/// a percent, a count or a word, never a value of the market unrounded
/// (decision 8).
pub const HAND_CELLS: &[(&str, &[&str])] = &[
    ("Day", &["B3", "B6", "B7", "B16", "B19"]),
    ("Stocks", &["E5", "E6", "E7", "E8"]),
    ("Cash", &["B3", "B4", "B6", "B7", "B9", "B10"]),
    (
        "Positions",
        &[
            "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9", "F2", "F3", "F4", "F5", "F6", "F7",
            "F8", "F9",
        ],
    ),
    (
        "Orders",
        &[
            "B30", "B31", "B32", "B33", "B34", "B35", "B36", "B37", "B38", "B39", "B40", "B41",
        ],
    ),
    ("Proctor", &["B38", "B39", "B40", "B41"]),
    ("Hustle", &["B18"]),
    (
        "Performance",
        &[
            "B4", "B5", "B8", "B17", "B18", "B19", "B20", "B21", "B22", "C17", "C18", "C19", "C20",
            "C21", "C22",
        ],
    ),
];

// ---- the digest ----------------------------------------------------------

/// The first 32 bits of the fractional parts of the cube roots of the first
/// 64 primes (FIPS 180-4, 4.2.2).
const ROUND: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// The first 32 bits of the fractional parts of the square roots of the first
/// eight primes (FIPS 180-4, 5.3.3).
const START: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// The SHA-256 of `bytes` (FIPS 180-4): the message padded with a one bit,
/// zeros and its length in bits to a whole number of 64-byte blocks, each
/// block's 64 words scheduled and compressed into the eight words of state.
pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut message = bytes.to_vec();
    let bits = (bytes.len() as u64).wrapping_mul(8);
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bits.to_be_bytes());
    let mut state = START;
    let mut w = [0u32; 64];
    for block in message.chunks_exact(64) {
        for (t, word) in block.chunks_exact(4).enumerate() {
            w[t] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16]
                .wrapping_add(s0)
                .wrapping_add(w[t - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for t in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ (!e & g);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add(choose)
                .wrapping_add(ROUND[t])
                .wrapping_add(w[t]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (word, more) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *word = word.wrapping_add(more);
        }
    }
    let mut out = [0u8; 32];
    for (k, word) in state.iter().enumerate() {
        out[k * 4..k * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// The SHA-256 of `bytes` as a replay's header writes it: 64 hex digits,
/// upper case.
pub fn sha256_hex(bytes: &[u8]) -> String {
    sha256(bytes).iter().map(|b| format!("{b:02X}")).collect()
}

/// The digest a replay names its cartridge by: the SHA-256 of the
/// cartridge's text with CRLF read as LF, as the repository stores it.
pub fn cartridge_digest(text: &str) -> String {
    sha256_hex(lf(text).as_bytes())
}

// ---- the replay's text ---------------------------------------------------

/// A replay read: the comment lines before its header, kept as written; the
/// header's name, version, cartridge digest and seed; and each frame the
/// page logged, by its number, with its rows as the recorder wrote them, the
/// last frame always among them.
#[derive(Clone, Debug, PartialEq)]
pub struct Replay {
    pub comments: Vec<String>,
    pub name: String,
    pub spec: u64,
    pub cartridge: String,
    pub seed: u64,
    pub frames: Vec<(u64, Vec<String>)>,
}

/// How far a row's parentheses leave it open, its texts' parentheses aside:
/// 0 for a row whole, -1 for a row that also closes its frame.
fn depth(row: &str) -> i64 {
    let mut depth = 0i64;
    let mut text = false;
    let mut escaped = false;
    for c in row.chars() {
        if text {
            match (escaped, c) {
                (true, _) => escaped = false,
                (false, '\\') => escaped = true,
                (false, '"') => text = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => text = true,
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
    }
    depth
}

/// A number of a header or a frame, or why it is none.
fn number(text: &str, what: &str) -> Result<u64, String> {
    text.trim()
        .parse()
        .map_err(|_| format!("the replay's {what} is not a whole number: {text:?}"))
}

impl Replay {
    /// A replay's text read, CRLF read as LF: its comment lines, its header,
    /// `(replay "<name>" (spec N) (cartridge "<64 hex>") (seed S))`, and its
    /// frames, `(frame N)` with no rows or `(frame N` with a row a line, the
    /// last closing it.
    pub fn read(text: &str) -> Result<Replay, String> {
        let text = lf(text);
        let mut lines = text.lines();
        let mut comments = Vec::new();
        let header = loop {
            match lines.next() {
                Some(line) if line.starts_with(';') => comments.push(line.to_string()),
                Some(line) => break line,
                None => return Err("the replay has no header".to_string()),
            }
        };
        let rest = header
            .strip_prefix("(replay \"")
            .ok_or_else(|| format!("the replay's header is no (replay ...): {header:?}"))?;
        let (name, rest) = rest
            .split_once("\" (spec ")
            .ok_or_else(|| format!("the replay's header names no version: {header:?}"))?;
        let (spec, rest) = rest
            .split_once(") (cartridge \"")
            .ok_or_else(|| format!("the replay's header names no cartridge: {header:?}"))?;
        let (cartridge, rest) = rest
            .split_once("\") (seed ")
            .ok_or_else(|| format!("the replay's header names no seed: {header:?}"))?;
        let seed = rest
            .strip_suffix("))")
            .ok_or_else(|| format!("the replay's header does not end at its seed: {header:?}"))?;
        if cartridge.len() != 64 || !cartridge.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!(
                "the replay's cartridge is no SHA-256: {cartridge:?}"
            ));
        }
        let mut frames: Vec<(u64, Vec<String>)> = Vec::new();
        let mut open: Option<(u64, Vec<String>)> = None;
        for line in lines {
            if let Some((_, rows)) = open.as_mut() {
                let row = line.trim_start();
                match depth(row) {
                    0 => rows.push(row.to_string()),
                    -1 => {
                        rows.push(row[..row.len() - 1].to_string());
                        frames.extend(open.take());
                    }
                    _ => return Err(format!("a row of the replay is not one form: {row:?}")),
                }
                continue;
            }
            if line.is_empty() {
                continue;
            }
            let rest = line
                .strip_prefix("(frame ")
                .ok_or_else(|| format!("a line of the replay is outside a frame: {line:?}"))?;
            match rest.strip_suffix(')') {
                Some(n) => frames.push((number(n, "frame")?, Vec::new())),
                None => open = Some((number(rest, "frame")?, Vec::new())),
            }
        }
        if let Some((n, _)) = open {
            return Err(format!("the replay's frame {n} never closes"));
        }
        if frames.windows(2).any(|w| w[0].0 >= w[1].0) {
            return Err("the replay's frames are out of order".to_string());
        }
        Ok(Replay {
            comments,
            name: name.to_string(),
            spec: number(spec, "version")?,
            cartridge: cartridge.to_string(),
            seed: number(seed, "seed")?,
            frames,
        })
    }

    /// The replay's last frame, its length.
    pub fn last(&self) -> u64 {
        self.frames.last().map_or(0, |(n, _)| *n)
    }

    /// The replay's header as the recorder writes it.
    pub fn header(&self) -> String {
        format!(
            "(replay \"{}\" (spec {}) (cartridge \"{}\") (seed {}))",
            self.name, self.spec, self.cartridge, self.seed
        )
    }

    /// The replay after its comment lines, LF, as the recorder writes it and
    /// as the page's round compares its own recording to.
    pub fn body(&self) -> String {
        let mut out = self.header();
        out.push('\n');
        for (n, rows) in &self.frames {
            if rows.is_empty() {
                out.push_str(&format!("(frame {n})\n"));
                continue;
            }
            out.push_str(&format!("(frame {n}"));
            for row in rows {
                out.push_str("\n  ");
                out.push_str(row);
            }
            out.push_str(")\n");
        }
        out
    }

    /// The whole text, its comment lines first, LF.
    pub fn text(&self) -> String {
        let mut out = String::new();
        for line in &self.comments {
            out.push_str(line);
            out.push('\n');
        }
        out.push_str(&self.body());
        out
    }

    /// Every frame from 1 to the last, each with its rows, none where the
    /// replay logged none.
    pub fn every_frame(&self) -> impl Iterator<Item = (u64, &[String])> + '_ {
        let mut logged = self.frames.iter().peekable();
        (1..=self.last()).map(move |n| match logged.next_if(|(at, _)| *at == n) {
            Some((_, rows)) => (n, rows.as_slice()),
            None => (n, &[][..]),
        })
    }
}

/// A row of the Input sheet, its address and its value as the row spells
/// it, or none for a row of another sheet, a person's edit.
pub fn input_cell(row: &str) -> Option<(&str, &str)> {
    let rest = row.strip_prefix("(cell \"Input\" \"")?;
    let (addr, value) = rest.split_once("\" ")?;
    Some((addr, value.strip_suffix(')')?))
}

/// Whether a key, a click, a command or an edit landed in a frame's rows: a
/// button of the Input sheet's first ten rows written 1, the left button
/// written down (`B13`), a key that went down (`B17`, a text not empty), a
/// command run (`B20`, a text not empty), or a row of another sheet, an edit.
/// A pointer's move alone is none.
pub fn acted(rows: &[String]) -> bool {
    rows.iter().any(|row| match input_cell(row) {
        None => true,
        Some(("B17" | "B20", value)) => value != "\"\"",
        Some(("B13", value)) => value == "1",
        Some((addr, value)) => {
            let row_of = addr[1..].parse::<u32>().unwrap_or(0);
            (1..=10).contains(&row_of) && value == "1"
        }
    })
}

// ---- the player ----------------------------------------------------------

/// A replay played over the engine as the host plays it (Alonzo's `host.js`,
/// a replay's frames): the cartridge loaded with the replay's seed, which a
/// cartridge whose manifest names its seed must hold already, as the page
/// refuses a replay holding another; then each frame the derived writes the
/// last step's Write sheet asked for, each its own write, then the frame's
/// rows, one write, then the step.
pub struct Player {
    pub cart: Cartridge,
    pending: Vec<String>,
    pub frame: u64,
}

impl Player {
    /// The cartridge's text loaded under its file's name for a replay of
    /// `seed`, or why not.
    pub fn new(text: &str, name: &str, seed: u64) -> Result<Player, String> {
        let cart = Cartridge::load(text, name, text.len())
            .map_err(|r| format!("line {}: {} {}", r.line, r.refusal.id, r.refusal.text))?;
        let own = crate::values(&cart, "Clock", "B3")
            .remove("B3")
            .unwrap_or_default();
        if own != seed.to_string() {
            return Err(format!(
                "the replay's seed is {seed}, and the cartridge's own, Clock!B3, {own}"
            ));
        }
        Ok(Player {
            cart,
            pending: Vec::new(),
            frame: 0,
        })
    }

    /// One frame: the derived writes, the frame's rows, the step, and the
    /// Write sheet's rows read for the next.
    pub fn frame(&mut self, rows: &[String]) -> Result<(), String> {
        self.frame += 1;
        let frame = self.frame;
        for row in std::mem::take(&mut self.pending) {
            self.cart.write(&row).map_err(|r| {
                format!("frame {frame}: {row}: {} {}", r.refusal.id, r.refusal.text)
            })?;
        }
        if !rows.is_empty() {
            self.cart.write(&rows.join("\n")).map_err(|r| {
                format!(
                    "frame {frame}: its rows: {} {}",
                    r.refusal.id, r.refusal.text
                )
            })?;
        }
        let step = self.cart.step(0);
        if !step.done {
            return Err(format!("frame {frame}: {}", step.row()));
        }
        self.pending = derived(&self.cart);
        Ok(())
    }
}

// ---- what a record holds -------------------------------------------------

/// A cell's value as a `grid` record holds it, as the page's reader reads it:
/// a number by the text the record spells it with, a text, a truth, or an
/// error by its name.
#[derive(Clone, Debug, PartialEq)]
pub enum Held {
    Number(String),
    Text(String),
    Truth(bool),
    Error(String),
}

/// Whether a symbol is a number as the page's reader reads one (`web/viewport.js`,
/// `readForms`): `-?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?`.
fn is_number(s: &str) -> bool {
    let s = s.strip_prefix('-').unwrap_or(s);
    let (mantissa, exponent) = match s.find(['e', 'E']) {
        Some(at) => (&s[..at], Some(&s[at + 1..])),
        None => (s, None),
    };
    let digits = |t: &str| !t.is_empty() && t.chars().all(|c| c.is_ascii_digit());
    let fine = match mantissa.split_once('.') {
        None => digits(mantissa),
        Some((whole, part)) => {
            (digits(whole) && (part.is_empty() || digits(part)))
                || (whole.is_empty() && digits(part))
        }
    };
    let exponent_fine = exponent.map_or(true, |e| {
        let e = e.strip_prefix(['+', '-']).unwrap_or(e);
        digits(e)
    });
    fine && exponent_fine
}

/// Each cell's value a `grid` record of `sheet` holds, by address: its
/// constants' `cell` rows and its formulas' `value` rows, as the page's
/// `recordValues` reads them.
pub fn record_values(record: &str, sheet: &str) -> Result<BTreeMap<String, Held>, String> {
    let forms = read_forms(record).map_err(|r| format!("the record of {sheet}: {}", r.text))?;
    let mut out = BTreeMap::new();
    for form in &forms {
        let Some(list) = form.as_list() else { continue };
        if list.count() != 4 || !(form.head_is("cell") || form.head_is("value")) {
            continue;
        }
        let (Form::Str(at_sheet), Form::Str(addr)) = (&list.items[1], &list.items[2]) else {
            continue;
        };
        if at_sheet != sheet {
            continue;
        }
        let held = match &list.items[3] {
            Form::Str(t) => Held::Text(t.clone()),
            Form::Sym(s) if s == "true" => Held::Truth(true),
            Form::Sym(s) if s == "false" => Held::Truth(false),
            Form::Sym(s) if is_number(s) => Held::Number(s.clone()),
            Form::List(e) if form_is_error(&list.items[3]) && e.count() == 2 => match &e.items[1] {
                Form::Str(name) => Held::Error(name.clone()),
                other => return Err(format!("{sheet}!{addr}: an error named by {other:?}")),
            },
            other => {
                return Err(format!(
                    "{sheet}!{addr} holds what no reader reads: {other:?}"
                ))
            }
        };
        out.insert(addr.to_ascii_uppercase(), held);
    }
    Ok(out)
}

fn form_is_error(form: &Form) -> bool {
    form.head_is("error")
}

/// The `grid` record of a sheet's window, as the page's `viewRecord` asks the
/// module for it.
pub fn grid_record(cart: &Cartridge, sheet: &str, window: &str) -> Result<String, String> {
    match cart.view("grid", sheet, Some(window)) {
        Ok(Viewed::Text(record)) => Ok(record),
        Ok(_) => Err(format!("the view of {sheet} {window} is no text")),
        Err(r) => Err(format!("the view of {sheet} {window}: {} {}", r.id, r.text)),
    }
}

/// The column's letters and the row of an address.
fn split_addr(addr: &str) -> (String, u32) {
    let at = addr
        .find(|c: char| c.is_ascii_digit())
        .unwrap_or(addr.len());
    (addr[..at].to_string(), addr[at..].parse().unwrap_or(0))
}

/// A column's number from its letters, A being 1.
fn column_number(letters: &str) -> u32 {
    letters
        .bytes()
        .fold(0, |n, b| n * 26 + u32::from(b - b'A' + 1))
}

/// The smallest range holding every address, as the page's `handHeld` views a
/// sheet over the range its cells span.
fn span(addrs: &[&str]) -> String {
    let cells: Vec<(String, u32)> = addrs.iter().map(|a| split_addr(a)).collect();
    let left = cells
        .iter()
        .min_by_key(|(c, _)| column_number(c))
        .map(|c| c.0.clone());
    let right = cells
        .iter()
        .max_by_key(|(c, _)| column_number(c))
        .map(|c| c.0.clone());
    let top = cells.iter().map(|c| c.1).min().unwrap_or(1);
    let bottom = cells.iter().map(|c| c.1).max().unwrap_or(1);
    format!(
        "{}{top}:{}{bottom}",
        left.unwrap_or_default(),
        right.unwrap_or_default()
    )
}

/// The hand's cells at this frame, in `HAND_CELLS`' order, each as the
/// record holds it, or none where the cell holds nothing.
pub fn hand_values(cart: &Cartridge) -> Result<Vec<Option<Held>>, String> {
    let mut out = Vec::new();
    for (sheet, addrs) in HAND_CELLS {
        let record = grid_record(cart, sheet, &span(addrs))?;
        let values = record_values(&record, sheet)?;
        out.extend(addrs.iter().map(|a| values.get(*a).cloned()));
    }
    Ok(out)
}

/// The hand's cells by name, `Sheet!A1`, in `HAND_CELLS`' order.
pub fn hand_names() -> Vec<String> {
    HAND_CELLS
        .iter()
        .flat_map(|(sheet, addrs)| addrs.iter().map(move |a| format!("{sheet}!{a}")))
        .collect()
}

// ---- what the session writes ---------------------------------------------

/// A text as a script's literal: in double quotes, every character outside
/// printable ASCII, a quote, a backslash and `<` escaped by its code, so
/// that no text a cell holds can end the hand's block or spell a token the
/// offline check refuses.
fn script_text(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        let plain = (' '..='~').contains(&c) && !matches!(c, '"' | '\\' | '<');
        if plain {
            out.push(c);
            continue;
        }
        let mut units = [0u16; 2];
        for unit in c.encode_utf16(&mut units) {
            out.push_str(&format!("{}u{:04x}", '\\', unit));
        }
    }
    out.push('"');
    out
}

/// A held value as the script's literal the page's `sndPlain` compares it to:
/// a number by its record's own spelling, which the page's reader reads by
/// `Number`, a text, a truth, an error as `{ error }`, and nothing as `null`.
fn script_value(held: &Option<Held>) -> String {
    match held {
        None => "null".to_string(),
        Some(Held::Number(text)) => text.clone(),
        Some(Held::Text(t)) => script_text(t),
        Some(Held::Truth(b)) => b.to_string(),
        Some(Held::Error(name)) => format!("{{ error: {} }}", script_text(name)),
    }
}

/// What the session writes: the golden, and the hand's frames, each the
/// frame's number and the hand's cells at it.
pub struct Session {
    pub golden: String,
    pub held: Vec<(u64, Vec<Option<Held>>)>,
    pub last: u64,
    pub seed: u64,
}

/// The session played over the cartridge: the hand's cells read at every
/// frame a key, a click, a command or an edit landed, at the frame after,
/// at each day's first frame (the Day sheet's `B3` moved, the first frame
/// among them) and at every 512th; and the golden's window at the last
/// frame.
pub fn play_session(replay: &Replay, cartridge: &str) -> Result<Session, String> {
    let name = alonzo::pass::file_name(CARTRIDGE);
    let mut player = Player::new(cartridge, name, replay.seed)?;
    let mut held = Vec::new();
    let mut acted_last = false;
    let mut day = String::new();
    for (n, rows) in replay.every_frame() {
        player.frame(rows)?;
        let acted_now = acted(rows);
        let today = crate::values(&player.cart, "Day", "B3")
            .remove("B3")
            .unwrap_or_default();
        let turned = today != day;
        day = today;
        if acted_now || acted_last || turned || n % HAND_EVERY == 0 {
            held.push((n, hand_values(&player.cart)?));
        }
        acted_last = acted_now;
    }
    let (sheet, window) = GOLDEN_WINDOW;
    let golden = grid_record(&player.cart, sheet, window)?;
    let extent = format!("(extent \"{sheet}\" \"{window}\")");
    if !golden.lines().any(|l| l == extent) {
        return Err(format!(
            "the golden's window, {sheet}!{window}, is not the {sheet} sheet's extent at the last frame: its record says no {extent}"
        ));
    }
    Ok(Session {
        golden,
        held,
        last: replay.last(),
        seed: replay.seed,
    })
}

/// The golden's file: its SPDX line and what it is, then the record.
pub fn golden_text(session: &Session) -> String {
    let (sheet, window) = GOLDEN_WINDOW;
    format!(
        "; SPDX-License-Identifier: AGPL-3.0-or-later AND CC-BY-SA-4.0\n\
         ; The {sheet} sheet's window, {window}, at the last frame of src/buffet.replay.vla, frame {},\n\
         ; as the page's host records it (HOUSE.4's second slice): written by oracle/examples/replay.rs\n\
         ; (cargo run --example replay) and never by hand.\n{}",
        session.last, session.golden
    )
}

/// The hand's file: a constructor registered as Buffet's in
/// `window.AlonzoHands`, made with the replay's seed, which answers each frame
/// the page asks with the cells it holds there, none at a frame it does not
/// hold; its table the cells' names once, then each frame it holds, its
/// number and the cells that moved since the frame held before, each by its
/// place in the names and its value.
pub fn hand_text(session: &Session) -> String {
    let names = hand_names();
    let mut out = String::new();
    out.push_str(
        "// SPDX-License-Identifier: AGPL-3.0-or-later\n\
         // SPDX-FileCopyrightText: 2026 Spreadsheet Company\n\
         //\n\
         // src/buffet.hand.js - Buffet's hand in the page (HOUSE.4's second slice), written by\n\
         // oracle/examples/replay.rs (cargo run --example replay) and never by hand. A hand (Alonzo's\n\
         // ENGINE.16): the builder writes this file into the page as the block hand-buffet, a script the\n\
         // page runs where it stands, and it registers one constructor under the cartridge's name in\n\
         // window.AlonzoHands. The loop oracle's replay case answers it each frame of the owner's session,\n\
         // src/buffet.replay.vla, and holds each cell it names to the module's record at that frame. Its\n\
         // cells are those the native test holds to the hand written in Rust, oracle/src/buffet.rs, at\n\
         // the same frames (oracle/tests/cartridge.rs, the_owners_session_holds_every_frame_to_the_hand):\n\
         // the clock shown and the day, the prices, each pocket's books, the positions, the last order, the\n\
         // exam, the hustle's fees and the card, each a whole number, a word or a truth, at every frame a\n\
         // key, a click or a command landed and the frame after, at each day's first frame and at every\n\
         // 512th. So the module in the browser is held to the native engine where the language's SIN and ^\n\
         // differ by platform, and no rule of the game is written a third time.\n\
         (function (hands) {\n  'use strict';\n",
    );
    out.push_str(&format!(
        "  var SEED = {}, LAST = {};\n",
        session.seed, session.last
    ));
    out.push_str("  // The cells, each Sheet!A1, a row of the table's places.\n  var CELLS = [\n");
    for chunk in names.chunks(8) {
        let line: Vec<String> = chunk.iter().map(|n| script_text(n)).collect();
        out.push_str(&format!("    {},\n", line.join(", ")));
    }
    out.push_str("  ];\n");
    out.push_str(
        "  // Each frame held: its number, then each cell that moved since the frame held before, its place in\n  // CELLS and its value, every cell at the first.\n  var HELD = [\n",
    );
    let mut before: Vec<Option<Held>> = vec![None; names.len()];
    let mut first = true;
    for (n, values) in &session.held {
        let mut row = vec![n.to_string()];
        for (k, v) in values.iter().enumerate() {
            if first || before[k] != *v {
                row.push(k.to_string());
                row.push(script_value(v));
            }
        }
        before.clone_from(values);
        first = false;
        out.push_str(&format!("    [{}],\n", row.join(", ")));
    }
    out.push_str("  ];\n");
    out.push_str(
        "  function BuffetHand(seed) {\n\
         \x20   if (seed !== SEED) throw new Error('the hand was written for the seed ' + SEED + ', and the replay names ' + seed);\n\
         \x20   this.at = 0; this.held = 0; this.now = [];\n\
         \x20   for (var k = 0; k < CELLS.length; k++) this.now.push(null);\n\
         \x20 }\n\
         \x20 // A frame's answer: at a frame held, its cells that moved taken, then every cell; at any other, none.\n\
         \x20 BuffetHand.prototype.frame = function (n) {\n\
         \x20   var row = HELD[this.at], cells = {};\n\
         \x20   if (row && row[0] < n) throw new Error('the hand holds frame ' + row[0] + ', and the page asked for frame ' + n + ' next');\n\
         \x20   if (n > LAST) throw new Error('the hand ends at frame ' + LAST + ', and the page asked for frame ' + n);\n\
         \x20   if (!row || row[0] !== n) return { cells: cells };\n\
         \x20   this.at++;\n\
         \x20   for (var i = 1; i < row.length; i += 2) this.now[row[i]] = row[i + 1];\n\
         \x20   for (var c = 0; c < CELLS.length; c++) cells[CELLS[c]] = this.now[c];\n\
         \x20   this.held++;\n\
         \x20   return { cells: cells };\n\
         \x20 };\n\
         \x20 // The frames it held: buffetHeld, beside the page's buffetFrames and buffetCells.\n\
         \x20 BuffetHand.prototype.counts = function () { return { held: this.held }; };\n\
         \x20 hands.buffet = BuffetHand;\n\
         })(window.AlonzoHands = window.AlonzoHands || {});\n",
    );
    out
}

/// The replay's file with its header naming `digest`: its comment lines and
/// its frames as they stand.
pub fn restamped(replay: &Replay, digest: &str) -> Replay {
    Replay {
        cartridge: digest.to_string(),
        ..replay.clone()
    }
}

/// A text as the working tree holds a `.vla`: CRLF (`.gitattributes`).
pub fn crlf(text: &str) -> String {
    lf(text).replace('\n', "\r\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FIPS 180-2's vectors, as the page's replay case holds its own digest
    /// to them: the empty message, "abc", the two-block message and a
    /// million a's.
    #[test]
    fn the_digest_is_the_standards() {
        let vectors = [
            (
                String::new(),
                "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
            ),
            (
                "abc".to_string(),
                "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD",
            ),
            (
                "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_string(),
                "248D6A61D20638B8E5C026930C3E6039A33CE45964FF2167F6ECEDD419DB06C1",
            ),
            (
                "a".repeat(1_000_000),
                "CDC76E5C9914FB9281A1C7E284D73E67F1809A48A497200E046D39CCC7112CD0",
            ),
        ];
        for (message, want) in vectors {
            assert_eq!(
                sha256_hex(message.as_bytes()),
                want,
                "{} bytes",
                message.len()
            );
        }
    }

    /// A replay read and written back is its text, a frame with rows, one
    /// without, a row whose text holds a parenthesis and the last frame; and
    /// the replays the page refuses are refused here.
    #[test]
    fn a_replay_reads_and_writes_back_as_it_was() {
        let text = concat!(
            "; SPDX-License-Identifier: AGPL-3.0-or-later AND CC-BY-SA-4.0\n",
            "(replay \"buffet\" (spec 6) (cartridge \"72705440AD4BC2946D08D331CD40AAC9963F2C05B14937E43C900B8DBF1508AE\") (seed 791886))\n",
            "(frame 1\n",
            "  (cell \"Input\" \"B1\" 0)\n",
            "  (cell \"Input\" \"B17\" \")\"))\n",
            "(frame 4\n",
            "  (cell \"Input\" \"B19\" \"Report\"))\n",
            "(frame 9)\n",
        );
        let replay = Replay::read(text).expect("reads");
        assert_eq!(replay.text(), text);
        assert_eq!(replay.last(), 9);
        assert_eq!(replay.seed, 791_886);
        assert_eq!(replay.frames[0].1[1], "(cell \"Input\" \"B17\" \")\")");
        let every: Vec<(u64, usize)> = replay.every_frame().map(|(n, r)| (n, r.len())).collect();
        assert_eq!(
            every,
            [
                (1, 2),
                (2, 0),
                (3, 0),
                (4, 1),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 0)
            ]
        );
        assert!(acted(&replay.frames[0].1), "a key that went down, ')'");
        assert!(!acted(&replay.frames[1].1), "a sheet shown is no act");
        for (bad, why) in [
            (text.replace("(frame 4\n", "(frame 1\n"), "out of order"),
            (text.replace("(frame 9)\n", "(frame 9\n"), "never closes"),
            (
                text.replace("(seed 791886)", "(seed x)"),
                "not a whole number",
            ),
            (text.replace("72705440", "727054"), "no SHA-256"),
        ] {
            let err = Replay::read(&bad).expect_err(why);
            assert!(err.contains(why), "{why}: {err}");
        }
    }

    /// The page's reader's numbers, and the texts a script's literal escapes.
    #[test]
    fn a_record_s_values_spell_as_the_page_reads_them() {
        for yes in ["0", "-5", "12345", "1.5", ".5", "5.", "1E+308", "-2.5e-3"] {
            assert!(is_number(yes), "{yes}");
        }
        for no in ["", "-", "e5", "1e", "1.2.3", "TRUE", "0x1"] {
            assert!(!is_number(no), "{no}");
        }
        assert_eq!(
            script_text("a\"b\\c</script>▲"),
            "\"a\\u0022b\\u005cc\\u003c/script>\\u25b2\""
        );
    }
}
