use regex::{Regex};

use crate::network_raw::components::{Inductor, VoltageSource, Resistor, Capacitor, parse_si};

use super::{circuit::Circuit, components::*, types::*};

impl Circuit {
  pub fn load(desc: &str) -> Circuit {
    let mut c = Circuit::new();
    let re = Regex::new(r"(\d+)\W(\d+)\s+(\w+)\((\d*\.?\d*)\s?(\w?)\)").unwrap();
    for line_orig in desc.lines() {
      let l = line_orig.split("#").next().unwrap_or_default().trim_start();
      if l.starts_with("BEGIN") || l.starts_with("END") ||  l.is_empty() {
        continue;
      }
  
      if l.starts_with("!") {
        for w in l.split_ascii_whitespace() {
            let x: Result<usize, _> = w.parse();
            if let Ok(x) = x {
              c.mark_internal(x.into());
            }
        }
        continue;
      }
  
      if let Some(cap) = re.captures(l) {
        let top : usize = cap.get(1).unwrap().as_str().parse().unwrap_or_default();
        let bottom : usize = cap.get(2).unwrap().as_str().parse().unwrap_or_default();
        let comp : String = cap.get(3).unwrap().as_str().parse().unwrap_or_default();
        let val : f64 = cap.get(4).unwrap().as_str().parse().unwrap_or_default();
        let unit : String = cap.get(5).unwrap().as_str().parse().unwrap_or_default();
  
        let val = parse_si(val, &unit);
  
        let comp : Option<BoxedComponent> = match comp.as_str() {
          "Resistor" => Some(Box::new(Resistor {val})),
          "Capacitor" => Some(Box::new(Capacitor {val})),
          "Inductor" => Some(Box::new(Inductor {val})),
          "Voltage" => Some(Box::new(VoltageSource {val})),
          "FixedResistor" => Some(Box::new(FixedResistor{val})),
          "Wire" => Some(Box::new(Wire{})),
          "Input" => Some(Box::new(InputPort{})),
          _ => None,
        };
        if let Some(comp) = comp {
          c.add_component(comp, top.into(), bottom.into());
        } else {
          println!("Error: could not parse line (unknown component): '{line_orig}'");
        }
      } else {
        println!("Error: could not parse line (regex fail): '{line_orig}'");
      }
    }
    c
  }
}