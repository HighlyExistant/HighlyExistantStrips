use std::{
    cell::{RefCell, UnsafeCell},
    collections::HashMap,
    hash::Hash,
    sync::{Arc, RwLock},
};

use crate::domain::{Domain, Fact};
#[derive(Clone, PartialEq, Eq)]
pub struct StateList<D: Domain> {
    state: HashMap<D::FactName, <D::FactName as Fact>::Enumeration>,
    hash: u64,
}

impl<D: Domain> Hash for StateList<D> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl<D: Domain> StateList<D> {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
            hash: 0u64,
        }
    }
    pub fn insert(
        &mut self,
        fact: D::FactName,
        enumeration: <D::FactName as Fact>::Enumeration,
    ) -> Option<<<D as Domain>::FactName as Fact>::Enumeration> {
        if let Some(prev) = self.state.get(&fact) {
            self.hash ^= D::FactName::to_uuid(&fact, prev.clone());
            self.hash ^= D::FactName::to_uuid(&fact, enumeration.clone());
        } else {
            self.hash ^= D::FactName::to_uuid(&fact, enumeration.clone());
        }

        self.state.insert(fact, enumeration)
    }
    pub fn get_value(
        &mut self,
        fact: &D::FactName,
    ) -> <<D as Domain>::FactName as Fact>::Enumeration {
        if let Some(val) = self.state.get(fact).cloned() {
            val
        } else {
            let default = D::FactName::default_value(fact);
            self.state.insert(fact.clone(), default.clone());
            default
        }
    }
}

pub struct Agent<D: Domain> {
    pub state: StateList<D>,
}

impl<D: Domain> Agent<D> {
    pub fn new() -> Self {
        Self {
            state: StateList::new(),
        }
    }
}
