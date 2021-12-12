use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

fn get_draws_and_boards(lines: Lines<BufReader<&[u8]>>, draws: & mut Vec<u8> , boards: & mut Vec<Board>) {
        let mut lines = lines.peekable();
        // get the number to draw, make an array
        let first_line = lines.next().unwrap().expect("error reading first line");
        let mut inner_map: Vec<u8> = first_line.split(',').map(|x| x.parse::<u8>().expect("error")).collect();
        draws.append(& mut inner_map);

        while lines.peek().is_some() {
            // setup boards
            let mut board = Board {
                values: [0; 25],
                marked: [false; 25]
            };
            for i in 0..6 {
                if i == 0 { lines.next(); continue; } // eat an empty line
                for (j, number) in lines.next().unwrap().expect("error reading line").split_whitespace().enumerate() {
                    board.values[(i - 1)*5+j] = number.parse::<u8>().expect("error");
                }
            }
            boards.push(board);
        }
}

pub struct Day4Puzzle1 {

}

impl Puzzle for Day4Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 4 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day04/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {

        let mut draws: Vec<u8> = Vec::new();
        let mut boards: Vec<Board> = Vec::new();
        get_draws_and_boards(self.get_lines(), & mut draws, & mut boards);

        for draw in draws {
            for b in &mut boards {
                if let Some(score) = b.mark_value(draw) {
                    // println!("{}", score * draw as i32);
                    return Ok((score * draw as i32).to_string());
                }
            }
        }

        Err(PuzzleError {})
    }
}

struct Board {
    values: [u8; 25],
    marked: [bool; 25]
}

impl Board {
    /**
     * mark the draw on the board, check for winning, if it makes it a winning board, return the score, else None
     */
    fn mark_value(& mut self, draw: u8) -> Option<i32> {
        for i in 0..25 {
            if self.values[i] == draw && !self.marked[i] {
                self.marked[i] = true;
                // check win
                if self.check_win() {
                    return Some(self.calc_score());
                }                
            }
        }
        None
    }

    fn check_win(&self) -> bool {
        // check rows
        for i in 0..5 {
            let mut win = true;
            for j in 0..5 {
                win &= self.marked[i*5+j];
            }
            if win {
                return true;
            }
        }

        // check columns
        for i in 0..5 {
            let mut win = true;
            for j in 0..5 {
                win &= self.marked[i+j*5];
            }
            if win {
                return true;
            }
        }

        false
    }

    fn calc_score(&self) -> i32 {
        let mut score: i32 = 0;
        for i in 0..25 {
            if !self.marked[i] {
                score += self.values[i] as i32;
            }
        }
        score
    }
}

pub struct Day4Puzzle2 {

}

impl Puzzle for Day4Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 4 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day04/input_tim.txt");
        Box::new(bytes)
    }
    fn run(&self) -> Result<String, PuzzleError> {
        let mut draws: Vec<u8> = Vec::new();
        let mut boards: Vec<Board> = Vec::new();
        get_draws_and_boards(self.get_lines(), & mut draws, & mut boards);
        let mut last_board = false; // toggle on when we detect that we're playing the only remaining board
        for draw in draws {
            let mut removes: Vec<usize> = Vec::new();
            for (i, b) in &mut boards.iter_mut().enumerate() {
                if let Some(score) = b.mark_value(draw) {
                    removes.push(i);
                    if last_board {
                        return Ok((score * draw as i32).to_string());
                    }
                }
            }
            // boards [a, b, c, d]
            // removes [1, 3]
            // removes.pop() -> 3
            // swap_remove(3)
            // boards [a, b, c]
            // removes [1]
            // removes.pop() -> 1
            // swap_remove(1)
            // boards [a, c]

            // remove the ones that would have won
            while let Some(r) = removes.pop() {
                boards.swap_remove(r);
                // normally a swap remove could create a problem with ordering
                // but since we pop() from removes and it's ordered, we are guarenteed
                // that none of the indexes in the removes array will be affected by a removal
                // of an earlier board
            }

            // now check if it's the last board and set the flag
            if boards.len() == 1 {
                last_board = true;
            }

        }
        Err(PuzzleError {})
    }
}