pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_x;
pub mod day_xi;
pub mod day_xii;

fn main() {
    let input = std::fs::read_to_string("./src/day_xii/test_input.txt")
        .expect("Something went wrong reading the file");
    let output = day_xii::part1(&input);

    println!("{}", output);
}
