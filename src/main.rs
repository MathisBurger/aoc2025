mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod generic;

fn main() {
    let input = generic::read_input();

    let mut day_string = String::new();
    std::io::stdin()
        .read_line(&mut day_string)
        .ok()
        .expect("Failed to read line");

    let day: u8 = day_string.trim().parse().expect("Invalid day given");

    match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        7 => day07::solve(input),
        8 => day08::solve(input),
        9 => day09::solve(input),
        10 => day10::solve(input),
        11 => day11::solve(input),
        12 => day12::solve(input),
        _ => println!("Day not found"),
    }
}
