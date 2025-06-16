#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Location {
    pub col: usize,
    pub line: usize,
}

impl Default for Location {
    fn default() -> Self {
        Location { col: 0, line: 0 }
    }
}

impl Location {
    pub fn new(line: usize, col: usize) -> Self {
        Location { col, line }
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    pub fn advance_col(&mut self, by: usize) {
        self.col += by
    }
}
