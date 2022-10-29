use crate::ga::run_with_ngspice;
use crate::circuit::circuit::*;

pub fn get_fitness(ckt: &Circuit) -> f64 {

    let mut score = 0.0;
    let d = run_with_ngspice(&ckt, ".ac dec 100 20 20000\n");
    if d.len() < 5 {
        return 1.0e+8;
    }

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
    let d = run_with_ngspice(&ckt, ".ac dec 200 10000 50meg\n");
    for (f, p) in d {
        println!("{}; {}", f, 20.0 * p.log10());
    }
    println!("---------------------");
    println!("end of circuit tf dump");
    println!("{}", ckt.to_string());
}