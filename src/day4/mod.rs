pub fn part1(input: &str) -> String {
    let mut sum = 0;
    for (winning_numbers_str, my_numbers_str) in input.lines().map(|line| {
        // leave behind (winning_numbers, my_numbers)
        let temp = line.split(':').collect::<Vec<&str>>()[1]
            .split('|')
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        (temp[0], temp[1])
    }) {
        let winning_numbers = winning_numbers_str
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        println!("{:?}", winning_numbers);
        println!("{:?}", my_numbers_str);

        let my_numbers = my_numbers_str
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut count = 0;

        for my_number in my_numbers {
            if winning_numbers.contains(&my_number) {
                if count == 0 {
                    count += 1;
                } else {
                    count *= 2;
                }
            }
        }
        sum += count;
    }
    sum.to_string()
}
pub fn part2(input: &str) -> String {
    // make vec filled with 1s of the length of the input lines
    let mut copies = vec![1; input.lines().count()];

    for (i, (winning_numbers_str, my_numbers_str)) in input
        .lines()
        .map(|line| {
            // leave behind (winning_numbers, my_numbers)
            let temp = line.split(':').collect::<Vec<&str>>()[1]
                .split('|')
                .map(|x| x.trim())
                .collect::<Vec<&str>>();
            (temp[0], temp[1])
        })
        .enumerate()
    {
        let current_copies = copies[i];
        let winning_numbers = winning_numbers_str
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let my_numbers = my_numbers_str
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut count = 0;

        for my_number in my_numbers {
            if winning_numbers.contains(&my_number) {
                count += 1;
            }
        }

        for y in i + 1..=i + count {
            if let Some(x) = copies.get_mut(y) {
                *x += current_copies;
            }
        }

        println!("{:?}", copies);
    }

    copies.iter().sum::<u32>().to_string()
}
