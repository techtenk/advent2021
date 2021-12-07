use std::io;
use std::fs::File;
use crate::errors::PuzzleError;
use crate::Puzzle;

pub struct Day2Puzzle1 {

}

impl Puzzle for Day2Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 2 Puzzle 1"
    }
    fn run (&self, lines: &mut io::Lines<io::BufReader<File>>) -> Result<String, PuzzleError> {
        let mut depth = 0;
        let mut x = 0;
        for line in lines {
            let instruction = line.unwrap();
            let mut ins_iter = instruction.split(" ");
            let direction = ins_iter.next().unwrap();
            let magnitude = ins_iter.next().unwrap();
            match direction {
                "forward" => x = x + magnitude.parse::<i32>()?,
                "down" => depth = depth + magnitude.parse::<i32>()?,
                "up" => depth = depth - magnitude.parse::<i32>()?,
                &_ => ()
            }
        }
        println!("{}", depth * x);
        Ok((depth * x).to_string())
    }
}

pub struct Day2Puzzle2 {

}

impl Puzzle for Day2Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 2 Puzzle 2"
    }
    fn run (&self, lines: &mut io::Lines<io::BufReader<File>>) -> Result<String, PuzzleError> {
        let mut depth = 0;
        let mut x = 0;
        let mut aim = 0;
        for line in lines {
            let instruction = line.unwrap();
            let mut ins_iter = instruction.split(" ");
            let direction = ins_iter.next().unwrap();
            let magnitude = ins_iter.next().unwrap();
            match direction {
                "forward" => {
                    x = x + magnitude.parse::<i32>()?;
                    depth = depth + aim * magnitude.parse::<i32>()?;
                },
                "down" => aim = aim + magnitude.parse::<i32>()?,
                "up" => aim = aim - magnitude.parse::<i32>()?,
                &_ => ()
            }
        }
        println!("{}", depth * x);
        Ok((depth * x).to_string())
    }
}