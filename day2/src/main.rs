use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let path = "input";

    let file = File::open(path)?;

    let file2 = File::open(path)?;

    let answer = part_1(file);

    println!("answer is {}", answer);

    let part2_answer = part_2(file2);

    println!("part 2 answer is {}", part2_answer);

    return Ok(());
}

fn part_2(file: File) -> u32 {
    parse_games(file)
        .map(|game| game.minimum_set().expect("game with no sets"))
        .map(|set| set.power())
        .sum()
}

fn part_1(file: File) -> u32 {
    let constraint = Set {
        red: 12,
        blue: 14,
        green: 13,
    };

    parse_games(file)
        .filter(|game| is_game_possible(&constraint, game))
        .map(|game| game.id)
        .sum::<u32>()
}

/// takes a file, parses it, and returns an iterator over games parsed
/// from the file
fn parse_games(file: File) -> impl Iterator<Item = Game> {
    let buffered = BufReader::new(file);
    buffered
        .lines()
        .filter_map(|line_result| get_game(&(line_result.ok()?)))
}

fn is_game_possible(constraint: &Set, game: &Game) -> bool {
    let red = constraint.red;
    for set in &game.sets {
        if set.red > red {
            return false;
        }
    }

    let blue = constraint.blue;
    for set in &game.sets {
        if set.blue > blue {
            return false;
        }
    }

    let green = constraint.green;
    for set in &game.sets {
        if set.green > green {
            return false;
        }
    }

    return true;
}

fn get_game(line: &str) -> Option<Game> {
    let mut splits = line.split(": ");

    let game_label = splits.next()?;

    let mut game_label_splits = game_label.split(' ');

    game_label_splits.next()?;

    let id = game_label_splits.next()?.parse().expect("Error parsing ID");

    return Some(Game {
        id,
        sets: get_sets(splits.next()?),
    });
}

fn get_sets(games: &str) -> Vec<Set> {
    return games
        .split("; ")
        .map(|s| parse_set(s).expect("Error parsing set"))
        .collect();
}

fn parse_set(set_str: &str) -> Result<Set, Box<dyn Error>> {
    let mut set = Set::build_empty();
    let colors = set_str.split(", ");

    for color_str in colors {
        set.assign_color_from_str(color_str)?
    }

    return Ok(set);
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    /// assigns a color to Self using a string that looks like "3 blue"
    fn assign_color_from_str(&mut self, s: &str) -> Result<(), Box<dyn Error>> {
        let mut splits = s.split(' ');

        let quantity: u32 = splits
            .next()
            .ok_or("error splitting quantity")?
            .parse()
            .expect("error parsing quantity");

        let colour = splits.next().ok_or("error splitting colour")?;

        match colour {
            "red" => self.red = quantity,
            "blue" => self.blue = quantity,
            "green" => self.green = quantity,
            _ => return Err(Into::into("ah")),
        }

        return Ok(());
    }
    fn build_empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    sets: Vec<Set>,
    id: u32,
}

impl Game {
    fn minimum_set(&self) -> Option<Set> {
        Some(Set {
            red: self.sets.iter().map(|set| set.red).max()?,
            green: self.sets.iter().map(|set| set.green).max()?,
            blue: self.sets.iter().map(|set| set.blue).max()?,
        })
    }
}
