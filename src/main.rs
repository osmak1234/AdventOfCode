pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn main() {
    let input = std::fs::read_to_string("./src/day9/input.txt")
        .expect("Something went wrong reading the file");
    let output = day9::part2(&input);

    println!("{}", output);
}
