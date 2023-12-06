pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

fn main() {
    let input = std::fs::read_to_string("./src/day4/input.txt")
        .expect("Something went wrong reading the file");

    let output = day4::part2(&input);

    println!("{}", output);
}
