fn read_data(path: &str) -> Vec<Vec<char>> {
    let contents = std::fs::read_to_string(path).unwrap();
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_silver(data: &Vec<Vec<char>>) -> u32 {
    let total_sum = 0;

    total_sum
}

fn calculate_gold(data: &Vec<Vec<char>>) -> i32 {
    let total_sum = 0;

    total_sum
}

pub fn solve(path: &str) {
    let data = read_data(path);
    println!("input: {:?}", data);

    let silver_sum = calculate_silver(&data);
    println!("Silver sum: {}", silver_sum);
}
