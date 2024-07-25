//! AC arc flash calculations according to IEEE 1584 (for 3-phase AC systems, 208 V - 15,000 V).

mod common;
mod cubicle;
mod e_afb;
pub(crate) mod equations;
mod i_arc;
mod multistep;
mod tables;

#[cfg(test)]
mod tests;

pub use common::*;
pub use cubicle::*;
pub use e_afb::*;
pub use i_arc::*;
pub use multistep::*;
