use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Error;

fn get_gamma_epsilon<'a>(lines: &'a Vec<Result<String, Error>>) -> Result<[i32; 2], PuzzleError> {
    // fold to reduce it to an array, one value for each position, if the value ends positive, then 1 is the predomninant for that position
    let result = lines.iter().fold([0; 12], |carry, next| {
        let mut next_carry = carry.clone();
        let value = next.as_ref().unwrap();
        if let Ok(uvalue) = u16::from_str_radix(&value, 2) {
            for i in 0..12 {
                let mask = 0b100000000000 >> i;
                next_carry[i] += if (mask & uvalue) > 0 { 1 } else { -1 };
            }
        }
        next_carry
    });

    // now convert that array into a gamma ratio by checking pos/neg of each position
    let mut gamma = 0;
    for i in 0..12 {
        if result[i] > 0 {
            gamma = gamma | (0b100000000000 >> i);
        } else if result[i] == 0 {
            // puzzle 2 has this possibility:
            // for gamma: "If 0 and 1 are equally common, keep values with a 1 in the position being considered."
            // this won't change puzzle 1 since it should be guaranteed not to happen
            gamma = gamma | (0b100000000000 >> i);
        }
    }

    // flip the bits for epsilon
    let epsilon = gamma ^ 0b111111111111;

    // return them both
    Ok([
        gamma,
        epsilon
    ])
}

fn calc_ratio(mut lines: Vec<Result<String, Error>>, use_gamma: bool) -> i32 {
    for i in 0..12 {
        if let Ok([gamma, epsilon]) = get_gamma_epsilon(&lines) {
            lines = lines.drain_filter(|item| {
                if let Ok(checked_value) = item {
                    // check the i bit, eliminating whatever doesn't match gamma
                    // first mask just the relevant bit in gamma
                    let gamma_or_epsilon = if use_gamma { gamma } else { epsilon };
                    let should_equal = (0b100000000000 >> i) & gamma_or_epsilon;
                    if should_equal > 0 {
                        // gamma has a '1' at the relevant location, so should the item
                        return ((0b100000000000 >> i) & u16::from_str_radix(checked_value, 2).expect("error")) > 0;
                    } else {
                        // gamma has a '0' at the relevant location, so should the item
                        let uvalue = u16::from_str_radix(checked_value, 2).expect("error");
                        return ((0b100000000000 >> i) & uvalue) == 0;
                    }
                }
                false
            }).collect::<Vec<_>>();
            if lines.len() < 2 {
                break;
            }
        }
    }
    let ratio = i32::from_str_radix(lines.pop().unwrap().unwrap().as_ref(), 2).unwrap();
    ratio
}

pub struct Day3Puzzle1 {

}

impl Puzzle for Day3Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 3 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day03/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        let lines = self.get_lines();

        if let Ok([gamma, epsilon]) = get_gamma_epsilon(&lines.collect()) {
            return Ok((gamma * epsilon).to_string());
        }

        Err(PuzzleError {})
    }
}

pub struct Day3Puzzle2 {

}

impl Puzzle for Day3Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 3 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day03/input_tim.txt");
        Box::new(bytes)
    }
    fn run(&self) -> Result<String, PuzzleError> {
        // now this is interesting, we are going to filter until only one is left, but each time recalculate gamma/epsilon
        // first the "oxygen generator rating" which is based on gamma
        let oxygen_remaining_lines: Vec<Result<String, Error>> = self.get_lines().collect();
        let oxygen = calc_ratio(oxygen_remaining_lines, true);

        // then the "C02 which is based on epsilon"
        let co2_remaining_lines: Vec<Result<String, Error>> = self.get_lines().collect();
        let co2 = calc_ratio(co2_remaining_lines, false);

        Ok((oxygen * co2).to_string())
    }
}