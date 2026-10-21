use std::rc::Rc;

use crate::{action::Action, domain::Domain, planner::node::PlanNode};

pub struct Plan<D: Domain> {
    pub(crate) actions: Vec<Rc<dyn Action<D>>>,
    current_step: usize,
}

impl<D: Domain> From<PlanNode<D>> for Plan<D> {
    fn from(node: PlanNode<D>) -> Self {
        let mut actions = vec![];
        let mut current = Some(node);
        while let Some(node) = current {
            let action = if let Some(action) = node.inner().action.clone() {
                action
            } else {
                break;
            };
            actions.push(action);
            current = node.inner().parent.clone();
        }
        Self {
            current_step: 0,
            actions,
        }
    }
}

impl<D: Domain> Iterator for Plan<D> {
    type Item = Rc<dyn Action<D>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(action) = self.actions.get(self.current_step).cloned() {
            self.current_step += 1;
            Some(action)
        } else {
            None
        }
    }
}
