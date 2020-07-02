use crate::{AtomicAction, IoResult};

pub trait Printer {
    fn print(&mut self, action: &AtomicAction) -> IoResult;
}
