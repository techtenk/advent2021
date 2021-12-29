use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;

pub struct Day15Puzzle1 {

}

impl Puzzle for Day15Puzzle1 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 15 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day15/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let mut lines = self.get_lines();
        let graph: Vec<Vec<u16>> = parse_input::<100>(&mut lines);
        // println!("parsed the graph");
        let mut shortest_path_tree_set: Box<[bool; 10000]> = Box::new([false; 10000]);
        let mut remaining_nodes: Vec<u16> = Vec::new();
        for i in 1..10000 {
            remaining_nodes.push(i);
        }
        let mut distance_graph: Box<[u16; 10000]> = Box::new([u16::MAX; 10000]);

        distance_graph[0] = 0;

        for _ in 0..10000 {
            // find the shortest distance from src to node
            // by checking it's adjacent nodes and current min distance
            // only update it's distance in distance_graph if the distance found is less than
            // what exists already
            let short_index = min_distance(&distance_graph, &shortest_path_tree_set);

            // mark this index as processed
            shortest_path_tree_set[short_index] = true;

            // update the neighbors
            for i in 0..10000 {
                if !shortest_path_tree_set[i] // if the node isn't finished
                && graph[short_index][i] > 0 // there is an edge from this node to that (it's a neighbor)
                && distance_graph[short_index] != u16::MAX // check so we don't overflow
                && distance_graph[short_index] + graph[short_index][i] < distance_graph[i] { // it's actually a shorter path
                    distance_graph[i] = distance_graph[short_index] + graph[short_index][i]; // update the shortest path to that node from src
                }
            }
        }

        // println!("{:?}", distance_graph);
        Ok(distance_graph[9999].to_string())
    }
}

// find the node with the shortest distance from src that hasn't been finalized
fn min_distance(distance_graph: &[u16; 10000], spt_set: &[bool; 10000]) -> usize {
    let mut min = u16::MAX;
    let mut min_index = 0;
    for (i, v) in spt_set.iter().enumerate() {
        if !v && distance_graph[i] <= min {
            min = distance_graph[i];
            min_index = i;
        }
    }
    min_index as usize
}

fn parse_input<const SIZE: usize>(lines: & mut Lines<BufReader<&[u8]>>) -> Vec<Vec<u16>> {
    // const INIT: Vec<u16> = Vec::new();
    let mut input_chart: Vec<Vec<u16>> = Vec::new();
    for (i, l) in lines.enumerate() {
        input_chart.push(Vec::new());
        for c in l.expect("error").chars() {
            input_chart[i].push(c.to_digit(10).expect("error") as u16);
        }
    }

    // println!("trying to allocate return array");
    let mut return_array: Vec<Vec<u16>> = Vec::new();
    for _ in 0..SIZE*SIZE {
        return_array.push(vec![0; SIZE*SIZE]);
    }
    // println!("was able to allocate return array");
    for (i, node_risk) in input_chart.iter().flatten().enumerate() {
        // fill in the edge graph by looking at the risks of entering each node
        // each node defines up to four edges, leave the rest 0 for no connection
        let to_node = i;
        // traversing 'from' will be the row, 'to' will be the column
        // left neighbor
        let col = i % SIZE;
        let row = i / SIZE; // integer division

        // left neighbor
        if col > 0 {
            return_array[i-1][to_node] = *node_risk;
        }

        // right neighbor
        if col < SIZE - 1 {
            return_array[i+1][to_node] = *node_risk;
        }

        // top neighbor
        if row > 0 {
            return_array[i-SIZE][to_node] = *node_risk;
        }

        // bottom neighbor
        if row < SIZE - 1 {
            return_array[i+SIZE][to_node] = *node_risk;
        }
    }
    return_array
}
