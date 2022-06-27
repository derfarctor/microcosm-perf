mod book;
mod keys;
mod microcosm;

use book::{BOOK, NUMBOOK};
use keys::{KEYS, NUMKEYS};
use microcosm::message_from_lines_and_key;

fn main() {
    let mut lines: [&[u16]; 13] = [&[]; 13];
    let key: &[u16] = NUMKEYS[10].0;
    let line_idxs = [7, 11, 8, 3, 11, 3, 2, 3, 0, 11, 3, 11, 14];
    for i in 0..13 {
        lines[i] = NUMBOOK[i][line_idxs[i]];
        println!("{}", BOOK[i][line_idxs[i]]);
    }
    println!("{}", KEYS[10].0);
    let message = message_from_lines_and_key(lines, key, 1);
    println!("{}", message);
}
