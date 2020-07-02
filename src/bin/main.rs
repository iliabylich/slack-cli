extern crate ui_abstract;
extern crate ui_primitives;
extern crate ui_terminal;
extern crate slack_http;

use ui_abstract::{Point, AtomicAction, Screen};
use ui_primitives::{Rectangle, Label, FromAtomicAction};
use ui_terminal::{TerminalScreen};
use slack_http::{SlackClient, SlackResult};

fn main() -> SlackResult<()> {
    let mut screen = TerminalScreen::new();

    screen.push_object(Box::new(Rectangle { top_left: Point { line: 5, col: 5}, bottom_right: Point { line: 11, col: 30 } }));
    screen.push_object(Box::new(Label { at: Point { line: 8, col: 15 }, text: String::from("hello") }));
    screen.push_object(
        Box::new(
            FromAtomicAction::new(
                &AtomicAction::MoveAt {
                    point: Point {
                        line: 20, col: 1
                    }
                }
            )
        )
    );

    screen.redraw().unwrap_or_else(|err| panic!("Failed to draw, {}", err));

    let client = SlackClient::new_from_env()?;
    let conversations = client.list_conversations()?;
    println!("{:#?}", conversations);

    let users = client.list_users()?;
    println!("{:#?}", users);

    let messages = client.conversation_history("C016CGERTDF")?;
    println!("{:#?}", messages);

    Ok(())
}
