# skills — signal-harness

Before editing this repo, read:

- the `ethos` skill — the contract is an ethos file, and the Rust is generated
- the `datom` skill — the text dialect the `datom` feature projects into
- `ARCHITECTURE.md`
- `signal-persona`'s `ethos/signal.ethos`, which owns the imported nouns

This crate owns only the ordinary Signal contract for router ↔ harness
traffic. Do not add runtime, storage, or CLI behavior here.

## Invariants

- The contract is changed by editing `ethos/signal.ethos` and regenerating with
  `ethos-zero`; never by editing `src/generated/signal.rs`.
- Regenerated output is committed in the same change; `build.rs` is the gate.
- Socket paths, socket modes, owner identity and timestamps are imported from
  `signal-persona`; do not duplicate them.
- Every `Query`, `Response` and `HarnessStreamEvent` variant needs a frame
  round-trip witness in `tests/contract.rs`.
- Every canonical line in `examples/canonical.datom` must actualize into a
  contract head; `every_canonical_datom_line_actualizes_into_a_contract_head`
  is the gate on that file. An ungated canonical file drifts silently.
- A bare ethos enum variant whose name matches a declared type generates a
  payload-carrying variant. Tag enums such as `HarnessOperationKind` therefore
  use names that are not themselves declared types.
