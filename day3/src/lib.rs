use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn pt1() -> Result<u32, io::Error> {
    let path = "input";

    let file = File::open(path)?;

    let buffered = BufReader::new(file);

    // create an iterator over the lines of the file
    let mut lines_iter = buffered.lines().filter_map(|line_result| line_result.ok());

    // instantiate a sliding window of 3 lines
    let mut window = Window::new();

    let result = window.process_lines(&mut lines_iter);

    let answer = result.part_numbers.iter().map(|x| x.number).sum::<u32>();

    println!("part numbers {:?}", result.part_numbers);
    println!("sum {}", answer);

    return Ok(answer);
}

pub fn pt2(path: &str) -> Result<u32, io::Error> {
    let file = File::open(path)?;

    let buffered = BufReader::new(file);

    // create an iterator over the lines of the file
    let mut lines_iter = buffered.lines().filter_map(|line_result| line_result.ok());

    // instantiate a sliding window of 3 lines
    let mut window = Window::new();

    let result = window.process_lines(&mut lines_iter);

    let answer = result.gears.iter().map(|g| g.gear_ratio).sum();
    println!("sum {}", answer);

    return Ok(answer);
}

struct Window {
    previous: Option<Vec<char>>,
    middle: Option<Vec<char>>,
    next: Option<Vec<char>>,
    row: usize,
}

impl Window {
    fn new() -> Self {
        Self {
            previous: None,
            middle: None,
            next: None,
            row: 0,
        }
    }

    fn process_lines(&mut self, lines: &mut impl Iterator<Item = String>) -> ProcessingResult {
        let mut prospective_gears = HashMap::new();
        let mut part_numbers = Vec::new();
        // move the sliding window forward one line at a time
        loop {
            self.move_forward(lines.next());
            self.scan_for_part_numbers(&mut prospective_gears, &mut part_numbers);

            // end when the sliding window becomes empty
            if self.is_empty() {
                break;
            }
        }

        let gears = gears(prospective_gears);

        return ProcessingResult {
            gears,
            part_numbers,
        };
    }

    fn move_forward(&mut self, next_line: Option<String>) -> () {
        self.previous = self.middle.take();
        self.middle = self.next.take();
        self.next = next_line.map(|s| s.chars().collect());
        if self.previous.is_some() {
            self.row += 1;
        }
    }

    fn is_empty(&self) -> bool {
        self.previous.is_none() && self.middle.is_none() && self.next.is_none()
    }

    /// scans the middle line for part numbers, scans for potential gears around part numbers
    fn scan_for_part_numbers(
        &mut self,
        prospective_gears: &mut HashMap<Coordinate, ProspectiveGear>,
        part_numbers: &mut Vec<NumberInfo>,
    ) -> () {
        if let Some(line) = &self.middle {
            let nums = scan_for_numbers(&line, self.row);

            for num in nums.into_iter() {
                // look for surrounding symbols
                let surrounding_symbols = self.surrounding_symbols(&num);

                // if there are surrounding symbols, num is a part number
                if !surrounding_symbols.is_empty() {
                    // look for asterisks around the part number
                    for symbol in surrounding_symbols.into_iter() {
                        if symbol.symbol == '*' {
                            // increment the number of part numbers around the asterisk
                            let gear = prospective_gears
                                .entry(symbol.coordinate.clone())
                                .or_insert(ProspectiveGear {
                                    part_numbers_count: 0,
                                    gear_ratio: 1,
                                });
                            gear.part_numbers_count += 1;
                            gear.gear_ratio *= num.number;
                        };
                    }

                    part_numbers.push(num);
                }
            }
        }
    }

    fn surrounding_symbols(&self, number_info: &NumberInfo) -> Vec<Symbol> {
        let mut result = Vec::new();
        if let Some(m) = &self.middle {
            // check left
            // guard prevents usize subtraction from panicking
            if number_info.coordinate.column > 0 {
                let left_idx = number_info.coordinate.column - 1;
                if let Some(c) = m.get(left_idx) {
                    if is_symbol(*c) {
                        let symbol = Symbol {
                            symbol: *c,
                            coordinate: Coordinate {
                                row: self.row,
                                column: left_idx,
                            },
                        };
                        result.push(symbol);
                    }
                }
            }

            // check right
            let right_idx = number_info.coordinate.column + number_info.num_digits;
            if let Some(c) = m.get(right_idx) {
                if is_symbol(*c) {
                    let symbol = Symbol {
                        symbol: *c,
                        coordinate: Coordinate {
                            row: self.row,
                            column: right_idx,
                        },
                    };
                    result.push(symbol);
                }
            }
        }

        // check top
        if let Some(p) = &self.previous {
            let left_boundary = if number_info.coordinate.column == 0 {
                0
            } else {
                number_info.coordinate.column - 1
            };

            let right_boundary = min(left_boundary + number_info.num_digits + 2, p.len());

            for (idx, c) in p[left_boundary..right_boundary].iter().enumerate() {
                if is_symbol(*c) {
                    let symbol = Symbol {
                        symbol: *c,
                        coordinate: Coordinate {
                            row: self.row - 1,
                            column: idx + left_boundary,
                        },
                    };
                    result.push(symbol);
                }
            }
        }

        // check bottom
        if let Some(t) = &self.next {
            let left_boundary = if number_info.coordinate.column == 0 {
                0
            } else {
                number_info.coordinate.column - 1
            };

            let right_boundary = min(left_boundary + number_info.num_digits + 2, t.len());

            for (idx, c) in t[left_boundary..right_boundary].iter().enumerate() {
                if is_symbol(*c) {
                    let symbol = Symbol {
                        symbol: *c,
                        coordinate: Coordinate {
                            row: self.row + 1,
                            column: idx + left_boundary,
                        },
                    };
                    result.push(symbol);
                }
            }
        }

        return result;
    }
}

