use crate::ui::point::Point;
use crate::ui::action::AtomicAction;
use crate::ui::visual::VisualObject;

#[derive(Debug, Clone)]
pub struct Label {
    pub at: Point,
    pub text: String
}

impl VisualObject for Label {
    fn to_actions(&self) -> Vec<AtomicAction> {
        let mut result = vec![];

        result.push(AtomicAction::MoveAt { point: self.at.clone() });

        for char in self.text.chars() {
            result.push(AtomicAction::Print { char });
        }

        result
    }

    fn clone_boxed(&self) -> Box<dyn VisualObject> {
        Box::new((*self).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Label;

    #[test]
    fn it_prints() {
        assert_prints!(
            [5, 5],
            Label { text: String::from("abc"), at: Point { line: 2, col: 1 } },
            vec![
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', 'a', 'b', 'c', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
            ]
        );
    }
}
