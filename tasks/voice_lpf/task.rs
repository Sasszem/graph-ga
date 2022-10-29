use galib::ga::*;
use galib::circuit::circuit::*;
use galib::circuit::components::*;

fn run_spice(ckt: &Circuit) -> Vec<(f64, f64)> {
    use galib::circuit::types::{ComponentId};

    let conn = ckt.get_connections();
    let src = conn[&ComponentId{id: 1}];
    let dst = conn[&ComponentId{id: 3}];
    // one of them is the same, take the other
    let node = if dst.1 == src.0 || dst.1 == src.1 {dst.0 } else {dst.1};
    let gnd = if src.0 == dst.0 || src.0 == dst.1 {src.0} else {src.1};
    
    run_with_ngspice(&ckt, gnd, &format!(".ac dec 100 20 20000\n.print ac vm({})\n", node.id))
}

pub fn get_fitness(ckt: &Circuit) -> f64 {
    let d = run_spice(ckt);

    if d.len() < 5 {
        return 1.0e+8;
    }
    let mut score = 0.0;
    score += d.iter().map(|(f, r)| {
        let mag = 20.0*r.log10();
        if f > &4000.0 {
            let d = mag + 46.0;
            if d > 0.0 {
                d*100.0
            }
            else {
                0.0
            }
        } else if &3400.0 > f {
            let d = (mag + 6.0).abs();
            if d < 0.5 {
                d*100.0
            } else {
                d*5000.0
            }
        } else {0.0}
    }).sum::<f64>();

    score += (ckt.component_count() as f64) * 200.0;
    return score.abs() + 1.0;
}


pub fn print_result(ckt: &Circuit) {
    println!("begin circuit tf dump");
    println!("---------------------");
    let d = run_spice(&ckt);
    for (f, p) in d {
        println!("{}; {}", f, 20.0 * p.log10());
    }
    println!("---------------------");
    println!("end of circuit tf dump");
    println!("{}", ckt.to_string());
}

fn mutate(ckt: &mut Circuit, gen: u32) {
    let settle = gen > 350;
    for _ in if settle {0..3} else {0..7} {
        do_mutation_n_tries(ckt, 10, if settle {&MUT_CHOICES_SETTLE} else {&MUT_CHOICES_MOD});
    }
}

fn main() {
    let mut ckt = Circuit::new();
    let input = CInputPort{ }.into();
    let input_r = CFixedResistor{val: 50.0}.into();
    let load_r = CFixedResistor{val: 50.0}.into();
    let trans: Component = CResistor{val: 0.0}.into();
    let gnd = galib::circuit::types::CircuitNode { id: 0 };
    let u_int = ckt.new_node();
    ckt.mark_internal(u_int);
    let mid_gen = ckt.new_node();
    let mid_out = ckt.new_node();
    ckt.add_component(input, u_int, gnd);
    ckt.add_component(input_r, mid_gen, u_int);
    ckt.add_component(load_r, mid_out, gnd);
    
    let mid_1 = ckt.new_node();
    let mid_2 = ckt.new_node();
    let mid_3 = galib::circuit::types::CircuitNode { id: 100 };
    ckt.mark_internal(mid_3);

    ckt.add_component(trans.clone(), mid_gen, mid_1);
    ckt.add_component(trans.clone(), mid_1, mid_2);
    ckt.add_component(trans.clone(), mid_2, mid_3);
    ckt.add_component(trans, mid_3, mid_out);
    
    galib::ga::do_ga(&ckt, 500, 1000, get_fitness, print_result, mutate, galib::ga::crossover_random_swap, "voice_lpf_fitness.csv", "voice_lpf_checkpoint.csv");
}