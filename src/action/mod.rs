use std::fmt::Display;

use ordered_float::NotNan;

use crate::{action::effect::Effect, agent::Agent, condition::Condition, domain::Domain};
pub mod effect;
pub trait Action<D: Domain> {
    fn name(&self) -> Option<String> {
        None
    }
    fn preconditions(&self) -> &Condition<D>;
    fn effects(&self) -> &Effect<D>;
    fn cost(&self, goals: &Condition<D>) -> NotNan<f32> {
        
        NotNan::new(1.0).unwrap()
    }
    fn action(&self, agent: &Agent<D>, delta: f32) -> bool;
}

impl<D: Domain> Display for dyn Action<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.name().unwrap_or("unnamed_action".to_string());
        f.write_str(val.as_str())
    }
}