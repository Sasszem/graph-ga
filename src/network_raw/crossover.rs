use rand::{thread_rng, Rng, seq::IteratorRandom};

use crate::network_raw::types::ComponentDirection;

use super::{circuit::Circuit, types::*};

pub fn crossover(first: &Circuit, second: &Circuit) -> (Circuit, Circuit) {
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
    let mut remove_from_first : Vec<crate::network_raw::types::ComponentId> = Vec::new();
    for &node in mapping.left_values() {
        first.graph[&node].iter().filter(|(o_node, cid, dir)| matches!(dir, ComponentDirection::FORWARD) && mapping.contains_left(o_node) && !first.components[cid].is_fixed()).for_each(|(o_node, cid, _)| {first_parts.push((node, *o_node, first.components[cid].clone())); remove_from_first.push(*cid);});
        first.graph.entry(node).or_insert_with(|| Vec::new()).retain(|(node, cid, _)| !mapping.contains_left(node)  || first.components[cid].is_fixed());
    }
    let mut remove_from_second : Vec<crate::network_raw::types::ComponentId> = Vec::new();
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

pub fn crossover_2(first: &Circuit, second: &Circuit) -> (Circuit, Circuit) {
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



#[cfg(test)]
mod test {
    use super::crossover;

    const FIRST_CKT : &str = "
    3 2 LoadResistor(50)
    9 7 Wire()
    3 15 Resistor(180)
    8 4 Capacitor(8.2 u)
    4 8 Inductor(10 u)
    15 1 Inductor(300.00000000000006 u)
    1 9 Inductor(1000 m)
    12 11 Capacitor(2.7 n)
    7 12 Capacitor(18 n)
    10 14 Wire()
    5 13 Resistor(10)
    5 10 Capacitor(149.99999999999997 n)
    6 4 Inductor(56 u)
    14 6 Inductor(390 m)
    3 5 Resistor(2.7)
    2 1 Voltage(1000 m)
    4 1 LoadResistor(50)
    13 1 Inductor(270 n)
    11 8 Inductor(68 n)
    13 10 Capacitor(22.000000000000004 u)
    ! 2 
    "; 

    const SECOND_CKT : &str = "
    6 9 Capacitor(3.3 u)
    10 11 Capacitor(999.9999999999999 n)
    1 6 Capacitor(10.000000000000002 u)
    10 6 Resistor(1.3 k)
    5 11 Resistor(15 k)
    9 5 Capacitor(3.9 n)
    5 10 Resistor(2.2)
    5 8 Inductor(1.2 u)
    12 6 Inductor(1.5 u)
    3 5 Resistor(1.8 k)
    2 1 Voltage(1000 m)
    8 4 Resistor(1000)
    7 3 Wire()
    4 1 LoadResistor(50)
    9 8 Resistor(1.5)
    11 7 Resistor(2.7 k)
    8 7 Wire()
    3 2 LoadResistor(50)
    8 12 Inductor(18 u)
    ! 2 
    ";

    #[test]
    fn test_crossover() {
        let first = crate::network_raw::load::load(FIRST_CKT);
        let second = crate::network_raw::load::load(SECOND_CKT);
        let (first, second) = crossover(&first, &second);
        println!("New first: {}", first.to_string());
        println!("New second: {}", second.to_string());
    }
}