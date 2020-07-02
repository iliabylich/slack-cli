extern crate ui;
extern crate ui_primitives;
extern crate terminal_ui;
extern crate slack_http;

use ui::{Point, AtomicAction, Screen};
use ui_primitives::{Rectangle, Label, FromAtomicAction};
use terminal_ui::{TerminalScreen};
use slack_http::{SlackClient, Error};

fn main() -> Result<(), Error> {
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
    let channels = client.list_channels()?;
    println!("{:#?}", channels);

    let users = client.list_users()?;
    println!("{:#?}", users);

    Ok(())
}
