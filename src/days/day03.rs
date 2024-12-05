use std::str::FromStr;

use regex::Regex;

pub fn solve(path: &str) {
    let data = read_data(path);

    println!("input: {:?}", data);

    let silver_sum = calculate_silver(&data);
    println!("Silver sum: {}", silver_sum);

    let gold_sum = calculate_gold(&data);
    println!("Gold sum: {}", gold_sum);
}

fn read_data(path: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(path).unwrap();
    contents.lines().map(|line| line.to_string()).collect()
}

fn calculate_silver(data: &Vec<String>) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total_sum = 0;

    for line in data {
        for cap in re.captures_iter(line) {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();
            total_sum += x * y;
        }
    }

    total_sum
}

fn calculate_gold(data: &Vec<String>) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total_sum = 0;

    // Use an iterator to split the data into segments
    let joined_data = data.join("\n");
    let mut segments = joined_data.split("don't()");

    if let Some(first_segment) = segments.next() {
        let mut current_segment = first_segment;

        while !current_segment.is_empty() {
            for cap in re.captures_iter(current_segment) {
                let x: u32 = cap[1].parse().unwrap();
                let y: u32 = cap[2].parse().unwrap();
                total_sum += x * y;
            }

            current_segment = segments.next().unwrap_or("");
        }
    }

    total_sum
}
