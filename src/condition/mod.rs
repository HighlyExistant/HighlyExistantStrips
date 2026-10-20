use crate::{
    agent::StateList,
    domain::{Domain, Fact},
};

#[derive(Debug, Clone)]
pub enum Condition<D: Domain> {
    And(Vec<Condition<D>>),
    Or(Vec<Condition<D>>),
    Eq(D::FactName, <D::FactName as Fact>::Enumeration),
}

impl<D: Domain> Condition<D> {
    /// Returns whether a condition was satisfied.
    /// This depends on which condition specifically
    /// was chosen:
    /// * `Condition::And`: If a single condition is satisfied,
    /// returns true
    /// * `Condition::Or`: If a single condition is satisfied,
    /// returns true
    pub fn condition_satisfied(&self, state: &mut StateList<D>) -> bool {
        match self {
            Condition::And(values) | Condition::Or(values) => {
                for val in values {
                    if val.condition_satisfied(state) {
                        return true;
                    }
                }
                false
            }
            Condition::Eq(val, enumeration) => state.get_value(val) == enumeration.clone(),
        }
    }
    pub fn add_conditions(&mut self, condition: Condition<D>) {
        let current_conditions = if let Self::And(current) = self {
            current
        } else {
            *self = Self::And(vec![self.clone()]);
            self.add_conditions(condition);
            return;
        };
        let mut conditions = if let Self::And(condition) = condition {
            condition
        } else {
            vec![condition]
        };
        current_conditions.append(&mut conditions);
    }
}

impl<D: Domain> Condition<D> {
    pub fn eval_pop(&mut self, state: &mut StateList<D>) -> bool {
        match self {
            Condition::And(values) => {
                let mut i = 0;
                while i < values.len() {
                    if values[i].eval_pop(state) {
                        values.swap_remove(i);
                        continue;
                    }
                    i += 1;
                }
                values.len() == 0
            }
            Condition::Or(values) => {
                for val in values {
                    if val.eval(state) {
                        return true;
                    }
                }
                false
            }
            Condition::Eq(val, enumeration) => state.get_value(val) == enumeration.clone(),
        }
    }
    pub fn eval(&self, state: &mut StateList<D>) -> bool {
        match self {
            Condition::And(values) => {
                for val in values {
                    if !val.eval(state) {
                        return false;
                    }
                }
                true
            }
            Condition::Or(values) => {
                for val in values {
                    if val.eval(state) {
                        return true;
                    }
                }
                false
            }
            Condition::Eq(val, enumeration) => state.get_value(val) == enumeration.clone(),
        }
    }
}
