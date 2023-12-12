use core::panic;
use std::fmt::format;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeType {
    HorizontalStraight,
    VerticalStraight,
    TopToRight,
    TopToLeft,
    LeftToBotom,
    RightToBottom,
    Start,
    None,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl PipeType {
    pub fn possible_travel_out_directions(&self) -> Vec<Direction> {
        match self {
            PipeType::HorizontalStraight => vec![Direction::Left, Direction::Right],
            PipeType::VerticalStraight => vec![Direction::Up, Direction::Down],
            PipeType::TopToRight => vec![Direction::Up, Direction::Right],
            PipeType::TopToLeft => vec![Direction::Up, Direction::Left],
            PipeType::LeftToBotom => vec![Direction::Left, Direction::Down],
            PipeType::RightToBottom => vec![Direction::Right, Direction::Down],
            PipeType::Start => vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
            PipeType::None => vec![],
        }
    }
    pub fn possible_travel_in_directions(&self) -> Vec<Direction> {
        match self {
            PipeType::HorizontalStraight => vec![Direction::Left, Direction::Right],
            PipeType::VerticalStraight => vec![Direction::Up, Direction::Down],
            PipeType::TopToRight => vec![Direction::Down, Direction::Left],
            PipeType::TopToLeft => vec![Direction::Down, Direction::Right],
            PipeType::LeftToBotom => vec![Direction::Up, Direction::Right],
            PipeType::RightToBottom => vec![Direction::Up, Direction::Left],
            PipeType::Start => vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
            PipeType::None => vec![],
        }
    }

    pub fn from(ch: char) -> PipeType {
        match ch {
            '|' => PipeType::VerticalStraight,
            '-' => PipeType::HorizontalStraight,
            'L' => PipeType::TopToRight,
            'J' => PipeType::TopToLeft,
            '7' => PipeType::LeftToBotom,
            'F' => PipeType::RightToBottom,
            '.' => PipeType::None,
            'S' => PipeType::Start,
            _ => panic!("Invalid input while converting to enum"),
        }
    }
    pub fn to_display(&self) -> char {
        match self {
            PipeType::HorizontalStraight => '─',
            PipeType::VerticalStraight => '│',
            PipeType::TopToRight => '└',
            PipeType::TopToLeft => '┘',
            PipeType::LeftToBotom => '┐',
            PipeType::RightToBottom => '┌',
            PipeType::None => ' ',
            PipeType::Start => 'S',
        }
    }
}

fn display(enum_input: Vec<Vec<PipeType>>) -> String {
    let mut string = String::new();
    enum_input
        .iter()
        .map(|row| row.iter().map(PipeType::to_display).collect::<String>())
        .for_each(|row| string.push_str(&format!("{}\n", row)));

    string
}

fn color_wisited(enum_input: Vec<Vec<PipeType>>, visited: Vec<(usize, usize)>) -> String {
    let mut string = String::new();
    enum_input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, cell)| {
                    if visited.contains(&(x, y)) {
                        if visited.len() / 2 == visited.iter().position(|&p| p == (x, y)).unwrap() {
                            return "\x1b[92mX\x1b[0m".to_string();
                        }
                        format!("\x1b[93m{}\x1b[0m", cell.to_display())
                    } else {
                        cell.to_display().to_string()
                    }
                })
                .collect::<String>()
        })
        .for_each(|row| string.push_str(&format!("{}\n", row)));

    string
}

