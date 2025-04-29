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
