use std::str::FromStr;

use regex::Regex;

pub fn solve(path: &str) {
    let data = read_data(path);

    println!("input: {:?}", data);

    let silver_sum = calculate_silver(&data);
    println!("Silver sum: {}", silver_sum);

    let gold_sum = calculate_gold(&data.join("\n"));
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

fn calculate_gold(data: &String) -> i32 {
    // Append a do() to the start and don't() to the end of the data
    let line = format!("do(){}don't()", data);

    // Match mul() segments enclosed by do() and don't()
    let re = Regex::new(r"do\([\s\S]*?mul\((\d+),(\d+)\)[\s\S]*?don't\(\)").unwrap();

    let total_sum: i32 = re
        .captures_iter(&line)
        .flat_map(|cap| {
            let segment = cap[0].to_owned(); // Avoid borrowing the captured segment
            Regex::new(r"mul\((\d+),(\d+)\)")
                .unwrap()
                .captures_iter(&segment)
                .map(|inner_cap| {
                    let x: i32 = inner_cap[1].parse().unwrap();
                    let y: i32 = inner_cap[2].parse().unwrap();
                    x * y
                })
                .collect::<Vec<_>>() // Avoid borrowing
        })
        .sum();

    total_sum
}
