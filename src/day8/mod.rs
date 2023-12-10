use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();

    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| if ch == 'L' { 0 } else { 1 })
        .collect::<Vec<u32>>();

    let mut directions = directions.iter().cycle();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines.skip(1) {
        let (label, (left, right)) = parse_node_line(line);
        nodes.insert(label, (left, right));
    }

    let mut steps = 0;

    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        steps += 1;
        let (left, right) = nodes.get(current_node).unwrap();
        let direction = directions.next().unwrap();

        if *direction == 0 {
            current_node = left;
        } else {
            current_node = right;
        }
    }

    steps.to_string()
}

fn parse_node_line(line: &str) -> (&str, (&str, &str)) {
    let label = line.split('=').next().unwrap().trim();
    let left = line
        .split('=')
        .nth(1)
        .unwrap()
        .split(',')
        .next()
        .unwrap()
        .trim();
    let right = line
        .split('=')
        .nth(1)
        .unwrap()
        .split(',')
        .nth(1)
        .unwrap()
        .trim();
    // removes the last character, which is a ')', not needed
    let right = &right[..right.len() - 1];
    // removes the first character, which is a '(', not needed
    let left = &left[1..];

    (label, (left, right))
}
pub fn part2(input: &str) -> String {
    let mut lines = input.lines();

    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| if ch == 'L' { 0 } else { 1 })
        .collect::<Vec<u32>>();

    let mut directions = directions.iter().cycle();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines.skip(1) {
        let (label, (left, right)) = parse_node_line(line);
        nodes.insert(label, (left, right));
    }

    let mut current_nodes = Vec::new();

    for (label, _) in nodes.iter() {
        if label.ends_with('A') {
            current_nodes.push(label);
        }
    }

    let mut path_lengths = Vec::new();

    while let Some(starting_node) = current_nodes.pop() {
        let mut current_node = starting_node;

        let mut steps = 0;

        while !current_node.ends_with('Z') {
            steps += 1;
            let (left, right) = nodes.get(current_node).unwrap();
            let direction = directions.next().unwrap();

            if *direction == 0 {
                current_node = left;
            } else {
                current_node = right;
            }
        }

        path_lengths.push(steps);
    }

    // find the lcm of all the path lengths

    let mut lcm_ = path_lengths[0];

    for length in path_lengths.iter().skip(1) {
        lcm_ = lcm(lcm_, *length);
    }

    lcm_.to_string()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}
