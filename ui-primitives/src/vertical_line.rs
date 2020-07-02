use ui::{Point, AtomicAction, VisualObject};

#[derive(Debug, Clone)]
pub struct VerticalLine {
    pub top: Point,
    pub length: i32
}

impl VisualObject for VerticalLine {
    fn to_actions(&self) -> Vec<AtomicAction> {
        let mut result = vec![];

        result.push(AtomicAction::MoveAt { point: self.top.clone() });

        for _i in 0..self.length {
            result.push(AtomicAction::Print { char: '┃' });
            result.push(AtomicAction::MoveLeft { n: 1 });
            result.push(AtomicAction::MoveDown { n: 1 });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints() {
        assert_prints!(
            (5, 5),
            VerticalLine { top: Point { line: 1, col: 1 }, length: 3 },
            vec![
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', '┃', ' ', ' ', ' ', '\n',
                ' ', '┃', ' ', ' ', ' ', '\n',
                ' ', '┃', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
            ]
        );
    }
}
