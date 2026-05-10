//! Architectural-truth round-trip tests for the
//! `signal-persona-harness` channel.

use signal_core::{FrameBody, Reply, Request, SemaVerb};
use signal_persona_harness::{
    DeliveryCancellation, DeliveryCompleted, DeliveryFailed, DeliveryFailureReason, Frame,
    HarnessCrashed, HarnessEvent, HarnessName, HarnessRequest, HarnessStarted, HarnessStopped,
    InteractionPrompt, InteractionResolved, MessageBody, MessageDelivery, MessageSender,
    MessageSlot,
};

fn harness() -> HarnessName {
    HarnessName::new("designer")
}

fn round_trip_request(request: HarnessRequest) -> HarnessRequest {
    let frame = Frame::new(FrameBody::Request(Request::assert(request)));
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, SemaVerb::Assert);
            payload
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_event(event: HarnessEvent) -> HarnessEvent {
    let frame = Frame::new(FrameBody::Reply(Reply::operation(event)));
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply(Reply::Operation(event)) => event,
        other => panic!("expected reply operation, got {other:?}"),
    }
}

#[test]
fn message_delivery_round_trips() {
    let request = HarnessRequest::MessageDelivery(MessageDelivery {
        harness: harness(),
        sender: MessageSender::new("operator"),
        body: MessageBody::new("harness delivery test"),
        message_slot: MessageSlot::new(1024),
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn interaction_prompt_round_trips() {
    let request = HarnessRequest::InteractionPrompt(InteractionPrompt {
        harness: harness(),
        interaction_id: "i-abc".into(),
        prompt: "Approve commit?".into(),
        options: vec!["yes".into(), "no".into()],
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn delivery_cancellation_round_trips() {
    let request = HarnessRequest::DeliveryCancellation(DeliveryCancellation {
        harness: harness(),
        message_slot: MessageSlot::new(7),
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn delivery_completed_round_trips() {
    let event = HarnessEvent::DeliveryCompleted(DeliveryCompleted {
        harness: harness(),
        message_slot: MessageSlot::new(1024),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn delivery_failed_round_trips_for_each_reason() {
    for reason in [
        DeliveryFailureReason::TransportRejected,
        DeliveryFailureReason::HumanInputIntervened,
        DeliveryFailureReason::HarnessStoppedBeforeDelivery,
    ] {
        let event = HarnessEvent::DeliveryFailed(DeliveryFailed {
            harness: harness(),
            message_slot: MessageSlot::new(1024),
            reason: reason.clone(),
        });
        assert_eq!(round_trip_event(event.clone()), event);
    }
}

#[test]
fn interaction_resolved_round_trips() {
    let event = HarnessEvent::InteractionResolved(InteractionResolved {
        harness: harness(),
        interaction_id: "i-abc".into(),
        chosen: "yes".into(),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_started_round_trips() {
    let event = HarnessEvent::HarnessStarted(HarnessStarted { harness: harness() });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_stopped_round_trips() {
    let event = HarnessEvent::HarnessStopped(HarnessStopped { harness: harness() });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_crashed_carries_typed_detail() {
    let event = HarnessEvent::HarnessCrashed(HarnessCrashed {
        harness: harness(),
        detail: "PTY fd was closed".into(),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn from_impl_lifts_message_delivery_into_request() {
    let payload = MessageDelivery {
        harness: harness(),
        sender: MessageSender::new("operator"),
        body: MessageBody::new("via from"),
        message_slot: MessageSlot::new(42),
    };
    let request: HarnessRequest = payload.clone().into();
    assert_eq!(request, HarnessRequest::MessageDelivery(payload));
}

#[test]
fn from_impl_lifts_delivery_completed_into_event() {
    let payload = DeliveryCompleted {
        harness: harness(),
        message_slot: MessageSlot::new(42),
    };
    let event: HarnessEvent = payload.clone().into();
    assert_eq!(event, HarnessEvent::DeliveryCompleted(payload));
}
