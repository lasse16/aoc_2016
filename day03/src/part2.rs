pub fn solve(input: &str) -> usize {
    let triangles: Vec<Triangle> = parse_input(input);
    triangles.into_iter().filter(|x| x.is_possible()).count()
}

fn parse_input(input: &str) -> Vec<Triangle> {
    let columns = input.lines().map(|line| line.split_whitespace());
    let column_1: Vec<u16> = columns
        .clone()
        .map(|mut cols| cols.next().unwrap().parse().unwrap())
        .collect();
    let column_2: Vec<u16> = columns
        .clone()
        .map(|mut cols| cols.nth(1).unwrap().parse().unwrap())
        .collect();
    let column_3: Vec<u16> = columns
        .clone()
        .map(|mut cols| cols.nth(2).unwrap().parse().unwrap())
        .collect();
    let mut triangles = Vec::new();
    for col in [column_1, column_2, column_3] {
        let mut iter = col.into_iter();
        while let Some(a) = iter.next() {
            triangles.push(Triangle{
                a,
                b: iter.next().unwrap(),
                c: iter.next().unwrap()
            });
        }
    }
    triangles
}

#[derive(Debug)]
struct Triangle {
    a: u16,
    b: u16,
    c: u16,
}

impl Triangle {
    fn is_possible(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.c + self.b > self.a
    }
}
