#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub line: i32,
    pub col: i32
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left() {
        let point = Point { line: 10, col: 20 };
        assert_eq!(point.left(2), Point { line: 10, col: 18 });
    }

    #[test]
    fn test_right() {
        let point = Point { line: 10, col: 20 };
        assert_eq!(point.right(2), Point { line: 10, col: 22 });
    }

    #[test]
    fn test_up() {
        let point = Point { line: 10, col: 20 };
        assert_eq!(point.up(2), Point { line: 8, col: 20 });
    }

    #[test]
    fn test_down() {
        let point = Point { line: 10, col: 20 };
        assert_eq!(point.down(2), Point { line: 12, col: 20 });
    }
}
