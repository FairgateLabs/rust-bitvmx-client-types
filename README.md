# rust-bitvmx-client-types

The BitVMX wire contract — `IncomingBitVMXApiMessages`, `OutgoingBitVMXApiMessages`, the union
message types, and their supporting types — published verbatim from
[`rust-bitvmx-client`](https://github.com/FairgateLabs/rust-bitvmx-client) on every release tag.

## This repo is generated

Every `.rs` file that carries a `// GENERATED FILE - DO NOT EDIT` header is copied byte-for-byte
from the client repo by `scripts/mirror.py` (in the client repo) and is overwritten on every
regenerate. **Never hand-edit a generated file** — CI in the client repo regenerates and diffs
against this repo on every release, and a hand edit here will be silently discarded and then fail
the diff check.

`mirror.toml` records the source tag and the file list the generator copied.

## Hand-written surface

Everything else in this repo is hand-written and never touched by the generator:

- `Cargo.toml` — package manifest; dependencies must track the client's manifest at the source tag.
- `README.md` — this file.
- `.gitattributes` — line-ending normalization so the regenerate-and-diff CI guard is stable across
  platforms.
- `src/lib.rs` — the module tree, a flat re-export of the contract, and re-exports of the upstream
  types the contract references but does not own.
- `src/errors.rs` — a hand-maintained twin of the client's `BitVMXError`, restricted to the
  variants the copied code actually constructs. Guarded by compilation: if a variant's name or
  payload changes upstream, the copied code fails to build here before this repo is published.
- `src/types/mod.rs`, `src/program/mod.rs`, `src/program/variables/mod.rs`,
  `src/program/protocols/mod.rs`, `src/program/protocols/union/mod.rs`,
  `src/program/protocols/union/common/mod.rs` — module declarations and re-exports only, so
  `crate::`-qualified paths inside the generated files resolve exactly as they do in the client.
