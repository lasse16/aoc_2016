use ndarray;

const ROW_COUNT: usize = 6;
const COLUMN_COUNT: usize = 50;

#[derive(Debug)]
enum Instruction {
    Rectangle(RectangleInstruction),
    Rotate(RotateInstruction),
}

#[derive(Debug)]
struct RectangleInstruction {
    a: usize,
    b: usize,
}

impl RectangleInstruction {
    fn carry_out(&self, mut light_matrix: ndarray::Array2<bool>) {
        for i in 0..self.b {
            for j in 0..self.a {
                light_matrix[[i, j]] = true;
            }
        }
    }
}

#[derive(Debug)]
struct RotateInstruction {
    direction: Direction,
    a: usize,
    b: usize,
}

impl RotateInstruction {
    fn carry_out(&self, mut light_matrix: ndarray::Array2<bool>) {
        match self.direction {
            Direction::Row => light_matrix
                .row_mut(self.a)
                .as_slice_mut()
                .unwrap()
                .rotate_right(self.b),
            Direction::Column => light_matrix
                .column_mut(self.a)
                .as_slice_mut()
                .unwrap()
                .rotate_right(self.b),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Row,
    Column,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        let value = value.trim();
        match value {
            "row" => Direction::Row,
            "column" => Direction::Column,
            _ => panic!(),
        }
    }
}

impl From<&str> for RotateInstruction {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl From<&str> for RectangleInstruction {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl Instruction {
    fn carry_out(self, light_matrix: ndarray::Array2<bool>) {
        match self {
            Instruction::Rectangle(i) => i.carry_out(light_matrix),
            Instruction::Rotate(i) => i.carry_out(light_matrix),
        }
    }
}

pub(crate) fn solve(input: &str) -> String {
    let mut lights_rows_cols = ndarray::Array2::<bool>::from_elem((COLUMN_COUNT, ROW_COUNT), false);
    let instructions: Vec<Instruction> = input.lines().map(|line| line.into()).collect();
    for instruction in instructions {
        instruction.carry_out(lights_rows_cols);
    }
    let mut count = lights_rows_cols.iter().filter(|x| **x).count();
    count.to_string()
}

#[cfg(test)]
mod test {
    use crate::part1::RectangleInstruction;

    #[test]
    fn rectangle_instruction() {
        let instruction = RectangleInstruction { a: 3, b: 2 };
        let mut test_array = ndarray::Array2::<bool>::from_elem((4, 3), false);
        instruction.carry_out(test_array);
        print!("{}", test_array);
        todo!();
    }
}
