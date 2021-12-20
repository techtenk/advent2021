use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

pub struct Day11Puzzle1 {

}

impl Puzzle for Day11Puzzle1 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 11 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day11/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        let total_flashed = run_simulation(lines, 100, PuzzleReturn::TotalFlashes);
        
        Ok(total_flashed.to_string())
    }
}

pub struct Day11Puzzle2 {

}

impl Puzzle for Day11Puzzle2 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 11 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day11/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        let steps = run_simulation(lines, 1000, PuzzleReturn::StepsUntilSync);
        
        Ok(steps.to_string())
    }
}

#[derive(PartialEq)]
enum PuzzleReturn {
    TotalFlashes,
    StepsUntilSync,
}

fn run_simulation(lines: Lines<BufReader<&[u8]>>, steps: u16, return_val: PuzzleReturn) -> u32 {
    let mut octo_grid = build_octo_grid(lines);

    // simulate the steps
    let mut total_flashed: u32 = 0;
    // capture the step we are on when they all flashes together
    for step in 1..=steps {
        let mut flashed_this_step = 0;
        let mut additional_increments: Vec<usize> = Vec::new();
        for octo in octo_grid.iter_mut() {
            let (flashed, ai) = octo.increment_energy();
            additional_increments.extend(ai);
            total_flashed += flashed as u32;
            flashed_this_step += flashed as u32;
        }
        
        while !additional_increments.is_empty() {
            let mut ai: Vec<usize> = Vec::new();
            {
                // println!("additional increments: {:?}", additional_increments);
                for next in additional_increments.iter() {
                    // println!("now increment {}", *next);
                    let (flashed, add_inc) = octo_grid.get_mut(*next).expect("error").increment_energy();
                    ai.extend(add_inc);
                    total_flashed += flashed as u32;
                    flashed_this_step += flashed as u32;
                }
            }
            additional_increments = ai;
        }
        for octo in octo_grid.iter_mut() {
            octo.reset();
        }
        if return_val == PuzzleReturn::StepsUntilSync && flashed_this_step == 100 {
            return step as u32;
        }
        // println!("Flashes after step {}: {}", step, total_flashed);
    }
    total_flashed
}

fn build_octo_grid(lines: Lines<BufReader<&[u8]>>) -> Vec<Octo> {
    let mut octo_grid = Vec::new();

    
    let octo_grid_mut = & mut octo_grid;
    // instantiate the Octos
    for l in lines {
        for n in l.unwrap().chars().map(|n| n.to_string().parse::<u8>()) {
            octo_grid_mut.push(Octo::new(Some(n.unwrap())));
        }
    }


    for (i, octo) in octo_grid_mut.iter_mut().enumerate() {
        let mut neighbors: Vec<usize> = Vec::new();

        // left neighbor
        if i.checked_rem(10) != Some(0) {
            neighbors.push(i-1);
        }
        // right neighbor
        if i.checked_rem(10) != Some(9) {
            neighbors.push(i+1);
        }
        // top neighbor
        if i > 9 {
            neighbors.push(i-10);
        }
        // bottom neighbor
        if i < 90 {
            neighbors.push(i+10);
        }
        // top left neighbor
        if i.checked_rem(10) != Some(0) && i > 9 {
            neighbors.push(i-11);
        }
        // top right neighbor
        if i.checked_rem(10) != Some(9) && i > 9 {
            neighbors.push(i-9);
        }
        // bottom left neighbor
        if i.checked_rem(10) != Some(0) && i < 90 {
            neighbors.push(i+9);
        }
        // bottom right neighbor
        if i.checked_rem(10) != Some(9) && i < 90 {
            neighbors.push(i+11);
        }
        octo.neighbors = neighbors;
    }

    octo_grid
}

struct Octo {
    energy: u8,
    has_flashed: bool,
    neighbors: Vec<usize>,
}

impl Octo {

    fn new(energy: Option<u8>) -> Self {
        
        Octo {
            energy: energy.unwrap_or(0),
            has_flashed: false,
            neighbors: Vec::new(),
        }
    }

    fn increment_energy(&mut self) -> (u8, Vec<usize>) {
        self.energy += 1;
        if self.energy > 9 && !self.has_flashed {
            return (1, self.flash());
        }
        (0, Vec::new())
    }
    fn flash(& mut self) -> Vec<usize> {
        self.has_flashed = true;
        self.neighbors.clone()
    }
    fn reset(&mut self) {
        if self.has_flashed {
            self.energy = 0;
            self.has_flashed = false;
        }
    }

}