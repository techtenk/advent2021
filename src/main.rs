use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::num::ParseIntError;

fn main() {

    let file = "./Day01/puzzle_01/input_tim.txt";

    if let Ok(lines) = read_lines(file) {
        let result = Day1Puzzle2::run(lines);
        match result {
            Ok(_) => (),
            Err(_) => println!("Something went wrong!")
        }
    }
}

struct PuzzleError {
}

impl From<ParseIntError> for PuzzleError {
    fn from(_: ParseIntError) -> Self {
        PuzzleError { }
    }
}

impl From<std::io::Error> for PuzzleError {
    fn from(_: std::io::Error) -> Self {
        PuzzleError { }
    }
}


trait Puzzle {
    fn run (lines: io::Lines<io::BufReader<File>>) -> Result<bool, PuzzleError>;
}

struct Day1Puzzle1 {

}

impl Puzzle for Day1Puzzle1 {
    fn run (lines: io::Lines<io::BufReader<File>>) -> Result<bool, PuzzleError> {
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
        println!("{}", count);
        Ok(true)
    }
}

struct Day1Puzzle2 {

}

impl Puzzle for Day1Puzzle2 {
    fn run (lines: io::Lines<io::BufReader<File>>) -> Result<bool, PuzzleError> {
        let mut previous: i32 = 0;
        let mut count: i32 = -1;
        // a, b, and c will contain a Result, use the question mark to unwrap it and pass errors up the stack
        // let mut a_opt = lines.nth(0);
        // let mut a = a_opt.unwrap()?.parse::<i32>()?;
        // let mut b = lines.nth(1).unwrap()?.parse::<i32>()?;
        // let mut c = lines.nth(2).unwrap()?.parse::<i32>()?;
        let mut a;
        let mut b = 0;
        let mut c = 0;
        for line in lines {
            a = b;
            b = c;
            c = line.unwrap().parse::<i32>()?;
            // now we just do the same thing as Puzzle 1, except different
            if a > 0 && (a + b + c) > previous {
                count = count + 1;
            }
            previous = a + b + c;

        }
        println!("{}", count);
        Ok(true)
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}