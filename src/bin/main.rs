use slack_data::{SlackResult, SlackWorker, SlackState};
use slack_worker::{Spawn};
use slack_ui::RenderWorker;

use std::sync::{Arc, Mutex};

fn main() -> SlackResult<()> {
    let state = Arc::new(Mutex::new(SlackState::default()));

    let mut handles = vec![];

    let mut render_worker = RenderWorker::new("Render", Arc::clone(&state), vec![]);
    let mut slack_worker = SlackWorker::new("Slack", Arc::clone(&state), vec![render_worker.sender.clone()]);

    handles.push(render_worker.spawn());
    handles.push(slack_worker.spawn());

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
