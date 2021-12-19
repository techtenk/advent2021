use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

pub struct Day9Puzzle1 {

}

impl Puzzle for Day9Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 9 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day09/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        let height_map = build_height_map(lines);

        Ok(assess_risk(height_map).to_string())
    }
}

pub struct Day9Puzzle2 {

}

impl Puzzle for Day9Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 9 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day09/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        let height_map = build_height_map(lines);

        let mut basins: Vec<Vec<Reading>> = Vec::new();

        // walk the map and add the Readings to new basins as needed
        for y in 0..100 {
            for x in 0..100 {
                let reading = height_map[x][y];
                if reading.unwrap().height.unwrap() == 9 {
                    continue;
                }
                // search all the current basins for it's neighbors, if we find any then add this reading to the same basin, otherwise create a new one
                let neighbors: Vec<Reading> = reading.unwrap().get_neighbors();
                let mut vec_to_create_or_merge: Vec<& mut Vec<Reading>> = Vec::new(); // create if this is empty, merge if it has more than one, otherwise just add
                for b in basins.iter_mut() {
                    // if any of the Readings in neighbor are also in b, then add b to vec_to_create_or_merge
                    if neighbors.iter().any(|neighbor| b.iter().any(|item| *item == *neighbor && height_map[neighbor.x][neighbor.y].unwrap().height.unwrap() < 9)) {
                        vec_to_create_or_merge.push(b);
                    }
                }

                // add, merge or create
                if vec_to_create_or_merge.is_empty() {
                    // println!("making new basin from {} {}", reading.unwrap().x, reading.unwrap().y);
                    basins.push(vec!(reading.unwrap()));
                } else if vec_to_create_or_merge.len() > 1 {
                    // println!("({},{}) matches more than one basin. Merge them.", x, y);
                    let (first_basin, rest) = vec_to_create_or_merge.split_at_mut(1);
                    
                    for v in rest.iter_mut() {
                        // println!("merging basin {:?} with {:?}", v, first_basin);
                        first_basin[0].extend(v.drain(0..));
                    }
                    // now add the reading to the merged basin
                    first_basin[0].push(reading.unwrap());
                    // remove empty vectors
                    basins.drain_filter(|basin| basin.is_empty());
                } else {
                    // println!("adding {} {} to basin: {:?}", reading.unwrap().x, reading.unwrap().y, vec_to_create_or_merge[0]);
                    // exactly one, just add the Reading to it
                    vec_to_create_or_merge[0].push(reading.unwrap());
                }
            }
        }

        // sort the basins by len
        basins.sort_unstable_by(|a, b| {
            b.len().cmp(&(a.len()))
        });
        let result = basins[0].len() * basins[1].len() * basins[2].len();
        Ok(result.to_string())
    }
}

fn build_height_map(lines: Lines<BufReader<&[u8]>>) -> [[Option<Reading>; 100]; 100] {
    let mut height_map: [[Option<Reading>; 100]; 100] = [[None; 100]; 100];
    for (y, l) in lines.enumerate() {
        let heights = l.expect("error");
        for (x, h_str) in heights.chars().enumerate() {
            let h = h_str.to_string().parse::<u32>().expect("error");
            height_map[x][y] = Some(Reading { x, y, height: Some(h) });
        }
    }
    height_map
}

fn assess_risk(height_map: [[Option<Reading>; 100]; 100]) -> u32 {
    let mut total_risk = 0;
    for x in 0..100 {
        for y in 0..100 {
            total_risk += height_map[x][y].unwrap().get_risk_level(&height_map);
        }
    }
    total_risk
}

#[derive(Copy,Clone,Debug)]
struct Reading {
    x: usize,
    y: usize,
    height: Option<u32>
}

impl Reading {
    fn get_risk_level(&self, map: &[[Option<Reading>; 100]; 100]) -> u32 {
        // if it's the lowest of all it's neighbors, return height+1,  else fall to returning 0
        if self.is_lowpoint(map) {
            return self.height.unwrap() + 1;
        }
        0 // else no risk
    }

    fn is_lowpoint(&self, map: &[[Option<Reading>; 100]; 100]) -> bool {
        // check neighbors
        // left neighbor
        if self.x > 0 && map[self.x-1][self.y].unwrap().height <= self.height {
            return false;
        }
        // right neighbor
        if self.x < 99 && map[self.x+1][self.y].unwrap().height <= self.height {
            return false;
        }
        // top neighbor
        if self.y > 0 && map[self.x][self.y-1].unwrap().height <= self.height {
            return false;
        }
        // bottom neighbor
        if self.y < 99 && map[self.x][self.y+1].unwrap().height <= self.height {
            return false;
        }
        true
    }

    fn get_neighbors(&self) -> Vec<Reading> {
        let mut return_val = Vec::new();
        // top
        if self.y > 0 {
            return_val.push(Reading { x: self.x, y: self.y - 1, height: None });
        }
        // bottom
        if self.y < 99 {
            return_val.push(Reading { x: self.x, y: self.y + 1, height:  None });
        }
        // left
        if self.x > 0 {
            return_val.push(Reading { x: self.x - 1, y: self.y, height: None });
        }
        if self.x < 99 {
            return_val.push(Reading { x: self.x + 1, y: self.y, height: None });
        }
        // right
        return_val
    }
}

impl PartialEq for Reading {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}