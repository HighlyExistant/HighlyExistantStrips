use std::rc::Rc;

use he_strips::{
    action::{Action, effect::Effect},
    agent::Agent,
    condition::Condition,
    domain::{Domain, Fact},
    planner::StripsPlanner,
};
use ordered_float::NotNan;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct BasicDomain;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum FactNames {
    IsComputerOn,
    IsInRoom,
    IsDoorOpen,
    IsDoorDestroyed,
    IsComputerPluggedIn,
    HasBazooka,
}

impl Fact for FactNames {
    type Enumeration = bool;
    fn default_value(&self) -> Self::Enumeration {
        false
    }
    fn to_uuid(&self, enumeration: Self::Enumeration) -> u64 {
        enumeration as u64
    }
}

impl Domain for BasicDomain {
    type FactName = FactNames;
}

pub struct DummyAction {
    cost: f32,
    preconditions: Condition<BasicDomain>,
    effects: Effect<BasicDomain>,
    name: String,
}

impl DummyAction {
    pub fn new(
        name: String,
        preconditions: Condition<BasicDomain>,
        effects: Effect<BasicDomain>,
        cost: f32,
    ) -> Rc<dyn Action<BasicDomain>> {
        Rc::new(Self {
            cost,
            preconditions,
            effects,
            name,
        })
    }
}

impl Action<BasicDomain> for DummyAction {
    fn name(&self) -> Option<String> {
        Some(self.name.clone())
    }
    fn action(&self, agent: &he_strips::agent::Agent<BasicDomain>, delta: f32) -> bool {
        true
    }
    fn effects(&self) -> &he_strips::action::effect::Effect<BasicDomain> {
        &self.effects
    }
    fn preconditions(&self) -> &Condition<BasicDomain> {
        &self.preconditions
    }
    fn cost(&self, goals: &Condition<BasicDomain>) -> ordered_float::NotNan<f32> {
        NotNan::new(self.cost).unwrap()
    }
}

fn main() {
    let open_door = DummyAction::new(
        "open_door".to_string(),
        Condition::And(vec![Condition::Eq(FactNames::IsDoorOpen, false)]),
        Effect::new([(FactNames::IsDoorOpen, true)]),
        1.0,
    );
    let enter_room = DummyAction::new(
        "enter_room".to_string(),
        Condition::And(vec![
            Condition::Eq(FactNames::IsInRoom, false),
            Condition::Or(vec![
                Condition::Eq(FactNames::IsDoorOpen, true),
                Condition::Eq(FactNames::IsDoorDestroyed, true),
            ]),
        ]),
        Effect::new([(FactNames::IsInRoom, true)]),
        1.0,
    );
    let fire_bazooka_door = DummyAction::new(
        "fire_bazooka_door".to_string(),
        Condition::And(vec![
            Condition::Eq(FactNames::HasBazooka, true),
            Condition::Eq(FactNames::IsDoorDestroyed, false),
        ]),
        Effect::new([(FactNames::IsDoorDestroyed, true)]),
        0.5,
    );
    let leave_room = DummyAction::new(
        "leave_room".to_string(),
        Condition::And(vec![
            Condition::Eq(FactNames::IsInRoom, true),
            Condition::Or(vec![
                Condition::Eq(FactNames::IsDoorOpen, true),
                Condition::Eq(FactNames::IsDoorDestroyed, true),
            ]),
        ]),
        Effect::new([(FactNames::IsInRoom, false)]),
        1.0,
    );
    let plug_computer = DummyAction::new(
        "plug_computer".to_string(),
        Condition::And(vec![
            Condition::Eq(FactNames::IsComputerPluggedIn, false),
            Condition::Eq(FactNames::IsInRoom, true),
        ]),
        Effect::new([(FactNames::IsComputerPluggedIn, true)]),
        1.0,
    );
    let turn_on_computer = DummyAction::new(
        "turn_on_computer".to_string(),
        Condition::And(vec![
            Condition::Eq(FactNames::IsComputerOn, false),
            Condition::Eq(FactNames::IsComputerPluggedIn, true),
        ]),
        Effect::new([(FactNames::IsComputerOn, true)]),
        1.0,
    );
    let planner: StripsPlanner<BasicDomain> = StripsPlanner::new(&[
        open_door.clone(),
        enter_room.clone(),
        leave_room.clone(),
        fire_bazooka_door.clone(),
        plug_computer.clone(),
        turn_on_computer.clone(),
    ]);
    let mut agent: Agent<BasicDomain> = Agent::new();
    agent.state.insert(FactNames::HasBazooka, true);
    agent.state.insert(FactNames::IsInRoom, true);
    let goal = Condition::Eq(FactNames::IsInRoom, false);
    let node = planner.build_plan(&mut agent, goal);
    if let Some(plan) = node {
        for i in plan {
            println!("{}", i);
        }
    } else {
        println!("Plan not found");
    }
}
