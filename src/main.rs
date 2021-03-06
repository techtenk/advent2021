#![feature(drain_filter)]
#![feature(hash_drain_filter)]
#![feature(linked_list_cursors)]
#![warn(clippy::all)]
mod puzzles;
mod errors;
use crate::puzzles::Puzzle;

fn main() {

    let puzz: Vec<Box<dyn Puzzle>> = crate::puzzles::get_puzzles();

    for p in puzz {
        let result = p.run();
        match result {
            Ok(answer) => println!("{}\n\tAnswer: {} \n", p.get_puzzle_name(), answer),
            Err(_) => println!("Something went wrong!")
        }
    }

}