use crate::errors::PuzzleError;
use crate::Puzzle;
use std::io::Lines;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::LinkedList;

pub struct Day14Puzzle1 {

}

impl Puzzle for Day14Puzzle1 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 14 Puzzle 1"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day14/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        let (template, rules) = parse_input(lines);

        // build the intial polymer
        let mut polymer: LinkedList<char> = LinkedList::new();
        for char in template.chars() {
            polymer.push_back(char);
        }

        for _ in 0..10 {
            apply_rules(&rules, & mut polymer);
        }

        let counts: HashMap<char, u32> = count_letters(&polymer);
        let mut max_count = 0;
        let mut min_count = std::u32::MAX;
        counts.iter().for_each(|entry| {
            if *entry.1 > max_count {
                max_count = *entry.1;
            }
            if *entry.1 < min_count {
                min_count = *entry.1;
            }
        });
        // println!("Template: {}\nRules:{:?}", template, rules);
        // println!("Polymer after x steps: {:?}", polymer);
        Ok((max_count - min_count).to_string())
    }
}

pub struct Day14Puzzle2 {

}

impl Puzzle for Day14Puzzle2 {

    fn get_puzzle_name(&self) -> &'static str {
        "Day 14 Puzzle 2"
    }
    fn get_input(&self) -> Box<&'static [u8]> {
        let bytes = include_bytes!("../../Day14/input_tim.txt");
        Box::new(bytes)
    }
    fn run (&self) -> Result<String, PuzzleError> {
        
        let lines = self.get_lines();
        
        let (template, rules) = parse_input(lines);

        let mut current_pairs: HashMap<String, u64> = HashMap::new();
        let mut last = template.chars().next().unwrap();
        let mut counts: HashMap<char, u64> = HashMap::new();
        
        *counts.entry(last).or_insert(0) += 1;
        for c in template.chars().skip(1) {
            let mut key = last.to_string();
            key.push_str(&c.to_string());
            let entry = current_pairs.entry(key).or_insert(0);
            *entry += 1;
            *counts.entry(c).or_insert(0) += 1;
            last = c;
        }
        // println!("initial counts: {:?}", counts);
        // println!("initial current pairs: {:?}", current_pairs);
        for _ in 0..40 {
            // apply rules
            let mut new_pairs: HashMap<String, u64> = HashMap::new();
            for rule in rules.iter() {
                if current_pairs.contains_key(rule.0) {
                    // println!("applying rule: {:?}", rule);
                    // zero out that entry and add volumes of two new pairs
                    let entry = current_pairs.get_mut(rule.0).unwrap();
                    let num_pairs = *entry;
                    *entry = 0;

                    // count the new letter we are inserting
                    // println!("adding {} {} to count", num_pairs, *rule.1);
                    *counts.entry(*rule.1).or_insert(0) += num_pairs;

                    // two new pairs are (first char of rule.0, rule value) and (rule value, second char of rule.0)
                    let mut first_pair = rule.0.chars().next().unwrap().to_string();
                    first_pair.push_str(&rule.1.to_string());
                    let mut second_pair = rule.1.to_string();
                    second_pair.push_str(&rule.0.chars().nth(1).unwrap().to_string());

                    // insert them into new pairs, which will update current_pairs after we've completed all the rules
                    *new_pairs.entry(first_pair).or_insert(0) += num_pairs;
                    *new_pairs.entry(second_pair).or_insert(0) += num_pairs;
                }
            }
            // now update new_pairs
            // println!("new_pairs is now: {:?}", new_pairs);
            new_pairs.iter().for_each(|entry| {
                *current_pairs.entry(entry.0.to_string()).or_insert(0) += entry.1;
            });
            // println!("current_pairs is now: {:?}", current_pairs);
            // println!("count is now: {:?}", counts);
        }

        // println!("final counts: {:?}", counts);
        
        let max_count = counts.iter().fold(0,|carry, entry| {
            if *entry.1 > carry {
                return *entry.1;
            }
            carry
        });
        let min_count = counts.iter().fold(std::u64::MAX,|carry, entry| {
            if *entry.1 < carry {
                return *entry.1;
            }
            carry
        });
        
        // println!("Template: {}\nRules:{:?}", template, rules);
        // println!("Polymer after x steps: {:?}", polymer);
        Ok((max_count - min_count).to_string())
    }
}

fn count_letters(polymer: &LinkedList<char>) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for char in polymer {
        let entry = counts.entry(*char).or_insert(0);
        *entry += 1;
    }

    counts
}

fn apply_rules(rules: &HashMap<String, char>, polymer: & mut LinkedList<char>) {
    let mut cursor = polymer.cursor_front_mut();
    cursor.move_next();

    while let Some(next) = cursor.current().cloned() {
        let mut pair: String = cursor.peek_prev().cloned().unwrap().to_string();
        pair.push_str(&next.to_string());
        // println!("next pair is: {}", pair);
        if rules.contains_key(&pair) {
            let insertion = rules.get(&pair).expect("error");
            cursor.insert_before(*insertion);
        }
        cursor.move_next();
    }
}

fn parse_input(mut lines: Lines<BufReader<&[u8]>>) -> (String, HashMap<String, char>){
    let template = lines.next().unwrap().expect("error");

    let mut rules: HashMap<String, char> = HashMap::new();
    for l in lines {
        let l_parsed = l.expect("error");
        if l_parsed.contains("->") {
            let mut split = l_parsed.split("->"); 
            rules.insert(split.next().expect("error").trim().to_string(), split.next().expect("error").trim().chars().next().unwrap());
        }
        
    
    }
    (template, rules)
}