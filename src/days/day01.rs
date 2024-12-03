pub fn solve(path: &str) {
    let (left, right) = read_data(path);

    let silver_sum = calculate_silver(&left, &right);
    println!("Silver Sum: {}", silver_sum);

    let gold_sum = calculate_gold(&left, &right);
    println!("Gold Sum: {}", gold_sum);
}

fn read_data(path: &str) -> (Vec<u32>, Vec<u32>) {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        left.push(parts[0].parse::<u32>().unwrap());
        right.push(parts[1].parse::<u32>().unwrap());
    }

    (left, right)
}

fn calculate_silver(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();

    left_sorted.sort();
    right_sorted.sort();

    let mut sum = 0;
    for i in 0..left_sorted.len() {
        sum += if left_sorted[i] > right_sorted[i] {
            left_sorted[i] - right_sorted[i]
        } else {
            right_sorted[i] - left_sorted[i]
        };
    }

    sum
}

fn calculate_gold(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut gold_sum = 0;
    for i in 0..left.len() {
        gold_sum += left[i] * right.iter().filter(|&x| *x == left[i]).count() as u32;
    }

    gold_sum
}
