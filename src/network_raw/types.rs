use std::{collections::HashMap, hash::Hash};

use rand::{seq::SliceRandom, thread_rng};


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct CircuitNode {
    pub id: usize,
}

impl std::convert::From<usize> for CircuitNode {
    fn from(s: usize) -> Self {
        CircuitNode { id: s }
    }
}

pub enum AllowedSeriesConnections {
    NONE,
    TOP,
    BOTTOM,
    BOTH,
}

impl AllowedSeriesConnections {
    pub fn replace_allowed(&self, top: CircuitNode, bottom: CircuitNode, new: CircuitNode) -> (CircuitNode, CircuitNode, Option<(CircuitNode, CircuitNode)>){
        match self {
            Self::NONE => (top, bottom, None),
            Self::TOP => (new, bottom, Some((top, new))),
            Self::BOTTOM => (top, new, Some((new, bottom))),
            Self::BOTH => [Self::TOP, Self::BOTTOM].choose(&mut thread_rng()).unwrap().replace_allowed(top, bottom, new)
        }
    }
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ComponentId {
    pub id: usize
}
impl std::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cid = {}", self.id)
    }
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ComponentDirection {
    FORWARD,
    BACKWARD
}

pub type ComponentsList = HashMap<ComponentId, BoxedComponent>;
pub type Graph = HashMap<CircuitNode, Vec<(CircuitNode, ComponentId, ComponentDirection)>>;
pub type BoxedComponent = Box<dyn super::components::Component + Send + Sync>;