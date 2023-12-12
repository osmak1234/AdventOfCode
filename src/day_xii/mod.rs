fn parse_input(input: &str) -> Vec<(Vec<Tile>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let springs = get_tiles(parts.next().unwrap());

            let groups = parts
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            (springs, groups)
        })
        .collect::<Vec<(Vec<Tile>, Vec<u32>)>>()
}

enum Tile {
    Spring,
    Unknown,
    None,
}

fn get_tiles(map: &str) -> Vec<Tile> {
    map.chars()
        .map(|c| match c {
            '.' => Tile::None,
            '?' => Tile::Unknown,
            _ => Tile::Spring,
        })
        .collect::<Vec<Tile>>()
}

pub fn part1(input: &str) -> String {
    // (spring map, connected springs)
    let data = parse_input(input);

    for row in data {
        let map = row.0;
        let spring_groups = row.1;
    }

    "".to_owned()
}
