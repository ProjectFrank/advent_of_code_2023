use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn pt1(path: &str) -> Result<u32, io::Error> {
    let file = File::open(path)?;

    let buffered = BufReader::new(file);

    // create an iterator over the lines of the file
    let lines_iter = buffered.lines().filter_map(|line_result| line_result.ok());

    let mut total_score: u32 = 0;

    for line in lines_iter {
        if let Some(card) = Card::from_line(&line) {
            total_score += card.points();
        };
    }

    return Ok(total_score);
}

pub fn pt2(path: &str) -> Result<u32, io::Error> {
    let file = File::open(path)?;

    let buffered = BufReader::new(file);

    // create an iterator over the lines of the file
    let lines_iter = buffered.lines().filter_map(|line_result| line_result.ok());

    let mut card_frequencies: HashMap<u32, u32> = HashMap::new();

    // the largest card number, assumes card numbers go up line-by-line
    let mut last_card_number = 0;

    for line in lines_iter {
        if let Some(card) = Card::from_line(&line) {
            // increment frequency of the current card
            let num_cards = card_frequencies.get(&card.number).unwrap_or(&0) + 1;

            card_frequencies.insert(card.number, num_cards);

            let num_winning = card.num_winning();

            // increase frequency of cards following the current card
            // by the frequency of the current card
            for n in (card.number + 1)..(card.number + 1 + num_winning) {
                let count = card_frequencies.entry(n).or_insert(0);
                *count += num_cards;
            }

            last_card_number = card.number;
        };
    }

    let mut num_cards = 0;

    // find the total number of cards
    for (k, v) in card_frequencies {
        // exclude card numbers that don't actually exist
        if k <= last_card_number {
            num_cards += v;
        }
    }

    return Ok(num_cards);
}

fn chars_to_numbers(chars: &str, digits: &mut Vec<u32>) -> Vec<u32> {
    // make sure digits buffer is clear
    digits.clear();
    let mut numbers = Vec::new();

    for c in chars.chars() {
        // if c is a digit, add to digits buffer
        if let Some(d) = c.to_digit(10) {
            digits.push(d);
        } else if digits.len() > 0 {
            // otherwise, flush the buffer
            numbers.push(to_num(&digits));
            digits.clear();
        }
    }

    if digits.len() > 0 {
        numbers.push(to_num(&digits));
    }

    numbers
}

fn to_num(digits: &[u32]) -> u32 {
    (0..digits.len())
        .rev()
        .zip(digits.iter())
        .map(|(power, digit)| 10_u32.pow(u32::try_from(power).expect("error")) * digit)
        .sum()
}

#[derive(PartialEq, Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
    /// the card number
    number: u32,
}

impl Card {
    fn from_line(line: &str) -> Option<Self> {
        let mut splits = line.split(':');

        // parse out the card number
        let card_label = splits.next()?;
        let mut card_label_digits: Vec<u32> = Vec::new();

        for c in card_label.chars() {
            if let Some(d) = c.to_digit(10) {
                card_label_digits.push(d);
            }
        }

        let mut card = Self {
            winning_numbers: HashSet::new(),
            numbers: Vec::new(),
            number: to_num(&card_label_digits),
        };

        let mut x = splits.next()?.split('|');
        let winning_chars = x.next()?;

        let mut digits: Vec<u32> = Vec::new();

        card.winning_numbers =
            HashSet::from_iter(chars_to_numbers(winning_chars, &mut digits).into_iter());

        let num_chars = x.next()?;

        card.numbers = chars_to_numbers(num_chars, &mut digits);

        return Some(card);
    }

    /// returns number of winning numbers on the card
    fn num_winning(&self) -> u32 {
        let mut num_winning = 0;

        for n in self.numbers.iter() {
            if self.winning_numbers.contains(&n) {
                num_winning += 1;
            }
        }
        return num_winning;
    }

    fn points(&self) -> u32 {
        // track count of winning cards
        let num_winning = self.num_winning();

        if num_winning == 0 {
            return 0;
        } else {
            return 2_u32.pow(num_winning - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_to_card_works() {
        let result = Card::from_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let winning_numbers = HashSet::from_iter([41, 48, 83, 86, 17].into_iter());
        assert_eq!(
            result,
            Some(Card {
                number: 1,
                winning_numbers,
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            })
        );
    }

    #[test]
    fn score_works() {
        let card = Card::from_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        assert_eq!(8, card.points());
    }

    #[test]
    fn pt1_works() {
        assert_eq!(13, pt1("test_input").unwrap());
        assert_eq!(18519, pt1("input").unwrap());
    }

    #[test]
    fn pt2_works() {
        assert_eq!(30, pt2("test_input").unwrap());
        assert_eq!(11787590, pt2("input").unwrap());
    }
}
