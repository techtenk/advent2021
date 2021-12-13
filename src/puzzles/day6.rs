use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

pub struct Day6Puzzle1 {

}

impl Puzzle for Day6Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 6 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day06/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let mut lines = self.get_lines();
    
        Ok(get_fish_count(& mut lines, 80).to_string())
    }
}

fn get_fish_count(lines: & mut Lines<BufReader<&[u8]>>, days: u16) -> u64 {
    let mut fish_by_day: [u64; 9] = [0; 9];
    // fill in the initial state
    for n in lines.next().expect("error").expect("Error").split(',') {
        fish_by_day[n.parse::<usize>().expect("error")] += 1;
    }
    for _ in 1..=days {
        fish_by_day.rotate_left(1);
        fish_by_day[6] += fish_by_day[8];
    }
    fish_by_day.iter().sum::<u64>()
}

pub struct Day6Puzzle2 {

}

impl Puzzle for Day6Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 6 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day06/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let mut lines = self.get_lines();
    
        Ok(get_fish_count(& mut lines, 256).to_string())
    }
}