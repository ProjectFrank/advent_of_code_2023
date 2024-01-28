use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const DIGIT_WORDS: [(&str, u32); 10] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
];

fn find_digit(line: &[char], idx: usize) -> Option<u32> {
    let char = line.get(idx)?;
    let opt = char.to_digit(10);
    if opt.is_some() {
        return opt;
    }

    for (word, digit) in DIGIT_WORDS {
        let word_chars: Vec<char> = word.chars().collect();
        let len = word_chars.len();
        let end = idx + len;
        if let Some(line_segment) = line.get(idx..end) {
            if line_segment == &word_chars[..] {
                return Some(digit);
            }
        };
    }

    return None;
}

fn main() -> Result<(), Error> {
    let path = "input";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut codes: Vec<u32> = Vec::new();

    for l in buffered.lines() {
        println!("looking at line: {}", l.as_ref().expect("ahh"));
        let mut digits = Vec::new();
        if let Ok(line) = l {
            let chars: Vec<char> = line.chars().collect();
            for i in 0..chars.len() {
                if let Some(d) = find_digit(&chars[..], i) {
                    digits.push(d)
                }
            }

            let first_digit = digits.first().expect("no digits found in line");
            let last_digit = digits.last().expect("no digits found in line");
            let code = first_digit * 10 + last_digit;
            println!(
                "first: {}\nlast: {}\ncode: {}",
                first_digit, last_digit, code
            );

            codes.push(code);
        }
    }

    let sum: u32 = codes.into_iter().sum();

    println!("answer is {}", sum);

    Ok(())
}
