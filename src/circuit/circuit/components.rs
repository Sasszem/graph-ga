use super::*;

impl Circuit {
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
}