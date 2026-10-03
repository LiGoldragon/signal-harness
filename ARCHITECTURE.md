# signal-harness — architecture

The ordinary Signal contract for the `router` ↔ `harness` channel.

## Direction

The channel is bidirectional and push-shaped. The router initiates delivery,
prompting, cancellation, status enquiry and transcript subscription. The
harness pushes lifecycle, adapter and resolution observations back without
being polled, and pushes transcript observations for as long as a subscription
is open.

## Surface

`ethos/signal.ethos` is the single source of the contract. Its request root
generates `Query`, its reply root generates `Response`, and the declared
`HarnessStreamEvent` enum carries the per-subscription stream.

| Request | Meaning |
|---|---|
| `MessageDelivery` | Deliver a typed message body from a sender into a harness's delivery path, at a message slot. |
| `InteractionPrompt` | Put a typed question with a fixed option set to the harness. |
| `DeliveryCancellation` | Withdraw a delivery still outstanding at a message slot. |
| `HarnessStatusQuery` | Ask one harness for its health and readiness. |
| `WatchHarnessTranscript` | Open a transcript subscription; answered with `HarnessTranscriptSnapshot`. |
| `UnwatchHarnessTranscript(HarnessTranscriptToken)` | Close the subscription the token names. |
| `UsageSnapshotQuery` | Read one fresh snapshot of every known Claude and Codex subscription's quota windows and every live session's context; answered with `UsageSnapshot`. Daemon-level: it names no harness instance. |

| Reply | Meaning |
|---|---|
| `DeliveryCompleted` / `DeliveryFailed` | The slot's delivery resolved; failure carries a typed `DeliveryFailureReason`. |
| `InteractionResolved` | The prompt was answered; carries the chosen option. |
| `HarnessRequestUnimplemented` | The request reached the surface but the runtime path is not built; carries `HarnessOperationKind` and `HarnessUnimplementedReason`. |
| `HarnessStatus` | The harness's `HarnessHealth` and `HarnessReadiness`. |
| `HarnessStarted` / `HarnessStopped` / `HarnessCrashed` | Harness lifecycle; the crash carries a detail string. |
| `AdapterReady` / `AdapterInputAccepted` / `AdapterOutput` / `AdapterProgress` / `AdapterCompletion` / `AdapterConfirmationNeeded` / `AdapterStalled` / `AdapterExited` | The adapter's own sequenced observations, each carrying an `AdapterEventSequence`. |
| `HarnessTranscriptSnapshot` | A subscription opened; carries its token and the current sequence. |
| `HarnessSubscriptionRetracted` | The subscription the token names is closed. |
| `UsageSnapshot` | One read-only usage snapshot: per-provider `SubscriptionObservation`s and per-session `SessionContextObservation`s. |

| Stream event | Meaning |
|---|---|
| `TranscriptObservation` | One transcript line at a transcript sequence. |
| `ClaudeSessionObservation` | One Claude session state observation: identity, model, launch shape, counts, transcript path, response, accumulated context, activity timestamp, lifecycle. |

Types that are declared but carried by no root — `ModelRequest`,
`ModelResolutionRequest`, `ModelResolved`, `ModelUnavailable`,
`SessionLaunchRequest`, `SessionLaunched`, `SessionLaunchRefused` — are the
model-resolution and session-launch vocabulary the daemon and its callers
share ahead of the operations that will carry them.

`HarnessDaemonConfiguration` is the harness daemon's typed startup record. It
imports its socket paths, socket modes and owner identity from
`signal-persona` and carries the per-instance
`HarnessInstanceConfiguration` set, including the Pi RPC JSONL adapter
configuration.

## Usage snapshot

`UsageSnapshot` is paced by invocation: one `UsageSnapshotQuery` yields one
snapshot. It carries no subscription, watch or history.

- Every provider and session result carries its own `ObservationTime`
  (epoch nanoseconds); the snapshot carries `SnapshotTime`.
- A provider result is `Observed(SubscriptionUsage)` or
  `Unavailable(UsageUnavailable)` with a stable `UsageUnavailableReason`; one
  provider's failure never removes another's result. A Codex result lists the
  `AccountHomes` that answered for the same account, so same-account homes are
  one subscription.
