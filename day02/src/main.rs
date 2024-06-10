mod part1;
mod part2;

fn main() {
    let file = include_str!("input.txt");
    println!("{}", part1::day02(file));
    println!("{}", part2::day02(file));
}
