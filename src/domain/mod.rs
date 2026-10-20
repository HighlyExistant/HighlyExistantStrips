use std::{fmt::Debug, hash::Hash};

pub trait Fact: Clone + 'static + Hash + Eq + Debug {
    type Enumeration: Eq + Clone + Debug;
    fn default_value(&self) -> Self::Enumeration;
    fn to_uuid(&self, enumeration: Self::Enumeration) -> u64;
}

pub trait Domain: Clone + Eq + Debug + 'static {
    type FactName: Fact;
}
