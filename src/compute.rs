use crate::input::{get_keys, get_lines, get_mode};
use crate::logger;
use crate::microcosm::message_from_lines_and_key;
use colour::e_red_ln;
use itertools::Itertools;
use std::sync::{atomic, atomic::AtomicUsize, Arc, Mutex};
use std::{thread, time};
type CombinationsTested = Arc<AtomicUsize>;
use rustc_hash::FxHashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

type WordList = Arc<FxHashMap<String, bool>>;

pub fn start_manual() {
    println!("\nPlease enter the lines you would like to test\nfor each poem. Use a space between each line\nnumber (e.g. 1 2 6 10) or just press enter if\nyou have no educated guesses.\n");
    let mut combinations: Vec<Vec<&[u16]>> = Vec::with_capacity(14);
    for i in 0..13 {
        combinations.push(get_lines(i));
    }
    println!("\nPlease enter the keys you would like to test for.\nUse a space between each number, or enter L for a\nlist of possible keys, or just press enter if you\nhave no educated guesses.\n");
    let keys = get_keys();
    combinations.push(keys);
    let mode = get_mode();
    if mode == "Y" {
        begin_compute(combinations, true);
    } else if mode == "N" {
        begin_compute(combinations, false);
    } else {
        println!("Running with and without nemesizer shift, one after the other.");
        begin_compute(combinations.clone(), true);
        begin_compute(combinations, false);
    }
}

pub fn start_random() {
    println!("Feature not yet added. Sorry.");
}

fn get_complexity(combinations: &Vec<Vec<&[u16]>>) -> u64 {
    let mut complexity = 1u64;
    for i in 0..14 {
        complexity *= combinations[i].len() as u64;
    }
    complexity
}

fn begin_compute(combinations: Vec<Vec<&'static [u16]>>, use_offset: bool) {
    let complexity = get_complexity(&combinations);
    if use_offset {
        println!("\nStarting with nemesizer shift.")
    } else {
        println!("\nStarting without nemesizer shift.")
    }
    println!("Total combinations: {}", complexity);
    let file = File::open("words.txt");
    if file.is_err() {
        e_red_ln!("File words.txt was not found, terminating.");
        process::exit(1);
    }
    let reader = BufReader::new(file.unwrap());
    let mut words_raw = FxHashMap::default();

    for line in reader.lines() {
        words_raw.insert(line.unwrap().trim_end().to_string(), true);
    }
    let words: WordList = Arc::new(words_raw);
    println!("Loaded word list successfully.");
    let num_cpus = num_cpus::get();
    let mut split_on = 0;
    let mut num_threads = 1;
    for i in 0..combinations.len() {
        if num_threads == 1 && combinations[split_on].len() > num_cpus {
            split_on = i;
        }
        if combinations[i].len() % num_cpus == 0 {
            split_on = i;
            num_threads = num_cpus;
            break;
        } else if combinations[i].len() >= combinations[split_on].len()
            && combinations[i].len() < num_cpus
        {
            num_threads = combinations[i].len();
            split_on = i;
        }
    }
    println!(
        "Found {} cpus, splitting computation over {} threads...",
        num_cpus, num_threads
    );
    let split_combinations = get_split_combinations(combinations, split_on, num_threads, num_cpus);
    /*
    for combg in &split_combinations {
        println!("Outer comb length: {}", combg.len());
        for comb in combg {
            println!("Inner length: {}", comb.len());
        }
    }
    */
    let terminator = Arc::new(Mutex::new(false));
    let combinations_tested = Arc::new(AtomicUsize::new(0));

    let log_combinations = combinations_tested.clone();
    let terminated = terminator.clone();

    let logger = thread::spawn(move || {
        logger::threaded_logger(log_combinations, terminated, complexity);
    });
    let mut tracker = vec![];
    for i in 0..num_threads {
        let combinations = split_combinations[i].clone();
        let combinations_tested_handle = combinations_tested.clone();
        let words_handle = words.clone();
        let new_compute;
        if use_offset {
            new_compute = thread::spawn(|| {
                return compute_offset(combinations, combinations_tested_handle, words_handle);
            });
        } else {
            new_compute = thread::spawn(|| {
                return compute(combinations, combinations_tested_handle, words_handle);
            });
        }
        tracker.push(new_compute);
    }
    let start_time = time::Instant::now();
    for handle in tracker {
        let _ = handle.join().unwrap_or_else(|error| {
            e_red_ln!("Error ending combination worker thread: {:?}", error);
        });
    }
    let runtime = start_time.elapsed();
    let time_ran = runtime.as_secs() as f64 + runtime.subsec_millis() as f64 / 1000.0;
    *terminator.lock().unwrap() = true;
    let _ = logger.join().unwrap_or_else(|error| {
        e_red_ln!("Error ending logger thread: {:?}", error);
    });
    thread::sleep(time::Duration::from_millis(1500));
    let tested = combinations_tested.load(atomic::Ordering::Relaxed);
    eprintln!(
        "\nTested: {} combinations in {:.2} seconds.\nAverage rate: {:.0} combinations per second.",
        tested,
        time_ran,
        tested as f64 / time_ran
    );
}

