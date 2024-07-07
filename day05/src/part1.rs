pub fn solve(input: &str) -> String {
    let mut index: u64 = 0;
    let mut result = Vec::new();
    for _ in 0..8 {
        let mut digest = md5::compute(format!("{}{}",input,index));
        while!digest.starts_with(b"00000"){
            index += 1;
            digest = md5::compute(format!("{}{}",input,index));
        }
        println!("{}",index);
        result.push(format!("{:x}",digest).chars().nth(6).unwrap());
    }
    result.into_iter().map(|x| x.to_string()).collect()
}

#[cfg(test)]
mod test {
    use crate::part1;

    #[test]
    fn example() {
        let input = "abc";
        assert_eq!(part1::solve(input),"18f47a30".to_string());
    }
}
