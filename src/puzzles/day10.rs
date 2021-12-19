use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;
use std::collections::VecDeque;
use std::collections::HashMap;

pub struct Day10Puzzle1 {

}

impl Puzzle for Day10Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 10 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day10/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();

        let mut closing_chars: HashMap<char, u32> = HashMap::new();
        closing_chars.insert(')', 3);
        closing_chars.insert(']', 57);
        closing_chars.insert('}', 1197);
        closing_chars.insert('>', 25137);

        let (corrupt_array, _) = build_lines_collections(lines);

        // tada. now read the last character in each of the chunk_array and add the point values up
        let mut result = 0;
        for chunk in corrupt_array {
            // println!("reading chunk: {:?}", chunk);
            if let Some(last_char) = chunk.back() {
                if let Some(syntax_error) = closing_chars.get(last_char) {
                    // println!("syntax error value is {}", syntax_error);
                    result += syntax_error;
                }
            }
        }
        Ok(result.to_string())
    }
}

fn build_lines_collections(lines: Lines<BufReader<&[u8]>>) -> (Vec<VecDeque<char>>, Vec<VecDeque<char>>) {

    // key is opening character, value ending char
    let mut opening_chars: HashMap<char, char> = HashMap::new();
    opening_chars.insert('(', ')');
    opening_chars.insert('[', ']');
    opening_chars.insert('{', '}');
    opening_chars.insert('<', '>');

    let mut corrupt_array: Vec<VecDeque<char>> = Vec::new();
    let mut incomplete_array: Vec<VecDeque<char>> = Vec::new();
    for l in lines {
        let mut next_deque: VecDeque<char> = VecDeque::new();
        let mut next_expected_close: Option<char> = None;
        let mut corrupt = false;
        for c in l.expect("error").chars() {
            // println!("\nprocessing {}", c);
            if opening_chars.keys().any(|x| x == &c ) {
                // println!("pushing {} onto next_deque", c);
                next_deque.push_back(c);
                next_expected_close = Some(*opening_chars.get(&c).unwrap());
                // println!("next_deque is now {:?}", next_deque);
                // println!("next expected close is {:?}", next_expected_close);
                continue;
            } else if let Some(nec) = next_expected_close {
                if c == nec {
                    // println!("found correct close: {}", c);
                    next_deque.pop_back();
                    let oc: char = *next_deque.back().unwrap_or(&'x');
                    let nec = opening_chars.get(&oc);
                    next_expected_close = nec.copied();
                    // println!("next_deque is now {:?}", next_deque);
                    // println!("next expected close is {:?}", next_expected_close);
                    continue;
                }
            } else {
                // trying to close a chunk before ever opening one!
                // fall through to the wrong closing handling
            }
            // println!("didn't find the correct close. expected: {:?} but found {}", next_expected_close, c);
            // if we didn't continue, then we didn't get what we expect. save the progress to chunk_array and break
            next_deque.push_back(c);
            corrupt_array.push(next_deque.clone());
            corrupt = true;
            break;
        }
        if !corrupt {
            // println!("added incomplete line: {:?}", next_deque);
            incomplete_array.push(next_deque);
        }
    }
    (corrupt_array, incomplete_array)
}

pub struct Day10Puzzle2 {

}

impl Puzzle for Day10Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 10 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day10/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        let lines = self.get_lines();
        let mut point_system: HashMap<char, u64> = HashMap::new();
        point_system.insert('(', 1);
        point_system.insert('[', 2);
        point_system.insert('{', 3);
        point_system.insert('<', 4);
        let mut scores_table: Vec<u64> = Vec::new();
        let (_, incomplete_array) = build_lines_collections(lines);

        // hooray, now read off the point values
        for mut incomplete in incomplete_array {
            // score this incomplete line
            let mut score: u64 = 0;
            while let Some(n) = incomplete.pop_back() {
                score = score * 5 + point_system.get(&n).unwrap();
            }
            scores_table.push(score);
        }

        scores_table.sort_unstable();
        // println!("scores table: {:?}", scores_table);
        Ok(scores_table[scores_table.len().checked_div(2).unwrap()].to_string())
    }
}