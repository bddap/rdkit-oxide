# RDGeneral Module Map

Location: `rdkit/Code/RDGeneral`

RDGeneral is the **utility toolbox** for the entire RDKit project.  It provides
cross-cutting infrastructure that other modules rely on: logging, exception
hierarchy, thread helpers, dictionaries, value/variant types, hash utilities
and assorted small algorithms.  This document highlights the most important
parts from a portability standpoint.

-------------------------------------------------------------------------------

## 1. Core headers

| File | Purpose |
|------|---------|
| `Invariant.h / Invariant.cpp` | `PRECONDITION`, `CHECK_INVARIANT` run-time assertions; if they fail they throw `Invariant Violation` exception. |
| `Exceptions.h` | Base class `RDException` plus many specialised subclasses (`IndexError`, `ConformerException`, …). |
| `BadFileException.h`, `FileParseException.h` | Frequently thrown by file parsers. |
| `RDProps.h` | Generic property map (`std::map<std::string, RDAny>`) with setter/getter templates. Adopted by Atom, Bond, ROMol, Conformer, etc. |
| `RDAny.h` / `RDValue*.h` | Small-footprint type-erased value container predating `std::any` (2003). RDValue uses tagged-union implementation optimised for PODs. |
| `Dict.h` | Lightweight dictionary wrapper used by SWIG to expose property maps as Python dict-like objects. Supports iteration and pickling. |
| `utils.h / utils.cpp` | Convenience functions `tokenize()`, `strip()`, `lowercase()`, etc. |
| `types.h / types.cpp` | RDKit-wide typedefs (`INT_VECT`, `DOUBLE_VECT`, etc.) and helper `makeVector`, `makeList` functions. |

-------------------------------------------------------------------------------

## 2. Logging subsystem (`RDLog.*`)

Basis for `rdkit.rdBase._rdkit.*` Python logger.

• Implements `rdLogger` class (wrapping ostream) that can optionally **tee** to
  an extra stream or file.  
• Global singletons: `rdAppLog`, `rdInfoLog`, `rdDebugLog`, `rdErrorLog`.  
• Macros `BOOST_LOG(rdInfoLog) << "message" << std::endl;` used throughout the
  codebase.  Log levels can be enabled/disabled via `enable_logs("rdDebug")`.

-------------------------------------------------------------------------------

## 3. Thread and concurrency helpers

| File | Contents |
|------|----------|
| `RDThreads.h` | Abstraction layer mapping to *OpenMP*, *C++17 std::thread*, or single-threaded stubs depending on compile flags.  Provides `RD_THREAD_LOCAL` macro, `parallel_for` wrapper. |
| `ConcurrentQueue.h` | Lock-free multi-producer multi-consumer queue (Michael/Scott) with optional thread-safe iterator; used by multithreaded fingerprinting and minimisation. |
| `testConcurrentQueue.cpp` – unit tests. |

-------------------------------------------------------------------------------

## 4. Misc helpers

* `BetterEnums.h` – compile-time enum introspection macro for pretty printing.
* `ControlCHandler.h` – sets custom SIGINT handler that raises C++ exception so
  long-running loops can break cleanly.
* `LocaleSwitcher.*` – RAII helper that temporarily sets numeric locale to
  “C” during file parsing (comma decimal separators cause havoc otherwise).
* `hash/` – Single-header portable hashing utilities (fallback when `std::hash` is insufficient for floats); used by `SparseIntVect`.
* `hanoiSort.h` – Variation of stack-based topological sort used by depiction.
* `Ranking.h` – Template that maintains top-N elements with custom comparator
  (min-heap wrapper).  Utilised by scaffold network.

-------------------------------------------------------------------------------

## 5. Catch2 regression tests

* `catch_dict.cpp`, `catch_logs.cpp`, `testRDValue.cpp`, `testConcurrentQueue.cpp`, `testDict.cpp`

-------------------------------------------------------------------------------

## 6. Dependencies

Relies only on STL, Boost headers, and (optionally) OpenMP.  No external libs.

-------------------------------------------------------------------------------

## 7. Rust-port notes

1. **Logging** → Use `tracing` crate with features `log`, `env-logger`.
2. **RDAny/RDValue** → Rust has `enum RDAny { Bool(bool), Int(i32), Float(f64), String(String), Boxed(Arc<dyn Any>) }`.  Or rely on `serde_json::Value` for generic property but keep typed getters.
3. **ConcurrentQueue** → replace with `crossbeam::queue::SegQueue` or `tokio::sync::mpsc` depending on async.
4. **Invariant** → `debug_assert!` or custom `ensure!()` macro returning `Result`.
5. **Thread abstraction** unnecessary thanks to Rust’s `rayon` and ownership.

-------------------------------------------------------------------------------

End of RDGeneral map.
