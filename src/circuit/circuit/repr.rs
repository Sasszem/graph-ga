use super::*;

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
    pub fn as_spice(&self) -> (String, CircuitNode) {
        let conn = self.get_connections();
        let src = conn[&ComponentId{id: 1}];
        let dst = conn[&ComponentId{id: 3}];
        // one of them is the same, take the other
        let node = if dst.1 == src.0 || dst.1 == src.1 {dst.0 } else {dst.1};
        let gnd = if src.0 == dst.0 || src.0 == dst.1 {src.0} else {src.1};
        return ("* SIMULATED CIRCUIT\n".to_string() + &conn.iter().map(|(id, (c1, c2))| self.components[id].as_spice(*id, if c1.id==gnd.id {CircuitNode{id: 0}} else {*c1}, if c2.id==gnd.id {CircuitNode{id: 0}} else {*c2})).collect::<Vec<_>>().join("\n") + "\n", node);
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

}