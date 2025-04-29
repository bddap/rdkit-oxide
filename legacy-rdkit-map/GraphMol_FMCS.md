# GraphMol – FMCS (Fast Maximum Common Substructure) Sub-module Map

Location: `rdkit/Code/GraphMol/FMCS`

This directory contains RDKit’s C++ implementation of the **FMCS** algorithm –
a fast heuristic search for the *maximum common substructure* (MCS) shared by a
set of molecules.  It underpins the public Python wrapper `rdFMCS.FindMCS()` and
is able to match dozens of drug-sized molecules within seconds thanks to
aggressive pruning, hashing caches and optional timeouts.

--------------------------------------------------------------------------------

## 1. Public interface (`FMCS.h`)

The header is exported and therefore part of the stable API.

### Type aliases / enums
* `AtomComparator`, `BondComparator`, `RingComparator` – predefined comparison
  policies selectable from Python (`AtomCompareElements`, …).
* `MCSAtomCompareParameters`, `MCSBondCompareParameters` – fine-grained flags
  such as `MatchValences`, `RingMatchesRingOnly`, `MatchStereo`, etc.

### Function-pointer hooks
```
MCSAtomCompareFunction      // user atom matcher
MCSBondCompareFunction      // user bond matcher
MCSAcceptanceFunction       // accept/reject a complete candidate
MCSFinalMatchCheckFunction  // called during growth for pruning
MCSProgressCallback         // progress / timeout hook
```

### `struct MCSParameters`
High-level knob that owns all the above plus:
* `bool StoreAll` – keep degenerate solutions
* `double Threshold` – fraction of molecules that must match (≤1)
* `unsigned int Timeout` – seconds (0 = no limit)
* Helper setters `setMCSAtomTyperFromEnum()`/`…ConstChar()` used by SWIG.

### `struct MCSResult`
Return value of `findMCS()` containing
* `NumAtoms`, `NumBonds`
* `std::string SmartsString` – SMARTS encoding of result
* `ROMOL_SPTR QueryMol` – ready-to-use query molecule
* `bool Canceled` – true if timeout / progress abort occurred

### Free functions
* `parseMCSParametersJSON()` – convenience JSON API.
* 3× overloaded `findMCS()` – the workhorse; variants that accept parameter
  struct, raw JSON string, or individual knobs for backward compatibility.

--------------------------------------------------------------------------------

## 2. Internal building blocks

| File | Purpose |
|------|---------|
| `Graph.h` | Thin wrapper around a `boost::adjacency_list` whose **vertices are atom indices** and **edges are bond indices** of the source molecule.  Enables cheap mapping back to RDKit objects. |
| `Seed.h` / `Seed.cpp` | A *growing candidate subgraph*.  Holds vectors of query/target mapping plus convenience methods `addAtom()`, `addBond()`. |
| `SeedSet.h` | Priority queue (`std::multiset`) of seeds ordered by descending size so the largest candidates are tried first. Provides `next()` / `insertOrDiscard()` with duplicate detection. |
| `Target.h` | Light wrapper around a molecule being matched against the current query (`FMCS::Graph TargetMolGraph;` + pre-computed atom/bond labels). |
| `TargetMatch.{h,cpp}` | Stores an actual mapping between **one** target and the current query seed. Implements VF2-like neighbour extension to test whether a grown seed still matches. |
| `SubstructureCache.*` | Hash table accelerating repeated substructure queries (works together with Morgan labelling). Enabled by `FAST_SUBSTRUCT_CACHE` macro. |
| `DuplicatedSeedCache.*` | Detects graph-isomorphic seeds that differ only by atom numbers to avoid redundant work. |
| `MatchTable.*` | Compact `N × M` boolean matrix used during matching to rule out impossible atom/bond correspondences early. |
| `DebugTrace.h` | Macro gates printing and inexpensive counters used while developing the algorithm. `#ifdef VERBOSE_STATISTICS_ON`. |
| `MaximumCommonSubgraph.*` | **Core search engine** (class `MaximumCommonSubgraph`). Combines everything above into an iteratively growing breadth-first search. |

--------------------------------------------------------------------------------

## 3. Search algorithm overview (simplified)

1. **Pre-processing**
   • Convert each input `ROMol` to an `FMCS::Graph` and store inside `Target`.  
   • Build atom/bond label tables according to `AtomTyper` / `BondTyper`.  
   • Compute global `ThresholdCount = ceil(Threshold · nMols)`.

2. **Initial seeds** – Single bonds (or atoms for degenerate cases) are pushed
   into a `SeedSet` priority queue.

3. **Main loop** (`MaximumCommonSubgraph::growSeeds()`):
   a. Pop the largest seed.
   b. Attempt to *grow* by one bond in all possible ways – each growth is
      immediately validated against `ThresholdCount` molecules via
      `TargetMatch` and user comparators.
   c. Failed growth is discarded; successful growth is inserted into the queue
      (unless a duplicate seen in `DuplicatedSeedCache`).
   d. `MCSProgressCallback` is invoked every N seeds; returning `false` stops
      the search (timeout / user abort).

4. **Result generation** – Once the queue is empty or the search was aborted,
   the largest accepted seed is converted into a SMARTS string via
   `generateResultSMARTSAndQueryMol()` using `SmartsWrite.h` utilities.

The algorithm is *not* guaranteed to find a mathematically maximum common
subgraph in all cases (NP-complete), but empirical evidence shows identical
results to the exact algorithm in >99 % of DrugBank pairs while being orders of
magnitude faster.

--------------------------------------------------------------------------------

## 4. Unit tests

* `testFMCS_Unit.cpp` – C++ test validating element, bond and ring comparison
  permutations, timeout handling and JSON parameter parsing.
* `Wrap/testFMCS.py` (under SWIG wrappers) – python-level regression suite used
  by CI.  
* Additional real-world regression data in `testData/` (SMILES files).

These tests will need porting in **Stage 5**; see `progress.md`.

--------------------------------------------------------------------------------

## 5. Dependencies

• Core `GraphMol` (Atom, Bond, ROMol).  
• `Boost.Graph` for candidate/target graphs.  
• `DataStructs/SparseIntVect` for label hashing in cache.  
• Optional boost chrono for `nanoClock()` timing.

--------------------------------------------------------------------------------

## 6. Rust-port notes

1. Replace `boost::adjacency_list` with `petgraph::Graph<AtomIdx, BondIdx>`; the
   graph is small (≤100 vertices) and undirected.  
2. Represent `Seed` as a `Vec<NodeIndex>` + `BitSet` for bonds; deduplication
   can use `hashbrown::HashSet` with canonicalised edge list as key.  
3. Callback hooks map naturally to Rust `Fn` traits with `&dyn` trait objects.  
4. Timeout can be implemented with `std::time::Instant::elapsed()` checked in
   the main loop (no cross-thread cancel needed).  
5. SMARTS rendering will rely on the to-be-implemented `chem_smarts` crate
   planned in the Fingerprints notes.

--------------------------------------------------------------------------------

End of FMCS map.
