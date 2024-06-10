#[derive(Debug, Eq, Hash, PartialEq)]
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
    let patterns = input.lines().map(|line| {
        line.trim()
            .chars()
            .map(|c| c.into())
            .collect::<Vec<Direction>>()
    });
    let mut start = '5';
    let mut output = Vec::new();
    for pattern in patterns {
        start = apply_pattern(start, pattern);
        output.push(start);
    }
    output.into_iter().map(|x| x.to_string()).collect()
}

fn apply_pattern(start: char, pattern: Vec<Direction>) -> char {
    let mut start = start;
    for dir in pattern {
        start = step_number(start, dir);
    }
    start
}

fn step_number(start: char, dir: Direction) -> char {
    // why u no global const hashmap, rust?
    // or hashmap literal ?!
    let step_result_1: HashMap<Direction, char> = HashMap::from([(Direction::Down, '3')]);
    let step_result_2: HashMap<Direction, char> =
        HashMap::from([(Direction::Down, '6'), (Direction::Right, '3')]);
    let step_result_3: HashMap<Direction, char> = HashMap::from([
        (Direction::Down, '7'),
        (Direction::Right, '4'),
        (Direction::Up, '1'),
        (Direction::Left, '2'),
    ]);
    let step_result_4: HashMap<Direction, char> = HashMap::from([
        (Direction::Down, '8'),
        (Direction::Left, '3'),
    ]);
    let step_result_5: HashMap<Direction, char> = HashMap::from([
        (Direction::Right, '6'),
    ]);
    let step_result_6: HashMap<Direction, char> = HashMap::from([
        (Direction::Down, 'A'),
        (Direction::Right, '7'),
        (Direction::Up, '2'),
        (Direction::Left, '5'),
    ]);
    let step_result_7: HashMap<Direction, char> = HashMap::from([
        (Direction::Down, 'B'),
        (Direction::Right, '8'),
        (Direction::Up, '3'),
        (Direction::Left, '6'),
    ]);
    let step_result_8: HashMap<Direction, char> = HashMap::from([
        (Direction::Down, 'C'),
        (Direction::Right, '9'),
        (Direction::Up, '4'),
        (Direction::Left, '7'),
    ]);
    let step_result_9: HashMap<Direction, char> = HashMap::from([
        (Direction::Left, '8'),
    ]);
    let step_result_a: HashMap<Direction, char> = HashMap::from([
        (Direction::Right, 'B'),
        (Direction::Up, '6'),
    ]);
    let step_result_b: HashMap<Direction, char> = HashMap::from([
        (Direction::Down, 'D'),
        (Direction::Right, 'C'),
        (Direction::Up, '7'),
        (Direction::Left, 'A'),
    ]);
    let step_result_c: HashMap<Direction, char> = HashMap::from([
        (Direction::Up, '8'),
        (Direction::Left, 'B'),
    ]);
    let step_result_d: HashMap<Direction, char> = HashMap::from([
        (Direction::Up, 'B'),
    ]);
    let step_results = HashMap::from([
        ('1',step_result_1),
        ('2',step_result_2),
        ('3',step_result_3),
        ('4',step_result_4),
        ('5',step_result_5),
        ('6',step_result_6),
        ('7',step_result_7),
        ('8',step_result_8),
        ('9',step_result_9),
        ('A',step_result_a),
        ('B',step_result_b),
        ('C',step_result_c),
        ('D',step_result_d),
    ]);
    *step_results.get(&start).unwrap().get(&dir).unwrap_or(&start)
}

use std::collections::HashMap;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "ULL
            RRDDD
            LURDL
            UUUUD";
        assert_eq!(day02(input), "5DB3");
    }
}
