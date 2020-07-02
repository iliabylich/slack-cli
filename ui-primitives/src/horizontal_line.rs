use ui_abstract::{Point, AtomicAction, VisualObject};

#[derive(Debug, Clone)]
pub struct HorizontalLine {
    pub left: Point,
    pub length: i32
}

impl VisualObject for HorizontalLine {
    fn to_actions(&self) -> Vec<AtomicAction> {
        let mut result = vec![];

        result.push(AtomicAction::MoveAt { point: self.left.clone() });

        for _i in 0..self.length {
            result.push(AtomicAction::Print { char: '━' })
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
            HorizontalLine { left: Point { line: 2, col: 1 }, length: 3 },
            vec![
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', '━', '━', '━', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
            ]
        );
    }
}
