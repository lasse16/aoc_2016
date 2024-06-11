mod part1;
mod part2;

fn main() {
    let file = include_str!("input.txt");
    println!("{}", part1::solve(file));
    println!("{}", part2::solve(file));
}
