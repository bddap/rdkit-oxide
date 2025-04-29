# Progress Tracker

This file is updated incrementally to survive context compression.  Each top-level TODO corresponds to one deliverable in `job.md`.  Nested lists can be arbitrarily deep.

## Stage 1 – Root summary (complete ✅)

- [x] Create `legacy-rdkit-map/overall.md` containing a high-level map of RDKit’s root modules.
  - Path: `legacy-rdkit-map/overall.md`

## Stage 2 – In-depth maps (complete ✅)

- [x] Produce detailed symbol maps for every first-level module in `rdkit/Code/`:
  - [x] Catalogs (legacy-rdkit-map/Catalogs.md)
  - [x] ChemicalFeatures (legacy-rdkit-map/ChemicalFeatures.md)
  - [x] DataManip (legacy-rdkit-map/DataManip.md)
  - [x] DataStructs (legacy-rdkit-map/DataStructs.md)
  - [x] DistGeom (legacy-rdkit-map/DistGeom.md)
  - [x] Features (legacy-rdkit-map/Features.md)
  - [x] ForceField (legacy-rdkit-map/ForceField.md)
  - [x] Geometry (legacy-rdkit-map/Geometry.md)
  - [x] GraphMol (legacy-rdkit-map/GraphMol.md)
    - [x] Core Atom/Bond/ROMol (legacy-rdkit-map/GraphMol_Core.md)
    - [x] FileParsers (legacy-rdkit-map/GraphMol_FileParsers.md)
    - [x] Fingerprints (legacy-rdkit-map/GraphMol_Fingerprints.md)
    - [x] Descriptors (legacy-rdkit-map/GraphMol_Descriptors.md)
    - [x] Substructure search (legacy-rdkit-map/GraphMol_Substructure.md)
    - [x] Reaction chemistry (legacy-rdkit-map/GraphMol_Reaction.md)
    - [x] Depictor (legacy-rdkit-map/GraphMol_Depictor.md)
    - [x] Stereochemistry & CIPLabeler (legacy-rdkit-map/GraphMol_Stereo.md)
    - [x] FilterCatalog (legacy-rdkit-map/GraphMol_FilterCatalog.md)
    - [x] FMCS (legacy-rdkit-map/GraphMol_FMCS.md)
    - [x] DistGeomHelpers (legacy-rdkit-map/GraphMol_DistGeomHelpers.md)
    - [x] ForceFieldHelpers (legacy-rdkit-map/GraphMol_ForceFieldHelpers.md)
  - [-] JavaWrappers (skip – no Rust port)
  - [-] ML (skip – utility machine-learning helpers, not needed for Rust port)
  - [-] MinimalLib (skip – wasm viewer only)
  - [x] Numerics (legacy-rdkit-map/Numerics.md)
  - [-] PgSQL (skip – no Rust port)
  - [x] Query (legacy-rdkit-map/Query.md)
  - [-] RDBoost (skip – no Rust port)
  - [x] RDGeneral (legacy-rdkit-map/RDGeneral.md)
  - [x] RDStreams (legacy-rdkit-map/RDStreams.md)
  - [x] SimDivPickers (legacy-rdkit-map/SimDivPickers.md)

  - [ ] Recursively expand the checklist for each sub-module once its parent summary is done. (ongoing as we dive deeper)

## Stage 2.5 – Unit-test catalogue (complete ✅)

- [x] Enumerate every unit test, with location and explanation, into `legacy-rdkit-map/tests.md`.
  - [x] RDGeneral tests (legacy-rdkit-map/tests.md)
  - [x] DataStructs tests (legacy-rdkit-map/tests.md)
  - [x] Geometry & DistGeom tests (legacy-rdkit-map/tests.md)
  - [x] ForceField tests (legacy-rdkit-map/tests.md)
  - [x] GraphMol core + submodules tests
    - [x] Core basics (tests.md)
    - [x] Chirality & Stereo (tests.md)
    - [x] FileParsers (tests.md)
    - [x] Fingerprints (tests.md)
    - [x] Descriptors (tests.md)
    - [x] Reactions (tests.md)
    - [x] FMCS (tests.md)
    - [x] DistGeomHelpers (tests.md)
    - [x] ForceFieldHelpers (tests.md)
    - [x] RGroupDecomposition (tests.md)
    - [x] SimDivPickers tests (tests.md)
    - [x] Numerics tests (tests.md)
    - [x] Misc/other modules tests (tests.md)

