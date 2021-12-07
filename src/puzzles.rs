pub mod day1;
pub mod day2;
use std::io;
use std::fs::File;
use crate::puzzles::day1::*;
use crate::puzzles::day2::*;
use crate::errors::PuzzleError;

pub trait Puzzle {
    fn run (&self, lines: &mut std::io::Lines<io::BufReader<File>>) -> Result<String, PuzzleError>;
    fn get_puzzle_name(&self) -> &'static str;
}

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    let mut puzzles: Vec<Box<dyn Puzzle>> = Vec::new();

    // day 1
    puzzles.push(Box::new(Day1Puzzle1 {}));
    puzzles.push(Box::new(Day1Puzzle2 {}));

    // day 2
    puzzles.push(Box::new(Day2Puzzle1 {}));
    puzzles.push(Box::new(Day2Puzzle2 {}));

    puzzles
}