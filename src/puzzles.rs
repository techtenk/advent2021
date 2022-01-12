pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

use crate::puzzles::day1::*;
use crate::puzzles::day2::*;
use crate::puzzles::day3::*;
use crate::puzzles::day4::*;
use crate::puzzles::day5::*;
use crate::puzzles::day6::*;
use crate::puzzles::day7::*;
use crate::puzzles::day8::*;
use crate::puzzles::day9::*;
use crate::puzzles::day10::*;
use crate::puzzles::day11::*;
// use crate::puzzles::day12::*;
use crate::puzzles::day13::*;
use crate::puzzles::day14::*;
use crate::puzzles::day15::*;
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
        reader.lines()
    }
}

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    let mut puzzles: Vec<Box<dyn Puzzle>> = vec!{
        Box::new(Day1Puzzle1 {}),
        Box::new(Day1Puzzle2 {}),

        Box::new(Day2Puzzle1 {}),
        Box::new(Day2Puzzle2 {}),
        
        Box::new(Day3Puzzle1 {}),
        Box::new(Day3Puzzle2 {}),
        
        Box::new(Day4Puzzle1 {}),
        Box::new(Day4Puzzle2 {}),
        
        Box::new(Day5Puzzle1 {}),
        Box::new(Day5Puzzle2 {}),
        
        Box::new(Day6Puzzle1 {}),
        Box::new(Day6Puzzle2 {}),
        
        Box::new(Day7Puzzle1 {}),
        Box::new(Day7Puzzle2 {}),
        
        Box::new(Day8Puzzle1 {}),
        Box::new(Day8Puzzle2 {}),
        
        Box::new(Day9Puzzle1 {}),
        Box::new(Day9Puzzle2 {}),
        
        Box::new(Day10Puzzle1 {}),
        Box::new(Day10Puzzle2 {}),
        
        Box::new(Day11Puzzle1 {}),
        Box::new(Day11Puzzle2 {}),

        // Box::new(Day12Puzzle1 {}),
        // Box::new(Day12Puzzle2 {}),

        Box::new(Day13Puzzle1 {}),
        Box::new(Day13Puzzle2 {}),

        Box::new(Day14Puzzle1 {}),
        Box::new(Day14Puzzle2 {}),

        Box::new(Day15Puzzle1 {}),
        Box::new(Day15Puzzle2 {}),

    };
    
    puzzles
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }