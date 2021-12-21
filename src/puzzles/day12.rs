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
        let bytes = include_bytes!("../../Day12/input_test.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        let cave_map = Arc::new(build_map(lines));

        let result = find_number_paths_to_end(cave_map, "start".to_string(), Vec::new());
        
        Ok(result.to_string())
    }
}

fn find_number_paths_to_end(map: Arc<HashMap<String, Cave>>, current_cave: String, visited: Vec<String>) -> u32 {
    // base cases
    if current_cave == "end" {
        return 1;
    }
    // for each valid connection
    let map_ref = Arc::clone(&map);
    let current = map_ref.get(&current_cave).expect("error");
    let mut thread_handles: Vec<JoinHandle<u32>> = Vec::new();
    for conn in &current.connections {
        // if the connection is an invalid step, such as:
        // visiting a small cave we have already visited
        // visiting "start" again -- since start is small, we'll just handle it the same
        // then dead end, skip this one
        if visited.contains(conn) {
            // if it's small, continue, else fall through to create a new thread
            let next_cave = map_ref.get(conn).expect("error");
            if !next_cave.is_large() {
                continue;
            }
        }
        // otherwise spin up a recursive thread
        let mut new_visited = visited.clone();
        new_visited.push(current_cave.clone());
        let thread_map_ref = Arc::clone(&map);
        let conn_clone = (*conn).clone();
        // println!("I'll spawn a thread for {} to {}", current_cave, conn);
        let handle = std::thread::spawn(move || {
            find_number_paths_to_end(thread_map_ref, conn_clone, new_visited)
        });
        thread_handles.push(handle);
    }
    thread_handles.into_iter().fold(0, |carry, h| { carry + h.join().unwrap()})
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
    name: String,
    large: bool,
    connections: Vec<String>
}

impl Cave {
    fn new(name: String) -> Self {
        let large = name.to_ascii_lowercase() != name;
        Cave {
            name,
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
    fn is_connected(&self, other: String) -> bool {
        self.connections.iter().any(|conn| *conn == other)
    }
}