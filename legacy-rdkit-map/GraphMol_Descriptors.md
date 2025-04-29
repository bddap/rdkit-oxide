# GraphMol – Descriptors Sub-module Map

Location: `rdkit/Code/GraphMol/Descriptors`

`Descriptors` hosts **physicochemical property calculators** implemented as C++
functions that operate on an `RDKit::ROMol`.  The sub-package is eclectic: it
includes simple “one-liner” counts (e.g. heavy-atom count), sophisticated
fragment-based logP models (Wildman–Crippen), whole-molecule topological
indices (BCUT), and 3-D shape descriptors (WHIM, USR, PBF…).  Most algorithms
are exposed through free functions living in a corresponding header and, when
appropriate, a helper class for parameter storage or data caching.  The public
API is aggregated in `MolDescriptors.h` and, since 2016, a lightweight
registry mechanism in `Property.{h,cpp}` allows run-time discovery of
descriptors.

-------------------------------------------------------------------------------

## 1. Aggregation and utilities

| File | Purpose |
|------|---------|
| `MolDescriptors.{h,cpp}` | Convenience wrappers `calcAMW()`, `calcExactMW()`, `calcNumAtoms()`, etc.  Pulls in several other headers and re-exports their functions. |
| `Property.{h,cpp}` | **Descriptor registry** (`Properties::registerProperty()`) and query helpers (`makePropertyQuery`).  Enables dynamic selection and composition of descriptors without hard-coding symbol names. |
| `RegisterDescriptor.h` | `#define REGISTER_DESCRIPTOR(name,version,func)` macro used by individual descriptor implementations to auto-register with the registry at static init time. |

-------------------------------------------------------------------------------

## 2. 0-/1-D physicochemical descriptors (counts & fragments)

| Header | Key public functions | Notes |
|--------|----------------------|-------|
| `Crippen.h` | `getCrippenAtomContribs()`, `calcCrippenDescriptors()`, `calcClogP()`, `calcMR()` | Wildman–Crippen atom-type model; provides `CrippenParamCollection` singleton with SMARTS patterns. |
| `Lipinski.h` | `NumHBA()`, `NumHBD()`, `NumRotatableBonds()`, `FractionCSP3()` | Reproduces Lipinski & Veber rules; uses SMARTS queries hard-coded in the source. |
| `MolSurf.h` | `calcTPSA()`, `labuteASA()` | Topological polar surface area (Eriksson/Leeson) and Labute approximate ASA. |
| `OxidationNumbers.h` | `calcOxidationNumbers()` | Iterative electron bookkeeping across bonds; returns vector per atom. |
| `MQN.h` | `calcMQN()` | 42-bit Molecular Quantum Numbers fingerprint (counts of hetero atoms, rings, etc.). |
| `ConnectivityDescriptors.h` | `Chi0()`, `Kappa1()` etc. | Kier–Hall χ and κ indices derived from graph invariants. |

-------------------------------------------------------------------------------

## 3. 2-D topological descriptors based on distance matrices

| File | Synopsis |
|------|----------|
| `AUTOCORR2D.{h,cpp}` | 2-D autocorrelation of atomic properties over topological distances (defined by `maxPath=8`).  Returns 192-element vector (× property kinds). |
| `BCUT.{h,cpp}` | Calculate Burden eigenvalues of a charge/weight modified adjacency matrix; returns three eigenvalues per weighting scheme. |
| `RDF.{h,cpp}` | Radial Distribution Function in bond-distance space (bin size 1). Optional Gaussian broadening via `fuzz`. |
| `GETAWAY.{h,cpp}` | GEometry, Topology and Atom-Weights AssemblY descriptors (3 × 57 values). Combines geometry and adjacency information. |

-------------------------------------------------------------------------------

## 4. 3-D geometry-based descriptors (require coordinates)

All these functions take a `confId` parameter (defaults to active conformer
`-1`) and will throw if the molecule lacks 3-D coordinates.

| Header | Output length | Description |
|--------|---------------|-------------|
| `AUTOCORR3D.h` | 15 | 3-D autocorrelation (distance bins in Å). |
| `MORSE.h` | 256 | 3-D MoRSE (Molecular Representation of Structures based on Electron diffraction). |
| `WHIM.h` | 114 | Weighted Holistic Invariant Molecular descriptors derived from the covariance matrix of coordinates. |
| `RDF.h` | 256 | 3-D Radial Distribution Function (continuous). |
| `PMI.h` | 3 | Principal moments of inertia and axis ratios. |
| `PBF.h` | 1 | Plane of Best Fit distance — shape flatness measure. |
| `USRDescriptor.h` | 12 | Ultrafast Shape Recognition descriptor (centroid + 3 reference points). |

Supporting infrastructure:

* `Data3Ddescriptors.{h,cpp}` – tables of per-element constants (vdW radii, molar mass, electronegativity, etc.) accessed by the above algorithms.
* `MolData3Ddescriptors.{h,cpp}` – wrapper class that pre-computes atomic properties (caches expensive look-ups across multiple 3-D descriptors).

-------------------------------------------------------------------------------

## 5. Partial-charge / electronic descriptors

| File | Purpose |
|------|---------|
| `EEM.{h,cpp}` | Electronegativity Equalisation Method charges; provides `EEM(const ROMol&, std::vector<double>& charges, std::string model="eem")`.  Charge values are later used by e.g. GETAWAY.

-------------------------------------------------------------------------------

## 6. Tests

Descriptor implementations are exhaustively unit-tested; key files:

* Generic C++ Catch tests: `catch_tests.cpp` (registry & smoke tests).
* One test file per major descriptor (e.g. `testBCUT.cpp`, `testWHIM.cpp`, `testPBF.cpp`).
* 3-D algorithms additionally have Python wrappers tested in `test3D.py`.

All test sources reside alongside implementation and rely on data under
`test_data/` (SDF with reference values) or inline `SMILES` cases.

-------------------------------------------------------------------------------

## 7. Dependencies

• Core `GraphMol` structures (`ROMol`, `Conformer`).
• `DataStructs` – vector containers (`ExplicitBitVect`, `SparseIntVect`).
• `Geometry`: point algebra for 3-D coordinates.
• `Boost::numeric::ublas` for eigenvalues (BCUT) and linear algebra in several
  3-D descriptors.

-------------------------------------------------------------------------------

## 8. Rust-port notes

1. Provide a `descriptor` crate with trait:
   ```rust
   pub trait Descriptor {
       type Output;
       fn name(&self) -> &'static str;
       fn calculate(&self, mol: &Molecule) -> Self::Output;
   }
   ```
2. Create sub-modules mirroring the C++ layout (`topological`, `geom3d`,
   `physchem`, `registry`).  Keep each algorithm self-contained to ease
   parallel porting.
3. Use `nalgebra` or `ndarray` for linear algebra; `petgraph` already used by
   core GraphMol rewrite can furnish topological distances.

-------------------------------------------------------------------------------

End of Descriptors map.
