use crate::ui::AtomicAction;

pub trait VisualObject {
    fn to_actions(&self) -> Vec<AtomicAction>;
}
