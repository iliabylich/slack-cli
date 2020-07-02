use crate::{Point, AtomicAction, VisualObject, HorizontalLine, VerticalLine};

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point
}


impl VisualObject for Rectangle {
    fn to_actions(&self) -> Vec<AtomicAction> {
        let top_left = &self.top_left;
        let bottom_right = &self.bottom_right;
        let top_right = Point { line: top_left.line, col: bottom_right.col };
        let bottom_left = Point { line: bottom_right.line, col: top_left.col };

        let width = top_right.col - top_left.col - 1;
        let height = bottom_left.line - top_left.line - 1;

        let mut top_line = HorizontalLine { left: top_left.right(1), length: width }.to_actions();
        let mut bot_line = HorizontalLine { left: bottom_left.right(1), length: width }.to_actions();
        let mut left_line = VerticalLine { top: top_left.down(1), length: height }.to_actions();
        let mut right_line = VerticalLine { top: top_right.down(1), length: height }.to_actions();

        let mut result = vec![];

        result.push(AtomicAction::MoveAt { point: top_left.clone() });
        result.push(AtomicAction::Print { char: '┏' });

        result.append(&mut top_line);

        result.push(AtomicAction::MoveAt { point: top_right.clone() });
        result.push(AtomicAction::Print { char: '┓' });

        result.append(&mut bot_line);

        result.push(AtomicAction::MoveAt { point: bottom_right.clone() });
        result.push(AtomicAction::Print { char: '┛' });

        result.append(&mut right_line);

        result.push(AtomicAction::MoveAt { point: bottom_left.clone() });
        result.push(AtomicAction::Print { char: '┗' });

        result.append(&mut left_line);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    use crate::{Point, screen_helper::assert_prints};

    #[test]
    fn it_prints() {
        assert_prints(
            (5, 5),
            Box::new(Rectangle { top_left: Point { line: 0, col: 0 }, bottom_right: Point { line: 2, col: 3 } }),
            vec![
                '┏', '━', '━', '┓', ' ', '\n',
                '┃', ' ', ' ', '┃', ' ', '\n',
                '┗', '━', '━', '┛', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
            ]
        );
    }
}
