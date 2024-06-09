use std::collections::HashSet;
#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: i32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
struct Error;

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        let c = c.trim();
        let mut c_iter = c.chars();
        let direction: Direction = c_iter.next().unwrap().try_into().unwrap();
        let count = c_iter.collect::<String>().parse().unwrap();
        Ok(Self { direction, count })
    }
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(Error),
        }
    }
}

pub fn day01(input: &str) -> i32 {
    let instructions: Vec<Instruction> = parse_instructions(input);
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut count_x: i32 = 0;
    let mut count_y: i32 = 0;
    let dir_value = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut current_direction = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Left => {
                // +4 to avoid negative values as -x % 4 = -x
                current_direction = ((current_direction as i8 - 1 + 4) % 4) as usize;
            }
            Direction::Right => {
                current_direction = ((current_direction as i8 + 1) % 4) as usize;
            }
        }
        let (x, y) = dir_value[current_direction];
        for _ in 1..=instruction.count {
            count_x += x;
            count_y += y;
            if !visited_positions.insert((count_x, count_y)) {
                return count_x.abs() + count_y.abs();
            }
        }
    }
    -1
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .map(|x| x.trim())
        .map(|y| y.try_into().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "R8, R4, R4, R8";
        assert_eq!(day01(input), 4);
    }
}
