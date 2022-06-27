pub mod book;
pub mod data;
mod input;
pub mod keys;
mod microcosm;

use data::LOGO;
use input::{get_keys, get_lines};
use microcosm::message_from_lines_and_key;
use std::io::{self, stdin, BufRead, Write};
const OPTS: [&str; 2] = ["1", "2"];

fn main() {
    print!("{}", LOGO);
    println!("This tool is designed to help solve Microcosm. \nYou can either enter manually which lines and keys you'd\nlike to try combinations of, or let the computer try randomly.\n\nAdditionally, if manual mode is chosen, functionality for\nnemesizer's theorised shift based upon L[X] at the end\nof keys can be enabled and will apply automatically.\n");
    print!("Mode (1 for manual, 2 for random): ");
    io::stdout()
        .flush()
        .expect("There was an error flushing the console buffer.");

    let mut choice = String::new();
    let stdin = stdin();
    stdin
        .lock()
        .read_line(&mut choice)
        .expect("There was an error reading the line.");
    while !OPTS.contains(&choice.trim_end()) {
        println!("The mode entered was invalid.\n");
        print!("Mode (1 for manual, 2 for random): ");
        io::stdout()
            .flush()
            .expect("There was an error flushing the console buffer.");
        let mut new_choice = String::new();
        stdin
            .lock()
            .read_line(&mut new_choice)
            .expect("There was an error reading the line.");
        choice = new_choice;
    }

    if choice.trim_end() == "1" {
        compute_manual();
    } else if choice.trim_end() == "2" {
        compute_random();
    }
}

fn compute_manual() {
    println!("\nPlease enter the lines you would like to test\nfor each poem. Use a space between each line\nnumber (e.g. 1 2 6 10) or just press enter if\nyou have no educated guesses.\n");
    let mut lines: Vec<Vec<Vec<u16>>> = Vec::with_capacity(13);
    for i in 0..13 {
        lines.push(get_lines(i));
    }
    println!("\nPlease enter the keys you would like to test for.\nUse a space between each number, or enter L for a\nlist of possible keys, or just press enter if you\nhave no educated guesses.\n");
    let (keys, offsets) = get_keys();
    let mut lines_slice = [[0u16].as_slice(); 13];
    for i in 0..13 {
        lines_slice[i] = lines[i][0].as_slice();
    }
    let message = message_from_lines_and_key(lines_slice, &keys[0], offsets[0]);
    println!("{}", message);
}

fn compute_random() {}
