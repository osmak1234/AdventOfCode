pub fn part1(input: &str) -> String {
    let mut sum_of_id = 0;

    input.lines().enumerate().for_each(|(i_plus_1, line)| {
        let split = line.split(": ").collect::<Vec<&str>>()[1];
        let split = split.split(';').collect::<Vec<&str>>();

        let mut possible = true;

        for set in split.iter() {
            let cubes = set.split(", ").collect::<Vec<&str>>();

            for cube in cubes.iter() {
                let parts = cube.split_whitespace().collect::<Vec<&str>>();
                let count = parts[0].parse::<i32>().unwrap();
                let color = parts[1];

                match color {
                    "red" if count > 12 => possible = false,
                    "green" if count > 13 => possible = false,
                    "blue" if count > 14 => possible = false,
                    _ => {}
                }
            }
        }

        if possible {
            // `i_plus_1` needs to be adjusted for the proper ID
            sum_of_id += i_plus_1 as i32 + 1;
        }
    });

    sum_of_id.to_string()
}

pub fn part2(input: &str) -> String {
    let mut sum_of_power = 0;

    input.lines().for_each(|line| {
        let split = line.split(": ").collect::<Vec<&str>>()[1];
        let split = split.split(';').collect::<Vec<&str>>();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for set in split.iter() {
            let cubes = set.split(", ").collect::<Vec<&str>>();

            for cube in cubes.iter() {
                let parts = cube.split_whitespace().collect::<Vec<&str>>();
                let count = parts[0].parse::<i32>().unwrap();
                let color = parts[1];

                match color {
                    "red" if count > min_red => min_red = count,
                    "green" if count > min_green => min_green = count,
                    "blue" if count > min_blue => min_blue = count,
                    _ => {}
                }
            }
        }

        sum_of_power += min_red * min_green * min_blue;
    });

    sum_of_power.to_string()
}
