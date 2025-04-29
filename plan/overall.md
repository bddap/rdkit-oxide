# RDKit Rust Port – High-Level Plan

This document sketches out the architecture and milestones for an idiomatic
Rust rewrite of the C++ RDKit core (bindings out-of-scope).

-------------------------------------------------------------------------------

## 1. Workspace layout

`./` will be a Cargo workspace containing multiple crates, roughly
mirroring the existing C++ module boundaries while observing Rust idioms:

```
./
  Cargo.toml          # workspace definition
  crates/
    rdkit-core/       # common types, error enums, logging, utilities
    datastructs/      # bit vectors, FPB, sparse vectors
    geometry/         # Point3D, transforms, uniform grids
    numerics/         # thin wrappers around nalgebra + optimizers
    forcefield/       # generic FF engine + contrib traits
    forcefield-uff/   # UFF parameterisation
    forcefield-mmff/  # MMFF94 parameterisation
    distgeom/         # bounds matrix, triangle smoothing, embedding support
    graphmol/         # Atom, Bond, ROMol, conformers, descriptors, etc.
    simdivpickers/    # pickers and clustering utils
    catalog/          # hierarchical catalog infrastructure
    features/         # explicit/implicit chemical features
    io/               # streams, gzip support, molecule I/O later
    tests/            # cross-crate integration tests (translated from C++)
```

Crates will have explicit APIs; only `rdkit-core` will be a public dependency
for external users.  Everything else is `pub(crate)` unless needed.

In addition to the workspace‐level `tests` crate (for cross-crate integration
scenarios), *every* library crate will embed its own unit tests next to the
implementation code using the standard Rust pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_property_holds() {
        /* … */
    }
}
```

This keeps fine-grained tests close to the code they verify while allowing
larger black-box tests in the workspace’s `tests/` directory.

-------------------------------------------------------------------------------

## 2. External dependencies

* `nalgebra` (+ `nalgebra-sparse`) for vectors, matrices, transforms.
* `bitvec` for low-level bit manipulations (though custom bit-packing may still
  be required for fingerprint parity).
* `petgraph` for graph algorithms (molecule graph, catalogs).
* `rayon` for optional data-parallel algorithms (MaxMinPicker, embedding).
* `flate2`/`gzip` for compressed stream handling.
* `serde` (+ `bincode`) for fast serialisation of catalogs & parameter tables.

-------------------------------------------------------------------------------

## 3. Translation strategy

1. **Foundations first**  
   a. Implement `rdkit-core` error types (`Error`, `Result`, `Invariant` macro),
      logging facade (maps to `log` crate), small utility traits.  
   b. Port `geometry` (Point3D, Transform{2,3}D, UniformGrid3D).  
   c. Port `datastructs` – ExplicitBitVect (dense), SparseIntVect, BitOps.

2. **Numerics + Optimiser**  
   Port BFGS optimiser using nalgebra; expose generic `Minimise` trait.

3. **ForceField engine**  
   Translate generic engine (`ForceField` struct) with trait object list of
   `dyn Contrib`.  Provide UFF parameterisation first.  Use const generics to
   fix dimension (3) and allow zero-cost abstraction.

4. **DistGeom**  
   Bounds matrix, triangle smoothing, random distance mat sampling, embedding.

5. **GraphMol core**  
   Atoms / bonds / ROMol with petgraph; conformers store vectors of Point3D.  
   Descriptors & fingerprints gradually ported.

6. **Remaining subsystems**  
   SimDivPickers, Catalogs, Features, etc.  Many of these depend mostly on
   datastructs + basic geometry and can be parallelised.

-------------------------------------------------------------------------------

## 4. API design notes / idiomatic changes

* `Option`/`Result` instead of null-ptr / exceptions.  Panic only for logic
  errors (`debug_assert!`).
* Use Rust enums for atom types, bond stereochemistry, etc.  C++ integer flags
  replaced with strongly-typed bitflags (`bitflags` crate).
* Trait-based polymorphism replaces class inheritance (e.g. `Contrib`
  hierarchy, picker strategies).
* Parameter tables generated at build time (`build.rs`) into `phf` static maps
  for O(1) lookup.
* Thread safety by default (`Send`/`Sync` derived) – lock-free where possible,
  `parking_lot` mutex if needed.

-------------------------------------------------------------------------------

## 5. Testing strategy

* Create one Rust test module per original Catch2 file; use `#[test]` + quick-
  check where meaningful.
* Keep unit tests inline with the crate source (`mod tests { … }`) so they run
  with `cargo test -p <crate>`.  Reserved integration tests live under
  `rust-oxide/tests/`.
* Re-implement helper asserts like `CHECK_CLOSE` via `approx` crate.
* Translation order mirrors implementation order so the test suite remains
  green after each milestone.
* Any test not yet ported is added to `rust-oxide/shame.md` with rationale.

-------------------------------------------------------------------------------

## 6. Milestones / timeline (tentative)

1. Workspace bootstrapped; `rdkit-core`, `geometry` crate compiling (2 weeks).
2. `datastructs` port with ExplicitBitVect + unit tests (2–3 weeks).
3. `forcefield` generic engine + UFF minimal terms; energy tests passing
   (3–4 weeks).
4. DistGeom embedding (ETKDG) reproduces reference coordinates (4–6 weeks).
5. Basic `graphmol` (atoms, bonds, stereochem) + SMILES parsing (keeper for
   later) (6+ weeks).
6. Remaining subsystems & optimisation (ongoing).

Timeline is flexible; adjust upon discoveries.

-------------------------------------------------------------------------------

## 7. Open questions / research spikes

* Should we wrap nalgebra types directly or introduce lightweight newtypes to
  keep API independent from backend?  Start with newtypes for flexibility.
* Decide whether to keep per-atom `usize` index stable or use petgraph’s NodeId.
* Investigate compile-time generation of UFF/MMFF parameter tables (build
  script vs. `include_bytes!`).
* Evaluate `ndarray` vs. `nalgebra` for certain numeric needs.

-------------------------------------------------------------------------------

_Last updated: Stage 4 kick-off._
