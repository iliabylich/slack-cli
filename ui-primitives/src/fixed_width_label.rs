use crate::{Label};
use ui_abstract::{AtomicAction, VisualObject};

#[derive(Debug)]
pub struct FixedWidthLabel {
    pub width: i32,
    pub height: i32,
    pub labels: Vec<Label>
}

impl FixedWidthLabel {
    pub fn new(width: i32, label: Label) -> Self {
        let mut lines = vec![];
        let mut line = vec![];
        let mut last_word = String::from("");

        for letter in label.text.chars() {
            match letter {
                '\n' => {
                    line.push(last_word.clone());
                    last_word.clear();

                    lines.push(line.join(" "));
                    line.clear();
                },
                ' ' => {
                    line.push(last_word.clone());
                    last_word.clear();
                }
                other => {
                    last_word.push(other);
                    if line.join(" ").len() + 1 + last_word.len() >= width as usize {
                        lines.push(line.join(" "));
                        line.clear();
                    }
                }
            };
        }

        if !line.is_empty() {
            lines.push(line.join(" "));
        }

        let mut labels = vec![];

        for (idx, line) in lines.iter().enumerate() {
            let label = Label { text: line.clone(), at: label.at.down(idx as i32) };
            labels.push(label);
        };

        Self { width, height: labels.len() as i32, labels }
    }
}

impl VisualObject for FixedWidthLabel {
    fn to_actions(&self) -> Vec<AtomicAction> {
        let mut result = vec![];

        for label in self.labels.clone() {
            result.append(&mut label.to_actions());
        }

        result
    }
}
