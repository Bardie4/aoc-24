use std::str::FromStr;

use regex::Regex;

fn read_data(path: &str) -> Vec<Vec<char>> {
    let contents = std::fs::read_to_string(path).unwrap();
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_silver(data: &Vec<Vec<char>>) -> u32 {
    let mut total_sum = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let target = ['X', 'M', 'A', 'S'];

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'X' {
                for &(dx, dy) in &directions {
                    let mut found = true;
                    for k in 0..target.len() {
                        let ni = i as isize + k as isize * dx;
                        let nj = j as isize + k as isize * dy;
                        if ni < 0
                            || nj < 0
                            || ni >= data.len() as isize
                            || nj >= data[i].len() as isize
                        {
                            found = false;
                            break;
                        }
                        if data[ni as usize][nj as usize] != target[k] {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        total_sum += 1;
                    }
                }
            }
        }
    }

    total_sum
}

fn calculate_gold(data: &Vec<Vec<char>>) -> i32 {
    let mut total_sum = 0;
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'A' {
                for &(dx, dy) in &directions {
                    let ni1 = i as isize + dx;
                    let nj1 = j as isize + dy;
                    let ni2 = i as isize - dx;
                    let nj2 = j as isize - dy;

                    if ni1 >= 0
                        && nj1 >= 0
                        && ni2 >= 0
                        && nj2 >= 0
                        && ni1 < data.len() as isize
                        && nj1 < data[i].len() as isize
                        && ni2 < data.len() as isize
                        && nj2 < data[i].len() as isize
                    {
                        let first = data[ni1 as usize][nj1 as usize];
                        let second = data[ni2 as usize][nj2 as usize];
                        if (first == 'M' && second == 'S') || (first == 'S' && second == 'M') {
                            total_sum += 1;
                        }

                        let third 
                    }
                }
            }
        }
    }

    total_sum
}

pub fn solve(path: &str) {
    let data = read_data(path);
    println!("input: {:?}", data);

    let silver_sum = calculate_silver(&data);
    println!("Silver sum: {}", silver_sum);

    let gold_sum = calculate_gold(&data);
    println!("Gold sum: {}", gold_sum);
}
