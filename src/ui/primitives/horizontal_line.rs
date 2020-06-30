use crate::ui::point::Point;
use crate::ui::action::AtomicAction;
use crate::ui::visual::VisualObject;

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

    fn clone_boxed(&self) -> Box<dyn VisualObject> {
        Box::new((*self).clone())
    }
}


#[cfg(test)]
mod tests {
    use super::HorizontalLine;

    #[test]
    fn it_prints() {
        assert_prints!(
            [5, 5],
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
