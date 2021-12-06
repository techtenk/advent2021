use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {

    let file = "./Day01/puzzle_01/input_tim.txt";

    if let Ok(lines) = read_lines(file) {
        Day1Puzzle1::run(lines);
    }
}

trait Puzzle {
    fn run (lines: io::Lines<io::BufReader<File>>);
}

struct Day1Puzzle1 {

}

impl Puzzle for Day1Puzzle1 {
    fn run (lines: io::Lines<io::BufReader<File>>) {
        let mut previous :i32 = 0;
        let mut count :i32 = -1; // the first "increase" doesn't count, so rather than doing extra checks, just start at -1
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
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}