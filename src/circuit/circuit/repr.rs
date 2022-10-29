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
    pub fn as_spice(&self, gnd: CircuitNode) -> String {
        let conn = self.get_connections();
        return "* SIMULATED CIRCUIT\n".to_string() + &conn.iter().map(|(id, (c1, c2))| self.components[id].as_spice(*id, if c1.id==gnd.id {CircuitNode{id: 0}} else {*c1}, if c2.id==gnd.id {CircuitNode{id: 0}} else {*c2})).collect::<Vec<_>>().join("\n") + "\n";
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