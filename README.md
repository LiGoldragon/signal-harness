# signal-harness

The Signal contract between **`router`** and **`harness`** — bidirectional.
The router sends delivery requests; the harness pushes lifecycle, adapter and
transcript observations back.

The contract is declared in `ethos/signal.ethos`; `build.rs` asserts the
committed `src/generated/signal.rs` matches a fresh `ethos-zero` generation.
The wire is binary rkyv. The optional `datom` feature adds the Datom text
projection of every contract type.

Three roots carry the traffic:

- `Query` — the seven router- and CLI-initiated operations.
- `Response` — the nineteen harness-initiated replies and observations.
- `HarnessStreamEvent` — the per-subscription transcript observations.

`HarnessDaemonConfiguration` is the harness daemon's typed startup record —
the binary configuration message the Persona manager encodes, never flags.

## See also

- `ARCHITECTURE.md` — channel role + boundaries
- `signal-persona` — owns the socket-path, socket-mode, owner-identity and
  timestamp nouns this contract imports
- `signal-message` — upstream channel that drives these deliveries
- `signal-terminal` — terminal control channel carrying prompt patterns,
  input gates, and write-injection acknowledgements
