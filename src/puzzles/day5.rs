use crate::errors::PuzzleError;
use crate::Puzzle;
use std::collections::HashMap;
use std::io::Lines;
use std::io::BufReader;

fn count_overlaps(lines: Lines<BufReader<&[u8]>>, include_diags: bool) -> usize {
    let mut vents: Vec<[u16; 4]> = Vec::new();
    for line in lines {
        let instruction = line.unwrap_or_else(|_| " ".to_string());
        let first_coord_string = instruction.split("->").next().expect("error");
        let sec_coord_string = instruction.split("->").nth(1).expect("error");
        let x1 = first_coord_string.split(',').next().expect("error").trim().parse::<u16>().expect("error");
        let y1 = first_coord_string.split(',').nth(1).expect("error").trim().parse::<u16>().expect("error");
        let x2 = sec_coord_string.split(',').next().expect("error").trim().parse::<u16>().expect("error");
        let y2 = sec_coord_string.split(',').nth(1).expect("error").trim().parse::<u16>().expect("error");

        vents.push([x1, y1, x2, y2]);

    }

    // remove all vents that aren't horizontal or vertical
    if !include_diags {
        vents = vents.drain_filter(|v| v[0] == v[2] || v[1] == v[3]).collect();
    }

    // for vent in vents {
    //     println!("{:?}", vent);
    // }
    // check each vent against all the others, return and fill out the HashMap of intersects
    // key is [x,y] and the value is the number of intersects
    let mut intersections: HashMap<[u16; 2], u8> = HashMap::new();
    let mut points_hash: HashMap<[u16; 4], Vec<[u16; 2]>> = HashMap::new();
    for (i, vent) in vents.iter().enumerate() {
        // println!("working on vent: {:?}", vent);
        if let Some(remaining_vents) = vents.get((i+1)..vents.len()) {
            for v in remaining_vents {
                let intersects: Vec<[u16; 2]> = get_intersects(& mut points_hash, vent, v);
                // now we have the points that intersect, let's add/increment the HashMap
                for i in intersects {
                    let count = intersections.entry(i).or_insert(1);
                    *count += 1;
                }
            }
        }
        
    }
    // println!("{:?}", intersections);
    intersections.len()
}

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
        
        let lines = self.get_lines();
        
        Ok(count_overlaps(lines, false).to_string())
    }
}

pub struct Day5Puzzle2 {

}

impl Puzzle for Day5Puzzle2 {
    fn get_puzzle_name(&self) -> &'static str {
        "Day 5 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day05/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        Ok(count_overlaps(lines, true).to_string())
    }
}

fn get_intersects(points_hash: & mut HashMap<[u16; 4], Vec<[u16; 2]>>, vent1: &[u16; 4], vent2: &[u16; 4]) -> Vec<[u16; 2]> {
    let mut intersects = Vec::new();
    // now just check if vent1 and vent2 share any points
    // println!("get_intersects for {:?} and {:?}", vent1, vent2);
    
    if !points_hash.contains_key(vent1) {
        points_hash.insert(*vent1, get_points(vent1));
    }
    if !points_hash.contains_key(vent2) {
        points_hash.insert(*vent2, get_points(vent2));
    }
    let mut all_points: Vec<[u16; 2]> = points_hash.get(vent1).expect("error").to_vec();
    let vent2_points: Vec<[u16; 2]> = points_hash.get(vent2).expect("error").to_vec();
    all_points.extend(vent2_points);
    all_points.sort_unstable();
    // println!("now find intersects in {:?}", all_points);
    let mut last_point: Option<[u16; 2]> = None;
    all_points.iter().for_each(|p| {
        if let Some(lp) = last_point {
            if *p == lp {
                intersects.push(*p);
            }
        }
        last_point = Some(*p);
    });
    // println!("intersects: {:?}", intersects);
    intersects
}

fn get_points(vent: &[u16; 4]) -> Vec<[u16; 2]> {
    let mut points = Vec::new();

    let x_vec: Vec<u16>;
    let y_vec: Vec<u16>;
    if vent[0] < vent[2] {
        x_vec = (vent[0]..=vent[2]).collect::<Vec<_>>();
    } else {
        x_vec = (vent[2]..=vent[0]).rev().collect::<Vec<_>>();
    }
    if vent[1] < vent[3] {
        y_vec = (vent[1]..=vent[3]).collect::<Vec<_>>();
    } else {
        y_vec = (vent[3]..=vent[1]).rev().collect::<Vec<_>>();
    }

    if x_vec.len() > 1 && y_vec.len() == 1 {
        // horizontal line
        let y = y_vec.first().expect("error");
        for x in x_vec {
            points.push([x, *y]);
        }
    } else if y_vec.len() > 1 && x_vec.len() == 1{
        // vertical line
        let x = x_vec.first().expect("error");
        for y in y_vec {
            points.push([*x, y]);
        }
    } else {
        // println!("found diagonal line");
        // diagonal line
        for (i, x) in x_vec.iter().enumerate() {
            points.push([*x, *y_vec.get(i).expect("error")]);
        }
    }
    // println!("found points for {:?}: {:?}", vent, points);
    points
}