use crate::network_raw::types::*;
use super::{Component, format_si};

#[derive(Copy, Clone, Debug)]
pub struct VoltageSource {
    pub val: f64,
}
impl std::fmt::Display for VoltageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Voltage({})", format_si(self.val))
    }
}
impl Component for VoltageSource {
    fn as_spice(&self, id: ComponentId, top: CircuitNode, bot: CircuitNode) -> String {
        format!("V_{} {} {} {}", id.id, top.id, bot.id, self.val)
    }
    fn get_random() -> Box<Self> where Self: Sized {
        Box::new(VoltageSource { val: Self::random_val() })
    }
    fn random_val() -> f64 where Self: Sized {
        1.0
    }
    fn randomize_val(&mut self) {
        self.val = Self::random_val();
    }
    fn get_allowed_connections(&self) -> AllowedSeriesConnections {
        AllowedSeriesConnections::TOP
    }
}

#[derive(Copy, Clone, Debug)]
pub struct InputPort {
}
impl std::fmt::Display for InputPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input()")
    }
}
impl Component for InputPort {
    fn as_spice(&self, id: ComponentId, top: CircuitNode, bot: CircuitNode) -> String {
        format!("V_{} {} {} AC 1", id.id, top.id, bot.id)
    }
    
    fn get_random() -> Box<Self> where Self: Sized {
        Box::new(InputPort {  })
    }
    fn random_val() -> f64 where Self: Sized {
        0.0
    }
    fn get_allowed_connections(&self) -> AllowedSeriesConnections {
        AllowedSeriesConnections::TOP
    }
    fn randomize_val(&mut self) {
    }
}