fn get_split_combinations(
    combinations: Vec<Vec<&'static [u16]>>,
    split_on: usize,
    threads: usize,
    num_cpus: usize,
) -> Vec<Vec<Vec<&'static [u16]>>> {
    let mut split_combinations: Vec<Vec<Vec<&[u16]>>> = vec![];
    for _ in 0..threads {
        split_combinations.push(vec![]);
    }
    let mut denom = 1;
    if threads >= num_cpus {
        denom = combinations[split_on].len() / num_cpus;
    }
    for i in 0..14 {
        if i == split_on {
            for j in 0..threads {
                split_combinations[j].push(combinations[i][j * denom..j * denom + denom].to_vec());
            }
        } else {
            for j in 0..threads {
                split_combinations[j].push(combinations[i].clone());
            }
        }
    }
    split_combinations
}

fn compute(
    mut combinations: Vec<Vec<&[u16]>>,
    combinations_tested: CombinationsTested,
    words: WordList,
) {
    for i in 0..combinations[13].len() {
        combinations[13][i] = &combinations[13][i][..20];
    }
    let comb_generator = combinations
        .iter()
        .map(|x| x.iter())
        .multi_cartesian_product();
    let mut lines_and_key: [&[u16]; 14] = [&[]; 14];
    for comb in comb_generator {
        for i in 0..14 {
            lines_and_key[i] = comb[i];
        }
        message_from_lines_and_key(lines_and_key, &words);
        combinations_tested.fetch_add(1, atomic::Ordering::Relaxed);
    }
}

fn compute_offset(
    combinations: Vec<Vec<&[u16]>>,
    combinations_tested: CombinationsTested,
    words: WordList,
) {
    let comb_generator = combinations
        .iter()
        .map(|x| x.iter())
        .multi_cartesian_product();

    let mut offset;
    let mut lines_and_key: [&[u16]; 14] = [&[]; 14];
    for mut comb in comb_generator {
        offset = comb[13][20] as usize - 1;
        let key = &comb[13][..20];
        comb[13] = &key;
        for i in 0..14 - offset {
            lines_and_key[i] = comb[offset + i];
        }
        for i in 0..offset {
            lines_and_key[14 - offset + i] = comb[i];
        }
        /* DEBUG
        println!("OLD (shift {})", offset + 1);
        for line in comb {
            println!("{:?}", line);
        }
        println!("NEW");
        for line in lines_and_key {
            println!("{:?}", line);
        }
        */
        message_from_lines_and_key(lines_and_key, &words);
        combinations_tested.fetch_add(1, atomic::Ordering::Relaxed);
    }
}
