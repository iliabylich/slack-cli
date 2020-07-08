use ui_abstract::{VisualObject, AtomicAction, Point};
use slack_data::SlackState;
use ui_terminal::TerminalScreenSize;
use std::sync::{Arc, Mutex};
use ui_primitives::{Label, Rectangle};
use crate::Size;


#[derive(Debug)]
pub struct ConversationsList {
    terminal_size: Arc<Mutex<TerminalScreenSize>>,
    slack_state: Arc<Mutex<SlackState>>
}

impl ConversationsList {
    const LINES: Size = Size::Percentage(100);
    const COLS: Size = Size::Percentage(20);

    pub fn new(terminal_size: Arc<Mutex<TerminalScreenSize>>, slack_state: Arc<Mutex<SlackState>>) -> Self {
        Self { terminal_size, slack_state }
    }
}

impl VisualObject for ConversationsList {
    fn to_actions(&self) -> Vec<AtomicAction> {
        let mut result = vec![];

        let (total_lines, total_cols) = {
            let terminal_size = self.terminal_size.lock().unwrap();
            (terminal_size.lines, terminal_size.cols)
        };

        let lines = ConversationsList::LINES.on_screen(total_lines);
        let cols = ConversationsList::COLS.on_screen(total_cols);

        let panel = Rectangle {
            top_left: Point { line: 1, col: 1 },
            bottom_right: Point { line: lines, col: cols }
        };
        result.append(&mut panel.to_actions());

        let uptime = {
            self.slack_state.lock().unwrap().uptime
        };

        let top_label = Label { text: format!("uptime: {}", uptime), at: Point { line: 2, col: 4 } };
        result.append(&mut top_label.to_actions());

        let conversations = {
            self.slack_state.lock().unwrap().all_conversations.clone()
        };

        for (idx, conversation) in conversations.iter().enumerate() {
            let idx = idx as i32;
            let rect = Rectangle {
                top_left: Point { line: idx * 3 + 4, col: 2 },
                bottom_right: Point { line: idx * 3 + 3 + 3, col: cols - 1 }
            };
            result.append(&mut rect.to_actions());

            let label = Label {
                text: conversation.name.clone(),
                at: Point { line: idx * 3 + 4 + 1, col: 4 }
            };
            result.append(&mut label.to_actions());


        }

        result
    }
}
