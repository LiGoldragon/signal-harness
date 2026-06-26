//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as a `HarnessRequest`, `HarnessEvent`, or `HarnessStreamEvent`
//! and asserting the re-encoded text equals the canonical form.

#![cfg(feature = "nota-text")]

use nota::{NotaEncode, NotaSource};
use signal_harness::{
    AdapterCompletion, AdapterConfirmationNeeded, AdapterEventSequence, AdapterExitStatus,
    AdapterExited, AdapterInputAccepted, AdapterOutput, AdapterProgress, AdapterReady,
    AdapterStallReason, AdapterStalled, DeliveryCancellation, DeliveryCompleted, DeliveryFailed,
    DeliveryFailureReason, HarnessCrashed, HarnessEvent, HarnessHealth, HarnessName,
    HarnessOperationKind, HarnessReadiness, HarnessRequest, HarnessRequestUnimplemented,
    HarnessStarted, HarnessStatus, HarnessStatusQuery, HarnessStopped, HarnessStreamEvent,
    HarnessSubscriptionRetracted, HarnessTranscriptSequence, HarnessTranscriptSnapshot,
    HarnessTranscriptToken, HarnessUnimplementedReason, InteractionPrompt, InteractionResolved,
    MessageBody, MessageDelivery, MessageSender, MessageSlot, TranscriptObservation,
    WatchHarnessTranscript,
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
            HarnessRequest::WatchHarnessTranscript(WatchHarnessTranscript {
                harness: designer(),
            }),
            "(WatchHarnessTranscript (designer))",
        ),
        (
            HarnessRequest::UnwatchHarnessTranscript(token()),
            "(UnwatchHarnessTranscript (designer))",
        ),
    ];

    for (value, canonical_text) in expected {
        let text = value.to_nota();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let decoded = NotaSource::new(canonical_text)
            .parse::<HarnessRequest>()
            .expect("decode");
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
                operation: HarnessOperationKind::WatchHarnessTranscript,
                reason: HarnessUnimplementedReason::NotBuiltYet,
            }),
            "(HarnessRequestUnimplemented (designer WatchHarnessTranscript NotBuiltYet))",
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
            HarnessEvent::AdapterReady(AdapterReady {
                harness: designer(),
                sequence: AdapterEventSequence::new(1),
            }),
            "(AdapterReady (designer 1))",
        ),
        (
            HarnessEvent::AdapterInputAccepted(AdapterInputAccepted {
                harness: designer(),
                sequence: AdapterEventSequence::new(2),
                message_slot: MessageSlot::new(1024),
            }),
            "(AdapterInputAccepted (designer 2 1024))",
        ),
        (
            HarnessEvent::AdapterOutput(AdapterOutput {
                harness: designer(),
                sequence: AdapterEventSequence::new(3),
                text: "provider output".to_string(),
            }),
            "(AdapterOutput (designer 3 [provider output]))",
        ),
        (
            HarnessEvent::AdapterProgress(AdapterProgress {
                harness: designer(),
                sequence: AdapterEventSequence::new(4),
                status: "working".to_string(),
            }),
            "(AdapterProgress (designer 4 working))",
        ),
        (
            HarnessEvent::AdapterCompletion(AdapterCompletion {
                harness: designer(),
                sequence: AdapterEventSequence::new(5),
                message_slot: MessageSlot::new(1024),
            }),
            "(AdapterCompletion (designer 5 1024))",
        ),
        (
            HarnessEvent::AdapterConfirmationNeeded(AdapterConfirmationNeeded {
                harness: designer(),
                sequence: AdapterEventSequence::new(6),
                interaction_id: "confirm-1".to_string(),
                prompt: "Proceed?".to_string(),
                options: vec!["approve".to_string(), "decline".to_string()],
            }),
            "(AdapterConfirmationNeeded (designer 6 confirm-1 Proceed? [approve decline]))",
        ),
        (
            HarnessEvent::AdapterStalled(AdapterStalled {
                harness: designer(),
                sequence: AdapterEventSequence::new(7),
                reason: AdapterStallReason::CompletionTimeout,
            }),
            "(AdapterStalled (designer 7 CompletionTimeout))",
        ),
        (
            HarnessEvent::AdapterExited(AdapterExited {
                harness: designer(),
                sequence: AdapterEventSequence::new(8),
                status: AdapterExitStatus::Failure,
            }),
            "(AdapterExited (designer 8 Failure))",
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
        let text = value.to_nota();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let decoded = NotaSource::new(canonical_text)
            .parse::<HarnessEvent>()
            .expect("decode");
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
        let text = value.to_nota();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let decoded = NotaSource::new(canonical_text)
            .parse::<HarnessStreamEvent>()
            .expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}
