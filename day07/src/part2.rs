pub fn solve(input: &str) -> String {
    input
        .lines()
        .filter(|x| supports_ssl(x))
        .count()
        .to_string()
}

fn is_aba(input: &[char]) -> bool {
    input[0] == input[2] && input[1] != input[0]
}

fn supports_ssl(value: &str) -> bool {
    let string_parts: Vec<&str> = value.split(['[', ']']).collect();

    let (not_in_brackets, in_brackets): (Vec<(usize, &str)>, Vec<(usize, &str)>) = string_parts
        .into_iter()
        .enumerate()
        .partition(|(i, _part)| i % 2 == 0);

    let in_brackets = in_brackets.into_iter().map(|x| x.1);
    let not_in_brackets = not_in_brackets.into_iter().map(|x| x.1);

    let babs = not_in_brackets.flat_map(get_abas).map(invert_aba);

    for str in in_brackets {
        for bab in babs.clone() {
            if str.contains(&bab) {
                return true;
            }
        }
    }

    false
}

fn get_abas(part: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = part.chars().collect();
    let mut index = 0;
    let count = chars.len();
    while index < count - 2 {
        let input = &chars[index..index + 3];
        if is_aba(input) {
            result.push(input.iter().collect());
        }
        index += 1;
    }
    result
}

fn invert_aba(aba: String) -> String {
    let mut chars = aba.chars();
    let a = chars.next().unwrap();
    let b = chars.next().unwrap();
    format!("{}{}{}", b, a, b)
}

#[cfg(test)]
mod test {
    use crate::part2::supports_ssl;

    #[test]
    fn example() {
        let input = "aba[bab]xyz";
        assert!(supports_ssl(input))
    }
    #[test]
    fn example_false() {
        let input = "xyx[xyx]xyx";
        assert!(!supports_ssl(input))
    }
    #[test]
    fn example_2() {
        let input = "aaa[kek]eke";
        assert!(supports_ssl(input))
    }
    #[test]
    fn example_3() {
        let input = "zazbz[bzb]cdb";
        assert!(supports_ssl(input))
    }
}
