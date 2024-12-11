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

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'A' {
                // Try to grab the 4 corners
                let ul = (i as isize + 1, j as isize + 1);
                let ur = (i as isize + 1, j as isize - 1);
                let dl = (i as isize - 1, j as isize + 1);
                let dr = (i as isize - 1, j as isize - 1);

                if (ul.0 >= 0 && ul.1 >= 0)
                    && (ur.0 >= 0 && ur.1 >= 0)
                    && (dr.0 >= 0 && dr.1 >= 0)
                    && (dl.0 >= 0 && dl.1 >= 0)
                    && (ul.0 < data.len() as isize && ul.1 < data[i].len() as isize)
                    && (dr.0 < data.len() as isize && dr.1 < data[i].len() as isize)
                    && (ur.0 < data.len() as isize && ur.1 < data[i].len() as isize)
                    && (dl.0 < data.len() as isize && dl.1 < data[i].len() as isize)
                {
                    // Construct diagonal strings, diagonal1 and diagonal2
                    let diagonal1 = data[ul.0 as usize][ul.1 as usize].to_string()
                        + &data[i][j].to_string()
                        + &data[dr.0 as usize][dr.1 as usize].to_string();
                    let diagonal2 = data[dl.0 as usize][dl.1 as usize].to_string()
                        + &data[i][j].to_string()
                        + &data[ur.0 as usize][ur.1 as usize].to_string();

                    if (diagonal1 == "MAS" || diagonal1 == "SAM")
                        && (diagonal2 == "MAS" || diagonal2 == "SAM")
                    {
                        total_sum += 1;
                    }
                }
            }
        }
    }

    total_sum
}

pub fn solve(path: &str) {
    let data = read_data(path);

    let silver_sum = calculate_silver(&data);
    println!("Silver sum: {}", silver_sum);

    let gold_sum = calculate_gold(&data);
    println!("Gold sum: {}", gold_sum);
}
