use rand::{thread_rng, seq::{IteratorRandom, SliceRandom}};

use crate::network_raw::components::Wire;

use super::*;
use crate::network_raw::components::*;

fn mut_add_random_component_parallel(ckt: &mut Circuit) -> bool {
    // chose 2 random nodes - must not be the same
    if let (Some(top), Some(bottom)) = (ckt.get_random_node(), ckt.get_random_node()) {
        // let pair: Vec<_> = ckt.graph.keys().choose_multiple(&mut thread_rng(), 2).into_iter().map(|x| *x).collect();
        // if wire exists between poles
        if top == bottom || ckt.has_wire(top, bottom) {
            return false;
        }
        
        let comp = random_component();
        ckt.add_component(comp, top, bottom);
    
        // println!("Executed: add parallel {:?} {:?}", top, bottom);
        return true;
    } else {
        return false;
    }
}

fn mut_add_random_component_series(ckt: &mut Circuit) -> bool {
    // choose random component in graph
    if let Some(comp) = ckt.components.keys().choose(&mut thread_rng()) {
        let comp = *comp;
        if ckt.components[&comp].is_wire() || ckt.components[&comp].is_fixed() {
            return false;
        }
        let conn = ckt.get_connections();
        let (top, bott) = conn.get(&comp).unwrap();
        let (top, bott) = (*top, *bott);
        let allowed = ckt.components[&comp].get_allowed_connections();
        let new_node = ckt.new_node();
        if let (new_top, new_bottom, Some((insert_top, insert_bottom))) = allowed.replace_allowed(top, bott, new_node) {
            ckt.graph.get_mut(&top).unwrap().retain(|&x| x.0 != bott || x.1 != comp);
            ckt.graph.get_mut(&bott).unwrap().retain(|&x| x.0 != top || x.1 != comp);
            ckt.add_to_graph(comp, new_top, new_bottom);
            
            let new_comp = random_component();
            ckt.add_component(new_comp, insert_top, insert_bottom);
            // println!("Executed: add series");
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn mut_replace_component(ckt: &mut Circuit) -> bool {
    if let Some(&to_replace) = ckt.components.keys().choose(&mut thread_rng()) {
        if ckt.components.get(&to_replace).unwrap().is_fixed() {
            return false;
        }
        let new_component = random_component();
        ckt.components.insert(to_replace, new_component);
        // println!("Executed: replace");
        true
    } else {
        false
    }
}

fn mut_delete_component(ckt: &mut Circuit) -> bool {
    if let Some(&to_replace) = ckt.components.keys().choose(&mut thread_rng()) {

        if ckt.components.get(&to_replace).unwrap().is_fixed() {
            return false;
        }
        if *[true, false].choose(&mut thread_rng()).unwrap() {
            let (top, bottom) = ckt.get_connections()[&to_replace];
            
            ckt.components.remove(&to_replace);
            ckt.graph.iter_mut().for_each(|(_, conns)| conns.retain(|(_, comp, _)| *comp != to_replace));
            
            if !ckt.has_component(top, bottom) {
                let w = Box::new(Wire{});
                ckt.add_component(w, top, bottom);
            }
        } else {
            ckt.components.remove(&to_replace);
            ckt.graph.iter_mut().for_each(|(_, conns)| conns.retain(|(_, comp, _)| *comp != to_replace));
        }
        // println!("Executed: delete component");
        true
    } else {
        false
    }
}

fn mut_modify_component(ckt: &mut Circuit) -> bool {
    if let Some(&to_modify) = ckt.components.keys().choose(&mut thread_rng()) {
        if ckt.components.get(&to_modify).unwrap().is_fixed() {
            return false;
        }
        ckt.components.get_mut(&to_modify).unwrap().randomize_val();
        // println!("Executed: modify component");
        true
    } else {
        false
    }
}

fn is_unconnected_node(ckt: &Circuit, node: CircuitNode) -> bool {
    if let Some(edges) = ckt.graph.get(&node) {
        let mut cnt = 0;
        for (other_node, cid, _) in edges {
            if ckt.internal.contains(other_node) || ckt.components.get(cid).unwrap().is_fixed() {
                return false;
            }
            cnt += 1;
        }
        return cnt < 2;
    } else {
        return false;
    }
}


mod simpl {
    use super::*;

    pub fn simpl_remove(ckt: &mut Circuit) -> bool {
        /*
        Simplifications TODO:
        - combine series of same type into single using series combination
        - combine parallel of same type into single
        - remove any components parallel connected with a wire
        - merge nodes of wires
    
        Simpl done:
        - remove nodes & components connected to only one node
        */
    
        // println!("Executing simpl, ckt at begin: {}", ckt.to_string());
        let mut to_remove_comp = Vec::new();
        let mut to_remove_nodes: Vec<CircuitNode> = ckt.graph.keys().clone().filter(|&n| is_unconnected_node(ckt, *n)).copied().collect();
        
        while !to_remove_nodes.is_empty() {
            let n = to_remove_nodes.pop().unwrap();
            if let Some(ed) = ckt.graph.get(&n).cloned() {
                for (on, id, _) in ed {
                    ckt.graph.entry(on).and_modify(|f| f.retain(|(_, b, _)| b.id != id.id));
                    if is_unconnected_node(ckt, on) {
                        to_remove_nodes.push(on);
                    }
                    to_remove_comp.push(id);
                }
                ckt.graph.remove(&n);
            }
        }
        // println!("deleted components: {:?}", to_remove_comp);
        // let rm: Vec<_> = to_remove_comp.iter().map(|x| ckt.components[x].to_string()).collect();
        // println!("{:?}", rm);
        for k in to_remove_comp {
            ckt.components.remove(&k);
        }
        // println!("Resulting ckt graph: {:?}", ckt.graph);
        // println!("Executed simpl, ckt at end: {}", ckt.to_string());
        true
    }
    pub fn simpl_wire_combine(ckt: &mut Circuit) -> bool {
        // let n_prot = ckt.fixed_component_count();
        // println!("Merging wires: {}", ckt.to_string());
        let c = ckt.get_connections();
        let wires = ckt.components.iter().filter_map(|(cid, br)| if br.is_wire() {Some(*cid)} else {None});
        // let before = ckt.to_string();

        // has prot. component in parallel -> can't combine
        let to_combine:Vec<_> = wires.map(|cid| c.get(&cid).unwrap().clone() ).collect();
        // println!("To combine: {:?}", to_combine);
        // when we combine 2 nodes we have to delete all components between them
        for (first, second) in to_combine {
            if let Some(edges) = ckt.graph.get(&first) {
                let has_prot = edges.iter().any(|(on, cid, _)| ckt.components[cid].is_fixed() && on.id == second.id);
                if has_prot {
                    continue;
                }
            }
            // remove all components between them
            if let (Some(first_edges), Some(second_edges)) = (ckt.graph.get(&first), ckt.graph.get(&second)) {
                let first_edges = first_edges.clone();
                let second_edges = second_edges.clone();

                // merge both nodes into the first
                // graph[second] will be erased, but we need to erase cross components from first
                first_edges.iter().filter_map(|(node, cid, _)| if node.id == second.id {Some(*cid)} else {None}).for_each(|f| {ckt.components.remove(&f);});

                let mut add_to_first = Vec::new();
                // *-first components are ok, but *-second ones will need to be re-written to *-first
                second_edges.iter().for_each(|(other_node, cid, dir)| { // iter trough edges from second,
                    if let Some(other_end) = ckt.graph.get_mut(other_node) {
                        for p in other_end { // iter trough edges from other end, ie. iter trough k where exists k-second branches
                            if p.0.id == second.id {
                                p.0 = first;
                            }
                        }
                    }
                    if other_node.id != first.id {
                        add_to_first.push((*other_node, *cid, *dir))
                    }
                });

                for k in add_to_first {
                    ckt.graph.get_mut(&first).unwrap().push(k);
                }

                if let Some(edges) = ckt.graph.get_mut(&first) {
                    edges.retain(|(n, _, _)| n.id != second.id && n.id != first.id);
                }
                ckt.graph.remove(&second);
            }
        }
        // let n_prot_2 = ckt.fixed_component_count();
        // if n_prot_2 < n_prot {
        //     println!("This mutation removed a protected component! {}", before);
        //     println!("Componets:");
        //     for c in ckt.components.iter() {
        //         println!("   {} -> {}", c.0.id, c.1.to_string());
        //     }
        //     println!("After combine: {:?}", ckt.graph);
        //     println!("After combine: {}", ckt.to_string());
        // }
        true
    }

}

pub fn mut_simplify(ckt: &mut Circuit) -> bool {
    simpl::simpl_remove(ckt);
    simpl::simpl_wire_combine(ckt);
    true
}

pub const CHOICES_MOD: [(fn(&mut Circuit)->bool, f64, &str); 6] = [
    (mut_add_random_component_parallel, 0.2, "parallel"),
    (mut_add_random_component_series, 0.2, "series"),
    (mut_delete_component, 0.2, "delete"),
    (mut_modify_component, 0.8, "modify"),
    (mut_replace_component, 0.3, "replace"),
    (mut_simplify, 0.1, "simpl")
];
pub const CHOICES_SETTLE: [(fn(&mut Circuit)->bool, f64, &str); 4] = [
    //(mut_add_random_component_parallel, 0.1, "parallel"),
    //(mut_add_random_component_series, 0.1, "series"),
    (mut_delete_component, 0.3, "delete"),
    (mut_modify_component, 0.8, "modify"),
    (mut_replace_component, 0.4, "replace"),
    (mut_simplify, 0.1, "simpl")
];

pub fn do_mutation(ckt: &mut Circuit, choices: &[(fn(&mut Circuit)->bool, f64, &str)]) -> bool {
    let k = choices.choose_weighted(&mut thread_rng(), |x| x.1);
    let res = k.unwrap().0(ckt);

    res
}

pub fn do_mutation_n_tries(ckt: &mut Circuit, n: usize, choices: &[(fn(&mut Circuit)->bool, f64, &str)]) -> bool {
    for _ in 0..n {
        if do_mutation(ckt, choices) {
            return true;
        }
    }
    return false;
}
