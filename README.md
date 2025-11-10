# chemkit-oxide Workspace

Welcome to the work-in-progress **Rust rewrite of RDKit**.  The goal is to
provide a fully native, idiomatic Rust implementation of RDKit’s core
cheminformatics functionality – no C++ bindings involved.

> ⚠ **Status:** early prototyping phase (Stage 5 of the master plan).  Only a
> subset of datastructures and UFF bond/angle energy terms are implemented so
> far.

## Crate layout

```
crates/
  rdkit-core/        # common error types, invariant! macro, utilities
  datastructs/       # bit-vector containers, base64 helpers
  forcefield-uff/    # auto-generated UFF parameter table (build.rs)
  forcefield-core/   # generic force-field utilities – bond/angle energy so far
  geometry/          # (WIP) 3-D point / vector primitives
  … more crates will appear as the port grows …
```

Every crate is versioned together in this single workspace so a single
`cargo test --workspace` builds everything.

## Building & testing

```bash
# build all crates and run unit tests
cargo test --workspace

# run clippy on every target, denying warnings
cargo clippy --workspace --all-targets -- -D warnings
```

> The first build downloads the `nalgebra`, `bitvec`, etc. dependencies; subsequent
> builds will be incremental.

## Quick-start – computing a single UFF angle energy

```rust
use forcefield_core::angle_bend_energy;

fn main() {
    // Water H–O–H valence angle (~104.5°)
    let theta = 104.51_f64.to_radians();
    let e = angle_bend_energy("H_", "O_3", "H_", theta, 1.0, 1.0)
        .expect("energy");
    println!("Angle energy: {:.4} kcal/mol", e);
}
```

Compile and run with:

```bash
cargo run -q --example angle_energy
```

## Code generation

`crates/forcefield-uff/build.rs` parses `rdkit/Code/ForceField/UFF/Params.cpp`
at *build time* and emits a `phf::Map` containing one struct per atom label.
This ensures parameter lookup is `O(1)` while keeping the table source-of-truth
in sync with the original RDKit source.

## Contributing

We follow the standard Rust style (`rustfmt`, `clippy`) and require that the
workspace compiles and tests pass before every commit.

See `progress.md` for the granular task tracker and `plan/overall.md` for the
high-level architecture roadmap.

---

© The RDKit-Rust team – BSD (851) licence, identical to upstream RDKit.  This
repository also contains a vendored copy of the original RDKit C++ source for
reference; see its `license.txt`.
