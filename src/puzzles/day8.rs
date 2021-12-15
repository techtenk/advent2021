use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

pub struct Day8Puzzle1 {

}

impl Puzzle for Day8Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 8 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day08/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
    
        Ok(run_puzzle(lines, true).to_string())
    }
}

pub struct Day8Puzzle2 {

}

impl Puzzle for Day8Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 8 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day08/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
    
        Ok(run_puzzle(lines, false).to_string())
    }
}

fn  run_puzzle(lines: Lines<BufReader<&[u8]>>, first_puzzle: bool) -> u32 {
    let mut result = 0;
    let specials: [usize; 4] = [2, 3, 4, 7];
    for l in lines {
        let parsed = l.expect("error");
        if first_puzzle {
            result += solve_puzzle(parsed, |l| {
                let mut output = false;
                let mut r = 0;
                for s in l.split(' ') {
                    if !output && s.trim() != '|'.to_string() { continue; } else { output = true };
                    if specials.iter().any(|special| s.trim().len() == *special) {
                        r += 1;
                    }
                }
                r
            });
        } else {
            result += solve_puzzle(parsed, |l| {
                let mut is_output = false;
                let mut input: Vec<String> = Vec::new();
                let mut output: Vec<String> = Vec::new();
                for s in l.split(' ') {
                    if !is_output && s.trim() != '|'.to_string() { input.push(s.trim().to_string()) } else { is_output = true; };
                    if is_output && s.trim() != '|'.to_string() { output.push(s.trim().to_string()); }
                }
                // println!("output {:?}", output);
                let mut patterns: [&str; 10] = [""; 10];
                // now in the input find the special numbers
                let mut all = input.clone();
                all.extend(output.clone());
                for x in all.iter() {
                    match x.len() {
                        2 => {
                            // the "1" digit
                            patterns[1] = x;
                        },
                        3 => {
                            // the "7" digit
                            patterns[7] = x;
                        },
                        4 => {
                            // the "4" digit
                            patterns[4] = x;
                        },
                        7 => {
                            // the "8" digit
                            patterns[8] = x;
                        },
                        _ => ()
                    }
                }
                
                let mut solved = false;
                while !solved {
                    // println!("patterns is {:?}", patterns);
                    for x in input.iter() {
                        match x.len() {
                            5 => {
                                // 2, 3 or 5
                                let x_chars = x.chars().collect::<Vec<char>>();
                                // it's 3 if it contains the digits for 1
                                if !patterns[1].is_empty() && patterns[1].chars().all(|c| x_chars.contains(&c)) {
                                    patterns[3] = x;
                                }
                                // if it shares exactly 3 segments with 4, then it's a 5
                                else if !patterns[4].is_empty() && patterns[4].chars().fold(0, |carry, c| {
                                    // println!("is {} in {}?", c, x);
                                    if x_chars.contains(&c) {
                                        return carry + 1;
                                    }
                                    carry
                                }) == 3 {
                                    patterns[5] = x;
                                } else if !patterns[4].is_empty() && !patterns[1].is_empty() {
                                    // if there was a pattern for "4" to check against and we got to here, then it's process of elimination, it must be a 2
                                    patterns[2] = x;
                                }
                            },
                            6 => {
                                // 6, 9
                                let x_chars = x.chars().collect::<Vec<char>>();
                                // it's 9 if it contains the digits for 3
                                if !patterns[3].is_empty() && patterns[3].chars().all(|c| x_chars.contains(&c)) {
                                    patterns[9] = x;
                                }
                                // it's 6 if it contains the digits for 5
                                else if !patterns[5].is_empty() && patterns[5].chars().all(|c| x_chars.contains(&c)) {
                                    patterns[6] = x;
                                } else if !patterns[3].is_empty() && !patterns[5].is_empty() {
                                    patterns[0] = x;
                                }
                            },
                            _ => ()
                        }
                    }

                    // check if it's solved, it is if all the output patterns are represented in the patterns array
                    solved = output.iter().all(|o| {
                        let mut found = false;
                        patterns.iter().for_each(|p| {
                            // compare p (the input pattern) with o (the output pattern), if they contain all the same letters, we have that output
                            let pattern_chars = p.chars().collect::<Vec<char>>();
                            if !found && o.len() == p.len() { found = o.chars().all(|c| pattern_chars.contains(&c)); }
                            // if found { println!("found output {}", o)  }
                        });
                        // if !found { println!("did not find output {} ... patterns is {:?}", o, patterns);};
                        found
                    });
                }
                // println!("found all the digits: {:?}", patterns);
                // now make a lookup map
                // let mut lookup: HashMap<String, String> = HashMap::new();
                // patterns.iter().enumerate().for_each(|(k, v)| { lookup.insert(v.to_string(), k.to_string()); });

                let mut result: String = "".to_string();
                // now we have all letters translated, "read the output"
                println!("\npatterns: {:?}", patterns);
                println!("output: {:?}", output);
                for o in output {
                    // println!("check output {}", o);
                    // again find the output in the patterns array
                    for (i, p) in patterns.iter().enumerate() {
                        // println!("check if {} is in {:?}", o, patterns);
                        let pattern_chars = p.chars().collect::<Vec<char>>();
                        if o.len() == p.len() && o.chars().all(|c| pattern_chars.contains(&c)) {
                            println!("push to result: {}", i);
                            result.push_str(&i.to_string());
                        }
                    }
                }
                println!("result is {}", result);
                result.parse::<u32>().unwrap()
            });
        }
        
    }
    result
}

fn solve_puzzle<T>(line: String, puzzle_closure: T) -> u32
where T: Fn(String) -> u32
{
    puzzle_closure(line)
}