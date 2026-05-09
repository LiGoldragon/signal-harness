//! Architectural-truth round-trip tests for the
//! `signal-persona-harness` channel.

use signal_core::{FrameBody, Reply, Request, SemaVerb};
use signal_persona_harness::{
    CancelDelivery, DeliverMessage, DeliveryCompleted, DeliveryFailed, DeliveryFailureReason,
    Frame, HarnessCrashed, HarnessEvent, HarnessName, HarnessRequest, HarnessStarted,
    HarnessStopped, InteractionResolved, SurfaceInteraction,
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
fn deliver_message_round_trips() {
    let request = HarnessRequest::DeliverMessage(DeliverMessage {
        harness: harness(),
        sender: "operator".into(),
        body: "harness delivery test".into(),
        message_slot: 1024,
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn surface_interaction_round_trips() {
    let request = HarnessRequest::SurfaceInteraction(SurfaceInteraction {
        harness: harness(),
        interaction_id: "i-abc".into(),
        prompt: "Approve commit?".into(),
        options: vec!["yes".into(), "no".into()],
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn cancel_delivery_round_trips() {
    let request = HarnessRequest::CancelDelivery(CancelDelivery {
        harness: harness(),
        message_slot: 7,
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn delivery_completed_round_trips() {
    let event = HarnessEvent::DeliveryCompleted(DeliveryCompleted {
        harness: harness(),
        message_slot: 1024,
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn delivery_failed_round_trips_for_each_reason() {
    for reason in [
        DeliveryFailureReason::TransportRejected,
        DeliveryFailureReason::HumanRaceLost,
        DeliveryFailureReason::HarnessTeardown,
    ] {
        let event = HarnessEvent::DeliveryFailed(DeliveryFailed {
            harness: harness(),
            message_slot: 1024,
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
fn from_impl_lifts_deliver_message_into_request() {
    let payload = DeliverMessage {
        harness: harness(),
        sender: "operator".into(),
        body: "via from".into(),
        message_slot: 42,
    };
    let request: HarnessRequest = payload.clone().into();
    assert_eq!(request, HarnessRequest::DeliverMessage(payload));
}

#[test]
fn from_impl_lifts_delivery_completed_into_event() {
    let payload = DeliveryCompleted {
        harness: harness(),
        message_slot: 42,
    };
    let event: HarnessEvent = payload.clone().into();
    assert_eq!(event, HarnessEvent::DeliveryCompleted(payload));
}
