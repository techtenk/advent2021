use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
mod puzzles;
mod errors;
use crate::puzzles::Puzzle;

fn main() {


    let file = "./Day01/input_tim.txt";

    if let Ok(mut lines) = read_lines(file) {
        let puzz: Vec<Box<dyn Puzzle>> = crate::puzzles::get_puzzles();

        for p in puzz {
            let result = p.run(& mut lines);
            match result {
                Ok(answer) => println!("{}\n\tAnswer: {} \n", p.get_puzzle_name(), answer),
                Err(_) => println!("Something went wrong!")
            }
        }

        // let result = Day2Puzzle2::run(lines);
        // match result {
        //     Ok(answer) => println!("Answer: {} \n", answer),
        //     Err(_) => println!("Something went wrong!")
        // }
    } else {
        println!("Couldn't read file: {}", file);
        println!("Error: {:?}", read_lines(file));
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}