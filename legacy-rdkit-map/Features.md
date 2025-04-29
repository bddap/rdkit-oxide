# Features Module – Detailed Map

Location: `rdkit/Code/Features`

This is an **older, header‐only framework** for representing chemical (or more general) *features* with location and direction vectors.  It predates the newer `ChemicalFeatures` module and is largely kept for backward compatibility in certain RDKit sub-projects.

--------------------------------------------------------------------------------

## File overview

| File | Purpose |
|------|---------|
| `Feature.h` | Template definitions of `ExplicitFeature` and `ImplicitFeature` classes inside namespace `RDFeatures`.  Both are header‐only.
| `Feature.cpp` | Empty TU to force compilation when building static library. |
| `testFeatures.cpp` | Small unit test using `RDKit::Invariant` macros. |
| `CMakeLists.txt` | Builds `Features` library (mostly to export headers). |

--------------------------------------------------------------------------------

## Public API (Feature.h)

All templates take three type parameters with defaults:
```
FAMILYMARKER – type for feature family tag   (e.g. std::string)
TYPEMARKER   – type for feature type tag     (default = FAMILYMARKER)
LOCTYPE      – coordinate type               (default RDGeom::Point3D)
```

### 1. `ExplicitFeature<F,T,L>`
Represents a single point feature with optional *direction* vectors.

Data members (all public via getters/setters):
• `F d_family`   – family label.
• `T d_type`     – specific type.
• `L d_loc`      – coordinate.
• `std::vector<L> d_dirs` – zero or more direction vectors.

Operations: trivial constructors, accessors.

### 2. `ImplicitFeature<F,T,L>`
Weighted aggregate of multiple points (e.g. centroid of ring atoms for an aromatic feature).

Additional members:
• `std::vector<double> d_weights` – per-point weights.
• `std::vector<const L*> d_locs`  – pointers to external coordinates.
• `double d_weightSum` – running total.

Method `addPoint(const L* p, double weight)` adds a source point; `getLoc()` returns weighted centroid.  Directions list identical to `ExplicitFeature`.

--------------------------------------------------------------------------------

## Tests

`testFeatures.cpp` simply instantiates template classes with `std::string` and validates accessors; no complex behaviour.

--------------------------------------------------------------------------------

## Dependencies

• `Geometry/point.h` for `RDGeom::Point3D`.  
• `RDGeneral/Invariant.h` in tests only.

No linkage dependencies; templates compile anywhere.

--------------------------------------------------------------------------------

## Rust-port considerations

Given overlap with newer ChemicalFeatures API, we may omit this module in Rust or provide thin adapter types.  If retained:
• Generic `struct ExplicitFeature<F,T,L=Point3>` with `Vec<L>` directions.  
• `struct ImplicitFeature` holding `Vec<(L, f64)>`.

--------------------------------------------------------------------------------

End of module map.
