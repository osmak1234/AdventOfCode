pub fn part1(input: &str) -> String {
    // Time:      7  15   30
    // Distance:  9  40  200

    let mut sum = 0;

    let mut times = Vec::new();
    let mut best_distances = Vec::new();
    let mut lines = input.lines();

    for num in lines.next().unwrap().split(':').collect::<Vec<&str>>()[1].split_whitespace() {
        times.push(num.parse::<u32>().unwrap());
    }

    for num in lines.next().unwrap().split(':').collect::<Vec<&str>>()[1].split_whitespace() {
        best_distances.push(num.parse::<u32>().unwrap());
    }

    for (race_no, race_time) in times.iter().enumerate() {
        let mut possible_times = 0;
        for i in 0..*race_time {
            if (race_time - i) * i > best_distances[race_no] {
                possible_times += 1;
            }
        }

        if sum == 0 {
            sum += possible_times;
        } else {
            sum *= possible_times;
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();

    let time = lines.next().unwrap().split(':').collect::<Vec<&str>>()[1]
        .trim()
        .replace(' ', "")
        .parse::<u32>()
        .unwrap();

    println!("{}", time);
    let best_distance = lines.next().unwrap().split(':').collect::<Vec<&str>>()[1]
        .trim()
        .replace(' ', "")
        .parse::<u128>()
        .unwrap();

    let mut possible_times = 0;
    for i in 0..time {
        if ((time - i) as u128 * i as u128) > best_distance {
            possible_times += 1;
        }
    }

    possible_times.to_string()
}
