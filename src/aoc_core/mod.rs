use std::fmt::{Display, self};

pub struct Solution {
    pub part_1: i32,
    pub part_2: i32
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solution part 1: {}\nSolution part 2: {}", self.part_1, self.part_2)
    }
}
