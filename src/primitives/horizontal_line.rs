use crate::primitives::Point;
use crate::ui::AtomicAction;
use crate::primitives::VisualObject;

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
    use super::HorizontalLine;
    use crate::{primitives::Point, ui::screen_helper::assert_prints};

    #[test]
    fn it_prints() {
        assert_prints(
            (5, 5),
            Box::new(HorizontalLine { left: Point { line: 2, col: 1 }, length: 3 }),
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