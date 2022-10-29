use crate::ga::run_with_ngspice;
use crate::network_raw::circuit::*;

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

#[cfg(test)]
mod test {
    use crate::network_raw::load::{load};

    use super::*;

    const CKT : &str = "
    2 1 Input(0) # 1
    210 2 LoadResistor(50) # 2
    210 1 LoadResistor(50) # 3
    266 233 Capacitor(1.2 n) # 1695
    266 290 Resistor(47) # 1692
    233 278 Inductor(99.99999999999999 n) # 1673
    278 266 Inductor(1.15 u) # 1691
    1 210 Capacitor(389.99999999999994 n) # 1682
    278 233 Inductor(1.2 u) # 1683
    207 278 Capacitor(1.5 u) # 1681
    1 207 Capacitor(3 u) # 1698
    233 207 Resistor(22 k) # 1684
    210 290 Inductor(18 m) # 1632
    207 233 Capacitor(8.2 n) # 1477
    ! 2
    ";

    #[test]
    fn eval() {
        let circuit = load(CKT);
        print_result(&circuit);
    }
}
