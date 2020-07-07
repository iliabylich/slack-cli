use crate::AtomicAction;

pub trait VisualObject where Self: Send {
    fn to_actions(&self) -> Vec<AtomicAction>;
}
