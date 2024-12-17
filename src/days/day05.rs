use std::collections::HashMap;
use std::collections::HashSet;

fn read_data(path: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut test_sequences: Vec<Vec<i32>> = Vec::new();

    let parts: Vec<&str> = contents.split("\n\n").collect();

    // Parse the graph rules
    for line in parts[0].lines() {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<i32> = line.split('|').map(|n| n.trim().parse().unwrap()).collect();

        graph.entry(nums[0]).or_default().push(nums[1]);
        graph.entry(nums[1]).or_default();
    }

    // Parse the tests
    for line in parts[1].lines() {
        if line.is_empty() {
            continue;
        }
        let sequence: Vec<i32> = line.split(',').map(|n| n.trim().parse().unwrap()).collect();
        test_sequences.push(sequence);
    }

    (graph, test_sequences)
}

fn is_valid_sequence(graph: &HashMap<i32, Vec<i32>>, sequence: &Vec<i32>) -> bool {
    for i in 0..sequence.len() {
        let current = sequence[i];
        if let Some(must_come_after) = graph.get(&current) {
            for &after in must_come_after {
                for &prev in &sequence[0..i] {
                    if prev == after {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn calculate_silver(graph: &HashMap<i32, Vec<i32>>, sequences: &Vec<Vec<i32>>) -> i32 {
    let mut middle_numbers = Vec::new();

    for sequence in sequences {
        if is_valid_sequence(graph, sequence) {
            let middle_number_index = sequence.len() / 2;
            middle_numbers.push(sequence[middle_number_index]);
        }
    }

    middle_numbers.iter().sum::<i32>()
}

fn calculate_gold_sequence(graph: &HashMap<i32, Vec<i32>>, sequence: &[i32]) -> Vec<i32> {
    // Create a map of node -> number of nodes that must come before it (in-degree)
    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    let mut nodes: HashSet<i32> = HashSet::new();

    // Initialize in-degree for all nodes
    for &num in sequence {
        in_degree.insert(num, 0);
        nodes.insert(num);
    }

    // Calculate in-degree for each node
    for (&from, to_nodes) in graph {
        if nodes.contains(&from) {
            for &to in to_nodes {
                if nodes.contains(&to) {
                    *in_degree.entry(to).or_default() += 1;
                }
            }
        }
    }

    // Use a priority queue for nodes with 0 in-degree
    let mut queue: Vec<i32> = in_degree
        .iter()
        .filter(|(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut result = Vec::new();

    // Process nodes in topological order
    while let Some(current) = queue.pop() {
        result.push(current);

        // Decrease in-degree for all neighbors
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if nodes.contains(&neighbor) {
                    let degree = in_degree.get_mut(&neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }

    result
}

fn calculate_gold(graph: &HashMap<i32, Vec<i32>>, sequences: &[Vec<i32>]) -> i32 {
    let mut sum = 0;

    for sequence in sequences {
        if !is_valid_sequence(graph, sequence) {
            // Get corrected sequence
            let new_sequence = calculate_gold_sequence(graph, sequence);

            // Get middle number from corrected sequence
            if !new_sequence.is_empty() {
                let middle_idx = new_sequence.len() / 2;
                sum += new_sequence[middle_idx];
            }
        }
    }

    sum
}

pub fn solve(path: &str) {
    let (graph, sequences) = read_data(path);
    println!("Graph: {:?}", graph);
    println!("Sequences to test: {:?}", sequences);

    let silver_sum = calculate_silver(&graph, &sequences);
    println!("Silver sum: {}", silver_sum);

    let gold_sum = calculate_gold(&graph, &sequences);
    println!("Gold sum: {}", gold_sum);
}
