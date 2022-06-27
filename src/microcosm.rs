static ALPHABET: [&str; 26] = [
    " ", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
    "s", "t", "u", "v", "w", "x", "y",
];

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

fn string_from_totals(totals: &[u16; 20]) -> String {
    let mut message = String::with_capacity(20);
    for i in 0..20 {
        message.push_str(ALPHABET[(totals[i] % 26) as usize]);
    }
    message
}

pub fn message_from_lines_and_key(lines: [&[u16]; 13], key: &[u16], offset: u8) -> String {
    let mut lines_and_key: [&[u16]; 14] = [&[]; 14];
    if offset != 1 {
    } else {
        for i in 0..13 {
            lines_and_key[i] = lines[i];
        }
        lines_and_key[13] = key;
    }
    let totals = totals_from_lines(lines_and_key);
    string_from_totals(&totals)
}
