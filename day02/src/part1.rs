#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unexpected char {}", value),
        }
    }
}

pub fn day02(input: &str) -> String {
    let patterns = input
        .lines()
        .map(|line| line.trim().chars().map(|c| c.into()).collect::<Vec<Direction>>());
    let mut start = 5;
    let mut output = Vec::new();
    for pattern in patterns {
        start = apply_pattern(start, pattern);
        output.push(start);
    }
    output.into_iter().map(|x| x.to_string()).collect()
}

fn apply_pattern(start: i8, pattern: Vec<Direction>) -> i8 {
    let mut start = start;
    for dir in pattern {
        start = step_number(start, dir);
    }
    start
}

fn step_number(start: i8, dir: Direction) -> i8 {
    let mut value: i8 = start;
    match dir {
        Direction::Up => {
            value -= 3;
            if value < 1 {
                value = start;
            }
        }
        Direction::Down => {
            value += 3;
            if value > 9 {
                value = start;
            }
        }
        Direction::Left => {
            value -= 1;
            if [1, 4, 7].contains(&start) {
                value = start;
            }
        }
        Direction::Right => {
            value += 1;
            if [3, 6, 9].contains(&start) {
                value = start;
            }
        }
    };
    value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "ULL
            RRDDD
            LURDL
            UUUUD";
        assert_eq!(day02(input), "1985");
    }
}
