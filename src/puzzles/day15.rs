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
        let graph: Vec<Vec<u32>> = parse_input::<100>(&mut lines);
        // println!("parsed the graph");

        let mut shortest_path_tree_set: Vec<bool> = vec![false; 10000];
        let mut remaining_nodes: Vec<u32> = Vec::new();
        for i in 1..10000 {
            remaining_nodes.push(i);
        }
        let mut distance_graph: Vec<u32> = vec![u32::MAX; 10000];

        distance_graph[0] = 0;

        for _ in 0..10000 {
            // find the shortest distance from src to node
            // by checking it's adjacent nodes and current min distance
            // only update it's distance in distance_graph if the distance found is less than
            // what exists already
            let short_index = min_distance::<10000>(&distance_graph, &shortest_path_tree_set);

            // mark this index as processed
            shortest_path_tree_set[short_index] = true;

            // update the neighbors
            for i in 0..10000 {
                if !shortest_path_tree_set[i] // if the node isn't finished
                && graph[short_index][i] > 0 // there is an edge from this node to that (it's a neighbor)
                && distance_graph[short_index] != u32::MAX // check so we don't overflow
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
fn min_distance<const SIZE: usize>(distance_graph: &[u32], spt_set: &[bool]) -> usize {
    let mut min = u32::MAX;
    let mut min_index = 0;
    for (i, v) in spt_set.iter().enumerate() {
        if !v && distance_graph[i] <= min {
            min = distance_graph[i];
            min_index = i;
        }
    }
    min_index as usize
}

fn parse_input<const SIZE: usize>(lines: & mut Lines<BufReader<&[u8]>>) -> Vec<Vec<u32>> {
    // const INIT: Vec<u32> = Vec::new();
    let mut input_chart: Vec<Vec<u32>> = Vec::new();
    for (i, l) in lines.enumerate() {
        input_chart.push(Vec::new());
        for c in l.expect("error").chars() {
            input_chart[i].push(c.to_digit(10).expect("error") as u32);
        }
    }

    // println!("trying to allocate return array");
    let mut return_array: Vec<Vec<u32>> = Vec::new();
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

fn parse_input_puzzle2<const SIZE: usize>(lines: & mut Lines<BufReader<&[u8]>>) -> Vec<Vec<u32>> {
    // const INIT: Vec<u32> = Vec::new();
    let mut input_chart: Vec<Vec<u32>> = Vec::new();
    for (i, l) in lines.enumerate() {
        input_chart.push(Vec::new());
        for c in l.expect("error").chars() {
            input_chart[i].push(c.to_digit(10).expect("error") as u32);
        }
    }

    input_chart
}

pub struct Day15Puzzle2 {

}

impl Puzzle for Day15Puzzle2 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 15 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day15/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let mut lines = self.get_lines();
        let graph: RiskGraph = RiskGraph::new::<100>(&mut lines);

        // print the risk graph
        // for i in 0..2500 {
            
        //     if i % 50 == 0 {
        //         println!();
        //     }
        //     print!("{}", graph.get_individ_risk(i));
        // }

        // println!("parsed the graph");
        let mut shortest_path_tree_set: Vec<bool> = vec![false; 250000];
        let mut remaining_nodes: Vec<u32> = Vec::new();
        for i in 1..250000 {
            remaining_nodes.push(i);
        }
        let mut distance_graph: Vec<u32> = vec![u32::MAX; 250000];

        distance_graph[0] = 0;

        for _ in 0..250000 {
            // find the shortest distance from src to node
            // by checking it's adjacent nodes and current min distance
            // only update it's distance in distance_graph if the distance found is less than
            // what exists already
            let short_index = min_distance::<250000>(&distance_graph, &shortest_path_tree_set);

            // mark this index as processed
            shortest_path_tree_set[short_index] = true;

            // update the neighbors
            for i in 0..250000 {
                if !shortest_path_tree_set[i] // if the node isn't finished
                && graph.get(short_index, i) > 0 // there is an edge from this node to that (it's a neighbor)
                && distance_graph[short_index] != u32::MAX // check so we don't overflow
                && distance_graph[short_index] + graph.get(short_index, i) < distance_graph[i] { // it's actually a shorter path
                    distance_graph[i] = distance_graph[short_index] + graph.get(short_index, i); // update the shortest path to that node from src
                }
            }
        }

        // println!("{:?}", distance_graph);
        Ok(distance_graph[249999].to_string())
    }
}

#[allow(non_snake_case)]
struct RiskGraph {
    graph: Vec<Vec<u32>>,
    SIZE: usize
}

// RiskGraph only works for puzzle 2. Project the original graph onto a 5x5 grid
impl RiskGraph {
    fn new<const SIZE: usize>(lines: & mut Lines<BufReader<&[u8]>>) -> Self {
        RiskGraph {
            graph: parse_input_puzzle2::<SIZE>(lines),
            SIZE: SIZE * 5
        }
    }

    // specific to puzzle 2. Do the logic to project the 5x5 grid
    fn get(&self, origin: usize, dest: usize) -> u32 {
        if !self.are_neighbors(origin, dest) {
            return 0;
        }
        self.get_individ_risk(dest)
    }

    fn get_individ_risk(&self, node: usize) -> u32 {
        let col: usize = node % self.SIZE;
        let row: usize = node / self.SIZE;
        let mut risk_add: u32 = col as u32 / (self.SIZE / 5) as u32;
        risk_add += row as u32 / (self.SIZE / 5) as u32;
        let risk_before_rollover = self.graph[row % (self.SIZE / 5)][col % (self.SIZE / 5)] + risk_add;
        if risk_before_rollover > 9 {
            return risk_before_rollover % 9;
        }
        risk_before_rollover
    }

    fn are_neighbors(&self, a: usize, b: usize) -> bool {
        // left neighbor
        if a % self.SIZE > 0 && a-1 == b {
            return true;
        }
        // right neighbor
        if a % self.SIZE < self.SIZE-1 && a+1 == b {
            return true;
        }
        // top neighbor
        if a / self.SIZE > 0 && a - self.SIZE == b {
            return true;
        }
        // bottom neighbor
        if a / self.SIZE < self.SIZE-1 && a+self.SIZE == b {
            return true;
        }
        false
    }
}