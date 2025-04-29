# RDKit Unit Test Catalogue

This document is being built as part of *Stage 2.5* and will eventually cover
**every C++ and Python unit test** in the RDKit repository.  For each test file
we record its location, a short description of what is being verified, and
line-number anchors for individual `TEST_CASE`s (Catch2) or `TEST_ASSERT`
blocks.  The goal is to provide a one-stop mapping that can be used when porting
tests to Rust.

------------------------------------------------------------------------------

## RDGeneral

| File | Test blocks (line) | Purpose |
|------|--------------------|---------|
| `RDGeneral/catch_logs.cpp` | `TEST_CASE("LogStateSetter", 17)`<br>`TEST_CASE("GitHub Issue #5172", 69)`<br>`TEST_CASE("Tee to file", 117)` | Verifies rdLogger enable/disable logic, log-status helper, and tee-to-file behaviour. Ensures no log messages are emitted when the corresponding level is disabled and that tee creates the expected file. |
| `RDGeneral/catch_dict.cpp` | `TEST_CASE("Dict move semantics", 16)`<br>`TEST_CASE("RDProps move semantics", 45)` | Checks that `Dict` and `RDProps` remain valid after move construction / move assignment (regression for #5044). |
| `RDGeneral/testConcurrentQueue.cpp` | `testPushAndPop()` (31) – single-thread queue operations.<br>`testProducerConsumer()` (55) – producer/consumer with varying thread counts.<br>`testMultipleTimes()` (95) – stress-test 10 000 trials. | Validates lock-free `ConcurrentQueue` correctness under single and multi-threaded conditions. Compile-time guarded by `RDK_BUILD_THREADSAFE_SSS` and `RDK_TEST_MULTITHREADED`. |
| `RDGeneral/testDict.cpp` | main() function runs a series of `TEST_ASSERT` checks on `Dict` insert, erase, copy, and iterator behaviour. | Legacy pre-Catch unit executing via `testDict` executable. |
| `RDGeneral/testRDValue.cpp` | Tests implicit conversions, comparison operators, copy/move semantics, and serialisation of `RDValue`/`RDAny`. | Ensures variant type stores bool/int/float/string without loss and round-trips through `operator<<`. |

*Note*: `catch_main.cpp` only provides the Catch2 `main()` entry point and contains no tests itself.

------------------------------------------------------------------------------

Pending sections: DataStructs, Geometry/DistGeom, ForceField, GraphMol, SimDivPickers, Numerics, Misc.
Completed: RDGeneral, DataStructs

------------------------------------------------------------------------------

## Geometry & DistGeom

| File | Key tests | Purpose |
|------|-----------|---------|
| `Geometry/catch_tests.cpp` | `TEST_CASE("construct Point2D from Point3D")` – verifies 2D projection ctor.<br>`TEST_CASE("UniformGrid getGridIndex")` – bounds checking and dimension calc.<br>`TEST_CASE("UniformGrid copying")` – copy/serialise UniformGrid.<br>`TEST_CASE("UniformGrid get/setVal")` – occupancy updates and totals. | Ensures geometry primitives (`Point`, `UniformGrid3D`) behave correctly and serialise round‐trip. |
| `Geometry/testGrid.cpp` | Functions `test1()` – insert spheres into `UniformGrid3D` and compare occupancy.<br>Main aggregates tests. | Early regression tests predating Catch; focuses on grid neighbour enumeration accuracy. |
| `Geometry/testRealValueGrid.cpp` | Similar to above but for `UniformRealValueGrid3D` storing double values; checks interpolation and point distance weight. |
| `Geometry/testTransforms.cpp` | Multiple `TEST_ASSERT` blocks cover quaternion rotation matrices, axis–angle conversions, 2D/3D transform composition, and inversion accuracy. | Verifies `Transform3D` math. |
| `DistGeom/testDistGeom.cpp` | `test1()` – triangle smoothing of bounds matrix with analytical expectations.<br>`testIssue216()` – reproduces bug #216 ensuring initial coordinate generation from symmetric matrix yields unit edge lengths.<br>Main() runs and prints via RDLog. | Core validation of distance-geometry numeric routines (`BoundsMatrix`, `computeInitialCoords`). |

Pending sections: ForceField, GraphMol, SimDivPickers, Numerics, Misc.
------------------------------------------------------------------------------

## ForceField

| File | Key tests | Purpose |
|------|-----------|---------|
| `ForceField/catch_tests.cpp` | `TEST_CASE("Test DistanceConstraintContribs")` – absolute and relative distance constraints converge to expected values.<br>`TEST_CASE("Test AngleConstraintContribs")` – verify angle constraints and RMS convergence (lines ~60–140). | Generic constraint contrib tests using mini propane molecule; exercises force‐field creation via `FFConvenience`. |
| `ForceField/UFF/testUFFForceField.cpp` | `test1()` – low-level ForceField API distances, angle calculation.<br>`testUFFBuilder()` – build UFF force field for benzene, run minimization, check energy (lines ~200).<br>`testUFFConstraints()` – position, torsion, distance constraints (≥400). | Regression coverage for UFF energy terms and builder utilities. |
| `ForceField/MMFF/testMMFFForceField.cpp` | `testBasics()` – parameter loading from CSV, atom type assignments.<br>`testOptimization()` – minimize chloroethane and compare final energy to expected.<br>`testMultiThread()` – ensure thread-safety by optimizing 50 molecules in parallel (guarded by `RDK_THREADSAFE_SSS`). | Mirrors UFF tests but for MMFF94; ensures variant parameter selection (94 vs 94s) yields correct energies. |

Pending sections: GraphMol, SimDivPickers, Numerics, Misc.


------------------------------------------------------------------------------

## DataStructs

| File | Key tests (line) | Purpose |
|------|------------------|---------|
| `DataStructs/catch_tests.cpp` | `TEST_CASE("special cases for the limits of sparse vectors", 17)` | Regression for 32-bit overflow in `SparseBitVect` when size==UINT_MAX. Ensures `setBit`/`getBit` behave at numeric limits. |
| `DataStructs/testDatastructs.cpp` | Functions `Test<ExplicitBitVect>()`, `Test<SparseBitVect>()` (≈40-240) – generic bit-vector CRUD, logical ops, streaming round-trip.<br>`testBase64()` (~310) – verify base64 encode/decode of pickled vectors.<br>`testDiscreteValueVect()` (~420) – integer vector math, distance, serialization.<br>`testRealValueVect()` (~580) – floating-point vector math with tolerance.<br>Main() runs all and reports via RDLog. | Comprehensive legacy test covering *all* vector types, base64 helpers, and exception handling. |
| `DataStructs/testFPB.cpp` | `TEST_CASE("FPB reader basic", 25)` – load `.fpb` file (`test1.bin`) and query popcount/dim.<br>`TEST_CASE("FPB sub-byte bits", 70)` – regression for bit count not multiple of 8. | Validates fixed-binary fingerprint container format and random access. |
| `DataStructs/testMultiFPB.cpp` | `TEST_CASE("multi‐reader iterate", 23)` – iterate over multiple FPB shards.<br>`TEST_CASE("multi‐reader hasIndex", 56)` – lookups by fp index and pattern id. | Tests `MultiFPBReader` aggregation layer. |
| `DataStructs/Wrap/testBV.py` | `TestPickle`, `TestBitMath` classes – Python wrapper round-trip and operator overloads. |
| `DataStructs/Wrap/testDiscreteValueVect.py` | `TestDVV` – sum, scalar mult, chi-square distance via SWIG interface. |
| `DataStructs/Wrap/testRealValueVect.py` | `TestRVV` – same for real-valued vectors. |
| `DataStructs/Wrap/testFPB.py` | `TestFPBReader` – ensure Python API sees same fingerprints/metadata as C++. |

------------------------------------------------------------------------------

