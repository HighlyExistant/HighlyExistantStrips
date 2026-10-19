use std::collections::HashMap;

use crate::{agent::StateList, condition::Condition, domain::{Domain, Fact}};

pub struct Effect<D: Domain> {
    effects: HashMap<D::FactName, <D::FactName as Fact>::Enumeration>,
}

impl<D: Domain> Effect<D> {
    pub fn new<I: Into<HashMap<D::FactName, <D::FactName as Fact>::Enumeration>>>(effects: I) -> Self {
        Self { effects: effects.into() }
    }
    pub fn propogate_effect(&self, list: &StateList<D>) -> StateList<D> {
        let mut ret = list.clone();
        for (fact, enumeration) in &self.effects {
            ret.insert(fact.clone(), enumeration.clone());
        }
        ret
    }
}