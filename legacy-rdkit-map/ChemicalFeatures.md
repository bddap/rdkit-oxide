# ChemicalFeatures Module – Detailed Map

Location: `rdkit/Code/ChemicalFeatures`

This module introduces an abstraction for **chemical features** (hydrogen-bond donor/acceptor, hydrophobe, etc.) that may or may not be attached to an RDKit molecule.  The design is intentionally lightweight so that features can be manipulated in pharmacophore-based workflows without loading the full `GraphMol` infrastructure.

The directory builds a C++ library named **`ChemicalFeatures`** (see `CMakeLists.txt`).  Downstream modules – notably `GraphMol/Pharm3D` and the Python wrappers – depend on it.

--------------------------------------------------------------------------------

## File inventory

| File | Purpose |
|------|---------|
| `ChemicalFeature.h` | Declares abstract base interface `ChemicalFeature`.  Header-only. |
| `FreeChemicalFeature.h / .cpp` | Concrete implementation representing a *free-standing* feature (not linked to a molecule).  Implements binary (de)serialization. |
| `testChemicalFeatures.cpp` | C++ unit test of `FreeChemicalFeature` class. |
| `Wrap/*.cpp` | Boost.Python bindings compiling into module `rdChemicalFeatures`.  Out of scope for Rust port, but documented here. |
| `CMakeLists.txt` | Build file. |

--------------------------------------------------------------------------------

## Public symbols

All reside in `namespace ChemicalFeatures`.

### 1. Abstract class `ChemicalFeature`  (ChemicalFeature.h 12-32)

Minimal polymorphic interface, providing getters only:
```
int               getId()      const = 0;
const std::string& getType()   const = 0;   // e.g. "Donor"
const std::string& getFamily() const = 0;   // e.g. "HBond"
RDGeom::Point3D    getPos()    const = 0;   // 3-D coordinates
```

No data members – the interface is *pure*.

### 2. Concrete class `FreeChemicalFeature`  (FreeChemicalFeature.h / .cpp)

Derives publicly from `ChemicalFeature`.  Intended for pharmacophore or site-map features that arise outside a specific molecule.

Data members:
```
int              d_id         // optional user id (default −1)
std::string      d_family     // feature family
std::string      d_type       // specific feature type
RDGeom::Point3D  d_position   // 3-D coordinates
```

Key methods:
• Constructors overloads: full specification, family+pos, default blank, copy, and from pickle string.
• Overrides of all pure virtuals from `ChemicalFeature`.
• Mutators: `setId`, `setFamily`, `setType`, `setPos`.
• Serialization:
  – `std::string toString() const`  → writes a binary pickle with version header `ci_FEAT_VERSION` (0x20).
  – `void initFromString(const std::string&)` → inverse operation; handles legacy v1 pickles (0x10).

Serialization layout (`FreeChemicalFeature.cpp` 22-71):
```
uint32  version  (0x20)
int32   id
uint32  lenFamily (+1 for null)
char[]  family
uint32  lenType (+1)
char[]  type
double  x, y, z
```

### 3. Test program `testChemicalFeatures.cpp`

Exercises construction, getters, copy ctor, and (de)serialization for `FreeChemicalFeature`.
Calls:
• `RDKit::TEST_ASSERT` macros from `RDGeneral/test.h`.

--------------------------------------------------------------------------------

## Boost.Python wrappers (Wrap/)

*Not required for Rust port but briefly documented for completeness:*

• `Wrap/FreeChemicalFeature.cpp` exposes the class to Python, including pickling support (`chemfeat_pickle_suite`).
• `Wrap/rdChemicalFeatures.cpp` defines the module and sub-wrap function.

--------------------------------------------------------------------------------

## Dependencies

• `Geometry/point.h` – RDGeom::Point3D definition.                                     
• `RDGeneral/StreamOps.h`, `Invariant.h`, `utils.h` – stream helpers and assertions.

No reliance on `GraphMol` (except in Python wrapper include).  Therefore this module can compile without the rest of RDKit core.

--------------------------------------------------------------------------------

## Rust-port considerations

1. Represent `Point3D` as `nalgebra::Point3<f64>` or custom struct; ensure `Copy`.
2. Use `serde` with a `bincode` format for pickles, honouring existing byte layout for compatibility if needed.
3. Provide trait `ChemicalFeature` with associated getters; implement `FreeChemicalFeature` struct.
4. Consider using `u32` for version constants and `Option<i32>` for id (None = uninitialised).

--------------------------------------------------------------------------------

End of module map.
