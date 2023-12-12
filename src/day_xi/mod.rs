fn rows_without_a_star(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.chars())
        .enumerate()
        .filter(|(_, line)| line.clone().filter(|c| *c == '#').count() == 0)
        .map(|(i, _)| i as u32)
        .collect()
}

fn cols_without_a_star(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.chars())
        .enumerate()
        .map(|(_, line)| line.enumerate())
        .fold(
            vec![0; input.lines().next().unwrap().len()],
            |mut acc, line| {
                line.for_each(|(j, c)| {
                    if c == '#' {
                        acc[j] += 1;
                    }
                });
                acc
            },
        )
        .iter()
        .enumerate()
        .filter(|(_, count)| **count == 0)
        .map(|(i, _)| i as u32)
        .collect()
}

pub fn part1(input: &str) -> String {
    let mut stars: Vec<(u32, u32)> = Vec::new();

    let mut initial_width = input.lines().next().unwrap().len() as u32;
    let mut initial_height = input.lines().count() as u32;

    assert_eq!(initial_width, initial_height);

    input
        .lines()
        .map(|line| line.chars())
        .enumerate()
        .for_each(|(i, line)| {
            line.enumerate().for_each(|(j, c)| {
                if c == '#' {
                    stars.push((i as u32, j as u32));
                }
            });
        });

    let rows_without_a_star = rows_without_a_star(input);
    let cols_without_a_star = cols_without_a_star(input);

    // reconstruct the map
    let mut map = String::new();

    for i in 0..initial_height {
        for j in 0..initial_width {
            if stars.contains(&(i, j)) {
                map.push('#');
            } else {
                map.push('.');
            }
        }
        map.push('\n');
    }

    println!("{:?}", rows_without_a_star);
    println!("{:?}", cols_without_a_star);

    assert_eq!(input, map);

    // now we need to account for the expansion of the universe

    map
}
