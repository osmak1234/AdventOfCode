use std::collections::HashMap;

pub fn part1() {
    //read file
    let input = std::fs::read_to_string("./src/day1/input_first.txt")
        .expect("Something went wrong reading the file");

    let mut sum = 0;

    input.lines().for_each(|ln| {
        println!("{}", ln);
        let mut nums: Vec<i32> = Vec::new();
        ln.chars().for_each(|c| {
            if c.is_numeric() {
                nums.push(c.to_digit(10).unwrap() as i32);
            }
        });

        if nums.len() == 1 {
            sum += nums[0] * 11;
        } else {
            sum += nums[0] * 10 + nums.last().unwrap();
        }
    });

    println!("sum: {}", sum);
}

pub fn part2() {
    let input = std::fs::read_to_string("./src/day1/input_first.txt")
        .expect("Something went wrong reading the file");

    let mut sum = 0;

    let mut log = String::new();
    let numbers: HashMap<Vec<char>, i32> = [
        (vec!['o', 'n', 'e'], 1),
        (vec!['t', 'w', 'o'], 2),
        (vec!['t', 'h', 'r', 'e', 'e'], 3),
        (vec!['f', 'o', 'u', 'r'], 4),
        (vec!['f', 'i', 'v', 'e'], 5),
        (vec!['s', 'i', 'x'], 6),
        (vec!['s', 'e', 'v', 'e', 'n'], 7),
        (vec!['e', 'i', 'g', 'h', 't'], 8),
        (vec!['n', 'i', 'n', 'e'], 9),
    ]
    .iter()
    .cloned()
    .collect();

    input.lines().for_each(|ln| {
        println!("{}", ln);
        log.push_str(ln);
        let mut chars: Vec<char> = Vec::new();

        let mut nums: Vec<i32> = Vec::new();

        ln.chars().for_each(|c| {
            if c.is_numeric() {
                nums.push(c.to_digit(10).unwrap() as i32);
                chars.clear();
            } else {
                chars.push(c);

                for i in (0..chars.len()).rev() {
                    if numbers.contains_key(&chars[i..]) {
                        nums.push(*numbers.get(&chars[i..]).unwrap());
                        if chars.last() == Some(&'e') {
                            chars.clear();
                            chars.push('e');
                        } else if chars.last() == Some(&'n') {
                            chars.clear();
                            chars.push('n');
                        } else if chars.last() == Some(&'t') {
                            chars.clear();
                            chars.push('t');
                        } else {
                            chars.clear();
                        }

                        break;
                    }
                }
            }
        });

        if nums.len() == 1 {
            sum += nums[0] * 11;
            println!("{} * 11 = {}", nums[0], nums[0] * 11);
            log.push_str(&format!("{} * 11 = {}\n", nums[0], nums[0] * 11));
        } else {
            sum += nums[0] * 10 + nums.last().unwrap();
            println!(
                "{} + {} = {}",
                nums[0],
                nums.last().unwrap(),
                nums[0] * 10 + nums.last().unwrap()
            );
            log.push_str(&format!(
                "{} + {} = {}\n",
                nums[0],
                nums.last().unwrap(),
                nums[0] * 10 + nums.last().unwrap()
            ));
        }
    });

    println!("sum: {}", sum);
    log.push_str(&format!("sum: {}", sum));

    // write result to file
    std::fs::write("./src/day1/output_second.txt", log).expect("Unable to write file");
}
