use dyn_clone::DynClone;

use super::types::*;


pub trait Component : std::fmt::Display + DynClone{
    fn get_allowed_connections(&self) -> AllowedSeriesConnections {
        AllowedSeriesConnections::BOTH
    }
    fn is_fixed(&self) -> bool {
        !matches!(self.get_allowed_connections(), AllowedSeriesConnections::BOTH)
    }
    fn is_wire(&self) -> bool {
        false
    }
    fn random_val() -> f64 where Self: Sized;
    fn get_random() -> Box<Self> where Self: Sized;
    fn randomize_val(&mut self);
    fn as_spice(&self, id: ComponentId, top: CircuitNode, bot: CircuitNode) -> String;
    fn get_inductor(&self) -> Option<&Inductor> {
        None
    }
    fn get_resistor(&self) -> Option<&Resistor> {
        None
    }
    fn get_capacitor(&self) -> Option<&Capacitor> {
        None
    }
    fn add_series(&mut self, _other: &BoxedComponent) -> bool {
        false
    }
    fn add_parallel(&mut self, _other: &BoxedComponent) -> bool {
        false
    }
}
dyn_clone::clone_trait_object!(Component);

mod passives;
mod sources;
mod util;
pub use passives::*;
pub use sources::*;
pub use util::*;