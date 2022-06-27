use crate::book::NUMBOOK;
use crate::keys::{KEYS, NUMKEYS};

use std::io::{self, stdin, BufRead, Write};
pub fn get_lines(poem: usize) -> Vec<Vec<u16>> {
    print!("Lines from poem {}: ", poem + 1);
    io::stdout()
        .flush()
        .expect("There was an error flushing the console buffer.");
    let mut line_input = String::new();
    let stdin = stdin();
    stdin
        .lock()
        .read_line(&mut line_input)
        .expect("There was an error reading the line.");
    let valid = valid_lines(line_input, poem);

    return match valid {
        Ok(valid) => valid,
        Err(error) => {
            println!("The lines were formatted incorrectly: {}", error);
            return get_lines(poem);
        }
    };
}

pub fn get_keys() -> (Vec<Vec<u16>>, Vec<u8>) {
    print!("Key: ");
    io::stdout()
        .flush()
        .expect("There was an error flushing the console buffer.");
    let mut keys_input = String::new();
    let stdin = stdin();
    stdin
        .lock()
        .read_line(&mut keys_input)
        .expect("There was an error reading the line.");
    let keys_input_str = keys_input.trim_end();
    let keys: Vec<&str> = keys_input_str.split_whitespace().collect();
    let mut keys_as_vecs = Vec::new();
    let mut offsets = Vec::new();
    if keys_input_str == "L" {
        let mut i = 1;
        println!("");
        for key in KEYS {
            println!("Key {}: {} L{} (Offset {})", i, key.0, key.1, key.1);
            i += 1;
        }
        println!("");
        return get_keys();
    } else if keys_input_str == "" {
        keys_as_vecs.push(Vec::new());
    } else {
        for key in keys {
            let as_int = key.parse::<u16>();
            if as_int.is_err() {
                println!(
                    "The keys were formatted incorrectly: {} is not a valid number.\n",
                    key
                );
                return get_keys();
            } else {
                let key_int = as_int.unwrap();
                if key_int < 1 || key_int > 16 {
                    println!(
                        "The keys were formatted incorrectly: {} is not between 1 and 16.\n",
                        key
                    );
                    return get_keys();
                } else {
                    keys_as_vecs.push(NUMKEYS[key_int as usize - 1].0.to_vec());
                    offsets.push(NUMKEYS[key_int as usize - 1].1);
                }
            }
        }
    }
    println!("");
    (keys_as_vecs, offsets)
}

fn valid_lines(lines_input: String, poem: usize) -> Result<Vec<Vec<u16>>, &'static str> {
    let lines_input_str = lines_input.trim_end();
    let lines: Vec<&str> = lines_input_str.split_whitespace().collect();
    let mut lines_as_vecs = Vec::new();
    if lines_input_str == "" {
        lines_as_vecs.push(vec![]);
    } else {
        for line in lines {
            let as_int = line.parse::<u16>();
            if as_int.is_err() {
                return Err("not all lines were integers.");
            } else {
                let line_int = as_int.unwrap();
                if line_int < 1 || line_int > 16 {
                    return Err("line number out of range. Must be 1 - 16.");
                } else {
                    lines_as_vecs.push(NUMBOOK[poem][line_int as usize - 1].to_vec());
                }
            }
        }
    }
    Ok(lines_as_vecs)
}
