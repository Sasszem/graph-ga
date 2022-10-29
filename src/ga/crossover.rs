use rand::{thread_rng, Rng, seq::IteratorRandom};

use crate::circuit::types::ComponentDirection;

use crate::circuit::{*, types::*};

pub fn crossover_subgraph_swap(first: &Circuit, second: &Circuit) -> (Circuit, Circuit) {
    let (mut first, mut second) = (first.clone(), second.clone());
    
    let num_nodes = first.graph.keys().filter(|&x| !first.internal.contains(x)).count().min(second.graph.keys().filter(|&x| !second.internal.contains(x)).count());
    //println!("Nodes in circuit: {N}");
    let no_nodes = thread_rng().gen_range(2 .. (num_nodes/2+1).max(3).min(8));
    //println!("Want to crossover {no_nodes} nodes");


    let mut first_parts : Vec<(CircuitNode, CircuitNode, BoxedComponent)> = Vec::new();
    let mut second_parts : Vec<(CircuitNode, CircuitNode, BoxedComponent)> = Vec::new();
    let mut mapping : bimap::BiMap<CircuitNode, CircuitNode> = bimap::BiMap::new();

    // add seed node
    let seed = (first.get_random_node().unwrap(), second.get_random_node().unwrap());
    mapping.insert(seed.0, seed.1);

    
    for _ in 0..10 {
        if mapping.len() == no_nodes {
            break;
        }
        // have to choose a branch into a not yet discovered

        let (&first_base, &second_base) = mapping.iter().choose(&mut thread_rng()).unwrap();

        // pick two nodes to add to the spannign tree
        if let (Some((first_node, _first_cid, _first_dir)), Some((second_node, _second_cid, _second_dir))) = (
                first.graph[&first_base].iter().filter(|(node, cid, _)| !mapping.contains_left(node) && !first.components[cid].is_fixed() && !first.internal.contains(node)).next(),
                second.graph[&second_base].iter().filter(|(node, cid, _)| !mapping.contains_right(node) && !second.components[cid].is_fixed() && !second.internal.contains(node)).next()
            ) {
            mapping.insert(*first_node, *second_node);
        }
    }

    // we now have a group of nodes
    // now we only need to add the inner branches to the graphs
    // we have a few components & nodes now
    let mut remove_from_first : Vec<crate::circuit::types::ComponentId> = Vec::new();
    for &node in mapping.left_values() {
        first.graph[&node].iter().filter(|(o_node, cid, dir)| matches!(dir, ComponentDirection::FORWARD) && mapping.contains_left(o_node) && !first.components[cid].is_fixed()).for_each(|(o_node, cid, _)| {first_parts.push((node, *o_node, first.components[cid].clone())); remove_from_first.push(*cid);});
        first.graph.entry(node).or_insert_with(|| Vec::new()).retain(|(node, cid, _)| !mapping.contains_left(node)  || first.components[cid].is_fixed());
    }
    let mut remove_from_second : Vec<crate::circuit::types::ComponentId> = Vec::new();
    for &node in mapping.right_values() {
        second.graph[&node].iter().filter(|(o_node, cid, dir)| matches!(dir, ComponentDirection::FORWARD) && mapping.contains_right(o_node) && !second.components[cid].is_fixed()).for_each(|(o_node, cid, _)| {second_parts.push((node, *o_node, second.components[cid].clone())); remove_from_second.push(*cid);});
        second.graph.entry(node).or_insert_with(|| Vec::new()).retain(|(node, cid, _)| !mapping.contains_right(node) || second.components[cid].is_fixed());
    }
    
    // now remove marked components from the graphs
    first.components.retain(|x, _| !remove_from_first.contains(x));
    first.graph.iter_mut().for_each(
        |(_, vec)| {
            vec.retain(|(_, cid, _)| !remove_from_first.contains(cid));
        }
    );

    second.components.retain(|x, _| !remove_from_second.contains(x));
    second.graph.iter_mut().for_each(
        |(_, vec)| {
            vec.retain(|(_, cid, _)| !remove_from_second.contains(cid));
        }
    );

    // now add them back, but into the other graph
    for (n1, n2, c) in first_parts.into_iter() {
        second.add_component(c, *mapping.get_by_left(&n1).unwrap(), *mapping.get_by_left(&n2).unwrap());
    }
    for (n1, n2, c) in second_parts.into_iter() {
        first.add_component(c, *mapping.get_by_right(&n1).unwrap(), *mapping.get_by_right(&n2).unwrap());
    }

    (first, second)
}

pub fn crossover_random_swap(first: &Circuit, second: &Circuit) -> (Circuit, Circuit) {
    let (mut left, mut right) = (first.clone(), second.clone());

    let num_comps = first.components.iter().filter(|(_id, comp)| !comp.is_fixed()).count().min(second.components.iter().filter(|(_id, comp)|!comp.is_fixed()).count());


    let no_nodes = thread_rng().gen_range(2 .. num_comps.max(3));
    // do 2* no_nodes tries
    for _ in 0..(no_nodes*2) {
        if let (Some(&a), Some(&b)) = (
                        left .components.keys().filter(|c| !left.components[c].is_fixed()).choose(&mut thread_rng()),
                        right.components.keys().filter(|c| !right.components[c].is_fixed()).choose(&mut thread_rng())
                    ) {
            let (l, r) = (left.components.remove(&a).unwrap(), right.components.remove(&b).unwrap());
            left.components.insert(a, r);
            right.components.insert(b, l);
        }
    }


    (left, right)
}