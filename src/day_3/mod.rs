#[allow(clippy::nonminimal_bool)]

pub fn part1(input: &str) -> String {
    println!("Day 3, Part 1:\n {}", input);

    let lines = input.lines();

    // x y coordinates of all the symbols
    let mut symbols: Vec<(i32, i32)> = Vec::new();

    // (start, end, y) beginning and end of the number + y line number
    let mut numbers: Vec<(i32, i32, i32)> = Vec::new();

    for (y, line) in lines.enumerate() {
        let mut numbers_temp: (i32, i32, i32) = (0, 0, y as i32);
        let mut number_started: bool = false;

        // find the numbers, there can be multiple numbers in a line and they can be of any length
        for (x, symbol) in line.chars().enumerate() {
            match symbol {
                '0'..='9' => {
                    if !number_started {
                        numbers_temp.0 = x as i32;
                        number_started = true;
                    }
                }
                _ => {
                    if number_started {
                        numbers_temp.1 = x as i32 - 1;
                        number_started = false;
                        numbers.push(numbers_temp);
                    }
                }
            }

            // if the number is at the end of the line
            if number_started && x == line.len() - 1 {
                numbers_temp.1 = x as i32;
                number_started = false;
                numbers.push(numbers_temp);
            }
        }

        // find the symbols
        for (x, symbol) in line.chars().enumerate() {
            if !symbol.is_numeric() {
                match symbol {
                    '.' => {}
                    _ => symbols.push((x as i32, y as i32)),
                }
            }
        }
    }

    // all the numbers adjecent to a symbol should be summed up
    let mut sum: i32 = 0;

    'outer: for number in numbers {
        for symbol in &symbols {
            // if the number is above, below, left or right of the symbol or diagonal add it to the sum
            if (number.0 == symbol.0 && number.2 == symbol.1 - 1)
                || (number.0 == symbol.0 && number.2 == symbol.1 + 1)
                || (number.0 == symbol.0 - 1 && number.2 == symbol.1)
                || (number.0 == symbol.0 + 1 && number.2 == symbol.1)
                || (number.0 == symbol.0 - 1 && number.2 == symbol.1 - 1)
                || (number.0 == symbol.0 + 1 && number.2 == symbol.1 + 1)
                || (number.0 == symbol.0 - 1 && number.2 == symbol.1 + 1)
                || (number.0 == symbol.0 + 1 && number.2 == symbol.1 - 1)
                || (number.1 == symbol.0 && number.2 == symbol.1 - 1)
                || (number.1 == symbol.0 && number.2 == symbol.1 + 1)
                || (number.1 == symbol.0 - 1 && number.2 == symbol.1)
                || (number.1 == symbol.0 + 1 && number.2 == symbol.1)
                || (number.1 == symbol.0 - 1 && number.2 == symbol.1 - 1)
                || (number.1 == symbol.0 + 1 && number.2 == symbol.1 + 1)
                || (number.1 == symbol.0 - 1 && number.2 == symbol.1 + 1)
                || (number.1 == symbol.0 + 1 && number.2 == symbol.1 - 1)
            {
                let mut lines = input.lines();

                println!(
                    "{}",
                    lines
                        .nth(number.2 as usize)
                        .unwrap()
                        .chars()
                        .skip(number.0 as usize)
                        .take(number.1 as usize - number.0 as usize + 1)
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap()
                );

                let mut lines = input.lines();

                sum += lines
                    .nth(number.2 as usize)
                    .unwrap()
                    .chars()
                    .skip(number.0 as usize)
                    .take(number.1 as usize - number.0 as usize + 1)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                continue 'outer;
            }
        }
    }

    sum.to_string()
}

#[allow(clippy::nonminimal_bool)]
pub fn part2(input: &str) -> String {
    println!("Day 3, Part 2:\n {}", input);

    let lines = input.lines();

    // x y coordinates of all the symbols
    let mut gears: Vec<(i32, i32)> = Vec::new();

    // (start, end, y) beginning and end of the number + y line number
    let mut numbers: Vec<(i32, i32, i32)> = Vec::new();

    for (y, line) in lines.enumerate() {
        let mut numbers_temp: (i32, i32, i32) = (0, 0, y as i32);
        let mut number_started: bool = false;

        // find the numbers, there can be multiple numbers in a line and they can be of any length
        for (x, symbol) in line.chars().enumerate() {
            match symbol {
                '0'..='9' => {
                    if !number_started {
                        numbers_temp.0 = x as i32;
                        number_started = true;
                    }
                }
                _ => {
                    if number_started {
                        numbers_temp.1 = x as i32 - 1;
                        number_started = false;
                        numbers.push(numbers_temp);
                    }
                }
            }

            // if the number is at the end of the line
            if number_started && x == line.len() - 1 {
                numbers_temp.1 = x as i32;
                number_started = false;
                numbers.push(numbers_temp);
            }
        }

        // find the symbols
        for (x, symbol) in line.chars().enumerate() {
            if !symbol.is_numeric() {
                if let '*' = symbol {
                    gears.push((x as i32, y as i32))
                }
            }
        }
    }

    // all the numbers adjecent to a symbol should be summed up
    let mut sum = 0;

    for gear in &gears {
        let mut adjecent_numbers: Vec<i32> = Vec::new();

        for number in &numbers {
            // if the number is above, below, left or right of the symbol or diagonal add it to the sum
            if (number.0 == gear.0 && number.2 == gear.1 - 1)
                || (number.0 == gear.0 && number.2 == gear.1 + 1)
                || (number.0 == gear.0 - 1 && number.2 == gear.1)
                || (number.0 == gear.0 + 1 && number.2 == gear.1)
                || (number.0 == gear.0 - 1 && number.2 == gear.1 - 1)
                || (number.0 == gear.0 + 1 && number.2 == gear.1 + 1)
                || (number.0 == gear.0 - 1 && number.2 == gear.1 + 1)
                || (number.0 == gear.0 + 1 && number.2 == gear.1 - 1)
                || (number.1 == gear.0 && number.2 == gear.1 - 1)
                || (number.1 == gear.0 && number.2 == gear.1 + 1)
                || (number.1 == gear.0 - 1 && number.2 == gear.1)
                || (number.1 == gear.0 + 1 && number.2 == gear.1)
                || (number.1 == gear.0 - 1 && number.2 == gear.1 - 1)
                || (number.1 == gear.0 + 1 && number.2 == gear.1 + 1)
                || (number.1 == gear.0 - 1 && number.2 == gear.1 + 1)
                || (number.1 == gear.0 + 1 && number.2 == gear.1 - 1)
            {
                let mut lines = input.lines();

                adjecent_numbers.push(
                    lines
                        .nth(number.2 as usize)
                        .unwrap()
                        .chars()
                        .skip(number.0 as usize)
                        .take(number.1 as usize - number.0 as usize + 1)
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                );
            }
        }

        if adjecent_numbers.len() == 2 {
            sum += adjecent_numbers[0] * adjecent_numbers[1];
        }
    }

    sum.to_string()
}
