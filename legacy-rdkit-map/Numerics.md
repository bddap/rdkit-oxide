# Numerics Module Map

Location: `rdkit/Code/Numerics`

The **Numerics** subtree bundles light-weight, header-only (or nearly so)
mathematics utilities that RDKit uses for geometry processing and optimisation.
It pre-dates Eigen/Armadillo adoption inside RDKit and therefore re-implements
vectors, matrices, basic linear algebra, eigen-solvers, optimisers, and point
alignment.

Although these routines are not chemistry-specific, a handful of high-level
algorithms (force-field minimisation, ETKDG alignment, etc.) still depend on
them, so they will need equivalent implementations in the Rust port.

-------------------------------------------------------------------------------

## 1. Core data structures

| File | Class | Purpose |
|------|-------|---------|
| `Vector.h` | `RDNumeric::Vector<T>` | 1-D dynamic array with shared ownership (`boost::shared_array<T>`). Provides element-wise ops (+=, *=) and random number helpers. |
| `Matrix.h` | `Matrix<T>` | Rectangular `nRows × nCols` storage in row-major order with bounds-checked accessors and row/column extraction. |
| `SquareMatrix.h` | `SquareMatrix<T>` | Fixed shape `N × N` specialisation of `Matrix`, adds determinant, trace, Frobenius norm, inversion (by LU decomposition) for `double` type. |
| `SymmMatrix.h` | `SymmMatrix<T>` | Symmetric upper-triangle storage (packed) with fast access; used for covariance or Hessian matrices. Includes Cholesky decomposition. |
| `Conrec.h` | Header-only port of the classic **CONREC** contouring algorithm generating isocontour line segments from a 2-D scalar field.  Only employed by the matplotlib-style depiction code. |

-------------------------------------------------------------------------------

## 2. Sub-modules

### 2.1 Alignment

Folder: `Alignment/`

* `AlignPoints.*` – Implements **Kabsch / quaternion** method to align two sets
  of 3-D points. Returns sum-of-squared residuals and transformation matrix.
  Utilised by `Chem.rdMolAlign` and crystallography code.
* `Wrap/` – SWIG interface exposing `AlignPointsToPoints()` to Python.
* `testAlignment.cpp` – C++ correctness unit.

### 2.2 Optimizer

Folder: `Optimizer/`

| File | Class | Notes |
|------|-------|-------|
| `BFGSOpt.*` | `BFGSOpt` | Generic quasi-Newton optimiser template taking a functor that returns energy + gradient; supports line search backtracking. |
| `LinearSearch.cpp` | helper for Wolfe/golden-section line search. |
| `testOptimizer.cpp` | Minimises Rosenbrock function to validate implementation. |

Used by force-field conjugate-gradient when `useBFGS=true`.

### 2.3 EigenSolvers

Folder: `EigenSolvers/`

* `PowerEigenSolver.*` – Basic **power iteration** solver returning dominant
  eigenvalue/vector of a symmetric matrix; falls back to inverse iteration if
  needed.  Employed by `MolAlign/CEAlign` algorithm to align covariance
  matrices.
* `testEigenSolvers.cpp` – verifies accuracy on random matrices.

-------------------------------------------------------------------------------

## 3. Tests in module

* `testMatrices.cpp` – covers determinant, inversion and Cholesky routines for
  `SquareMatrix` and `SymmMatrix`.
* `testConrec.cpp` – visual regression of contouring output.

All tests are Catch2-based.

-------------------------------------------------------------------------------

## 4. Dependencies

• Only needs `RDGeneral` (Invariant, epoch timing) and C++ STL/Boost. No third-party BLAS.

-------------------------------------------------------------------------------

## 5. Rust-port notes

1. **Replace custom math types** with mature crates:
   • `nalgebra` for vectors/matrices (dynamic and fixed).  
   • `ndarray` for N-dim arrays if needed.
2. Alignment → `kabsch` algorithm can be implemented in ~100 LOC using
   `nalgebra::SVD` to obtain optimal rotation; reflection flag handled by sign
   of determinant.
3. Optimiser → reuse `argmin` crate’s BFGS implementation; define trait
   `EnergyGrad` compatible with force-field `System`.
4. Conrec → port existing small header verbatim or substitute with `contour` crate.
5. Eigen solver – available in `nalgebra-lapack` or write simple power
   iteration directly over matrix multiplication.

-------------------------------------------------------------------------------

End of Numerics map.
