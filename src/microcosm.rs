use crate::data::PRECALC_LETTERS;
use rustc_hash::FxHashMap;
type WordList = Arc<FxHashMap<String, bool>>;
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
    println!("\nAll words were english: {}", message);
    true
}
