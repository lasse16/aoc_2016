pub fn solve(input: &str) -> String {
    input
        .lines()
        .filter(|x| supports_tls(x))
        .count()
        .to_string()
}

fn is_palindrom(input: &[char]) -> bool {
    let mut index_a = 0;
    let mut index_b = 3;

    if input[0] == input[1] {
        return false;
    }

    while index_a < index_b {
        if input[index_a] != input[index_b] {
            return false;
        }
        index_a += 1;
        index_b -= 1;
    }
    true
}

fn supports_tls(value: &str) -> bool {
    // Does not work since there might be multiple parts in brackets
    let string_parts: Vec<&str> = value.split(['[', ']']).collect();

    let mut not_in_brackets = Vec::new();
    for (i, part) in string_parts.into_iter().enumerate() {
        if i % 2 == 0 {
            not_in_brackets.push(part)
        } else if contains_true_palindrom(part).0 {
            return false;
        }
    }

    not_in_brackets
        .into_iter()
        .any(|x| contains_true_palindrom(x).0)
}

fn contains_true_palindrom(part: &str) -> (bool, usize) {
    let chars: Vec<char> = part.chars().collect();
    let mut index = 0;
    let count = chars.len();
    while index < count - 3 {
        if is_palindrom(&chars[index..index + 4]) {
            return (true, index);
        }
        index += 1;
    }
    (false, 0)
}

fn visualize(part: &str) -> String {
    let (palindrom, index) = contains_true_palindrom(part);
    if palindrom {
        let mut part = part.to_string();
        part.insert_str(index + 4, "\x1b[0m");
        part.insert_str(index, "\x1b[93m");
        return part;
    }
    part.to_string()
}

#[cfg(test)]
mod test {
    use crate::part1::{is_palindrom, supports_tls, visualize};

    #[test]
    fn is_palindrom_anna() {
        let chars: Vec<char> = "anna".chars().collect();
        assert!(is_palindrom(&chars));
    }
    #[test]
    fn test_visualize() {
        println!("{}", visualize("anna"));
        assert_eq!("\x1b[93manna\x1b[0m", visualize("anna"));
        // assert!(false);
    }
    #[test]
    fn example() {
        let input = "abba[mnop]qrst";
        assert!(supports_tls(input))
    }
    #[test]
    fn example_false() {
        let input = "abcd[bddb]xyyx";
        assert!(!supports_tls(input))
    }
    #[test]
    fn example_2() {
        let input = "aaaa[qwer]tyui";
        assert!(!supports_tls(input))
    }
    #[test]
    fn example_3() {
        let input = "ioxxoj[asdfgh]zxcvbn";
        assert!(supports_tls(input))
    }
}
