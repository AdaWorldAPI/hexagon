# CLAUDE.md — Staunen

> **Updated**: 2026-03-12
> **Status**: Skeleton created, stubs only

---

## What This Is

The cognitive RISC kernel. 6 bitwise instructions. No GPU. No floats in the hot path.
8 crates in a workspace. See `ARCHITECTURE.md` for the full four-repo vision.

## The Invariant

If an operation requires float32 matrix multiplication, it doesn't belong here.
If it can be expressed as XOR/POPCOUNT/MAJORITY/AND-NOT/BLAKE3/THRESHOLD
on bitpacked integers that fit in L1 cache, it belongs here.

## Current State: STUBS ONLY

All 6 instruction modules are empty stubs. Implementation comes from:
- `ladybug-rs/src/spo/spo.rs` — the private reference implementation (READ FIRST)
- `rustynum/rustynum-core/` — AVX-512 SIMD primitives
- `holograph/src/bitpack.rs` — BitpackedVector operations

## Build

```bash
cargo check  # workspace, all crates
```

## Crate Dependency Order

```
staunen-core      (0 deps except std + blake3)
staunen-nsm       (depends: core)
staunen-cam       (depends: core)
staunen-bnn       (depends: core)
staunen-nars      (depends: core)
staunen-epiphany  (depends: core, nars)
staunen-amx       (depends: core)
staunen-bench     (depends: all)
```

## Role in Four-Repo Architecture

```
rustynum     = The Muscle    (SIMD substrate)
ladybug-rs   = The Brain     (BindSpace, server)
staunen      = The Bet       ← THIS REPO (6 instructions, no GPU)
lance-graph  = The Face      (query surface)
```

## What NOT To Do

```
× Don't add GPU dependencies
× Don't add float types to core instruction set
× Don't duplicate rustynum SIMD — import it
× Don't create a server binary (that's ladybug-rs/lance-graph's job)
× Don't add networking, HTTP, or database code (this is a LIBRARY)
```
