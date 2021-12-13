use crate::errors::PuzzleError;
use crate::Puzzle;
use std::collections::HashMap;

pub struct Day5Puzzle1 {

}

impl Puzzle for Day5Puzzle1 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 5 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day05/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        let mut vents: Vec<[u16; 4]> = Vec::new();
        let lines = self.get_lines();
        for line in lines {
            let instruction = line.unwrap_or_else(|_| " ".to_string());
            let first_coord_string = instruction.split("->").next().expect("error");
            let sec_coord_string = instruction.split("->").nth(1).expect("error");
            let x1 = first_coord_string.split(',').next().expect("error").trim().parse::<u16>().expect("error");
            let y1 = first_coord_string.split(',').nth(1).expect("error").trim().parse::<u16>().expect("error");
            let x2 = sec_coord_string.split(',').next().expect("error").trim().parse::<u16>().expect("error");
            let y2 = sec_coord_string.split(',').nth(1).expect("error").trim().parse::<u16>().expect("error");

            if x1 > x2 || y1 > y2 {
                // swap the points so it's always less to more
                vents.push([x2, y2, x1, y1]);
            } else {
                vents.push([x1, y1, x2, y2]);
            }
        }

        // remove all vents that aren't horizontal or vertical
        vents = vents.drain_filter(|v| v[0] == v[2] || v[1] == v[3]).collect();

        // for vent in vents {
        //     println!("{:?}", vent);
        // }
        // check each vent against all the others, return and fill out the HashMap of intersects
        // key is [x,y] and the value is the number of intersects
        let mut intersections: HashMap<[u16; 2], u8> = HashMap::new();
        for (i, vent) in vents.iter().enumerate() {
            // println!("working on vent: {:?}", vent);
            if let Some(remaining_vents) = vents.get((i+1)..vents.len()) {
                for v in remaining_vents {
                    let intersects: Vec<[u16; 2]> = get_intersects(vent, v);
                    // now we have the points that intersect, let's add/increment the HashMap
                    for i in intersects {
                        let count = intersections.entry(i).or_insert(1);
                        *count += 1;
                    }
                }
            }
            
        }
        // println!("{:?}", intersections);
        Ok(intersections.len().to_string())
    }
}

fn get_intersects(vent1: &[u16; 4], vent2: &[u16; 4]) -> Vec<[u16; 2]> {
    let mut intersects = Vec::new();
    // now just check if vent1 and vent2 share any points
    // println!("get_intersects for {:?} and {:?}", vent1, vent2);
    for x1 in vent1[0]..=vent1[2] {
        // println!("x1 = {}", x1);
        for y1 in vent1[1]..=vent1[3] {
            // next point to check in vent1 is x1, y1
            // print!("checking [{}, {}] against ", x1, y1);
            for x2 in vent2[0]..=vent2[2] {
                for y2 in vent2[1]..=vent2[3] {
                    // print!("\t[{}, {}]\n", x2, y2);
                    // next point to check in vent2 is x2, y2
                    if x1 == x2 && y1 == y2 {
                        // println!("found intersect: {}, {}", x1, x2);
                        intersects.push([x1, y1]);
                    }
                }
            }
        }
    }
    intersects
}