//! Signal contract — `router` ↔ `harness`.
//!
//! Read this file as the public interface of the delivery channel between the
//! routing actor and the harness actors. The channel carries:
//!
//! - **Delivery requests** from the router to a harness: "deliver this typed
//!   payload (a message, a system notification, a prompt) through this
//!   harness's terminal delivery path." These are the [`Query`] heads.
//! - **Harness observations** from the harness back to the router: lifecycle
//!   events (started / stopped / crashed), input acknowledgements, interaction
//!   resolutions. These are the [`Response`] heads.
//!
//! The channel is **bidirectional**: both sides initiate. The router sends
//! `MessageDelivery` / `InteractionPrompt` / `DeliveryCancellation`; the
//! harness pushes lifecycle and resolution heads independent of any request.
//!
//! Transcript watching opens a per-subscription flow: `WatchHarnessTranscript`
//! is answered with a `HarnessTranscriptSnapshot` carrying the
//! `HarnessTranscriptToken`, the token's observations arrive as
//! [`HarnessStreamEvent`] frames, and `UnwatchHarnessTranscript` closes the
//! subscription.
//!
//! `HarnessDaemonConfiguration` is the harness daemon's typed startup record —
//! the binary configuration message the Persona manager encodes, never flags.
//!
//! See `ARCHITECTURE.md` for the channel's role and boundaries.

pub mod generated;
pub use generated::signal::*;

use std::marker::PhantomData;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

/// A portable rkyv Signal frame whose target contract is carried in its type.
pub struct Signal<T> {
    bytes: Vec<u8>,
    target: PhantomData<fn() -> T>,
}

/// Data that can form a portable Signal frame.
pub trait Signalizable: Sized {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error>;
}

/// A frame exposes its peer-wire bytes for transport framing.
pub trait ByteViewable {
    fn bytes(&self) -> &[u8];
}

/// A typed portable Signal can restore the contract value it carries.
pub trait Restorable<T> {
    fn restore(&self) -> Result<T, rkyv::rancor::Error>;
}

impl Signalizable for Query {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}

impl Signalizable for Response {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}

impl Signalizable for HarnessStreamEvent {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}

impl Signalizable for HarnessDaemonConfiguration {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}

impl<T> From<Vec<u8>> for Signal<T> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            target: PhantomData,
        }
    }
}

impl<T> ByteViewable for Signal<T> {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Restorable<Query> for Signal<Query> {
    fn restore(&self) -> Result<Query, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}

impl Restorable<Response> for Signal<Response> {
    fn restore(&self) -> Result<Response, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}

impl Restorable<HarnessStreamEvent> for Signal<HarnessStreamEvent> {
    fn restore(&self) -> Result<HarnessStreamEvent, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}

impl Restorable<HarnessDaemonConfiguration> for Signal<HarnessDaemonConfiguration> {
    fn restore(&self) -> Result<HarnessDaemonConfiguration, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}
