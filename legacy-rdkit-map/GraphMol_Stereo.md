# GraphMol – Stereochemistry & CIPLabeler Map

Location: `rdkit/Code/GraphMol/Chirality.*` and `rdkit/Code/GraphMol/CIPLabeler`

RDKit distinguishes two layers of stereochemistry perception:

1. **Perception & assignment** – functions in `Chirality.cpp` detect chiral
   centers and double‐bond stereochemistry, setting `Atom::ChiralTag` and
   `Bond::BondStereo` flags.
2. **CIP labeling** – the *CIPLabeler* sub‐module implements the rigorous
   Cahn–Ingold–Prelog rules, producing R/S, E/Z, and other designations.

-------------------------------------------------------------------------------

## 1. Core chirality perception (`Chirality.*`)

| Symbol | Description |
|--------|-------------|
| `assignAtomChiralTags(ROMol&, bool replaceExistingTags)` | Traverses tetrahedral atoms, uses permutation parity of neighbours ordering; sets `CHI_TETRAHEDRAL_{CW,CCW}`. |
| `assignStereochemistry(ROMol&, bool cleanIt, bool force, bool flagPossible)` | Orchestrates atom+bond stereo perception, optionally removing previous info. |
| `detectBondStereochemistry(Bond&)` | Recognises cis/trans around double bonds with defined substituents; sets `BondStereo::STEREOE` or `STEREOZ`. |

Important helpers in the file:
* `isPseudoAtomStereoCenter()` – treat query/dummy atoms.
* `pickHighestPriority()` – implements CIP priority rule #1 (atomic number).

The perception step stores auxiliary properties (e.g., `_CIPCode`) on atoms and bonds for later retrieval.

-------------------------------------------------------------------------------

## 2. CIPLabeler sub‐module

### Design overview

The labeler constructs a **directed graph of stereo elements** (atoms and
bonds) where edges encode “priority higher than” relationships as required by
Rules 1–4 bis of the CIP system. The graph is topologically sorted and then
examined for cycles and pseudoasymmetry.

### Key classes (files)

| File | Class | Notes |
|------|-------|-------|
| `CIPMol.{h,cpp}` | `CIPMol` | Thin wrapper around `ROMol` exposing convenient iterators over “CIP relevant” atoms and bonds; caches atomic invariants (mass, atomic number, isotopic mass, h‐count). |
| `CIPLabeler.{h,cpp}` | `CIPLabeler` | Entry class with `assignCIPLabels(CIPMol&)` static method. Internally builds `Digraph` of `Edge`s between `StereoElement` derived structs. |
| `Digraph.{h,cpp}` | `Digraph` | Generic adjacency list with DFS utilities, cycle detection, ranking. |
| `Edge.{h,cpp}` | `Edge` | Contains pointer to `StereoElement`, inequality flag (`>`). |
| `Descriptor.h` | enum `Descriptor` (R, S, r, s, E, Z, M, P, undefined). |
| `StereoCenter.{h,cpp}` (*inline in CIPLabeler.cpp*) | `AtomStereoCenter`, `BondStereoCenter` | Provide `priority(vector<Substituent>)` calculation and final `Descriptor` assignment.

### Algorithm flow
1. Build `CIPMol` from `ROMol` – each atom/bond flagged as stereo candidate.
2. For each candidate create `StereoElement` with list of *substituents*, each
   substituent is an *ordered path* away from center; path comparison uses
   recursive breadth expansion until a difference is found (rule #1–3).
3. Generate pairwise `Edge`s whose direction encodes higher priority.
4. Topologically sort; if conflict or cycle detected, mark descriptor as
   *unknown*.
5. For atoms with two identical substituents apply pseudoasymmetric (r/s)
   determination.
6. Write resulting label into `_CIPCode` atom/bond property.

Complexities handled: multiple-bond duplicate neighbours, wiggly bonds
(`STEREOANY`), spring bonds, cumulenes (axial chirality), atropisomers.

-------------------------------------------------------------------------------

## 3. Tests

* `catch_chirality.cpp`, `catch_tests.cpp` inside `CIPLabeler` dir – unit tests
  directly exercising rule edge cases.
* Higher level Python tests under `Chem.Crippen` etc.

-------------------------------------------------------------------------------

## 4. Dependencies

• `GraphMol/RingInfo` for stereogenic axis detection in rings.  
• `Boost::optional` for lazy rule evaluation.  
• `DataStructs` for atom invariants hashing (tie breaker).

-------------------------------------------------------------------------------

## 5. Rust-port notes

1. Represent `Descriptor` as Rust `enum Descriptor { R, S, r, s, E, Z, Unknown }`.
2. Implement trait `StereoElement` with `fn priority(&self, other) -> Ordering`.
3. Use `petgraph::Graph` for digraph; topological sort via `algo::toposort`.
4. Heavy recursion for path comparison could leverage memoisation with `HashMap<(AtomId, depth), Vec<Invariant>>`.

-------------------------------------------------------------------------------

End of Stereochemistry map.
