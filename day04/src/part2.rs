use std::collections::HashMap;

use regex::Regex;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn solve(input: &str) -> usize {
    let real_rooms: Vec<Room> = input
        .lines()
        .map(|line| line.into())
        .filter(|room: &Room| room.is_real())
        .collect();
    for room in real_rooms {
        if room.decrypt() == "northpole object storage " {
            return room.id;
        }
    }
    0
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
            encrypted_name: captures[1].to_string(),
            id: captures[2].parse().unwrap(),
            checksum: captures[3].chars().collect(),
        }
    }
}

impl Room {
    fn is_real(&self) -> bool {
        let chars = count_chars(&self.encrypted_name.replace('-', ""));
        let mut chars_count: Vec<(char, usize)> = chars.into_iter().collect();
        chars_count.sort_unstable_by(|x, y| x.1.cmp(&y.1).then_with(|| y.0.cmp(&x.0)));
        let expected_checksum: Vec<char> =
            chars_count.into_iter().rev().take(5).map(|x| x.0).collect();
        expected_checksum == self.checksum
    }

    fn decrypt(&self) -> String {
        decrypt(&self.encrypted_name, self.id)
    }
}

fn decrypt(encrypted_value: &str, shift: usize) -> String {
    encrypted_value
        .chars()
        .map(|c| {
            if c == '-' {
                return ' ';
            }
            shift_char(c, shift)
        })
        .collect()
}

fn count_chars(encrypted_name: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in encrypted_name.chars() {
        let count = counts.get(&c).unwrap_or(&0);
        counts.insert(c, count + 1);
    }
    counts
}

fn shift_char(c: char, shift: usize) -> char {
    let shift = shift % 26;
    let idx = ALPHABET.chars().position(|ch| ch == c).unwrap();

    let shifted_idx = (idx + shift) % 26;

    ALPHABET.chars().nth(shifted_idx).unwrap()
}

#[cfg(test)]
mod test {
    use crate::part2::decrypt;

    #[test]
    fn example_decrypt() {
        let input = "qzmt-zixmtkozy-ivhz";
        assert_eq!(decrypt(input, 5), "very encrypted name")
    }
}