/// return gears that were found
fn gears(prospective_gears: HashMap<Coordinate, ProspectiveGear>) -> Vec<ProspectiveGear> {
    return prospective_gears
        .into_iter()
        .map(|(_k, v)| v)
        .filter(|pg| pg.part_numbers_count == 2)
        .collect();
}

// returns a vector of tuples of numbers and their starting indexes
fn scan_for_numbers(line: &Vec<char>, row: usize) -> Vec<NumberInfo> {
    let mut result = Vec::new();

    let mut consecutive_digits: Vec<u32> = Vec::new();

    let mut chars_iter = line.iter();

    let mut number_start: Option<usize> = None;

    let mut idx = 0;

    loop {
        let next_char = chars_iter.next();

        match next_char {
            Some(c) => {
                if let Some(d) = c.to_digit(10) {
                    consecutive_digits.push(d);

                    if number_start.is_none() {
                        number_start = Some(idx);
                    }
                } else {
                    flush_number(&mut consecutive_digits, &mut result, number_start, row);
                    number_start = None;
                }
            }
            None => {
                flush_number(&mut consecutive_digits, &mut result, number_start, row);

                break;
            }
        }
        idx += 1;
    }

    return result;
}

fn flush_number(
    consecutive_digits: &mut Vec<u32>,
    result: &mut Vec<NumberInfo>,
    number_start: Option<usize>,
    row: usize,
) -> () {
    if let Some(n) = number_start {
        let len = consecutive_digits.len();
        let number: u32 = (0..len)
            .rev()
            .zip(consecutive_digits.iter())
            .map(|(power, digit)| 10_u32.pow(u32::try_from(power).expect("error")) * digit)
            .sum();

        result.push(NumberInfo {
            number,
            coordinate: Coordinate { row, column: n },
            num_digits: len,
        });
    }
    consecutive_digits.clear();
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

struct ProcessingResult {
    gears: Vec<ProspectiveGear>,
    part_numbers: Vec<NumberInfo>,
}

#[derive(PartialEq, Debug)]
struct NumberInfo {
    number: u32,
    coordinate: Coordinate,
    num_digits: usize,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Coordinate {
    row: usize,
    column: usize,
}

#[derive(PartialEq, Debug)]
struct Symbol {
    symbol: char,
    coordinate: Coordinate,
}

#[derive(Debug)]
struct ProspectiveGear {
    part_numbers_count: u32,
    gear_ratio: u32,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn scan_for_numbers_works() {
        let input: Vec<char> = "467..114..".chars().collect();
        let row = 0;
        let result = scan_for_numbers(&input, row);
        assert_eq!(
            result,
            vec![
                NumberInfo {
                    number: 467,
                    coordinate: Coordinate { row, column: 0 },
                    num_digits: 3
                },
                NumberInfo {
                    number: 114,
                    coordinate: Coordinate { column: 5, row },
                    num_digits: 3
                }
            ]
        );
    }

    #[test]
    fn surrounding_symbol_works() {
        let row = 1;
        {
            let window = test_window(Some("617*......"), Some(".....+.58."), Some("..592....."));

            let part_number = NumberInfo {
                number: 58,
                coordinate: Coordinate { column: 7, row },
                num_digits: 2,
            };

            assert_eq!(window.surrounding_symbols(&part_number), vec![]);
        }

        {
            let window = test_window(Some("...*......"), Some("..35..633."), Some("......#..."));
            let part_number = NumberInfo {
                coordinate: Coordinate { row, column: 2 },
                number: 35,
                num_digits: 2,
            };

            assert_eq!(
                window.surrounding_symbols(&part_number),
                vec![Symbol {
                    symbol: '*',
                    coordinate: Coordinate { row: 0, column: 3 }
                }]
            );
        }

        {
            let window = test_window(Some(".....+.58."), Some("..592....."), Some("......755."));
            let part_number = NumberInfo {
                coordinate: Coordinate { row, column: 2 },
                number: 592,
                num_digits: 3,
            };

            assert_eq!(
                window.surrounding_symbols(&part_number),
                vec![Symbol {
                    symbol: '+',
                    coordinate: Coordinate { row: 0, column: 5 }
                }]
            );
        }
    }

    #[test]
    fn pt1_works() {
        assert_eq!(pt1().unwrap(), 519444);
    }

    #[test]
    fn pt2_works() {
        assert_eq!(pt2("test_input").unwrap(), 467835);
        assert_eq!(pt2("input").unwrap(), 74528807);
    }

    /// constructs a window from strings, useful for testing
    fn test_window(previous: Option<&str>, middle: Option<&str>, next: Option<&str>) -> Window {
        let to_chars = |s: &str| -> Vec<char> { s.chars().collect() };

        Window {
            previous: previous.map(to_chars),
            middle: middle.map(to_chars),
            next: next.map(to_chars),
            row: 1,
        }
    }
}
