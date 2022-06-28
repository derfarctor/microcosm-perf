use crate::input::{get_keys, get_lines, get_mode};
pub fn start_manual() {
    println!("\nPlease enter the lines you would like to test\nfor each poem. Use a space between each line\nnumber (e.g. 1 2 6 10) or just press enter if\nyou have no educated guesses.\n");
    let mut combinations: Vec<Vec<Vec<u16>>> = Vec::with_capacity(14);
    for i in 0..13 {
        combinations.push(get_lines(i));
    }
    println!("\nPlease enter the keys you would like to test for.\nUse a space between each number, or enter L for a\nlist of possible keys, or just press enter if you\nhave no educated guesses.\n");
    let keys = get_keys();
    combinations.push(keys);
    let mode = get_mode();
    let mut complexity = get_complexity(&combinations);
    if mode == "B" {
        complexity *= 2;
    }
    println!("\nTotal combinations: {}", complexity);
}

pub fn start_random() {}

fn get_complexity(combinations: &Vec<Vec<Vec<u16>>>) -> u64 {
    let mut complexity = 1u64;
    for i in 0..14 {
        complexity *= combinations[i].len() as u64;
    }
    complexity
}

fn begin_compute(combinations: &Vec<Vec<Vec<u16>>>) {
    let mut cpus = num_cpus::get();
    let mut perfect_split = false;
    let mut split_on = 0;
    
    for i in 0..combinations.len() {
        if cpus % combinations[i].len() == 0 {
            split_on = i;
            break;
        } else if combinations[i].len() > combinations[split_on].len() {
            split_on = i;
        }
    }
    let split_combinations = get_split_combinations(combinations, split_on);

}

fn get_split_combinations(combinations: Vec<Vec<Vec<u16>>>, split_on: usize) -> Vec<Vec<Vec<Vec<u16>>>> {
    let mut split_combinations: Vec<Vec<Vec<Vec<u16>>>> = vec![];
    for _ in 0..combinations[split_on].len() {
            split_combinations.push(vec![]);
    }
    for i in 0..14 {
        if i == split_on {
            for j in 0..combinations[i].len() {
                let part: Vec<_> = combinations[i][j];
                split_combinations[i][j].push(part);
            }
        }

            for j in 0..threads {
                    let part: Vec<_> = possibilities[i][j*denom..j*denom+denom].iter().cloned().collect();
                    split_possibilites[j].push(part);
            }
    } else {
            for j in 0..threads {
                    let part: Vec<_> = possibilities[i].iter().cloned().collect();
                    split_possibilites[j].push(part);
            }
    }
    }

}