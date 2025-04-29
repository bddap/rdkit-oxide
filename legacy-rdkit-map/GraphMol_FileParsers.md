# GraphMol – FileParsers Sub-module Map

Location: `rdkit/Code/GraphMol/FileParsers`

The FileParsers package collects **I/O support** for a wide variety of molecular file formats.  Suppliers act as iterators yielding RDKit molecules, while Writers (or “streams”) output molecules back to disk.  The directory also contains utilities and a modern *v2* API based on polymorphic `GeneralMolSupplier`/`GeneralMolWriter` base classes.

--------------------------------------------------------------------------------

## 1. Utilities and dispatch layer

* `FileParserUtils.h` – common helpers (string trimming, error macros, option flags like `sanitize=true`, `removeHs` etc.).
* `FileParsers.h` – convenience front-ends `MolFromSmiles()`, `MolFromMolBlock()`, `SmilesToMol()` etc. that choose appropriate parser.
* `FileWriters.h` – mirror of above for writing (`MolToSmiles()`, `MolToMolBlock()` etc.).
* `GeneralFileReader.h` – detects file type from extension and returns appropriate Supplier; used by v2 code.

--------------------------------------------------------------------------------

## 2. Suppliers (readers)

| File | Formats | Notes |
|------|---------|-------|
| `SmilesMolSupplier.cpp` | SMILES, including CSV/TSV with column selection | Streaming, memory-efficient, supports multithread chunking. |
| `SDFMolSupplier.cpp` | MDL SD-file | Random-access variant via `ForwardSDMolSupplier`. |
| `ForwardSDMolSupplier.cpp` | Forward-only SD iterator for low memory. |
| `PDBParser.cpp` (outside but included) | PDB, with bonding heuristics. |
| `TDTMolSupplier.cpp` | Tripos TDT. |
| `MaeMolSupplier.cpp` | Schrodinger MAE. |
| `CDXMLParser.cpp` | ChemDraw CDXML; uses tinyxml2. |
| `ProximityPickingMolSupplier.cpp` | Sequence (.fasta) to peptide builders, etc. |

Suppliers inherit from `MolSupplier` interface with methods:
```
bool atEnd() const;
ROMol *next();            // transfers ownership
void reset();              // rewind
```

Many also expose `moveTo(id)` for random access and `setProcessPrivateProps()` toggles.

--------------------------------------------------------------------------------

## 3. Writers

| File | Output | Key options |
|------|--------|-------------|
| `SmilesWriter.cpp` | SMILES/CSV | `isomericSmiles`, `kekuleSmiles`, include header. |
| `SDFWriter.cpp` | SDF/MOL | block properties, V2k vs V3k blocks. |
| `PDBWriter.cpp` | PDB | conformer → multiple MODEL records. |
| `TDTWriter.cpp` | TDT |

All derive from `MolWriter` interface (`write(const ROMol&)`, `flush()`, `close()`).

--------------------------------------------------------------------------------

## 4. Version-2 API

Files ending with `_v2` (e.g. `v2_file_parsers_catch.cpp`, `GeneralMolSupplier.h`) showcase a modernised design:
```
class GeneralMolSupplier { virtual std::unique_ptr<ROMol> next() = 0; };
class SmilesMolSupplierV2 : public GeneralMolSupplier { ... };
```
They support multi-threaded reading via `ThreadedMolSupplier` template and optional callback error handling.

--------------------------------------------------------------------------------

## 5. Tests

• `testMolSupplier.cpp`, `testMolWriter.cpp`, `testMultithreadedMolSupplier.cpp`, `v2_file_parsers_catch.cpp`, `v2_suppliers_catch.cpp`, plus data under `test_data/`.  These cover round-trip parsing, property handling, CTAB V2000 vs V3000, squiggle double bonds, etc.

--------------------------------------------------------------------------------

## Dependencies

• Core GraphMol classes, `DataStructs` for lazy property caching, `RDGeneral/FileParseException`.  
• External libs: zlib for gzipped SD files, tinyxml2 for CDXML, Schrodinger’s reader code under permissive license.

--------------------------------------------------------------------------------

## Rust-port strategy notes

1. Use `chemcore` crate’s SMILES and Molfile parser if available; otherwise port RDKit’s state machine.  
2. Follow iterator pattern with `impl Iterator<Item=Result<Molecule>>`.
3. For gzip/streaming rely on `flate2` and `bufreader`.

--------------------------------------------------------------------------------

End of FileParsers map.
