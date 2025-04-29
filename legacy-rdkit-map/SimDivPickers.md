# SimDivPickers Module Map

Location: `rdkit/Code/SimDivPickers`

This module contains **diversity selection algorithms** used to pick a subset
of items (typically molecules or fingerprints) that are as diverse as possible
according to a pre-computed distance matrix.  These pickers are utilised by
`rdSimDivPickers.MaxMinPicker`, `LeaderPicker`, etc. from Python and serve as a
foundation for building screening decks, clustering representatives, and
cross-validation folds.

Algorithms operate purely on numeric distances and are therefore agnostic to
chemistry—they can be reused for fingerprints, descriptor vectors, or RMSD
matrices.

-------------------------------------------------------------------------------

## 1. Common base – `DistPicker`

File: `DistPicker.h / .cpp`

• Provides virtual `pick(distMat, poolSize, pickSize)` method returning
  `INT_VECT` of indices.  
• Helper `getDistFromLTM()` converts `(i,j)` into index of a *lower-triangular
  packed* 1-D distance matrix.  
• `distmatFunctor` wrapper translates a raw 1-D matrix into a callable
  `double operator()(i,j)` functor consumed by lazy variants in subclasses.

-------------------------------------------------------------------------------

## 2. MaxMinPicker

Files: `MaxMinPicker.h / .cpp`

Algorithm: classic **MaxMin diversity** (a.k.a. Kennard–Stone) per Ashton et
al., QSAR 2002.

Implementation details:
1. `lazyPick()` template takes a callable distance function; maintains vector
   of *current minimal distances* for each candidate, updating it lazily when a
   new pick is added, avoiding full `N×k` recomputation.
2. Overloads allow seeding with fixed `firstPicks` and controlling RNG seed.
3. Public `pick()` convenience method builds `distmatFunctor` and dispatches to
   `lazyPick()`.

Complexities: O(N·k) memory, O(N·k) time with lazy update optimisation.

-------------------------------------------------------------------------------

## 3. LeaderPicker

Files: `LeaderPicker.h`

Algorithm: **Leader clustering** – iterate through pool; if distance to all
leaders exceeds threshold, promote to leader set.  Stops when `pickSize`
leaders collected.

Features:
• `threshold` parameter (default 0.0) can be tuned to control cluster radius.
• Optional multi-threaded implementation under `USE_THREADED_LEADERPICKER`
  (pthreads barrier, only enabled on Unix builds).  Splits candidate space into
  blocks processed by worker threads.

-------------------------------------------------------------------------------

## 4. HierarchicalClusterPicker

Files: `HierarchicalClusterPicker.h / .cpp`

• Builds a **single-linkage hierarchical tree** from the distance matrix then
  extracts cluster representatives so that no two picks are within
  `cutoff` (default 0.2) distance.
• Re-implements minimal subset of SciPy’s `fcluster` for internal use.  
• Accepts `isDistMatInMemory` flag to avoid loading large matrices twice.

-------------------------------------------------------------------------------

## 5. Command-line tool

`pickersCLI.cpp` – Minimal example executable that reads an Eigenvalue
distance matrix from stdin and outputs selected indices.  Mostly used in old
benchmark scripts.

-------------------------------------------------------------------------------

## 6. Python bindings

`Wrap/` SWIG interface exposes:
• `MaxMinPicker().LazyPick(distMat, poolSize, pickSize, seed)`  
• `LeaderPicker(threshold).Pick(distMat, poolSize, pickSize)`  
• `HierarchicalClusterPicker(cutoff).Pick(distMat, poolSize, pickSize)`

-------------------------------------------------------------------------------

## 7. Tests

* `catch_tests.cpp` – C++ unit tests for `getDistFromLTM()` and picker output.
* `testPickers.cpp` – Additional exhaustive tests over random matrices.

Python tests live under `rdkit/SimDivPickers/Wrap` (not listed here).

-------------------------------------------------------------------------------

## 8. Dependencies

• `DataStructs` for `INT_VECT` convenience typedef.  
• `RDGeneral` (utils, invariants, logging).  
• Optional `RDThreads` for multi-threaded leader picker.

-------------------------------------------------------------------------------

## 9. Rust-port notes

1. Represent distance matrix either as `Vec<f64>` lower-triangular or as a
   closure `|i,j| -> f64`.  Provide helper `lt_index(i,j) -> usize` mirroring
   `getDistFromLTM()`.
2. Implement `trait Picker { fn pick(&self, dist: &dyn Fn(usize,usize)->f64,
   pool: usize, k: usize) -> Vec<usize>; }` with concrete structs `MaxMinPicker`
   and `LeaderPicker`.
3. Parallel leader picker → use `rayon::scope` to split candidate array; atomics
   for shared “leaders” Vec.
4. Expose safe wrappers to Python via `pyo3` later if desired (out of scope).

-------------------------------------------------------------------------------

End of SimDivPickers map.
