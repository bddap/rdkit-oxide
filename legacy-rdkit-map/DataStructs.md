# DataStructs Module – Detailed Map

Location: `rdkit/Code/DataStructs`

The **DataStructs** library provides *generic data structures and algorithms* used throughout RDKit, most notably multiple **bit‐vector** implementations, similarity metrics, and fast binary fingerprint file IO.  Nearly every other core module depends on DataStructs.

--------------------------------------------------------------------------------

## 1. BitVector infrastructure

### 1.1 Abstract interface – `BitVect` (BitVect.h / BitVect.cpp)
• Pure virtual base exposing bit manipulation, counts, serialization.  
• Method `initFromText()` handles legacy pickle/base64 parsing.

### 1.2 Dense implementation – `ExplicitBitVect` (ExplicitBitVect.h/.cpp)
• Stores bits in `boost::shared_array<WORD_TYPE>` (dynamic).  
• Fast count via pre-computed popcount table.  
• Provides – and overloads – bitwise operators (`&`, `|`, `^`, `~`).

### 1.3 Sparse implementation – `SparseBitVect` (SparseBitVect.h/.cpp)
• Uses `boost::shared_array<unsigned int>` storing indices of on bits, grouped into *blocks* for cache efficiency.  
• Supports population count and similarity in `O(n_on)`.

### 1.4 Helper headers
• `BitOps.h/cpp` – collection of similarity/distance functions (Tanimoto, Dice, Cosine, …) plus `SimilarityWrapper` which auto-dispatches to `const ExplicitBitVect&` or `const SparseBitVect&`.  
• `BitVectUtils.h` – miscellaneous utilities (folding, picking on-bits, Morgan hashing).  
• `BitVects.h` – convenience `typedef` group: `ExplicitBitVect`, `SparseBitVect`, `ExplicitBitVect*`, etc.

--------------------------------------------------------------------------------

## 2. Integer and value vectors

| Class | File | Purpose |
|-------|------|---------|
| `SparseIntVect<T>` | SparseIntVect.h | Generic sparse map from unsigned int index → value `T` using `std::map` internally; supports arithmetic, dot product, pickling. |
| `DiscreteValueVect` | DiscreteValueVect.h/.cpp | Sparse int vector with bucketed counts (histogram). |
| `RealValueVect` | RealValueVect.h/.cpp | Dense `std::vector<double>` wrapper plus similarity metrics. |

--------------------------------------------------------------------------------

## 3. Distance/Similarity helpers

• `DiscreteDistMat.{h,cpp}` – calculates condensed pairwise distance matrix (wrapper around `RDDataManip::MetricMatrixCalc`).

• `Utils.cpp` – miscellaneous free functions (`rdkitHash`, `lazy_popcount`, etc.).

--------------------------------------------------------------------------------

## 4. Fingerprint Binary (FPB) support

### 4.1 `FPBReader` (FPBReader.h/.cpp)
Reads *FPB* files – a compact on-disk format for large collections of fingerprints.  Features:
  – Lazy or eager loading.  
  – Similarity searches (`getTanimotoNeighbors`, sub-linear via popcount index).  
  – Random access to `ExplicitBitVect` or raw byte vector.

### 4.2 `MultiFPBReader` (MultiFPBReader.h/.cpp)
Thin wrapper combining multiple FPB files and performing merged similarity searches.

--------------------------------------------------------------------------------

## 5. Common infrastructure

• `DatastructsException.h` – custom exception hierarchy (`DataStructsException`, `IndexException`, `RangeException`).  
• `DatastructsStreamOps.h` – stream read/write helpers mirroring `RDGeneral/StreamOps` but for datastructs.

• `base64.{h,cpp}` – standalone base64 encoder/decoder used by `BitVect` pickling.

--------------------------------------------------------------------------------

## 6. Tests and wrappers

### C++ tests
* `catch_tests.cpp` – modern Catch2 test-suite covering bitvector operations.
* `testDatastructs.cpp` – legacy unit tests.
* `testFPB.cpp`, `testMultiFPB.cpp` – integration tests for FPB readers.

### Python wrappers (DataStructs/Wrap)
Boost.Python exposes classes and functions to Python; also includes tests `testBV.py`, `testFPB.py`, `testRealValueVect.py`, etc.  These are out of scope for Rust port but serve as behavioral reference.

--------------------------------------------------------------------------------

## External dependencies

• Boost (shared_ptr, hashing).  
• `<intrin.h>` / builtin popcount where available.  
• `RDGeneral` for logging, stream ops, and invariant macros.

--------------------------------------------------------------------------------

## Rust-port considerations

1. Bit vectors: leverage `bitvec` crate for dense; implement custom RLE blocks for sparse.  Provide trait `BitVector` mirroring `BitVect` interface.
2. Popcount and similarity metrics: use `u64` and `u128` chunked bit operations; SIMD via `packed_simd` feature-gated.
3. FPB Reader: map file using `memmap2` and replace Boost dependencies with `Arc`/`Vec`.  Provide thread-safe API.
4. Serialization: use `bincode` for in-memory but keep compatibility adapter to read legacy pickles/base64.

--------------------------------------------------------------------------------

End of module map.
