pub mod day1;
pub mod day2;
fn main() {
    let input = std::fs::read_to_string("./src/day2/input.txt")
        .expect("Something went wrong reading the file");

    let output = day2::part2(&input);
    println!("{}", output);
}
