// Saving this cause it might be helpful in the future
struct Plane(Vec<Vec<char>>);
impl Plane {
    fn rows(&self) -> usize {
        self.0.len()
    }
    fn cols(&self) -> usize {
        self.0[0].len()
    }
    fn rotated_clockwise(&self, degrees: &u32) -> Self {
        let degrees = degrees % &360;
        match degrees {
            0 => self.clone(),
            90 => Self((0..self.cols())
                    .map(|col| {
                        (0..self.rows()).rev()
                        .map(|row| self.0[row][col])
                        .map(|c| rotated_clockwise(&c, &degrees))
                        .collect()
                    })
                    .collect()),
            180 => Self(((0..self.rows()).rev())
                    .map(|row| {
                        ((0..self.cols()).rev())
                        .map(|col| self.0[row][col])
                        .collect()
                    })
                    .collect()),
            270 => Self((0..self.cols()).rev()
                    .map(|col| {
                        (0..self.rows())
                        .map(|row| self.0[row][col])
                        .map(|c| rotated_clockwise(&c, &degrees))
                        .collect()
                    })
                    .collect()),
            _ => panic!("degrees must be divisible by 90"),
        }
    }
}

