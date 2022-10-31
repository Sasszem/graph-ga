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

    pub fn count_dynamic(&self) -> (usize, usize) {
        let (mut cnt_c, mut cnt_l) = (0,0);

        for (_id, c) in self.components.iter() {
            if let Some(_) = c.get_capacitor() {
                cnt_c += 1;
            }
            if let Some(_) = c.get_inductor() {
                cnt_l += 1;
            }
        }

        (cnt_c, cnt_l)
    }
}