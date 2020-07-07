use std::thread;
use std::sync::mpsc;

use crate::Message;

pub type Sender = mpsc::Sender<Message>;
pub type Receiver = mpsc::Receiver<Message>;

pub trait WorkerImplementation where Self: Send + std::fmt::Debug + 'static {
    type State;

    fn new(state: Self::State) -> Self;
    fn tick(&mut self, receiver: &Receiver, subscribers: &Vec<Sender>);
}

pub trait Spawn {
    type Implementation: WorkerImplementation;
    type State;

    fn new(name: &str, state: Self::State, subscribers: Vec<Sender>) -> Self;
    fn spawn(&mut self) -> thread::JoinHandle<()>;
}


pub struct Worker<Impl> where Impl: WorkerImplementation {
    name: String,
    pub sender: Sender,
    receiver: Option<Receiver>,
    subscribers: Option<Vec<Sender>>,
    implementation: Option<Impl>,
}

impl<Impl> Spawn for Worker<Impl> where Impl: WorkerImplementation {
    type Implementation = Impl;
    type State = Impl::State;

    fn new(name: &str, state: Self::State, subscribers: Vec<Sender>) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>();
        let implementation: Impl = Impl::new(state);

        Self {
            name: String::from(name),
            sender,
            receiver: Some(receiver),
            subscribers: Some(subscribers),
            implementation: Some(implementation)
        }
    }

    fn spawn(&mut self) -> thread::JoinHandle<()> {
        let (receiver, subscribers, mut implementation) = match (self.receiver.take(), self.subscribers.take(), self.implementation.take()) {
            (Some(receiver), Some(subscribers), Some(implementation)) => (receiver, subscribers, implementation),
            other => panic!("Worker {}: internal data is empty, can't spawn a worker twice, {:#?}", &self.name, other)
        };

        thread::spawn(move || {
            loop {
                match receiver.try_recv() {
                    Ok(Message::Exit) => break,
                    _ => ()
                };

                implementation.tick(&receiver, &subscribers);
            };
        })
    }
}
