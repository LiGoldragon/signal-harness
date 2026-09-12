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
//!
//! The portable rkyv frame and its three kinds come from `signal` and are
//! re-exported here, so a harness frame is the same type as every other
//! contract's frame and one generic transport carries them all.

pub mod generated;
pub use generated::signal::*;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};
