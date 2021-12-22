use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

pub struct Day13Puzzle1 {

}

impl Puzzle for Day13Puzzle1 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 13 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day13/input_test.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        let (manual, folds) = build_manual(lines);

        let small_man = fold_manual(&manual, folds[0]);

        // print_manual(&manual);
        
        Ok("working".to_string())
    }
}

fn print_manual(manual: &Vec<Vec<bool>>) {
    for (y, row) in manual.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn fold_manual(manual: &Vec<Vec<bool>>, fold: Fold) -> Vec<Vec<bool>> {
    let mut after_fold = Vec::new();

    match fold {
        // here's the real work!
        Fold::X(fold_x) => {
            // enumerate a reverse iterator over the right half of each row
            for (y, col) in manual.iter().enumerate() {
                let range = (((fold_x + 1) as usize)..col.len()).rev();
                for (x, right_x) in range.enumerate() {
                    println!("checking {},{} ({}) against {},{} ({})", x, y, manual[y][x], right_x, y, manual[y][right_x]);
                }
            }
        },
        Fold::Y(fold_y) => {
            // enumerate a reverse iterator over the bottom half of each column
            let range = (((fold_y + 1) as usize)..manual.len()).rev();
            for (y, right_y) in range.enumerate() {
                for (x, val) in manual[y].iter().enumerate() {
                    println!("checking {},{} ({}) against {},{} ({})", x, y, manual[y][x], x, right_y, manual[right_y][x]);
                }
            }
        }
    }

    after_fold
}

fn build_manual(lines: Lines<BufReader<&[u8]>>) -> (Vec<Vec<bool>>, Vec<Fold>) {
    let mut manual: Vec<Vec<bool>> = Vec::new();
    // use up a shitton of memory, why not?
    for _ in 0..2000 {
        let row = vec![false; 2000];
        manual.push(row);
    }
    let mut folds: Vec<Fold> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for l in lines {
        let l_unwrapped = l.unwrap();
        if l_unwrapped.contains(',') {
            let mut split = l_unwrapped.split(',');
            let x = split.next().expect("error").parse::<usize>().expect("error");
            let y = split.next().expect("error").parse::<usize>().expect("error");
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            manual[y][x] = true;
        } else if l_unwrapped.contains('=') {
            let mut fold_along_split = l_unwrapped.split(' ');
            let mut instructions_split = fold_along_split.nth(2).unwrap().split('=');
            if instructions_split.next().unwrap() == "x" {
                folds.push(Fold::X(instructions_split.next().unwrap().parse::<u16>().expect("error")));
            } else {
                folds.push(Fold::Y(instructions_split.next().unwrap().parse::<u16>().expect("error")));
            }
        }
    }
    println!("max x: {}, max y: {}", max_x, max_y);
    for row in manual.iter_mut() {
        row.truncate(max_x + 1);
    }
    manual.truncate(max_y + 1);
    (manual, folds)
}

#[derive(Copy, Clone, Debug)]
enum Fold {
    X(u16),
    Y(u16)
}