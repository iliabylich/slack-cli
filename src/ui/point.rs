#[derive(Debug, Clone)]
pub struct Point {
    pub line: i32,
    pub col: i32
}

#[allow(dead_code)]
impl Point {
    pub fn left(&self, n: i32) -> Self {
        Point { line: self.line, col: self.col - n }
    }

    pub fn right(&self, n: i32) -> Self {
        Point { line: self.line, col: self.col + n }
    }

    pub fn up(&self, n: i32) -> Self {
        Point { line: self.line - n, col: self.col }
    }

    pub fn down(&self, n: i32) -> Self {
        Point { line: self.line + n, col: self.col }
    }
}
