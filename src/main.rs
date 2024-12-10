mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage: <program> <day number (1-25)> <input file path>");
        return;
    }

    let day = match args[1].parse::<u8>() {
        Ok(n) if (1..=25).contains(&n) => n,
        _ => {
            println!("Day must be a number between 1 and 25");
            return;
        }
    };

    let input_file_path = &args[2];

    match day {
        1 => days::day01::solve(input_file_path),
        2 => days::day02::solve(input_file_path),
        3 => days::day03::solve(input_file_path),
        4 => days::day04::solve(input_file_path),
        5 => days::day05::solve(input_file_path),
        _ => unreachable!(),
    }
}
