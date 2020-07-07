use crate::AtomicAction;

pub trait VisualObject where Self: Send + std::fmt::Debug {
    fn to_actions(&self) -> Vec<AtomicAction>;
}
