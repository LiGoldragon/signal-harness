//! The subscription every Harness Nexus serves and the Flow Nexus depends on: Subscribe, answered by Subscribed with every living session, then a Changed as each event happens.

mod generated {
    pub mod signal;
}

pub use generated::signal::*;

/// The contract's authored Ethos.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
