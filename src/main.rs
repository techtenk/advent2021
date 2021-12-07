use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::num::ParseIntError;

fn main() {

    let file = "./Day02/input_tim.txt";

    if let Ok(lines) = read_lines(file) {
        let result = Day2Puzzle2::run(lines);
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
        println!("{}", count);
        Ok(true)
    }
}

struct Day2Puzzle1 {

}

impl Puzzle for Day2Puzzle1 {
    fn run (lines: io::Lines<io::BufReader<File>>) -> Result<bool, PuzzleError> {
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
        Ok(true)
    }
}

struct Day2Puzzle2 {

}

impl Puzzle for Day2Puzzle2 {
    fn run (lines: io::Lines<io::BufReader<File>>) -> Result<bool, PuzzleError> {
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