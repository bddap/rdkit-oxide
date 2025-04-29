# RDKit Source – Root‐Level Overview

This document provides a bird-eye map of the **RDKit** source tree as it exists in the `./rdkit` directory of this repository.  It is *not* an in-depth description of every class or function – that work will be done in later stages – but it should help a newcomer understand where the major subsystems live and how they relate to each other.

---

## Top-level layout inside `rdkit/`

| Path | Purpose |
|------|---------|
| `Code/` | All C++ library sources.  This is the heart of RDKit and is subdivided into functional modules such as `GraphMol`, `DataStructs`, `DistGeom`, etc.  Almost all core algorithms live here. |
| `Contrib/` | Third-party or community-contributed code that is not required for the main build. |
| `External/` | Bundled external libraries (e.g. Eigen, yaml-cpp). |
| `Docs/` | Sphinx documentation sources, notebooks, images, etc. |
| `Data/` | Data files used by the library (periodic table information, SMARTS patterns, charge tables, force-field parameters, etc.). |
| `Scripts/` | Miscellaneous helper scripts used during development or CI. |
| `Projects/` | Stand-alone experimental or demo sub-projects. |
| `Regress/`, `Fuzz/` | Inputs used for regression and fuzz testing. |
| Build glue (`CMakeLists.txt`, `.cmake` templates, `setup.cfg`) | CMake + setuptools build definitions, used to build the C++ core and the Python wrappers. |

The **C++** code under `Code/` is what we will eventually port to Rust.  Everything else can either stay as-is or will need only minor adjustments (e.g. data file paths).

---

## High-level structure of `rdkit/Code/`

Below is a concise catalogue of each first-level subdirectory in `Code/`, with a one-liner explaining its role and its main dependencies.  (Detailed, per-file breakdowns will be created in later stages.)

### 1. `Catalogs/`
Implements generic “catalogue” containers that map molecular sub-structure fingerprints to user-defined metadata.  Minimal dependency footprint: relies on `RDGeneral` for logging and `DataStructs` for bit vectors.

### 2. `ChemicalFeatures/`
Defines a *chemical feature* abstraction (aromatic ring, hydrogen-bond donor, etc.) and tools to perceive them from molecules.  Depends on `GraphMol` for molecule representation and on `DataStructs` for fingerprints.

### 3. `DataManip/`
Small collection of data-manipulation helpers; currently contains distance-matrix utilities.

### 4. `DataStructs/`
Low-level containers such as sparse/explicit bit vectors, FPB (fingerprint binary) readers, etc.  Core dependency for nearly every other module.

### 5. `Demos/`
C++ demonstration programs.  Not compiled into the main library.

### 6. `DistGeom/`
Distance-geometry algorithms for 3-D coordinate generation (embedding).  Uses `ForceField` for optimisation, `Numerics` for maths, and `GraphMol` for molecule access.

### 7. `Features/`
Old feature perception code (largely superseded by `ChemicalFeatures`).  Still used in some legacy workflows.

### 8. `ForceField/`
Generic force-field framework plus implementations for UFF and MMFF94.  Consumed by `DistGeom`, `SimDivPickers`, and `GraphMol` conformer-optimisation utilities.

### 9. `Geometry/`
Basic linear-algebra and geometry primitives (points, transforms, tetrahedral chirality helpers).  Wraps Eigen when available.

### 10. `GraphMol/`
Central molecule graph representation (atoms, bonds, conformers) and cheminformatics algorithms (substructure search, SMARTS parsing, reaction handling, etc.).  Nearly every other module ultimately relies on this.

### 11. `JavaWrappers/`
JNI bindings exposing RDKit to Java.  Out of scope for the Rust rewrite.

### 12. `ML/`
Mini-machine-learning utilities (naïve Bayes, decision trees) historically shipped with RDKit.  Mostly self-contained.

### 13. `MinimalLib/`
Small, header-only subset of RDKit for use in WebAssembly builds.

### 14. `Numerics/`
Optimised numerical routines (matrix algebra, SSA random projections, etc.).  Separate from `Geometry` so that pure-math code can compile without geometry primitives.

### 15. `PgSQL/`
PostgreSQL cartridge (C extensions) offering RDKit functions inside the database.  Relies on `GraphMol` and `DataStructs` but is otherwise independent of the rest of the core.

### 16. `Query/`
Abstract query-pattern tree (atoms, bonds, etc.) used by substructure search.  Shared with `GraphMol` but factored out to permit reuse by fingerprinting modules.

### 17. `RDBoost/`
Boost.Python glue that creates the official Python bindings.  This will not be ported to Rust (out of scope).

### 18. `RDGeneral/`
Utility infrastructure: logging, exceptions, invariant checking, timing, and serialization helpers.  The *platform* layer for the whole code-base.

### 19. `RDStreams/`
Streaming I/O helpers with gzip support.

### 20. `SimDivPickers/`
Algorithms for diversity picking (MaxMin, Butina clustering, etc.) used in library design workflows.

---

## Relationships at a glance

The dependency graph roughly looks like:

```
RDGeneral ─┬─> DataStructs ─┬─> GraphMol ─┬─> DistGeom
           │               │            ├─> ForceField
           │               │            ├─> ChemicalFeatures
           │               │            └─> SimDivPickers
           │               └─> Catalogs
           └─> Geometry ──> Numerics

Query is shared between GraphMol and DataStructs

PgSQL, JavaWrappers, RDBoost each sit on the fringe depending on core modules.
```

---

## Next steps

In Stage 2 we must delve into **each** subdirectory and recursively document every public symbol, file, and test.  A preliminary task list has been added to `progress.md`.
