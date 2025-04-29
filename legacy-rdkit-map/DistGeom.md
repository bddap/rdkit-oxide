# DistGeom Module – Detailed Map

Location: `rdkit/Code/DistGeom`

The **DistGeom** subsystem implements RDKit’s *distance‐geometry conformer generator*: it starts from a bounds matrix of atom–atom distance intervals, applies triangle smoothing and chiral/planar constraints, then runs an *Embedder* (metric‐MDS + random perturbation) to generate 3-D coordinates which are later refined by the ForceField module.

--------------------------------------------------------------------------------

## File inventory

| File | Purpose |
|------|---------|
| `BoundsMatrix.h` | Lightweight wrapper over `RDNumeric::SquareMatrix<double>` storing lower/upper distance bounds; provides helper accessors and validity checks. |
| `TriangleSmooth.{h,cpp}` | Implements triangle‐inequality propagation algorithm that tightens bounds (core of distance‐geometry). |
| `ChiralSet.h` | Data structure to encode tetrahedral chirality and cis–trans constraints for the triangle smoother. |
| `DistGeomUtils.{h,cpp}` | High-level utility functions: building initial bounds matrix from an RDKit molecule, adding distance constraints, choosing embedding dimension, calling eigendecomp to produce coordinates. |
| `ChiralViolationContribs.{h,cpp}` | Force-field form contributions penalising chiral‐constraint violations during post-embedding optimisation. |
| `DistViolationContribs.{h,cpp}` | Similar contributions for generic distance violations. |
| `FourthDimContribs.h` | Adds pseudo 4-D potential to help optimisation escape local minima. |
| `testDistGeom.cpp` | C++ test verifying triangle smoothing and embedding.
| `Wrap/` | Boost.Python wrappers exposing `rdDistGeom` module (create bounds matrix, `EmbedMolecule`, etc.). |
| `CMakeLists.txt` | Build targets for library and wrapper. |

--------------------------------------------------------------------------------

## Key public symbols (namespace DistGeom)

1. **`class BoundsMatrix`**  
   – Inherits `RDNumeric::SquareMatrix<double>` (N×N).  
   – Stores lower bounds in lower triangle and upper bounds in upper triangle.  
   – Methods: `getUpperBound`, `setUpperBound{IfBetter}`, `getLowerBound`, `setLowerBound{IfBetter}`, `checkValid`.  
   – Alias: `BoundsMatPtr` = `boost::shared_ptr<BoundsMatrix>`.

2. **`triangleSmooth()`** (TriangleSmooth.cpp)  
   – Propagates bounds via triangle inequality until convergence or max iterations.  
   – Overloads support optional chiral constraint list (`ChiralSet` vector).

3. **`pickRandomCoords()` / `computeInitialCoords()`** (DistGeomUtils.cpp)  
   – Use metric multidimensional scaling (MDS) on tightened bounds to produce 3-D coordinates (or 4-D when using fourth-dimension trick).

4. **`EmbedMolecule()` / `EmbedMultipleConfs()`** (wrapper-facing in DistGeomUtils.h)  
   – High-level orchestrators: build bounds from `ROMol`, smooth, embed, optionally minimize with UFF/MMFF.

5. Force-field contribution classes (`ChiralViolationContrib`, `DistViolationContrib`, `FourthDimContrib`) derive from `ForceFields::ForceFieldContrib` and add penalty terms while optimising coordinates.

--------------------------------------------------------------------------------

## Control/data flow

```
ROMol → constructBoundsMatrix()          // uses covalent radii + ring templates
       → TriangleSmooth (+chiral sets)
       → pickRandomCoords (MDS → coords)
       → ForceField (UFF/MMFF + Contribs)
       → Conformer object stored on ROMol
```

--------------------------------------------------------------------------------

## Dependencies

• `Geometry` and `Numerics` for matrices, eigen decomposition, random numbers.  
• `GraphMol` for molecule graph and stereochemistry info.  
• `ForceField` when doing post-embedding minimisation.

--------------------------------------------------------------------------------

## Rust-port notes

1. Use `nalgebra::DMatrix<f64>` or `ndarray::Array2` for bounds matrix; store lower/upper halves compactly.
2. Triangle smoothing can be implemented as iterative relaxation; consider using `rayon` for parallel updates.
3. Eigen decomposition / MDS: rely on `nalgebra-lapack` or `ndarray-linalg`.
4. Force-field contributions will map to traits in the forthcoming `forcefield` crate.
5. Bindings should return `Vec<[f64;3]>` per conformer.

--------------------------------------------------------------------------------

End of module map.
