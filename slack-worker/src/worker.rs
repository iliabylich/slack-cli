use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use crate::Message;

pub type Sender = mpsc::Sender<Message>;
pub type Receiver = mpsc::Receiver<Message>;

pub trait WorkerImplementation where Self: Send + 'static + Default + std::fmt::Debug {
    fn tick(&self, receiver: &Receiver, subscribers: &Vec<Sender>);
}

pub struct Worker<T> where T : WorkerImplementation {
    name: String,
    pub sender: Sender,
    receiver: Option<Receiver>,
    subscribers: Option<Vec<Sender>>,
    implementation: Option<T>,
}

impl<T> Worker<T> where T: WorkerImplementation {
    pub fn new(name: &str, subscribers: Vec<Sender>) -> Arc<Mutex<Self>> {
        let (sender, receiver) = mpsc::channel::<Message>();

        let worker = Self {
            name: String::from(name),
            sender,
            receiver: Some(receiver),
            subscribers: Some(subscribers),
            implementation: Some(Default::default())
        };

        Arc::new(Mutex::new(worker))
    }

    pub fn spawn(&mut self) -> thread::JoinHandle<()> {
        let (receiver, subscribers, implementation) = match (self.receiver.take(), self.subscribers.take(), self.implementation.take()) {
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
