const TEST_INPUT: &str = include_str!("../test_input");
const INPUT: &str = include_str!("../input");

pub fn pt1(input: &str) -> f64 {
    let mut input_iter = input.lines();
    let mut t = input_iter.next().unwrap().split_whitespace();

    // discard the "Time:" part
    t.next();

    let time_iter = t.map(|x| str::parse::<f64>(x).unwrap());

    let mut d = input_iter.next().unwrap().split_whitespace();

    // dicard the "Distance:" part
    d.next();

    let distance_iter = d.map(|x| str::parse::<f64>(x).unwrap());

    let product_of_ways_to_win = time_iter
        .zip(distance_iter)
        .map(|(time, record)| ways_to_win(time, record))
        .product();

    return product_of_ways_to_win;
}

pub fn pt2(input: &str) -> f64 {
    let mut input_iter = input.lines();
    let mut t = input_iter.next().unwrap().split_whitespace();

    // discard the "Time:" part
    t.next();

    let vs: Vec<&str> = t.collect();

    let time = str::parse::<f64>(&vs.join("")).unwrap();

    let mut d = input_iter.next().unwrap().split_whitespace();

    // dicard the "Distance:" part
    d.next();

    let distance = str::parse::<f64>(&d.collect::<Vec<&str>>().join("")).unwrap();

    ways_to_win(time, distance)
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
fn ways_to_win(race_length: f64, record: f64) -> f64 {
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
    let max = if v1.min(v2) == v1 { v2 } else { v1 };

    let mut min_ceil = min.ceil();

    min_ceil = if min_ceil == min {
        min_ceil + 1_f64
    } else {
        min_ceil
    };

    let mut max_floor = max.floor();

    max_floor = if max_floor == max {
        max_floor - 1_f64
    } else {
        max_floor
    };

    max_floor - min_ceil + 1_f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_works() {
        assert_eq!(288_f64, pt1(TEST_INPUT));
        assert_eq!(1083852_f64, pt1(INPUT));
    }

    #[test]
    fn pt2_works() {
        use super::*;
        assert_eq!(71503_f64, pt2(TEST_INPUT));
        assert_eq!(23501589_f64, pt2(INPUT));
    }
}

// 1.7 * (7 - 1.7) = 10
