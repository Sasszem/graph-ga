use regex::{Regex};

use crate::network_raw::components::{Inductor, VoltageSource, Resistor, Capacitor, parse_si};

use super::{circuit::Circuit, components::*, types::*};

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
        "LoadResistor" => Some(Box::new(LoadResistor{val})),
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


#[cfg(test)]
mod test {
  use super::load;
  #[test]
  fn test_load() {
    let data = "
    BEGIN CIRCUIT DESCRIPTION
      1 3 Resistor(82.5 k)
      3 1 LoadResistor(50)
      2 1 Voltage(1000 m)
      1 2 Wire()
      1 3 Resistor(9.53 k)
      1 3 Resistor(16.200000000000003)
      3 2 LoadResistor(50)
    END CIRCUIT";

    println!("Circuit: {}", load(data));
  }
}