## Stage 3 – Relationship graph (complete ✅)

- [x] Create `legacy-rdkit-map/map.md` with text graph of symbol relationships.
  - [x] Skeleton graph with root module examples (commit b02e15a^..)
  - [x] Expand DataStructs relationships (map.md)
    - [x] Expand GraphMol core relationships (map.md)
    - [x] Expand Fingerprints & Descriptors relationships (map.md)
  - [x] Expand ForceField relationships (map.md)
  - [x] Expand DistGeomHelpers & ForceFieldHelpers relationships (map.md)
  - [x] Expand SimDivPickers & Numerics relationships (map.md)
  - [x] Expand Catalogs relationships (map.md)
  - [x] Expand ChemicalFeatures relationships (map.md)
  - [x] Expand DataManip relationships (map.md)
  - [x] Expand Features relationships (map.md)
  - [x] Expand Geometry & DistGeom core relationships (map.md)
  - [x] Expand RDGeneral, RDStreams relationships (map.md)

## Stage 4 – Rust rewrite plan (in progress 🛠️)

 - [x] Draft high-level module plan in `plan/overall.md`.

## Stage 5 – Implementation (in progress 🛠️)

 - [x] Bootstrap Cargo workspace `./`
   - [x] root `Cargo.toml` with workspace members (commit <pending>)
   - [x] crates/rdkit-core (error types, invariant macro, basic logging shim)
   - [x] crates/datastructs (BitOps skeleton, ExplicitBitVect stub)
 - [ ] Configure continuous testing (`cargo test --workspace`) in CI script (future)
   - [ ] Add GitHub Actions workflow (linux + macOS + windows) running `cargo test` and `cargo clippy -- -D warnings`.
   - [ ] Cache cargo registry & build artefacts for speed.
   - [ ] Upload test coverage (grcov) (optional).

 - [x] Port BitOps functions (+ unit tests) – count, common bits, Tanimoto, bitwise ops, fold
 - [x] Port ExplicitBitVect core API (+ unit tests)
 - [x] Translate first Catch2 datastructs tests to Rust (SparseBitVect limit case, base64 round-trip)
 - [x] Resolve clippy nits in datastructs crate (base64 deprecation, needless lifetimes, len/is_empty, tests bool assert)
 - [x] Decide on parameter‐table generation strategy (build.rs prototype in forcefield-uff)
   - [x] Implement build-time extraction of UFF parameters into new crate `forcefield-uff` (commit <pending>)
   - [x] Auto-generate const parameter map via phf; clippy clean
   - [x] Add simple unit-test (H_ sanity check)
 - [x] Expose API for bond-stretch & angle-bend energy calculations
   - [x] Implement core formulas in new crate `forcefield-core` (+ unit tests) – `bond_rest_length`, `bond_stretch_energy`, `angle_force_constant`, `angle_bend_energy` (commit <pending>)
 - [x] Document workspace `README.md`
   - [ ] Explain crate layout & build instructions.
   - [ ] Describe code-generation (UFF parameters via build.rs).
   - [ ] Provide quickstart code snippet (compute energy of water).
   - [x] Implemented initial README with layout, build instructions, quickstart (commit <pending>).
 - [x] Scaffold geometry crate with Point3D
   - [x] geometry crate created with Point3D, vector ops, unit tests (commit <pending>).

