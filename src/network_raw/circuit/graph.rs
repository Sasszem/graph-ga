use super::*;

impl Circuit {

    pub fn has_wire(&self, first: CircuitNode, second: CircuitNode) -> bool {
        self.graph[&first].iter().any(|(other, cid, _)| self.components[cid].is_wire() && *other == second)
    }
    
    pub fn has_component(&self, first: CircuitNode, second: CircuitNode) -> bool {
        self.graph[&first].iter().any(|(other, _, _)| *other == second)
    }

    pub(in crate) fn add_to_graph(&mut self, cid: ComponentId, top: CircuitNode, bottom: CircuitNode) {
        self.graph.entry(top).or_insert(Vec::new()).push((bottom, cid, ComponentDirection::FORWARD));
        self.graph.entry(bottom).or_insert(Vec::new()).push((top, cid, ComponentDirection::BACKWARD));
        self.next_node = self.next_node.max(top.id + 1).max(bottom.id + 1);
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