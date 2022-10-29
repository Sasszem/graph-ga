use std::{collections::{HashMap, HashSet}};
use rand::{seq::IteratorRandom, thread_rng};

use super::{types::*};



#[derive(Clone)]
pub struct Circuit {
    pub(in super) components: ComponentsList,
    pub(in super) graph: Graph,
    pub(in super) next_node: usize,
    pub(in super) next_component: usize,
    pub(in super) internal: HashSet<CircuitNode>,
}

impl std::fmt::Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();
        for (id, (top, bot)) in self.get_connections() {
            s += &format!("   {} {} {} # {}\n",top.id, bot.id, self.components.get(&id).unwrap(), id.id);
        }
        let n = self.internal.iter().fold(String::new(), |x, y| format!("{}{} ", x, y.id));
        write!(f, "BEGIN CIRCUIT\n{s}   ! {}\nEND CIRCUIT", n)

    }
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

    pub fn as_spice(&self) -> (String, CircuitNode) {
        let conn = self.get_connections();
        let src = conn[&ComponentId{id: 1}];
        let dst = conn[&ComponentId{id: 3}];
        // one of them is the same, take the other
        let node = if dst.1 == src.0 || dst.1 == src.1 {dst.0 } else {dst.1};
        let gnd = if src.0 == dst.0 || src.0 == dst.1 {src.0} else {src.1};
        return ("* SIMULATED CIRCUIT\n".to_string() + &conn.iter().map(|(id, (c1, c2))| self.components[id].as_spice(*id, if c1.id==gnd.id {CircuitNode{id: 0}} else {*c1}, if c2.id==gnd.id {CircuitNode{id: 0}} else {*c2})).collect::<Vec<_>>().join("\n") + "\n", node);
    }
    pub fn mark_internal(&mut self, node: CircuitNode) {
        self.internal.insert(node);
    }
    pub fn component_count(&self) -> usize {
        self.components.iter().filter(|(_, c)| !c.is_fixed() && !c.is_wire()).count()
    }
    pub fn add_component(&mut self, component: BoxedComponent, top: CircuitNode, bottom: CircuitNode) -> ComponentId {
        let cid = ComponentId {
            id: self.next_component
        };
        self.next_component = self.next_component + 1;
        self.components.insert(cid, component);
        self.add_to_graph(cid, top, bottom);
        cid
    }

    pub fn dump(&self) {
        println!("Components: ");
        for (k,c) in self.components.iter() {
            println!("   {} -> {}", k.id, c.to_string());
        }
        println!("Connections:");
        for (g, e) in self.graph.iter() {
            println!("   {}: ", g.id);
            for (e, c, d) in  e.iter() {
                println!("      {} {} {:?}", e.id, c.id, d);
            }
        }
    }

    pub(in crate) fn get_random_node(&self) -> Option<CircuitNode> {
        self.graph.keys().filter(|&x| !self.internal.contains(x)).choose(&mut thread_rng()).copied()
    }

    pub fn has_wire(&self, first: CircuitNode, second: CircuitNode) -> bool {
        self.graph[&first].iter().any(|(other, cid, _)| self.components[cid].is_wire() && *other == second)
    }
    
    pub fn has_component(&self, first: CircuitNode, second: CircuitNode) -> bool {
        self.graph[&first].iter().any(|(other, _, _)| *other == second)
    }

    pub(in super) fn add_to_graph(&mut self, cid: ComponentId, top: CircuitNode, bottom: CircuitNode) {
        self.graph.entry(top).or_insert(Vec::new()).push((bottom, cid, ComponentDirection::FORWARD));
        self.graph.entry(bottom).or_insert(Vec::new()).push((top, cid, ComponentDirection::BACKWARD));
        self.next_node = self.next_node.max(top.id + 1).max(bottom.id + 1);
    }

    pub fn new_node(&mut self) -> CircuitNode {
        let next_id = self.next_node;
        self.next_node = self.next_node + 1;
        CircuitNode{id: next_id}
    }

    pub fn get_connections(&self) -> HashMap<ComponentId, (CircuitNode, CircuitNode)>{
        self.graph.iter().flat_map(
            |(node, conns)| {
                conns.iter().filter(
                    |(_, _, dir)| {
                        match dir {ComponentDirection::FORWARD => true, ComponentDirection::BACKWARD => false}
                    }
                ).map(
                    |(node2, component, _)| {
                        (*component, (*node, *node2))
                    }
                )
            }
        ).collect()
    }
}