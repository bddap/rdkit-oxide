# Progress Tracker

This file is updated incrementally to survive context compression.  Each top-level TODO corresponds to one deliverable in `job.md`.  Nested lists can be arbitrarily deep.

## Stage 1 – Root summary (this commit)

- [x] Create `legacy-rdkit-map/overall.md` containing a high-level map of RDKit’s root modules.
  - Path: `legacy-rdkit-map/overall.md`

## Stage 2 – In-depth maps (complete)

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

## Stage 2.5 – Unit test catalogue (in progress)

- [ ] Enumerate every unit test, with location and explanation, into `legacy-rdkit-map/tests.md`.
  - [x] RDGeneral tests (legacy-rdkit-map/tests.md)
  - [x] DataStructs tests (legacy-rdkit-map/tests.md)
  - [x] Geometry & DistGeom tests (legacy-rdkit-map/tests.md)
  - [x] ForceField tests (legacy-rdkit-map/tests.md)
  - [ ] GraphMol core + submodules tests
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

## Stage 3 – Relationship graph (pending)

- [ ] Create `legacy-rdkit-map/map.md` with text graph of symbol relationships.
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

## Stage 4 – Rust rewrite plan (pending)

- [ ] Draft high-level module plan in `plan/overall.md`.

## Stage 5 – Implementation (pending)

- [ ] Set up and try out Rust workspace in pwd.
- [ ] Translate core data structures (`BitVect`, etc.).
- [ ] ... (to be expanded later).

---

## Stage 2.5 status: complete ✅

Currently working on: **“Stage 3 – Relationship graph completeness sweep (minor pending subsystems).”**
