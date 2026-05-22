//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as a `HarnessRequest`, `HarnessEvent`, or `HarnessStreamEvent`
//! and asserting the re-encoded text equals the canonical form.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_persona_harness::{
    DeliveryCancellation, DeliveryCompleted, DeliveryFailed, DeliveryFailureReason, HarnessCrashed,
    HarnessEvent, HarnessHealth, HarnessName, HarnessOperationKind, HarnessReadiness,
    HarnessRequest, HarnessRequestUnimplemented, HarnessStarted, HarnessStatus, HarnessStatusQuery,
    HarnessStopped, HarnessStreamEvent, HarnessSubscriptionRetracted, HarnessTranscriptSequence,
    HarnessTranscriptSnapshot, HarnessTranscriptToken, HarnessUnimplementedReason,
    InteractionPrompt, InteractionResolved, MessageBody, MessageDelivery, MessageSender,
    MessageSlot, SubscribeHarnessTranscript, TranscriptObservation,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn designer() -> HarnessName {
    HarnessName::new("designer")
}

fn operator() -> MessageSender {
    MessageSender::new("operator")
}

fn body() -> MessageBody {
    MessageBody::new("hello-from-operator")
}

fn token() -> HarnessTranscriptToken {
    HarnessTranscriptToken {
        harness: designer(),
    }
}

#[test]
fn canonical_request_examples_round_trip() {
    let expected: Vec<(HarnessRequest, &str)> = vec![
        (
            HarnessRequest::MessageDelivery(MessageDelivery {
                harness: designer(),
                sender: operator(),
                body: body(),
                message_slot: MessageSlot::new(1024),
            }),
            "(MessageDelivery (designer operator hello-from-operator 1024))",
        ),
        (
            HarnessRequest::InteractionPrompt(InteractionPrompt {
                harness: designer(),
                interaction_id: "interaction-7".to_string(),
                prompt: "Approve write?".to_string(),
                options: vec!["yes".to_string(), "no".to_string()],
            }),
            "(InteractionPrompt (designer interaction-7 [Approve write?] [yes no]))",
        ),
        (
            HarnessRequest::DeliveryCancellation(DeliveryCancellation {
                harness: designer(),
                message_slot: MessageSlot::new(1024),
            }),
            "(DeliveryCancellation (designer 1024))",
        ),
        (
            HarnessRequest::HarnessStatusQuery(HarnessStatusQuery {
                harness: designer(),
            }),
            "(HarnessStatusQuery (designer))",
        ),
        (
            HarnessRequest::SubscribeHarnessTranscript(SubscribeHarnessTranscript {
                harness: designer(),
            }),
            "(SubscribeHarnessTranscript (designer))",
        ),
        (
            HarnessRequest::HarnessTranscriptRetraction(token()),
            "(HarnessTranscriptRetraction (designer))",
        ),
    ];

    for (value, canonical_text) in expected {
        let mut encoder = Encoder::new();
        value.encode(&mut encoder).expect("encode");
        let text = encoder.into_string();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let mut decoder = Decoder::new(canonical_text);
        let decoded = HarnessRequest::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}

#[test]
fn canonical_reply_examples_round_trip() {
    let expected: Vec<(HarnessEvent, &str)> = vec![
        (
            HarnessEvent::DeliveryCompleted(DeliveryCompleted {
                harness: designer(),
                message_slot: MessageSlot::new(1024),
            }),
            "(DeliveryCompleted (designer 1024))",
        ),
        (
            HarnessEvent::DeliveryFailed(DeliveryFailed {
                harness: designer(),
                message_slot: MessageSlot::new(1024),
                reason: DeliveryFailureReason::HumanInputIntervened,
            }),
            "(DeliveryFailed (designer 1024 HumanInputIntervened))",
        ),
        (
            HarnessEvent::InteractionResolved(InteractionResolved {
                harness: designer(),
                interaction_id: "interaction-7".to_string(),
                chosen: "yes".to_string(),
            }),
            "(InteractionResolved (designer interaction-7 yes))",
        ),
        (
            HarnessEvent::HarnessRequestUnimplemented(HarnessRequestUnimplemented {
                harness: designer(),
                operation: HarnessOperationKind::SubscribeHarnessTranscript,
                reason: HarnessUnimplementedReason::NotBuiltYet,
            }),
            "(HarnessRequestUnimplemented (designer SubscribeHarnessTranscript NotBuiltYet))",
        ),
        (
            HarnessEvent::HarnessStatus(HarnessStatus {
                harness: designer(),
                health: HarnessHealth::Running,
                readiness: HarnessReadiness::Ready,
            }),
            "(HarnessStatus (designer Running Ready))",
        ),
        (
            HarnessEvent::HarnessStarted(HarnessStarted {
                harness: designer(),
            }),
            "(HarnessStarted (designer))",
        ),
        (
            HarnessEvent::HarnessStopped(HarnessStopped {
                harness: designer(),
            }),
            "(HarnessStopped (designer))",
        ),
        (
            HarnessEvent::HarnessCrashed(HarnessCrashed {
                harness: designer(),
                detail: "out of memory".to_string(),
            }),
            "(HarnessCrashed (designer [out of memory]))",
        ),
        (
            HarnessEvent::HarnessTranscriptSnapshot(HarnessTranscriptSnapshot {
                harness: designer(),
                current_sequence: HarnessTranscriptSequence::new(0),
            }),
            "(HarnessTranscriptSnapshot (designer 0))",
        ),
        (
            HarnessEvent::HarnessSubscriptionRetracted(HarnessSubscriptionRetracted {
                token: token(),
            }),
            "(HarnessSubscriptionRetracted ((designer)))",
        ),
    ];

    for (value, canonical_text) in expected {
        let mut encoder = Encoder::new();
        value.encode(&mut encoder).expect("encode");
        let text = encoder.into_string();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let mut decoder = Decoder::new(canonical_text);
        let decoded = HarnessEvent::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}

#[test]
fn canonical_stream_event_examples_round_trip() {
    let expected: Vec<(HarnessStreamEvent, &str)> = vec![(
        HarnessStreamEvent::TranscriptObservation(TranscriptObservation {
            harness: designer(),
            sequence: HarnessTranscriptSequence::new(1),
            line: "hello".to_string(),
        }),
        "(TranscriptObservation (designer 1 hello))",
    )];

    for (value, canonical_text) in expected {
        let mut encoder = Encoder::new();
        value.encode(&mut encoder).expect("encode");
        let text = encoder.into_string();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let mut decoder = Decoder::new(canonical_text);
        let decoded = HarnessStreamEvent::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}
