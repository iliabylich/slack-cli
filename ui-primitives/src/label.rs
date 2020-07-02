use ui_abstract::{Point, AtomicAction, VisualObject};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints() {
        assert_prints!(
            (5, 5),
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
