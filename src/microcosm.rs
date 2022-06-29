use crate::book::NUMBOOK;
use crate::data::PRECALC_LETTERS;
use crate::keys::NUMKEYS;
use rustc_hash::FxHashMap;
type WordList = Arc<FxHashMap<String, bool>>;
use chrono;

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;

fn totals_from_lines(lines_and_key: [&[u16]; 14]) -> [u16; 20] {
    let mut totals = [0u16; 20];
    let mut n = 0;
    for i in 0..14 {
        for letter in lines_and_key[i] {
            totals[n] += letter;
            if n == 19 {
                n = 0;
            } else {
                n += 1;
            }
        }
    }
    totals
}
/*
pub fn message_from_lines_and_key(lines_and_key: [&[u16]; 14], words: &WordList) -> bool {
    let totals = totals_from_lines(lines_and_key);
    let mut message = String::with_capacity(20);
    for i in 0..20 {
        if PRECALC_LETTERS[totals[i] as usize] == " " || i == 19 {
            if !words.contains_key(&message) {
                return false;
            }
            message = String::with_capacity(20);
        } else {
            message.push_str(PRECALC_LETTERS[totals[i] as usize]);
        }
    }
    // At this stage the message was fully english words, so generate it again and print the information
    message = String::with_capacity(20);
    for i in 0..20 {
        message.push_str(PRECALC_LETTERS[totals[i] as usize]);
    }
    println!("\nAll words were english: {}", message);
    true
}
*/

pub fn message_from_lines_and_key(lines_and_key: [&[u16]; 14], words: &WordList) -> bool {
    let totals = totals_from_lines(lines_and_key);
    let mut message = String::with_capacity(20);
    let mut last = 0;
    for i in 0..20 {
        message.push_str(PRECALC_LETTERS[totals[i] as usize]);
        if PRECALC_LETTERS[totals[i] as usize] == " " || i == 19 {
            if !words.contains_key(&message[last..i]) {
                return false;
            }
            last = i + 1;
        }
    }
    // At this stage the message was fully english words, so generate it again and print the information
    println!("\n\nFound text, written to found.txt: {}\n", message);
    let f;
    if Path::new("found.txt").exists() {
        f = OpenOptions::new()
            .write(true)
            .append(true)
            .open("found.txt");
    } else {
        File::create("found.txt").expect("Failed to create found.txt");
        f = OpenOptions::new().write(true).open("found.txt");
    }
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open found.txt: {}", error);
        }
    };
    let mut f = BufWriter::new(f);
    write!(
        f,
        "\nAt: {}\nText: {}\nLines and key dump: {:?}\n",
        chrono::offset::Local::now(),
        message,
        lines_and_key
    )
    .expect("Failed to write to found.txt");
    true
}

fn get_readable_lines_and_key(lines_and_key: [&[u16]; 14]) -> String {
    for i in 0..14 {
        let matching_idx = 0;
        let mut page = 0;
        let matching_page = 0;
        let is_key = false;
        for j in 0..16 {
            let mut is_line_match = true;
            let is_key_match = true;
            for k in 0..lines_and_key[i].len() {
                for l in 0..13 {
                    if NUMBOOK[l][j][k] != lines_and_key[i][k] {
                        is_line_match = false;
                        break;
                    }
                    page = l
                }
                if is_line_match {
                    matching_idx = break;
                }
                if NUMKEYS[j][k] != lines_and_key[i][k] {}
            }
        }
    }
    String::from("UNUSED/UNFINISHED FUNCTION")
}
