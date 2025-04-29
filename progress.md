# Progress Tracker

This file is updated incrementally to survive context compression.  Each top-level TODO corresponds to one deliverable in `job.md`.  Nested lists can be arbitrarily deep.

## Stage 1 – Root summary (this commit)

- [x] Create `legacy-rdkit-map/overall.md` containing a high-level map of RDKit’s root modules.
  - Path: `legacy-rdkit-map/overall.md`

## Stage 2 – In-depth maps (pending)

- [ ] Produce detailed symbol maps for every first-level module in `rdkit/Code/`:
  - [ ] Catalogs
  - [x] Catalogs (legacy-rdkit-map/Catalogs.md)
  - [x] ChemicalFeatures (legacy-rdkit-map/ChemicalFeatures.md)
  - [x] DataManip (legacy-rdkit-map/DataManip.md)
  - [x] DataStructs (legacy-rdkit-map/DataStructs.md)
  - [x] DistGeom (legacy-rdkit-map/DistGeom.md)
  - [ ] Features
  - [ ] ForceField
  - [ ] Geometry
  - [ ] GraphMol
  - [ ] JavaWrappers (documentation only – no Rust port)
  - [ ] ML
  - [ ] MinimalLib
  - [ ] Numerics
  - [ ] PgSQL
  - [ ] Query
  - [ ] RDBoost (documentation only – no Rust port)
  - [ ] RDGeneral
  - [ ] RDStreams
  - [ ] SimDivPickers

- [ ] Recursively expand the checklist for each sub-module once its parent summary is done.

## Stage 2.5 – Unit test catalogue (pending)

- [ ] Enumerate every unit test, with location and explanation, into `legacy-rdkit-map/tests.md`.

## Stage 3 – Relationship graph (pending)

- [ ] Create `legacy-rdkit-map/map.md` with text graph of symbol relationships.

## Stage 4 – Rust rewrite plan (pending)

- [ ] Draft high-level module plan in `plan/overall.md`.

## Stage 5 – Implementation (pending)

- [ ] Set up Rust workspace under `rust-oxide/`.
- [ ] Translate core data structures (`BitVect`, etc.).
- [ ] ... (to be expanded later).

---

Currently working on: **“Stage 2 – Detailed map for `Features` module.”**
