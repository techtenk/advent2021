use std::io;
use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::BufRead;

pub struct Day1Puzzle1 {

}

impl Day1Puzzle1 {

}

impl Puzzle for Day1Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 1 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day01/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {

        let lines = self.get_lines();
        let mut previous: i32 = 0;
        let mut count: i32 = -1; // the first "increase" doesn't count, so rather than doing extra checks, just start at -1
        for line in lines {
            if let Ok(ok_line) = line {
                if let Ok(number) = ok_line.parse::<i32>() {
                    if number > previous {
                        count = count + 1;
                    }
                    previous = number;
                }
            }
        }
        Ok(count.to_string())
    }

}

pub struct Day1Puzzle2 {

}

impl Day1Puzzle2 {

}

impl Puzzle for Day1Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 1 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day01/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        let bytes = self.get_input();
        let slice: &[u8] = &bytes[..];
        let reader = io::BufReader::new(slice);
        let lines = reader.lines();

        let mut previous: i32 = 0;
        let mut count: i32 = -1;

        let mut a;
        let mut b = 0;
        let mut c = 0;
        for line in lines {
            a = b;
            b = c;
            // parse will yield a Result, use the question mark to unwrap it and propogate any errors up the stack
            c = line.unwrap().parse::<i32>()?;
            // now we just do the same thing as Puzzle 1, except different
            if a > 0 && (a + b + c) > previous {
                count = count + 1;
            }
            previous = a + b + c;

        }
        Ok(count.to_string())
    }
}