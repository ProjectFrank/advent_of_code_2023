const TEST_INPUT: &str = include_str!("../test_input");
const INPUT: &str = include_str!("../input");

pub fn pt1(input: &str) -> f64 {
    let mut input_iter = input.lines();
    let mut bleh = input_iter.next().unwrap().split_whitespace();

    // discard the "Time:" part
    bleh.next();

    let time_iter = bleh.map(|x| str::parse::<f64>(x).unwrap());

    let mut ooooh = input_iter.next().unwrap().split_whitespace();

    // dicard the "Distance:" part
    ooooh.next();

    let distance_iter = ooooh.map(|x| str::parse::<f64>(x).unwrap());

    let product_of_ways_to_win = time_iter
        .zip(distance_iter)
        .map(|(time, record)| {
            let (lower, upper) = button_hold_time(time, record);

            // Handle cases where a is greater than b or they are equal
            if lower >= upper {
                return 0_f64;
            }

            let lower_ceil = lower.ceil();

            let lower_bound = if lower_ceil == lower {
                lower_ceil + 1_f64
            } else {
                lower_ceil
            };

            let upper_floor = upper.floor();

            let upper_bound = if upper_floor == upper {
                upper - 1_f64
            } else {
                upper_floor
            };

            let ways_to_win = upper_bound - lower_bound + 1_f64;

            println!(
                "lower: {:?}, upper: {:?}, ways: {:?}",
                lower, upper, ways_to_win
            );

            ways_to_win
        })
        .product();

    return product_of_ways_to_win;
}

// let T be the time spent holding the button.
// Let L be the length of the race.
// Let d be the distance traveled

// d = speed * (L - T)
// d = T * (L - T)
// d = TL - T^2
// 0 = -T^2 + TL - d

// derivative of T
// L - 2T

// solving for of dd/dt = 0: T = L / 2

// takes the length of a race in millis
// returns a tuple of the time holding the button and the distance achieved
fn maximum(race_length: f64) -> (f64, f64) {
    let button_hold_time = race_length / 2_f64;
    (
        button_hold_time,
        (race_length - button_hold_time) * button_hold_time,
    )
}

// takes a race length and the record and returns the button hold time used to achieve the record
fn button_hold_time(race_length: f64, record: f64) -> (f64, f64) {
    // use the quadratic formula to obtain the different button hold times
    // a = -1
    // b = L
    // c = -d

    let a: f64 = -1_f64;
    let b: f64 = race_length;
    let c: f64 = -record;

    // quadratic formula: (-b +- sqrt(b^2 - 4ac)) / 2a
    let sqrt = (b.powf(2_f64) - 4_f64 * a * c).sqrt();

    let v1 = (-b - sqrt) / (2_f64 * a);
    let v2 = (-b + sqrt) / (2_f64 * a);

    let min = v1.min(v2);
    let other = if v1.min(v2) == v1 { v2 } else { v1 };

    (min, other)
    // the number of ways to win is the whole numbers between the two hold times obtained
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_works() {
        assert_eq!(288_f64, pt1(TEST_INPUT));
        assert_eq!(1083852_f64, pt1(INPUT));
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}

// 1.7 * (7 - 1.7) = 10
