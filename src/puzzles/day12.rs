use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct Day12Puzzle1 {

}

impl Puzzle for Day12Puzzle1 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 12 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day12/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        let cave_map = Arc::new(build_map(lines));

        let result = find_number_paths_to_end(cave_map, "start".to_string(), Vec::new(), false);
        
        Ok(result.to_string())
    }
}

fn find_number_paths_to_end(map: Arc<HashMap<String, Cave>>, current_cave: String, visited: Vec<String>, puzzle2: bool) -> u32 {
    // base cases
    if current_cave == "end" {
        // println!("{:?}", visited);
        return 1;
    }
    // for each valid connection
    let map_ref = Arc::clone(&map);
    let current = map_ref.get(&current_cave).expect("error");
    let mut thread_handles: Vec<JoinHandle<u32>> = Vec::new();
    let mut finished_threads = 0;
    let mut visited_sorted = visited.clone();
    visited_sorted.push(current_cave.clone());
    visited_sorted.sort_unstable();
    // println!("visited_sorted: {:?}", visited_sorted);
    for conn in &current.connections {
        // if the connection is an invalid step, such as:
        // visiting a small cave we have already visited
        // visiting "start" again -- since start is small, we'll just handle it the same
        // then dead end, skip this one
        if visited.contains(conn) {
            
            // if it's small, continue, else fall through to create a new thread
            let next_cave = map_ref.get(conn).expect("error");
            if !next_cave.is_large() {
                if puzzle2 {
                    // now if it's puzzle 2 we also need to check if this is the first small cave to be visited twice
                    if conn == "start" {
                        // of course can't visit start twice regardless
                        continue;
                    }

                    let mut last = "start".to_string();
                    let mut invalid = false;
                    for v in visited_sorted.iter() {
                        if last == "start" || Cave::new(v.to_string()).is_large() {
                            last = v.clone();
                        } else {
                            if *v == last {
                                // there has been a small cave visited twice, so this small cave cannot be
                                invalid = true;
                                break;
                            }
                            last = v.clone();
                        }
                    }
                    if invalid {
                        continue;
                    }
                } else {
                    continue; // if it's invalid, continue to skip it
                }
            }
            
            
        }
        // otherwise spin up a recursive thread
        let mut new_visited = visited.clone();
        new_visited.push(current_cave.clone());
        let thread_map_ref = Arc::clone(&map);
        let conn_clone = (*conn).clone();
        // println!("I'll spawn a thread for {} to {}", current_cave, conn);
        // let's spawn a new thread for each 4 steps, otherwise just use this thread
        if visited.len() % 4 == 0 {
            let handle = std::thread::spawn(move || {
                find_number_paths_to_end(thread_map_ref, conn_clone, new_visited, puzzle2)
            });
            thread_handles.push(handle);
        } else {
            finished_threads += find_number_paths_to_end(Arc::clone(&map), conn.to_string(), new_visited, puzzle2);
        }
        
    }
    finished_threads + thread_handles.into_iter().fold(0, |carry, h| { carry + h.join().unwrap()})
}

pub struct Day12Puzzle2 {

}

impl Puzzle for Day12Puzzle2 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 12 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day12/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        let cave_map = Arc::new(build_map(lines));

        let result = find_number_paths_to_end(cave_map, "start".to_string(), Vec::new(), true);
        
        Ok(result.to_string())
    }
}

fn build_map(lines: Lines<BufReader<&[u8]>>) -> HashMap<String, Cave> {
    let mut map: HashMap<String, Cave> = HashMap::new();
    for l in lines {
        let parts = l.expect("error");
        let mut split = parts.split('-');
        let first_cave = split.next().expect("error");
        let second_cave = split.next().expect("error");
        // first cave
        let first_cave_entry = match map.entry(first_cave.to_string()) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(Cave::new(first_cave.to_string()))
        };
        first_cave_entry.add_connection(second_cave.to_string());

        // second cave
        let second_cave_entry = match map.entry(second_cave.to_string()) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(Cave::new(second_cave.to_string()))
        };
        second_cave_entry.add_connection(first_cave.to_string());
    
    }
    map
}

struct Cave {
    large: bool,
    connections: Vec<String>
}

impl Cave {
    fn new(name: String) -> Self {
        let large = name.to_ascii_lowercase() != name;
        Cave {
            large,
            connections: Vec::new()
        }
    }
    fn is_large(&self) -> bool {
        self.large
    }
    fn add_connection(& mut self, other: String) {
        self.connections.push(other);
    }
}