use crate::errors::PuzzleError;
use crate::Puzzle;

pub struct Day3Puzzle1 {

}

impl Day3Puzzle1 {

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

        // fold to reduce it to an array, one value for each position, if the value ends positive, then 1 is the predomninant for that position
        let result = lines.fold([0; 12], |carry, next| {
            let mut next_carry = carry.clone();
            let value = next.unwrap();
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
                // halt on error, there are no instructions for if the position doesn't have a predominant 1 or 0
                return Err(PuzzleError{ })
            }
        }

        // flip the bits for epsilon
        let epsilon = gamma ^ 0b111111111111;

        Ok((gamma * epsilon).to_string())
    }
}