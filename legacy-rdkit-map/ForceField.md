# ForceField Module – Detailed Map

Location: `rdkit/Code/ForceField`

The **ForceField** subsystem provides RDKit’s internal molecular mechanics engine and the generic infrastructure used by `DistGeom` for post‐embedding optimisation.  Two parameterisations are bundled: *UFF* (Universal Force Field) and *MMFF94* (Merck Molecular Force Field).  The code is organised as follows:

```
ForceField/
  ├── ForceField.{h,cpp}            // core engine (atoms, coords, minimiser)
  ├── Contrib.h                     // base class for energy term contributors
  ├── *Constraint* files            // distance, angle, torsion, position
  ├── UFF/                          // UFF-specific contribs + parameters
  ├── MMFF/                         // MMFF-specific contribs + parameters
  ├── Wrap/                         // Boost.Python bindings
  └── catch_tests.cpp               // Catch2 unit tests
```

--------------------------------------------------------------------------------

## 1. Core engine

### 1.1 `class ForceField` (ForceField.h/cpp)
• Stores a list of `ForceFieldContrib*` objects (energy terms).  
• Manages atom coordinates (pointer to external `double xyz[]` or internal `std::vector<RDGeom::Point3D>`).  
• Provides API: `initialize()`, `calcEnergy()`, `calcGrad()`, `minimize(maxIts,tol)`, `addContrib()`, etc.  
• Uses **steepest‐descent** and **conjugate‐gradient** minimisers; steps limited by `d_stepSize`.

### 1.2 `class ForceFieldContrib` and derived **Constraint** term classes

Located in root directory:
* `DistanceConstraint`, `AngleConstraint`, `TorsionConstraint`, `PositionConstraint` each implement a harmonic penalty enforcing a geometric restraint; used by *Embedding* module.

Each has `.h/.cpp` plus a helper container file (`DistanceConstraints.cpp` collects functions to create many at once).

API pattern:
```
void getEnergy(double *pos, double &energy) const override;
void getGrad(double *pos, double *grad)   const override;
```

--------------------------------------------------------------------------------

## 2. Parameterised force fields

### 2.1 UFF (Universal Force Field) – `ForceField/UFF/`

Files:
• `BondStretch.*`, `AngleBend.*`, `Torsion.*`, `Nonbonded.*`, `Inversion.*` (for planar), and corresponding constraint headers.  
• `Contribs.h` aggregates all UFF contributor creation helpers.  
• Parameter data derived from original UFF paper, hard-coded in tables within cpp files.

### 2.2 MMFF94 – `ForceField/MMFF/`

Analogous set of contribs: bond, angle, stretch-bend, out-of-plane, torsion, improper, vdW, electrostatics.  Parameters are stored in generated tables (see `Params.cpp/h` not included here) pulled from Merck’s publication.

Both sub-modules expose helper functions:
```
UFF::constructForceField(const ROMol&, bool vdwScale, double confId)
MMFF::constructForceField(const ROMol&, const MMFFMolProperties&, ...)
```

--------------------------------------------------------------------------------

## 3. Boost.Python wrapper (Wrap/)

Exports:
* `UFFGetMoleculeForceField()` / `UFFOptimizeMolecule()`
* `MMFFGetMoleculeForceField()` / `MMFFOptimizeMolecule()`
* Classes `ForceField`, `ForceFieldContrib` exposed read-only.

Unit tests for wrapper live under `rdkit.ForceField` Python package (not in this C++ tree).

--------------------------------------------------------------------------------

## 4. Tests

* `catch_tests.cpp` – Catch2 tests covering minimiser convergence, constraint energy gradients, and a few regression geometries.

--------------------------------------------------------------------------------

## Dependencies

• `Geometry` (Point3D, transforms)  
• `Numerics` for matrix math  
• `GraphMol` for molecule typing and parameter assignment  
• `RDGeneral` for invariants, logging, stream ops.

--------------------------------------------------------------------------------

## Rust-port notes

1. Define trait `Contrib` with `energy(&coords) -> f64` and `grad(&coords, &mut grads)`; have sub-modules `uff`, `mmff` implement structs for each term.
2. Use `nalgebra::DVector<f64>` for coordinate array; provide slice view for performance.
3. Minimiser: implement conjugate-gradient or port existing algorithms from `argmin` crate.
4. Parameter tables can be generated into `phf` maps or plain arrays at build time.
5. Constraint classes will be reused by `DistGeom` to respect chiral/distance restraints, so keep them generic.

--------------------------------------------------------------------------------

End of module map.
