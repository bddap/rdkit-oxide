# DataManip Module – Detailed Map

Location: `rdkit/Code/DataManip`

DataManip contains **small, generally useful data-manipulation helpers**.  In the current tree it holds only one functional sub-module, *MetricMatrixCalc*, but the directory structure suggests room for expansion.  The code is header-only and template-based, allowing it to work with raw C arrays, STL containers, RDKit bit‐vector classes, or Python sequences in the Boost.Python wrapper.

--------------------------------------------------------------------------------

## Sub-module: MetricMatrixCalc

Path: `rdkit/Code/DataManip/MetricMatrixCalc`

### File list

| File | Description |
|------|-------------|
| `MetricMatrixCalc.h` | Templated calculator that produces pairwise *metric* matrices (distance or similarity), given a container of descriptors and a user-supplied metric function. |
| `MetricFuncs.h` | Collection of ready-to-use metric functions: Euclidean distance, Tanimoto distance, Tanimoto similarity. |
| `testMatCalc.cpp` | Minimal C++ smoke-test. |
| `Wrap/` | Boost.Python extension exposing a high-level API (`rdkit.DataManip.Metric.rdMetricMatrixCalc`) with NumPy integration. Includes Python unit test `testMatricCalc.py`. |
| `CMakeLists.txt` | Build rules for the wrapper only (core is header-only). |

### Public API details

All public symbols live in namespace `RDDataManip`.

1. Templated *free functions* in `MetricFuncs.h`:
   • `double EuclideanDistanceMetric(const T1&, const T2&, unsigned int dim)`  
   • `double TanimotoDistanceMetric(const T1&, const T2&, unsigned int dim)`  
   • `double TanimotoSimilarityMetric(const T1&, const T2&, unsigned int dim)`

   Internally, the Tanimoto flavours call `DataStructs::SimilarityWrapper` with either `TanimotoSimilarity` or a subtraction.

2. Class template `MetricMatrixCalc<vectType, entryType>` (MetricMatrixCalc.h 22-76)

   • Member `void setMetricFunc(double (*)(const entryType&, const entryType&, unsigned int))` – registers metric.
   • Member `void calcMetricMatrix(const vectType& descrips, unsigned nItems, unsigned dim, double* distMat)` – fills *lower-triangle* of pairwise matrix (length = nItems*(nItems-1)/2) using previously set function.

   The implementation relies only on `operator[]` of `vectType` and therefore works with raw pointers, `std::vector<entryType>`, `PySequenceHolder`, etc.

### Boost.Python wrapper (Wrap/rdMetricMatrixCalc.cpp)

Key exported helpers:

```
GetEuclideanDistMat(obj)   // obj = NumPy 2-D array | list(ndarray) | list(list)
GetTanimotoDistMat(obj)    // obj = list of BitVect
GetTanimotoSimMat(obj)     // obj = list of BitVect
```

The wrapper performs substantial runtime type checking and re-packages the result into a contiguous NumPy `double` array.

### Tests

* C++: `testMatCalc.cpp` – verifies Euclidean distance matrix on random data.
* Python: `Wrap/testMatricCalc.py` – extensive coverage of wrapper functions with NumPy arrays, nested lists, `ExplicitBitVect`, `SparseBitVect`.

--------------------------------------------------------------------------------

## Dependencies

• `DataStructs/BitOps.h` (for similarity wrappers)  
• `RDGeneral/Invariant.h` for assertions  
• No linkage dependencies – templates only.

--------------------------------------------------------------------------------

## Rust-port notes

1. Provide trait `MetricFn<T>` or simply accept `fn(&T,&T,usize)->f64`.
2. Generic `calc_metric_matrix` can live in a utility crate; using slices or `AsRef<[T]>` to emulate `operator[]`.
3. Distance matrix can return `Vec<f64>` in row-major lower-triangle order.
4. For Tanimoto metrics we can invoke logic from the forthcoming Rust port of `DataStructs::bitops`.

--------------------------------------------------------------------------------

End of module map.
