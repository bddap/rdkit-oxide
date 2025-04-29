# Geometry Module – Detailed Map

Location: `rdkit/Code/Geometry`

Geometry is a *tiny linear‐algebra helper layer* offering points, vectors, rigid‐body transforms, and simple 3-D grids.  It intentionally avoids heavyweight dependencies so that low‐level modules (DataStructs, DistGeom, ForceField) can compile even when Eigen is not available.

--------------------------------------------------------------------------------

## File catalogue

| File | Description |
|------|-------------|
| `point.h / point.cpp` | Templated `Point2D`, `Point3D`, `PointND` classes with arithmetic operators, distance helpers, normalization.  `Point3D` is widely used across RDKit. |
| `Vector.h` (in Numerics) is included but Geometry re-exports convenience typedefs. |
| `Transform.h` | Enumerations and base helpers shared by `Transform2D` and `Transform3D`.  Defines `AxisType` enum. |
| `Transform2D.{h,cpp}` | 3×3 homogeneous coordinate rigid transform (rotation + translation) for 2-D coordinates. |
| `Transform3D.{h,cpp}` | 4×4 homogeneous transform for 3-D: setters for rotation around axis, quaternion conversion, reflection, composition. |
| `Grid3D.h` | Abstract base for axis-aligned 3-D voxel grid with bounds and resolution. |
| `UniformGrid3D.cpp / UniformRealValueGrid3D.h` | Concrete integer and floating-point grid implementations supporting sphere queries, neighbour iteration. Widely used by pharmacophore and shape alignment code. |
| `GridUtils.{h,cpp}` | Helper functions: `getGridPointIndex`, `sphereNeighbors`, `closestIndices` etc. |
| `Utils.h` | Misc mathematical helpers (`angleDeg`, `signedAngle`, proportional clamps). |
| `catch_tests.cpp` | Comprehensive tests of transforms, point algebra, and grid queries. |
| `Wrap/` | Boost.Python module `rdGeometry` exposing `Point3D`, `Transform3D`, and grids. |

--------------------------------------------------------------------------------

## Highlighted public symbols

1. `RDGeom::Point3D` (point.h)  
   – Plain‐old‐data struct `{ double x,y,z; }` with arithmetic operators, dot/cross products, `normalize()`, `length()`. Implements `virtual Point` interface for polymorphism.

2. `RDGeom::Transform3D`  
   – Inherits `RDNumeric::SquareMatrix<double>` sized 4×4. Provides `SetTranslation`, `SetRotation(angle, axis)`, `operator*` overloads for point and transform composition.

3. Grid hierarchy  
   • `Grid3D` abstract template (voxel size, occupancy bitset).  
   • `UniformGrid3D` (bool occupancy) and `UniformRealValueGrid3D` (double values) implement neighbour search used by `ShapeMatchers` module.

--------------------------------------------------------------------------------

## Dependencies

• `Numerics` for `SquareMatrix` and `Vector` templates.  
• `boost::shared_ptr` for grid storage.  
• No reliance on `GraphMol`.

--------------------------------------------------------------------------------

## Rust-port notes

1. Map `Point3D` → `[f64;3]` or `nalgebra::Point3<f64>` with `Copy` semantics.
2. Provide `Transform3D` wrapper around `nalgebra::Matrix4<f64>`; implement builder methods.
3. 3-D grids can use `Vec<T>` linear storage; sphere neighbour query implemented with pre-computed offsets.

--------------------------------------------------------------------------------

End of module map.
