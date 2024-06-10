pub fn solve(input: &str) -> usize {
    let triangles: Vec<Triangle> = input.lines().map(|line| line.trim().into()).collect();
    triangles.into_iter().filter(|x| x.is_possible()).count()
}

#[derive(Debug)]
struct Triangle {
    a: u16,
    b: u16,
    c: u16,
}

impl From<&str> for Triangle {
    fn from(value: &str) -> Self {
        let mut values = value.split_whitespace();
        Triangle {
            a: values.next().unwrap().parse().unwrap(),
            b: values.next().unwrap().parse().unwrap(),
            c: values.next().unwrap().parse().unwrap(),
        }
    }
}

impl Triangle {
    fn is_possible(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.c + self.b > self.a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "5 10 25";
        assert_eq!(solve(input), 0)
    }
    #[test]
    fn expected_impossible_triangle() {
        let input = "5 10 25";
        let triangle = Triangle::from(input);
        assert!(!&triangle.is_possible());
    }
}
