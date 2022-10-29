use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::network_raw::types::BoxedComponent;

use super::{Component, Capacitor, Inductor, Resistor};


pub fn format_si(n: f64) -> String {
    match n {
        v if v > 1.0e+9 => format!("{} G", v / 1.0e+9),
        v if v > 1.0e+6 => format!("{} M", v / 1.0e+6),
        v if v > 1.0e+3 => format!("{} k", v / 1.0e+3),
        v if v > 1.0e+0 => format!("{}", v),
        v if v > 1.0e-3 => format!("{} m", v / 1.0e-3),
        v if v > 1.0e-6 => format!("{} u", v / 1.0e-6),
        v if v > 1.0e-9 => format!("{} n", v / 1.0e-9),
        v if v > 1.0e-12 => format!("{} p", v / 1.0e-12),
        _ => format!("{n}")
    }
}

const E48_SER: [f64; 49] = [
    1.00,	1.05,	1.10,
    1.15,	1.21,	1.27,
    1.33,	1.40,	1.47,
    1.54,	1.62,	1.69,
    1.78,	1.87,	1.96,
    2.05,	2.15,	2.26,
    2.37,	2.49,	2.61,
    2.74,	2.87,	3.01,
    3.16,	3.32,	3.48,
    3.65,	3.83,	4.02,
    4.22,	4.42,	4.64,
    4.87,	5.11,	5.36,
    5.62,	5.90,	6.19,
    6.49,	6.81,	7.15,
    7.50,	7.87,	8.25,
    8.66,	9.09,	9.53,
    10.0,
];

const E24_SER: [f64; 25] = 
[
    1.0,	1.1,	1.2,
    1.3,	1.5,	1.6,
    1.8,	2.0,	2.2,
    2.4,	2.7,	3.0,
    3.3,	3.6,	3.9,
    4.3,	4.7,	5.1,
    5.6,	6.2,	6.8,
    7.5,	8.2,	9.1,
    10.0,
];

const E12_SER: [f64;13] = [
    1.0,	1.2,	1.5,
    1.8,	2.2,	2.7,
    3.3,	3.9,	4.7,
    5.6,	6.8,	8.2,
    10.0,
];

fn random_e48() -> f64 {
    *E48_SER.choose(&mut thread_rng()).unwrap()
}
fn random_e24() -> f64 {
    *E24_SER.choose(&mut thread_rng()).unwrap()
}
fn random_e12() -> f64 {
    *E12_SER.choose(&mut thread_rng()).unwrap()
}


pub fn random_mantissa() -> f64 {
    let choices : [(fn()->f64, f64); 3] = [
        (random_e12, 0.5),
        (random_e24, 0.2),
        (random_e48, 0.05),
    ];
    choices.choose_weighted(&mut thread_rng(), |x| x.1).unwrap().0()
}

pub fn random_value(min_exp: i32, max_exp: i32) -> f64 {
    let exp : i32 = thread_rng().gen_range(min_exp..max_exp+1);
    let mul : f64 = (10.0_f64).powi(exp);
    mul * random_mantissa()
}

pub fn random_component() -> BoxedComponent {
    let choice: i32 = thread_rng().gen_range(1..3);
    match choice {
        0 => Resistor::get_random(),
        1 => Capacitor::get_random(),
        2 => Inductor::get_random(),
        _ => unreachable!(),
    }
}

pub fn parse_si(val: f64, unit: &str) -> f64 {
    let mul = match unit {
        "m" => 1.0e-3,
        "u" => 1.0e-6,
        "n" => 1.0e-9,
        "p" => 1.0e-12,
        "k" => 1.0e+3,
        "M" => 1.0e+6,
        "G" => 1.0e+9,
        _ => 1.0,
    };
    val * mul
}
