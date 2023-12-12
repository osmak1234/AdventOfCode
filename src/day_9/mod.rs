fn next_generation(generation: Vec<i32>) -> Vec<i32> {
    generation
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

pub fn part1(input: &str) -> String {
    let generations = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sum = 0;

    for generation in &generations {
        let mut evolution = Vec::<Vec<i32>>::new();

        evolution.push(generation.clone());

        while evolution.last().unwrap().iter().any(|&x| x != 0) {
            evolution.push(next_generation(evolution.last().unwrap().clone()));
        }

        let mut prediction = 0;

        for evolution_generation in evolution.iter().rev() {
            prediction += evolution_generation.last().unwrap();
        }

        sum += prediction;
    }

    println!("{:?}", generations);

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let generations = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                // only added this one line :D
                .rev()
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sum = 0;

    for generation in &generations {
        let mut evolution = Vec::<Vec<i32>>::new();

        evolution.push(generation.clone());

        while evolution.last().unwrap().iter().any(|&x| x != 0) {
            evolution.push(next_generation(evolution.last().unwrap().clone()));
        }

        let mut prediction = 0;

        for evolution_generation in evolution.iter().rev() {
            prediction += evolution_generation.last().unwrap();
        }

        sum += prediction;
    }

    println!("{:?}", generations);

    sum.to_string()
}