fn try_travel(
    direction: Direction,
    current_pipe: PipeType,
    potential_pipe: PipeType,
) -> Option<Direction> {
    if current_pipe
        .possible_travel_out_directions()
        .contains(&direction)
        && potential_pipe
            .possible_travel_in_directions()
            .contains(&direction)
    {
        match (direction, potential_pipe) {
            (Direction::Up, PipeType::VerticalStraight) => Some(Direction::Up),
            (Direction::Up, PipeType::LeftToBotom) => Some(Direction::Left),
            (Direction::Up, PipeType::RightToBottom) => Some(Direction::Right),
            (Direction::Up, PipeType::Start) => Some(Direction::Up),
            (Direction::Down, PipeType::VerticalStraight) => Some(Direction::Down),
            (Direction::Down, PipeType::TopToRight) => Some(Direction::Right),
            (Direction::Down, PipeType::TopToLeft) => Some(Direction::Left),
            (Direction::Down, PipeType::Start) => Some(Direction::Down),
            (Direction::Right, PipeType::HorizontalStraight) => Some(Direction::Right),
            (Direction::Right, PipeType::TopToLeft) => Some(Direction::Up),
            (Direction::Right, PipeType::LeftToBotom) => Some(Direction::Down),
            (Direction::Right, PipeType::Start) => Some(Direction::Right),
            (Direction::Left, PipeType::HorizontalStraight) => Some(Direction::Left),
            (Direction::Left, PipeType::TopToRight) => Some(Direction::Up),
            (Direction::Left, PipeType::RightToBottom) => Some(Direction::Down),
            (Direction::Left, PipeType::Start) => Some(Direction::Left),
            _ => None,
        }
    } else {
        None
    }
}

pub fn part1(input: &str) -> String {
    let mut map = Vec::<Vec<PipeType>>::new();

    input.lines().for_each(|ln| {
        map.push(ln.chars().map(PipeType::from).collect());
    });

    // Find the start location.
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &cell)| {
                if cell == PipeType::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .expect("Start position not found");

    let (mut x, mut y) = start;
    let mut current_pipe = PipeType::Start;
    let mut current_direction = Direction::Left; // Assuming we start moving to the right
    let mut visited: Vec<(usize, usize)> = vec![(x, y)];

    loop {
        let (new_x, new_y) = match current_direction {
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
            Direction::Up => (x, y - 1),
        };

        let next_pipe = map[new_y][new_x];
        if next_pipe == PipeType::None || visited.contains(&(new_x, new_y)) {
            if next_pipe == PipeType::Start {
                println!("Returned to start");
            } else {
                println!("Dead end at ({}, {})", new_x, new_y);
                break; // We've either hit a dead end or revisited a pipe
            }
        }

        // Determine the new travel direction
        if let Some(new_direction) = try_travel(current_direction, current_pipe, next_pipe) {
            current_direction = new_direction;
        } else {
            println!(
                "Invalid travel direction from ({}, {}) to ({}, {})",
                x, y, new_x, new_y
            );
            break; // No valid travel direction
        }

        // Move to the next pipe
        x = new_x;
        y = new_y;
        current_pipe = next_pipe;
        visited.push((x, y));

        // Check if we've returned to the start
        if current_pipe == PipeType::Start && (x, y) == start {
            println!("Returned to start");
            break;
        }
    }

    println!("{}", color_wisited(map.clone(), visited.clone()));

    (visited.len() / 2).to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            // remove last newline
            part1(
                r"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            "  ┌┐ \n ┌┘│ \nS┘ └┐\n│┌──┘\n└┘   \n"
        );
    }

    #[test]
    fn test_pipe_travel() {
        assert_eq!(
            try_travel(Direction::Up, PipeType::HorizontalStraight, PipeType::Start),
            None
        );
        assert_eq!(
            try_travel(
                Direction::Up,
                PipeType::HorizontalStraight,
                PipeType::TopToRight
            ),
            None
        );
        assert_eq!(
            try_travel(
                Direction::Up,
                PipeType::VerticalStraight,
                PipeType::VerticalStraight
            ),
            None
        );
        assert_eq!(
            try_travel(Direction::Up, PipeType::HorizontalStraight, PipeType::None),
            None
        );
        assert_eq!(
            try_travel(Direction::Right, PipeType::Start, PipeType::RightToBottom),
            None
        );
        assert_eq!(
            try_travel(Direction::Right, PipeType::Start, PipeType::LeftToBotom),
            Some(Direction::Down)
        );
    }
}
