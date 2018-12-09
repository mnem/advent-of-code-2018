use std::fs;
use regex::Regex;
use petgraph::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(debug_assertions)]
use petgraph::dot::{Dot, Config};

fn main() {
    let input = read_input();
    let result = process(&input, 60, 5);
    println!("Result: {}\n", result);
}

fn read_input() -> String {
    let input_filename = String::from("input.txt");
    fs::read_to_string(input_filename)
        .expect("Failed to read file")
}

fn process(input: &str, step_base_time: i32, num_workers: usize) -> i32 {
    let instructions = create_instruction_graph(input);

    // Get all the possible start steps and use them as starting points
    let mut available = Vec::new();
    for ready_step in instructions.externals(Incoming) {
        available.push(ready_step);
    }

    // Setup the timer and workers
    let mut timer = -1;
    let mut workers = vec![ Worker { node: None, time_remaining: 0 }; num_workers];

    // Follow the instructions
    let mut steps = String::new();
    let mut complete = HashSet::new();
    while available.len() > 0 || workers.iter().any( |w| {w.time_remaining > 0} ) {
        // Sort the available steps so we process them in order
        available.sort_by(|a, b| {
            let a_step = instructions.node_weight(*a).unwrap();
            let b_step = instructions.node_weight(*b).unwrap();
            a_step.cmp(b_step)
        });

        // Tick the workers times
        for worker in &mut workers {
            if let Some(node) = worker.node {
                if worker.time_remaining > 0 {
                    worker.time_remaining -= 1;
                }

                if worker.time_remaining == 0 {
                    complete.insert(node);
                    steps += instructions.node_weight(node).unwrap();
                    worker.node = None;
                }
            }
        }

        // Allocate tasks
        for worker in &mut workers {
            if worker.node.is_none() {
                // Worker is free
                if let Some(next_step) = get_next_step(&instructions, &mut available, &complete) {
                    worker.node = Some(next_step);
                    let node_str = instructions.node_weight(next_step).unwrap();
                    let node_letter = node_str.chars().next().unwrap();
                    let node_value = (node_letter as i32) - ('A' as i32) + 1;
                    worker.time_remaining = step_base_time + node_value;

                    // Add any new steps now available
                    for neighbor in instructions.neighbors_directed(next_step, Outgoing) {
                        if !complete.contains(&neighbor) && !available.contains(&neighbor) {
                            available.push(neighbor);
                        }
                    }
                } else {
                    // No next steps available, don't bother trying to hand
                    // out further tasks
                    break;
                }
            }
        }

        timer += 1;
    }

    #[cfg(debug_assertions)] {
        println!("Steps: {}", steps);
    }

    return timer;
}

fn get_next_step(instructions: &Graph<String, String, Directed, u32>, available: &mut Vec<NodeIndex<u32>>, complete: &HashSet<NodeIndex<u32>>) -> Option<NodeIndex<u32>> {
    // Work out the current step. The current step must available
    // and it must have had all it's prerequisites completed so
    // that it can be worked on
    let mut next_suitable_step_index = None;
    for (step_index, step) in available.iter().enumerate() {
        let mut satisfied = true;
        for step_prereq in instructions.neighbors_directed(*step, Incoming) {
            if !complete.contains(&step_prereq) {
                satisfied = false;
                break;
            }
        }

        if satisfied {
            next_suitable_step_index = Some(step_index);
            break;
        }
    }

    match next_suitable_step_index {
        Some(next_suitable_step_index) => Some(available.remove(next_suitable_step_index)),
        None => None,
    }
}

#[derive(Clone)]
struct Worker {
    node: Option<NodeIndex<u32>>,
    time_remaining: i32,
}

fn create_instruction_graph(input: &str) -> Graph<String, String, Directed, u32> {
    let raw_instructions = parse_raw_instructions(input);
    let mut instructions = Graph::<String, String>::new();
    let mut node_map = HashMap::new();
    for raw_instruction in &raw_instructions {
        let (step, depends_on) = raw_instruction;
        if !node_map.contains_key(step) {
            let node = instructions.add_node(step.clone());
            node_map.insert(step, node);
        }
        if !node_map.contains_key(depends_on) {
            let node = instructions.add_node(depends_on.clone());
            node_map.insert(depends_on, node);
        }
    }
    let mut edges = Vec::new();
    for raw_instruction in &raw_instructions {
        let (step, depends_on) = raw_instruction;
        let depends_on_node = node_map[depends_on];
        let step_node = node_map[step];
        edges.push((depends_on_node, step_node));
    }
    instructions.extend_with_edges(&edges);

    #[cfg(debug_assertions)] {
        println!("=====[DOT START]=====\n{:?}\n======[DOT END]======", Dot::with_config(&instructions, &[Config::EdgeNoLabel]));
    }

    return instructions;
}

fn parse_raw_instruction(instruction: &str) -> (String, String) {
    let re = Regex::new(r"Step (?P<previous>.).*step (?P<step>.)").unwrap();
    let captures = re.captures(instruction).unwrap();
    return (captures["step"].to_string(), captures["previous"].to_string());
}

fn parse_raw_instructions(instruction_text: &str) -> Vec<(String, String)> {
    let mut instructions = Vec::new();
    for instruction in instruction_text.lines() {
        let instruction = instruction.trim();
        if instruction.len() == 0 {
            continue;
        }
        instructions.push(parse_raw_instruction(instruction));
    }
    return instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw_instruction() {
        let input = "Step C must be finished before step A can begin.";
        let (step, previous) = parse_raw_instruction(input);
        assert_eq!("A", step);
        assert_eq!("C", previous);
    }

    #[test]
    fn test_parse_raw_instructions() {
        let input = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.\n";
        let result = parse_raw_instructions(input);

        let expected = vec![
            ("A".to_string(), "C".to_string()),
            ("F".to_string(), "C".to_string()),
            ("B".to_string(), "A".to_string()),
            ("D".to_string(), "A".to_string()),
            ("E".to_string(), "B".to_string()),
            ("E".to_string(), "D".to_string()),
            ("E".to_string(), "F".to_string()),
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_example() {
        let input = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.\n";
        let result = process(input, 0, 2);
        assert_eq!(15, result);
    }
}
