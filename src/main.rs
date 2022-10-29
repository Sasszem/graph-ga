
use ga::do_ga;
use network_raw::{circuit::Circuit, components::*};


// pub mod lib;
mod network_raw;
mod ga;
mod filter;


fn main() {
    let mut ckt = Circuit::new();
    let input = Box::new(InputPort{ });
    let input_r = Box::new(LoadResistor{val: 50.0});
    let load_r = Box::new(LoadResistor{val: 50.0});
    let trans = Box::new(Resistor{val: 0.0});
    let gnd = network_raw::types::CircuitNode { id: 0 };
    let u_int = ckt.new_node();
    ckt.mark_internal(u_int);
    let mid_gen = ckt.new_node();
    let mid_out = ckt.new_node();
    ckt.add_component(input, u_int, gnd);
    ckt.add_component(input_r, mid_gen, u_int);
    ckt.add_component(load_r, mid_out, gnd);
    
    let mid_1 = ckt.new_node();
    let mid_2 = ckt.new_node();
    let mid_3 = network_raw::types::CircuitNode { id: 100 };
    ckt.mark_internal(mid_3);

    ckt.add_component(trans.clone(), mid_gen, mid_1);
    ckt.add_component(trans.clone(), mid_1, mid_2);
    ckt.add_component(trans.clone(), mid_2, mid_3);
    ckt.add_component(trans, mid_3, mid_out);
    

    //do_ga(&ckt, 500, 100, divider::get_fitness, divider::print_result);
    do_ga(&ckt, 500, 1000, filter::get_fitness, filter::print_result);
}

// TODO: tolerances
// TODO: linearization
// TODO: mutation & crossover
// TODO: require copy on components
// TODO: controlled sources
// TODO: default get_linearized() impl. for components
// TODO: transistor parts

// TODO: simplification
// TODO: weighted probabilities of mutation
