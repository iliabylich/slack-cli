use super::action::{AtomicAction};

pub trait VisualObject {
    fn to_actions(&self) -> Vec<AtomicAction>;
    fn clone_boxed(&self) -> Box<dyn VisualObject>;
}
