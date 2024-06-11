use std::collections::HashMap;

use regex::Regex;

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.into())
        .filter(|room: &Room| room.is_real())
        .map(|room| room.id)
        .sum()
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Room {
    encrypted_name: String,
    id: usize,
    checksum: Vec<char>,
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"([a-z-]+)(\d+)\[([a-z]+)\]").unwrap();
        let captures = re.captures(value).unwrap();
        Room {
            encrypted_name: captures[1].replace('-', ""),
            id: captures[2].parse().unwrap(),
            checksum: captures[3].chars().collect(),
        }
    }
}

impl Room {
    fn is_real(&self) -> bool {
        let chars = count_chars(&self.encrypted_name);
        let mut chars_count: Vec<(char, usize)> = chars.into_iter().collect();
        chars_count.sort_unstable_by(|x, y| x.1.cmp(&y.1).then_with(|| y.0.cmp(&x.0)));
        let expected_checksum: Vec<char> =
            chars_count.into_iter().rev().take(5).map(|x| x.0).collect();
        expected_checksum == self.checksum
    }
}

fn count_chars(encrypted_name: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in encrypted_name.chars() {
        let count = counts.get(&c).unwrap_or(&0);
        counts.insert(c, count + 1);
    }
    counts
}

#[cfg(test)]
mod test {
    use crate::part1::{count_chars, solve};

    use super::Room;

    #[test]
    fn happy_path_char_counting() {
        let counts = count_chars("abc");
        assert_eq!(counts.get(&'a').unwrap(), &1_usize);
        assert_eq!(counts.get(&'b').unwrap(), &1_usize);
        assert_eq!(counts.get(&'c').unwrap(), &1_usize);
    }

    #[test]
    fn room_parsing_1() {
        let test_room = Room::from("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(test_room.encrypted_name, "aaaaabbbzyx");
        assert_eq!(test_room.id, 123_usize);
        assert_eq!(test_room.checksum, vec!['a', 'b', 'x', 'y', 'z']);
    }
    #[test]
    fn room_parsing_2() {
        let test_room = Room::from("a-b-c-d-e-f-g-h-987[abcde]");
        assert_eq!(test_room.encrypted_name, "abcdefgh");
        assert_eq!(test_room.id, 987_usize);
        assert_eq!(test_room.checksum, vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn room_parsing_3() {
        let test_room = Room::from("not-a-real-room-404[oarel]");
        assert_eq!(test_room.encrypted_name, "notarealroom");
        assert_eq!(test_room.id, 404);
        assert_eq!(test_room.checksum, vec!['o', 'a', 'r', 'e', 'l']);
    }
    #[test]
    fn is_real_testing() {
        let test_room = Room::from("aaaaa-bbb-z-y-x-123[abxyz]");
        let test_room2 = Room::from("a-b-c-d-e-f-g-h-987[abcde]");
        let test_room3 = Room::from("not-a-real-room-404[oarel]");
        let test_room4 = Room::from("totally-real-room-200[decoy]");
        assert!(test_room.is_real());
        assert!(test_room2.is_real());
        assert!(test_room3.is_real());
        assert!(!test_room4.is_real());
    }
    #[test]
    fn example() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]
                a-b-c-d-e-f-g-h-987[abcde]
                not-a-real-room-404[oarel]
                totally-real-room-200[decoy]";
        assert_eq!(solve(input), 1514);
    }
    #[test]
    fn example_without_parsing() {
        let test_room = Room::from("aaaaa-bbb-z-y-x-123[abxyz]");
        let test_room2 = Room::from("a-b-c-d-e-f-g-h-987[abcde]");
        let test_room3 = Room::from("not-a-real-room-404[oarel]");
        let test_room4 = Room::from("totally-real-room-200[decoy]");
        let value: usize = [test_room, test_room2, test_room3, test_room4]
            .into_iter()
            .filter(|room: &Room| room.is_real())
            .map(|room| room.id)
            .sum();
        assert_eq!(value, 1514);
    }
}
