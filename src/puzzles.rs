pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

use crate::puzzles::day1::*;
use crate::puzzles::day2::*;
use crate::puzzles::day3::*;
use crate::puzzles::day4::*;
use crate::puzzles::day5::*;
use crate::errors::PuzzleError;
use std::io;
use std::io::BufRead;

pub trait Puzzle {
    fn run (&self) -> Result<String, PuzzleError>;
    fn get_puzzle_name(&self) -> &'static str;
    fn get_input(&self) -> Box<&'static [u8]>;
    fn get_lines(&self) -> io::Lines<io::BufReader<&'static [u8]>>{
        let bytes = self.get_input();
        let slice = &bytes[..];
        let reader = io::BufReader::new(slice);
        let lines = reader.lines();
        lines
    }
}

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    let mut puzzles: Vec<Box<dyn Puzzle>> = Vec::new();

    // day 1
    puzzles.push(Box::new(Day1Puzzle1 {}));
    puzzles.push(Box::new(Day1Puzzle2 {}));

    // day 2
    puzzles.push(Box::new(Day2Puzzle1 {}));
    puzzles.push(Box::new(Day2Puzzle2 {}));

    // day 3
    puzzles.push(Box::new(Day3Puzzle1 {}));
    puzzles.push(Box::new(Day3Puzzle2 {}));

    // day 4
    puzzles.push(Box::new(Day4Puzzle1 {}));
    puzzles.push(Box::new(Day4Puzzle2 {}));

    // day 5
    puzzles.push(Box::new(Day5Puzzle1 {}));

    puzzles
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }