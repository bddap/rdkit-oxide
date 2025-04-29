# GraphMol – Substructure Search Sub-module Map

Location: `rdkit/Code/GraphMol/Substruct`

`Substruct` implements RDKit’s **SMARTS‐based substructure matcher** that
answers the core cheminformatics question: “Does molecule *M* contain query
*Q*?”  The public entry point is the set of `SubstructMatch()` overloads in
`SubstructMatch.h`.  Internally the algorithm is an *augmented* VF2 graph
isomorphism with domain‐specific atom/bond comparators and optional
post‐processing to enforce stereochemistry.

-------------------------------------------------------------------------------

## 1. Public API surface

| Header | Symbol | Explanation |
|--------|--------|-------------|
| `SubstructMatch.h` | `MatchVectType` | `std::vector<std::pair<int,int>>` mapping `queryAtomIdx → molAtomIdx`. |
| | `SubstructMatchParameters` | Struct of knobs controlling the search (chirality, aromatic/conjugated equivalence, recursion, multithread, enhanced stereo, property equivalence, etc.). |
| | `SubstructMatch()` (overloads) | Finds one or *all* matches between: `(ROMol,ROMol)`, `(ResonanceMolSupplier,ROMol)`, `(MolBundle,ROMol)` and permutations thereof.  Convenience templates accept STL containers and fill `MatchVectType`. |

Important default limits:
* `maxMatches = 1000` protects against combinatorial explosions.
* `maxRecursiveMatches` guards recursive SMARTS like `[$(c1ccc*)]`.

-------------------------------------------------------------------------------

## 2. Core implementation files

| File | Role |
|------|------|
| `SubstructMatch.cpp` | **Main driver**: prepares label functors, expands recursive queries, calls `boost::vf2_all()` (custom copy) and post‐processes matches. Handles multi‐thread dispatch when `numThreads` > 1. |
| `vf2.hpp` | Modified extract of *vflib* implementing VF2 **subgraph‐isomorphism** for Boost.Graph `adjacency_list`. Exposes `vf2_all()` that populates a vector of `(queryIdx,molIdx)` pairs. Includes RDKit‐specific pruning macros. |
| `ullmann.hpp` | Legacy Ullmann algorithm implementation kept for reference/benchmarks (no longer compiled by default). |
| `SubstructUtils.{h,cpp}` | Utility predicates for atom/bond/property compatibility, duplicate‐removal heuristics (`toPrime()`, `removeDuplicates()`), and helpers to rank matches by *degree of core substitution*. |

Algorithm flow (high level):
1. **Prepare comparators** – `detail::AtomLabelFunctor` and `BondLabelFunctor` wrap `atomCompat()`/`bondCompat()` with capture of `SubstructMatchParameters`.
2. **Expand recursive SMARTS** (`$(*.a.$(*)...)`) via `MatchSubqueries()` which constructs `RecursiveStructureQuery` objects and performs mini‐searches to cache valid atom candidates.
3. **Run VF2** – `boost::vf2_all()` enumerates embeddings subject to comparators and `params.maxMatches` cap.
4. **Enhanced stereo filter** – `enhancedStereoIsOK()` ensures OR/AND stereogroup semantics are respected.
5. **Uniquify results** – when `params.uniquify` is true, matches are canonicalised using dynamic bitset hashing.

-------------------------------------------------------------------------------

## 3. Supporting data structures & helpers

* `RecursiveStructureQuery` (defined in `RDKitQueries.h`) – a query atom that
  itself contains a molecule pattern; cached results are stored in
  `SUBQUERY_MAP`.
* `MolMatchFinalCheckFunctor` – final predicate passed to VF2 that applies
  chiral/enhanced stereo constraints and calls any user‐supplied
  `extraFinalCheck` callback.
* Primes trick: `toPrime()` maps atom indices in a match to a unique product
  of small prime numbers, enabling fast duplicate detection.

-------------------------------------------------------------------------------

## 4. Executables & benchmarks

| File | Purpose |
|------|---------|
| `cmd_match.cpp` | Tiny CLI demonstrating `SubstructMatch` (build target `SubstructMatchExamples`). |
| `bench.cpp` | Micro‐benchmark comparing VF2 vs Ullmann on a corpus of ~7 k molecules. |

-------------------------------------------------------------------------------

## 5. Tests

* C++ Catch tests: `catch_tests.cpp`, `test1.cpp` – cover atom/bond query options, stereochemistry, recursion, bundles, multi-threading.
* Python tests: `UnitTestSubstruct.py` (comprehensive SMARTS spec conformance, stereochemistry, thread safety, property flags).
* Data files: `bond-query.cdxml`, `list-query.cdxml`, `regress.txt` host regression SMARTS and answer sets.

-------------------------------------------------------------------------------

## 6. Dependencies

• `GraphMol/Core` classes for topology access (`ROMol.getTopology()`).
• `Query` module (`QueryObjects.h`) for atom/bond SMARTS trees.
• `Bo​ost.Graph` for adjacency_list and VF2 integration.
• `RDGeneral/RDThreads.h` abstracts `std::thread` vs TBB.

-------------------------------------------------------------------------------

## 7. Rust-port considerations

1. Use `petgraph::algo::isomorphism::vf2()` as starting point; supply custom
   `NodeMatcher`/`EdgeMatcher` closures replicating `atomCompat()` and
   `bondCompat()` logic.
2. Represent SMARTS queries as recursive enums akin to `chemquery` crate; ensure
   recursion caching via `HashMap<QueryAtomId, Vec<NodeIndex>>` to avoid
   repeated sub‐searches.
3. Multi‐thread search can be achieved with `rayon` parallel iterators over
   molecule batches.

-------------------------------------------------------------------------------

End of Substructure Search map.
