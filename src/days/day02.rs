// Example input
// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9

pub fn solve(path: &str) {
    let data = read_data(path);

    let silver_sum = data.iter().filter(|row| calculate_silver(row)).count();
    println!("Silver sum: {}", silver_sum);

    let gold_sum = data.iter().filter(|row| calculate_gold(row)).count();
    println!("Gold sum: {}", gold_sum);
}

fn calculate_gold(row: &Vec<u32>) -> bool {
    // Check if the row is valid
    // The row is valid if it is increasing or decreasing
    // An increasing row is valid if the difference between any two adjacent numbers is between +/- 1 and 3
    // A decreasing row is valid if the difference between any two adjacent numbers is between +/- 1 and 3
    // if removing a single value from the row makes it valid, then the row is valid

    for i in 0..row.len() {
        let mut new_row = row.clone();
        new_row.remove(i);
        if calculate_silver(&new_row) {
            return true;
        }
    }

    false
}

fn calculate_silver(row: &Vec<u32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..row.len() {
        let diff = row[i] as i32 - row[i - 1] as i32;

        // Check if difference is within +/- 1 to 3
        if diff < 1 || diff > 3 {
            increasing = false;
        }
        if diff > -1 || diff < -3 {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn read_data(path: &str) -> Vec<Vec<u32>> {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in contents.lines() {
        let row: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        grid.push(row);
    }

    grid
}
