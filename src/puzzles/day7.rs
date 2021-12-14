use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

pub struct Day7Puzzle1 {

}

impl Puzzle for Day7Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 7 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day07/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
    
        Ok(get_min_fuel(lines, false).to_string())
    }
}

pub struct Day7Puzzle2 {

}

impl Puzzle for Day7Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 7 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day07/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
    
        Ok(get_min_fuel(lines, true).to_string())
    }
}

fn get_min_fuel(mut lines: Lines<BufReader<&[u8]>>, pricey_fuel: bool) -> u32 {
    let mut crabs: [u32; 2000] = [0; 2000];
    let mut fuel_cost_array: [u32; 2000] = [0; 2000];
    for n in lines.next().expect("error").expect("error").split(',') {
        let parsed = n.parse::<usize>().expect("error");
        crabs[parsed] += 1;
        fuel_cost_array[parsed] += 1;
    }
    // i 0 -> 2000
    // j 2000 -> 0
    // couln't figure out reverse ranges last problem, so doing it manually
    let mut left = 0;
    let mut right = crabs.len() - 1;
    let mut fuel_cost: u32 = 0;
    while left != right {
        // what costs less, moving i or j
        if fuel_cost_array[left] < fuel_cost_array[right] {
            // move crabs
            fuel_cost += fuel_cost_array[left];
            if pricey_fuel {
                fuel_cost_array[left+1] += fuel_cost_array[left] + crabs[left];
            } else {
                fuel_cost_array[left+1] += fuel_cost_array[left];
            }
            crabs[left+1] += crabs[left];
            left += 1;
        } else {
            fuel_cost += fuel_cost_array[right];
            if pricey_fuel {
                fuel_cost_array[right-1] += fuel_cost_array[right] + crabs[right];
            } else {
                fuel_cost_array[right-1] += fuel_cost_array[right];
            }
            crabs[right-1] += crabs[right];
            right -= 1;
        }
    }
    fuel_cost
}