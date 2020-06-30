use crate::ui::action::AtomicAction;
use crate::ui::visual::VisualObject;

#[derive(Debug, Clone)]
pub struct FromAtomicAction {
    action: AtomicAction
}

#[allow(dead_code)]
impl FromAtomicAction {
    pub fn new(action: &AtomicAction) -> Self {
        Self { action: action.clone() }
    }
}

impl VisualObject for FromAtomicAction {
    fn to_actions(&self) -> Vec<AtomicAction> {
        vec![self.action.clone()]
    }

    fn clone_boxed(&self) -> Box<dyn VisualObject> {
        Box::new((*self).clone())
    }
}


#[cfg(test)]
mod tests {
    use super::FromAtomicAction;
    use crate::AtomicAction;

    #[test]
    fn it_prints() {
        assert_prints!(
            [5, 5],
            FromAtomicAction { action: AtomicAction::Print { char: 'x' } },
            vec![
                'x', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
                ' ', ' ', ' ', ' ', ' ', '\n',
            ]
        );
    }
}
