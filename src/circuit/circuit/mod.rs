use std::{collections::{HashMap, HashSet}};
use rand::{seq::IteratorRandom, thread_rng};

use super::{types::*};
pub mod load;
mod repr;
pub use repr::*;
mod nodes;
pub use nodes::*;
mod graph;
pub use graph::*;
mod components;
pub use components::*;

#[derive(Clone)]
pub struct Circuit {
    pub(in super) components: ComponentsList,
    pub(in super) graph: Graph,
    pub(in super) next_node: usize,
    pub(in super) next_component: usize,
    pub(in super) internal: HashSet<CircuitNode>,
}


impl Circuit {
    pub fn new() -> Self {
        Circuit {
            components: HashMap::new(),
            graph: HashMap::new(),
            next_component: 1,
            next_node: 1,
            internal: HashSet::new(),
        }
    }
}