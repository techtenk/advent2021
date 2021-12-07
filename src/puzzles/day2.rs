use std::io;
use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::BufRead;

pub struct Day2Puzzle1 {

}

impl Day2Puzzle1 {

}

impl Puzzle for Day2Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 2 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day02/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        let bytes = self.get_input();
        let slice: &[u8] = &bytes[..];
        let reader = io::BufReader::new(slice);
        let lines = reader.lines();

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

        Ok((depth * x).to_string())
    }
}

pub struct Day2Puzzle2 {

}

impl Day2Puzzle2 {

}

impl Puzzle for Day2Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 2 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day02/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        let bytes = self.get_input();
        let slice: &[u8] = &bytes[..];
        let reader = io::BufReader::new(slice);
        let lines = reader.lines();

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

        Ok((depth * x).to_string())
    }
}