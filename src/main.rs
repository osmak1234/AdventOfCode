pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

fn main() {
    //  let input = std::fs::read_to_string("./src/day7/input.txt")
    //      .expect("Something went wrong reading the file");
    //
    //  let output = day7::part1(&input);

    let input = std::fs::read_to_string("./src/day7/input.txt")
        .expect("Something went wrong reading the file");
    let output = day7::day07_p1_from_reddit(&input);

    println!("{}", output);
}
