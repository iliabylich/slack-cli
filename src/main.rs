mod ui;
mod primitives;
mod http;

use primitives::{Point, Rectangle, Label, FromAtomicAction};
use ui::{Screen, TerminalScreen, AtomicAction};
use http::SlackClient;

fn main() -> Result<(), String> {
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

    let client = match SlackClient::new_from_env() {
        Ok(client) => client,
        Err(err) => panic!("Failed to construct HTTP client, {}", err)
    };
    let channels = client.list_channels()?;
    println!("{:#?}", channels);

    let users = client.list_users()?;
    println!("{:#?}", users);

    Ok(())
}
