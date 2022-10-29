use super::*;


impl Circuit {
    pub fn mark_internal(&mut self, node: CircuitNode) {
        self.internal.insert(node);
    }

    pub(in crate) fn get_random_node(&self) -> Option<CircuitNode> {
        self.graph.keys().filter(|&x| !self.internal.contains(x)).choose(&mut thread_rng()).copied()
    }

    pub fn new_node(&mut self) -> CircuitNode {
        let next_id = self.next_node;
        self.next_node = self.next_node + 1;
        CircuitNode{id: next_id}
    }
}