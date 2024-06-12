use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    let mut columns: Vec<Vec<char>> = Vec::new();
    for _ in 0..lines.clone().first().unwrap().len() {
        columns.push(Vec::new())
    }
    for line in lines {
        for (index, c) in line.chars().enumerate() {
            columns.get_mut(index).unwrap().push(c);
        }
    }
    let mut res = Vec::new();
    for column in columns {
        let letter = *count_chars(column).iter().min_by_key(|c| c.1).unwrap().0;
        res.push(letter);
    }
    res.iter().collect()
}

fn count_chars(encrypted_name: Vec<char>) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in encrypted_name {
        let count = counts.get(&c).unwrap_or(&0);
        counts.insert(c, count + 1);
    }
    counts
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn example() {
        let input = "eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar";
        assert_eq!(solve(input), "advent")
    }
}
