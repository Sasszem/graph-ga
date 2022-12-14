use crate::circuit::types::*;
use super::{Component, format_si, random_value, closest_e48};

#[derive(Debug, Copy, Clone)]
pub struct Resistor {
    pub val: f64,
}

impl std::fmt::Display for Resistor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resistor({})", format_si(self.val))
    }
}

impl Component for Resistor {
    fn as_spice(&self, id: ComponentId, top: CircuitNode, bot: CircuitNode) -> String {
        format!("R_{} {} {} {}", id.id, top.id, bot.id, self.val)
    }
    fn get_random() -> Box<Self> where Self: Sized {
        Box::new(Resistor{val: Self::random_val()})
    }
    fn random_val() -> f64 where Self: Sized {
        random_value(0, 6)
    }
    fn randomize_val(&mut self) {
        self.val = Self::random_val();
    }
    fn get_resistor(&self) -> Option<&Resistor> {
        Some(self)
    } 
    fn add_series(&mut self, other: &BoxedComponent) -> bool {
        if let Some(other) = other.get_resistor() {
            self.val = closest_e48(self.val + other.val);
            true
        } else {
            false
        }
    }
    fn add_parallel(&mut self, other: &BoxedComponent) -> bool {
        if let Some(other) = other.get_resistor() {
            self.val = closest_e48((self.val.recip() + other.val.recip()).recip());
            true
        } else {
            false
        }
    }
}

impl std::convert::Into<BoxedComponent> for Resistor {
    fn into(self) -> BoxedComponent {
        Box::new(self)
    }
}




#[derive(Debug, Copy, Clone)]
pub struct Capacitor {
    pub val: f64,
}

impl std::fmt::Display for Capacitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Capacitor({})", format_si(self.val))
    }
}

impl Component for Capacitor {
    fn as_spice(&self, id: ComponentId, top: CircuitNode, bot: CircuitNode) -> String {
        format!("C_{} {} {} {}", id.id, top.id, bot.id, self.val)
    }
    fn get_random() -> Box<Self> where Self: Sized {
        Box::new(Capacitor {val: Self::random_val()})
    }
    fn random_val() -> f64 where Self: Sized {
        random_value(-9, -4)
    }
    fn randomize_val(&mut self) {
        self.val = Self::random_val();
    }
    fn get_capacitor(&self) -> Option<&Capacitor> {
        Some(self)
    }
    fn add_series(&mut self, other: &BoxedComponent) -> bool {
        if let Some(other) = other.get_capacitor() {
            self.val = closest_e48((self.val.recip() + other.val.recip()).recip());
            true
        } else {
            false
        }
    }
    fn add_parallel(&mut self, other: &BoxedComponent) -> bool {
        if let Some(other) = other.get_capacitor() {
            self.val = closest_e48(self.val + other.val);
            true
        } else {
            false
        }
    }
}

impl std::convert::Into<BoxedComponent> for Capacitor {
    fn into(self) -> BoxedComponent {
        Box::new(self)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Inductor {
    pub val: f64,
}

impl std::fmt::Display for Inductor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Inductor({})", format_si(self.val))
    }
}

impl Component for Inductor {
    fn as_spice(&self, id: ComponentId, top: CircuitNode, bot: CircuitNode) -> String {
        format!("L_{} {} {} {}", id.id, top.id, bot.id, self.val)
    }
    fn get_random() -> Box<Self> where Self: Sized {
        Box::new(Inductor{val: Self::random_val()})
    }
    fn random_val() -> f64 where Self: Sized {
        random_value(-8, -1)
    }
    fn randomize_val(&mut self) {
        self.val = Self::random_val();
    }
    fn get_inductor(&self) -> Option<&Inductor> {
        Some(self)
    }
    fn add_series(&mut self, other: &BoxedComponent) -> bool {
        if let Some(other) = other.get_inductor() {
            self.val = closest_e48(self.val + other.val);
            true
        } else {
            false
        }
    }
    fn add_parallel(&mut self, other: &BoxedComponent) -> bool {
        if let Some(other) = other.get_inductor() {
            self.val = closest_e48((self.val.recip() + other.val.recip()).recip());
            true
        } else {
            false
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Wire {
}

impl std::fmt::Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wire()")
    }
}

impl Component for Wire {
    fn as_spice(&self, id: ComponentId, top: CircuitNode, bot: CircuitNode) -> String {
        format!("R_{} {} {} 0", id.id, top.id, bot.id)
    }
    fn random_val() -> f64 where Self: Sized {
        0.0
    }
    fn get_random() -> Box<Self> where Self: Sized {
        Box::new(Wire{})
    }
    fn randomize_val(&mut self) {
    }
    fn is_wire(&self) -> bool {
        true
    }
}

impl std::convert::Into<BoxedComponent> for Inductor {
    fn into(self) -> BoxedComponent {
        Box::new(self)
    }
}



#[derive(Debug, Copy, Clone)]
pub struct FixedResistor {
    pub val: f64,
}

impl std::fmt::Display for FixedResistor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FixedResistor({})", format_si(self.val))
    }
}

impl Component for FixedResistor {
    fn as_spice(&self, id: ComponentId, top: CircuitNode, bot: CircuitNode) -> String {
        format!("R_{} {} {} {}", id.id, top.id, bot.id, self.val)
    }
    fn get_random() -> Box<Self> where Self: Sized {
        Box::new(FixedResistor{val: Self::random_val()})
    }
    fn random_val() -> f64 where Self: Sized {
        50.0
    }
    fn randomize_val(&mut self) {
        self.val = Self::random_val();
    }
    fn get_allowed_connections(&self) -> AllowedSeriesConnections {
        AllowedSeriesConnections::NONE
    }
    fn is_fixed(&self) -> bool {
        true
    }
}

impl std::convert::Into<BoxedComponent> for FixedResistor {
    fn into(self) -> BoxedComponent {
        Box::new(self)
    }
}