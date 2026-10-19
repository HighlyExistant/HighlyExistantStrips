use std::{cell::RefCell, cmp::Reverse, collections::HashSet, rc::Rc};

use ordered_float::NotNan;

use crate::{action::Action, agent::StateList, condition::Condition, domain::Domain};

pub struct PlanNodeInner<D: Domain> {
    pub(crate) parent: Option<PlanNode<D>>,
    pub(crate) action: Option<Rc<dyn Action<D>>>,
    pub(crate) goals: Condition<D>,
    pub(crate) state: StateList<D>,
    pub(crate) cost: NotNan<f32>,
}
#[derive(Clone)]
pub struct PlanNode<D: Domain> {
    inner: Rc<RefCell<PlanNodeInner<D>>>,
}

impl<D: Domain> PlanNode<D> {
    fn from_raw(
        parent: Option<PlanNode<D>>,
        action: Option<Rc<dyn Action<D>>>,
        state: &StateList<D>,
        goals: &Condition<D>,
    ) -> Self {
        let cost = if let Some(parent) = &parent {
            parent.inner.borrow().cost
                + action
                    .as_ref()
                    .map(|a| a.cost(goals))
                    .unwrap_or_else(|| NotNan::new(0.0).unwrap())
        } else {
            action
                .as_ref()
                .map(|a| a.cost(goals))
                .unwrap_or_else(|| NotNan::new(0.0).unwrap())
        };
        // Propogate the effects from the current action
        let (mut state, preconditions) = if let Some(action) = action.as_ref() {
            (
                action.effects().propogate_effect(state),
                Some(action.preconditions()),
            )
        } else {
            (state.clone(), None)
        };
        // Remove goals which are satisfied by this action
        let mut goals = goals.clone();
        if let Some(preconditions) = preconditions {
            goals.add_conditions(preconditions.clone());
        }
        goals.eval_pop(&mut state);
        // Add preconditions
        let inner = PlanNodeInner {
            parent,
            action,
            cost,
            state,
            goals,
        };
        PlanNode {
            inner: Rc::new(RefCell::new(inner)),
        }
    }
    pub fn from_state(state: &StateList<D>, goals: &Condition<D>) -> Self {
        Self::from_raw(None, None, state, goals)
    }
    pub fn new(
        parent: PlanNode<D>,
        action: Rc<dyn Action<D>>,
        state: &StateList<D>,
        goals: &Condition<D>,
    ) -> Self {
        Self::from_raw(Some(parent), Some(action), state, goals)
    }
    pub fn inner(&self) -> std::cell::Ref<'_, PlanNodeInner<D>> {
        self.inner.borrow()
    }
    pub fn cost(&self) -> NotNan<f32> {
        self.inner().cost
    }
}

impl<D: Domain> PartialEq for PlanNode<D> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.borrow().cost == other.inner.borrow().cost
    }
}

impl<D: Domain> Eq for PlanNode<D> {}

impl<D: Domain> PartialOrd for PlanNode<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.inner.borrow().cost).partial_cmp(&Reverse(other.inner.borrow().cost))
    }
}

impl<D: Domain> Ord for PlanNode<D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.inner.borrow().cost).cmp(&Reverse(other.inner.borrow().cost))
    }
}
