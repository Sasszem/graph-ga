// pub mod component;
// pub mod load;
// pub mod kirchoff;
// pub use load::load_network;
pub mod circuit;
pub use circuit::Circuit;
pub mod components;
pub mod types;
use types::*;
pub mod ga {
    use super::*;
    pub mod mutation;
    pub mod crossover;
    pub use mutation::*;
    pub use crossover::*;
}