- [ ] Datastructs crate – feature parity with C++
  - [x] Complete SparseBitVect API and translate associated tests (commit <pending>).
  - [x] Port RealValueVect and DiscreteValueVect, incl. serialisation (commit <pending>).
  - [x] Implement FPB fingerprint binary reader (streaming, memory-mapped).
    - Path: `crates/datastructs/src/fpb.rs`, tests in same module.

- [x] Geometry crate (created)
  - [x] Point3D and Vector3D types + basic linear algebra traits (crates/geometry).
  - [x] Transformation matrices / quaternion helpers (crates/geometry).

- [ ] forcefield-core expansion
  - [x] Torsion (dihedral) term energy + numerical gradient (crates/forcefield-core).
  - [x] Analytic per-atom gradient for torsion terms (crates/forcefield-core/field.rs).
  - [x] Inversion term energy + gradient (crates/forcefield-core).
  - [x] van-der-Waals / Lennard-Jones term (generic LJ 12-6) + derivative (crates/forcefield-core).
  - [x] Analytic per-atom gradient for Lennard-Jones terms (crates/forcefield-core/field.rs).
  - [x] Analytic per-atom gradient for bond-stretch terms (crates/forcefield-core/field.rs).
  - [x] Analytic per-atom gradient for angle-bend terms (crates/forcefield-core/field.rs).
  - [x] Analytic per-atom gradient for Lennard-Jones terms (crates/forcefield-core/field.rs).
  - [x] Analytic per-atom gradient for inversion (improper torsion) terms (crates/forcefield-core/field.rs).
  - [ ] Electrostatic Coulomb term based on GMP_Xi/Hardness (optional).
  - [x] Coulomb term energy + analytic gradient (crates/forcefield-core).
  - [x] Aggregate ForceField struct storing particles & computing total energy (crates/forcefield-core/field.rs).

  - [ ] Minimisation / optimisation engine
    - [x] Scaffold SDParams and steepest_descent placeholder (crates/forcefield-core/optim.rs).
    - [x] Flesh out gradient calculation and update coordinates (steepest descent implemented).
    - [x] Add simple Polak–Ribiere conjugate-gradient implementation (crates/forcefield-core/optim.rs).
    - [x] Provide `optimize_geometry()` helper wrapping algorithms (crates/forcefield-core/optim.rs).

- [ ] Unit-test translation – ForceField/UFF
  - [ ] Port `testUFFForceField.cpp` (bond, angle, torsion, vdW) to Rust.
  - [ ] Regression tests on methane, water, benzene geometries versus C++ energies.
    - [ ] Re-enable methane optimisation energy test once missing terms implemented (see shame.md).

- [ ] GraphMol core datastructs (upcoming major set)
  - [x] Atom, Bond enums / structs with properties (crates/rdkit-core/mol.rs).
  - [x] Single `Mol` struct (vec-based) placeholder (crates/rdkit-core/mol.rs). Switch to `petgraph` later if needed.
  - [ ] Implement `MolEditor` session wrapper with dirty flag & explicit commit.
  - [ ] Basic sanitisation & valence model.
  - [ ] SMILES parser producing `Mol`.

- [ ] Dependency evaluation / ecosystem
  - [ ] Decide whether to depend on `nalgebra` vs home-brew algebra.
  - [ ] Evaluate `petgraph` or custom graph for molecule.


 - [x] Implement numerical gradient + steepest descent optimiser (crates/forcefield-core/optim.rs)
   - [x] Added central finite-difference gradient, simple backtracking line search, RMS-gradient convergence.
   - [x] Unit test on distorted water molecule.

## Recent updates

- Added analytic per-atom gradient implementations:
  - Bond-stretch (`ForceField::bond_gradients`).
  - Angle-bend (`ForceField::angle_gradients`).
  - Integrated analytic gradients into optimisation engine (`optim.rs`).
- All workspace tests and clippy lints pass.

---

Currently working on: **Stage 5 – Implement Mol sanitisation (valence check) in MolBuilder::finish()**
