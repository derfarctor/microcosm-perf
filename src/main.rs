pub mod book;
pub mod data;
pub mod input;
pub mod keys;
mod compute;
mod microcosm;

use data::LOGO;
use microcosm::message_from_lines_and_key;
use std::io::{self, stdin, BufRead, Write};
use compute::{start_manual, start_random};
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
        start_manual();
    } else if choice.trim_end() == "2" {
        start_random();
    }
}

