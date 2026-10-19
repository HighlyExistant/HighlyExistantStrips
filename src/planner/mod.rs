use std::{collections::{BinaryHeap, HashMap}, rc::Rc};

use ordered_float::NotNan;

use crate::{action::Action, agent::{Agent, StateList}, condition::Condition, domain::Domain, planner::{node::PlanNode, plan::Plan}};

mod node;
mod plan;

pub struct StripsPlanner<D: Domain> {
    actions: Vec<Rc<dyn Action<D>>>,
}

impl<D: Domain> StripsPlanner<D> {
    pub fn new<I: Into<Vec<Rc<dyn Action<D>>>>>(actions: I) -> Self {
        Self { actions: actions.into() }
    }
    pub fn find_next_actions(&self, state: &StateList<D>, goal: &Condition<D>) -> impl Iterator<Item = &'_ Rc<dyn Action<D>>> {
        self.actions.iter().filter_map(move |action|{
            let mut propogate = action.effects().propogate_effect(&state);
            if goal.condition_satisfied(&mut propogate) {
                Some(action)
            } else {
                None
            }
        })
    }
    pub fn build_plan(&self, agent: &mut Agent<D>, goal: Condition<D>) -> Option<Plan<D>> {
        self.build_plan_node(agent, goal).map(|p|{
            Plan::from(p)
        })
    }
    fn build_plan_node(&self, agent: &mut Agent<D>, goal: Condition<D>) -> Option<PlanNode<D>> {
        let goal = Condition::And(vec![goal]);
        let mut heap: BinaryHeap<PlanNode<D>> = BinaryHeap::new(); // open list
        let current_plan = PlanNode::from_state(&agent.state, &goal);
        heap.push(current_plan);
        let mut map: HashMap<StateList<D>, NotNan<f32>> = HashMap::new(); // close list
        let mut heap_count = 0;
        while let Some(plan) = heap.pop() {
            println!("{:?}", plan.inner().goals);
            if plan.inner().goals.eval(&mut agent.state) { // best plan found
                println!("HEAP PUSH COUNT {}", heap_count);
                return Some(plan);
            }
            // swift through all actions which satisfy some condition in the goal
            for action in self.find_next_actions(&plan.inner().state, &plan.inner().goals) { 
                let next = PlanNode::new(
                    plan.clone(), 
                    action.clone(), 
                    &plan.inner().state,
                    &plan.inner().goals
                );

                println!("can use {:?} cost {:?}", action.name(), next.cost());
                if map.get(&next.inner().state).cloned().unwrap_or(NotNan::new(f32::INFINITY).unwrap()) <= next.cost() {
                    // skip if an action with the same state exists, but with better cost
                    continue;
                } else {
                    map.insert(next.inner().state.clone(), next.cost());
                }
                heap.push(next);
                heap_count += 1;
            }
        }
        None
    }
}