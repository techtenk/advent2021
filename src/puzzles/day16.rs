use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

pub struct Day15Puzzle1 {

}

impl Puzzle for Day15Puzzle1 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 15 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day16/input_test.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let mut lines = self.get_lines();

        Ok("working".to_string())
    }
}

enum ReadingMode {
    VERSION,
    TYPE,
    LITERAL,
    LITERAL_LAST,
    OPERATOR_11,
    OPERATOR_15,
    OP_SUB_PACKET
}