- `QuotaLimits` lists every provider-reported limit and each of its windows.
  Percentages are `UsedBasisPoints` / `RemainingBasisPoints` (hundredths of a
  percent). `ResetBasis` and `WindowDurationBasis` say where the reset and
  window length came from: declared by the provider, implied by the
  provider's window name, or `Unknown`. `AbsoluteLimit` is
  `NotExposedByProvider`: neither provider reports tokens or money per window.
- `PaceDerivation` is `Derived(QuotaPace)` only with all its operands in the
  reply (remaining, seconds until the named reset, window duration) and is
  `Unknown(PaceUnknownReason)` when an operand is unknown or the reset has
  passed. `PaceVarianceBasisPoints` is used minus even-pace used: positive
  means ahead of an even burn.
- `UnrecognizedWindowNames` retains provider windows that are present but not
  understood, rather than dropping them.
- `SessionContext` carries `ContextBasis` and `ContextFreshness`
  (`Exact`, `Proxy`, `Superseded`, `Unknown`); a value absent from its source
  is `None`, never zero.
- No field carries credentials, credential paths, raw provider bodies,
  process arguments or environment contents.

## Generation

```sh
ethos-zero 'Generate.{ <repo>/ethos/signal.ethos <repo>/src/generated }'
```

`src/generated/signal.rs` is committed. `build.rs` regenerates from the ethos
source at build time and asserts equality with the committed file, so a drifted
generation fails the build rather than the review. No `Datomic` implementation
is hand-written for a declared type.

A bare ethos enum variant whose name matches a declared type generates a
payload-carrying variant. `HarnessOperationKind` is a pure tag, so its variants
are named `DeliverMessage`, `PromptInteraction`, `CancelDelivery`,
`QueryHarnessStatus`, `WatchTranscript`, `UnwatchTranscript` — names that are
not themselves declared types.

## Frames

`src/lib.rs` carries the portable frame surface only: `Signal<T>`,
`Signalizable`, `ByteViewable`, and `Restorable<T>`. Archiving is rkyv;
`Signal<T>` carries its target contract in its type so received bytes restore
into the contract they were framed from. `Query`, `Response`,
`HarnessStreamEvent` and `HarnessDaemonConfiguration` each bear the frame
surface.

## Boundaries

This crate carries only wire vocabulary and codecs. It does not own:

- the `harness` daemon runtime or its adapters;
- socket binding or transport framing policy;
- subscription bookkeeping;
- transcript storage;
- model resolution or session launching.

Terminal control traffic lives in `signal-terminal`; upstream message traffic
in `signal-message`; component lifecycle nouns in `signal-persona`.

## Constraints

| Constraint | Witness |
|---|---|
| The contract shape is declared, never hand-written. | `build.rs` asserts `src/generated/signal.rs` equals a fresh generation from `ethos/signal.ethos`. |
| Shared Persona nouns are imported, not copied. | `HarnessDaemonConfiguration` and `ClaudeSessionObservation` carry `signal_persona::` types. |
| Every root round-trips over the real wire. | `tests/contract.rs` archives and restores `Query`, `Response`, `HarnessStreamEvent` and `HarnessDaemonConfiguration` through received bytes. |
| Malformed bytes are rejected rather than misread. | `malformed_archive_is_rejected`. |
| Canonical Datom text stays true to the contract. | `every_canonical_datom_line_actualizes_into_a_contract_head` actualizes every line of `examples/canonical.datom`. |
| Contract crate dependencies name one exact published revision. | Every `LiGoldragon` dependency in `Cargo.toml` is pinned by `rev`. |
| Contract code contains no runtime. | Source contains no actors, tokio, storage, or socket implementation. |
| Behavior is homed in traits. | The `no-free-functions` and `no-inherent-methods` Nix checks. |

## Code Map

```text
ethos/signal.ethos       the contract source
build.rs                 freshness assertion over the committed generation
src/generated/signal.rs  generated contract types
src/lib.rs               portable rkyv Signal frame surface
examples/canonical.datom canonical Datom projection of every contract head
tests/contract.rs        rkyv frame and Datom witnesses
```
