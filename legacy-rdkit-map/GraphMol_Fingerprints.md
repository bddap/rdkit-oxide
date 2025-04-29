# GraphMol – Fingerprints Sub-module Map

Location: `rdkit/Code/GraphMol/Fingerprints`

This directory contains RDKit’s implementation of several **topological fingerprints** as well as the modern *FingerprintGenerator* framework introduced in v2020.09.  Fingerprints are bit strings (or count vectors) encoding substructure features of molecules; they are used for similarity search, clustering, machine learning, etc.

--------------------------------------------------------------------------------

## 1. Legacy (pre-generator) APIs

| File | Fingerprint | Key public helpers |
|------|-------------|--------------------|
| `AtomPairs.*` | Atom Pair (Carhart) | `std::string GetAtomPairCode(const ROMol&, ...)`, `SparseIntVect<boost::uint32_t> *GetAtomPairFingerprint(...)`. |
| `TopologicalTorsionGenerator.*` | Topological torsions | Similar helpers `GetTopologicalTorsionFingerprint`. |
| `Fingerprints.cpp` | Wrapper dispatching to AtomPair or Torsion based on enum. |

These functions directly build `SparseIntVect<std::uint32_t>` or `ExplicitBitVect` without an object-oriented abstraction.

--------------------------------------------------------------------------------

## 2. New FingerprintGenerator framework

Introduced to unify parameters and support on-the-fly customisation.

### Core classes
• `FingerprintGenerator.h / .cpp` – template `FingerprintGenerator<MoleculeType, OutputType>` with virtual methods `getFingerprint(const ROMol&)`, `initEnvGenerator()`, etc.  
• `FingerprintUtil.{h,cpp}` – generic helper functions for adding bits, hashing paths, folding counts.

### Concrete generators

| File | Generator | Notes |
|------|-----------|-------|
| `AtomPairGenerator.*` | Implements atom-pair FP using `AtomPairArguments` options (minDistance, maxDistance, countSimulation). |
| `TopologicalTorsionGenerator.*` | Modern generator for torsions with variable bond count. |
| `RDKitFPGenerator.*` (in same dir) | Classic RDKit Daylight-like fingerprint (path length 1-7). |
| `MorganGenerator.*` (subdir) | Circular Morgan/ECFP/FCFP with radius, feature invariants, bit info. |
| `MHFPFingerprints.*` | MinHash on circular substructures (MHFP6). Requires `sha1` hashing and `std::unordered_set`.

Each generator has accompanying `Arguments` struct capturing parameters and a factory function `getXFPGenerator()` returning `std::unique_ptr<FingerprintGenerator<>>`.

--------------------------------------------------------------------------------

## 3. Utility / shared files

* `FingerprintUtil.h/cpp` – functions: `numBitsToProcess()`, `sumTwoUnsigned()` (overflow-safe), `truncateHash()`, `addBitToFingerprint()`.
* `MurmurHash3.h` – Copy of public-domain hash used by Morgan.

--------------------------------------------------------------------------------

## 4. Tests

* `catch_tests.cpp` – legacy atom-pair/torsion test cases.  
* `fpgen_catch_tests.cpp`, `testFingerprintGenerators.cpp` – extensive parameterised tests of new framework including bit-info harvesting.  
* `testMHFPFingerprint.cpp` – validates MinHash fingerprint vs known vectors.

--------------------------------------------------------------------------------

## Dependencies

• Core GraphMol (ROMol, Bond, Atom invariants)  
• `DataStructs` for `SparseIntVect` and bit vectors  
• `Boost::functional/hash` for path hashing, and MurmurHash3 implementation.

--------------------------------------------------------------------------------

## Rust-port notes

1. Represent fingerprint as `bitvec::BitVec` or `Vec<u32>` counts.  
2. Provide trait `FingerprintGenerator` with `fn fingerprint(&self, mol: &Molecule) -> FP`.  
3. Morgan and AtomPair require path enumeration; reuse petgraph BFS/DFS.  
4. MurmurHash3 → use `twox-hash` or re-implement 32-bit variant for identical results when needed.

--------------------------------------------------------------------------------

End of Fingerprints map.
