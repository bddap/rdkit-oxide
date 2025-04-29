# GraphMol – Reaction Chemistry Sub-module Map

Location: `rdkit/Code/GraphMol/ChemReactions` (+ helper `ChemTransforms`)

RDKit’s reaction engine allows defining transformations via SMARTS/SMIRKS,
reading legacy MDL RXN, running substructure‐based reacts, enumerating product
libraries, and performing common medicinal‐chemistry functionalisation macros.

This map is split into three parts:
1. **Reaction core types and I/O** (`ChemReactions`).
2. **Enumeration framework** (`ChemReactions/Enumerate`).
3. **Higher‐level convenience transforms** (`ChemTransforms`).

-------------------------------------------------------------------------------

## 1. Reaction core (ChemReactions)

| File | Key classes / functions | Purpose |
|------|-------------------------|---------|
| `Reaction.h / Reaction.cpp` | `ChemicalReaction` | Holds vectors of reactant, agent, product `ROMol` plus atom‐mapping tables, helper API: `runReactants()`, `validate()`, `initialize()` etc. |
| `ReactionPickler.{h,cpp}` | `RxnPickler` | Binary serialisation to string or stream. |
| `PreprocessRxn.{h,cpp}` | `cleanReactant()` etc. | Sanitises reaction inputs: removes mapping from un-reactive atoms, kekulises, sets conjugation flags. |
| `DaylightParser.cpp` | `RxnSmartsToChemicalReaction()` / `RxnSmilesToChemicalReaction()` | Parse reaction SMARTS/SMILES strings following Daylight grammar. |
| `MDLParser.cpp` | `RxnBlockToChemicalReaction()` | Reads V2000/V3000 RXN blocks. |
| `CDXMLParser.cpp`, `MoleculeParser.cpp`, `PNGParser.cpp` | Less common formats, rely on tinyxml2 or RDKit PNG metadata. |
| `validate.{h,cpp}` (part of Reaction.cpp) | Consistency checks on atom‐mapping and stoichiometry. |
| **Tests**: `catch_tests.cpp` covers parsing and runReactants correctness.

### Execution algorithm (`ChemicalReaction::runReactants`)
1. Check reactant count matches definition.
2. For each reactant pattern perform `SubstructMatch` to obtain atom mappings.
3. Combine matches across reactants via Cartesian product; optional
   enumeration‐caps via `maxProducts`.
4. Copy template product molecules (`initialize()`) and apply mapped atom
   property changes (remove atoms, change bonds, etc.).
5. Sanitise products & return vector<ROMOL*>.

Threading: Multi‐reactant enumeration can use OpenMP if compiled with it.

-------------------------------------------------------------------------------

## 2. Enumeration framework (ChemReactions/Enumerate)

Designed to create large virtual libraries without materialising full Cartesian
products in memory.

| Important headers | Concept |
|-------------------|---------|
| `EnumerateBase.h` | Abstract `ReactionEnumeratorBase` exposing `next()` and `size()`. |
| `EnumerateTypes.h` | Policy enums `EnumerationStrategy`, `ReagentChooser`, etc. |
| `Enumerate.h / Enumerate.cpp` | Factory `enumerateReaction()` returning concrete enumerator based on strategy. |
| `CartesianProduct.h` | Naïve exhaustive enumeration strategy. |
| `EvenSamplePairs.{h,cpp}` | Random sampling strategy ensuring even coverage of reactants (uses `boost::random`). |
| `EnumerationPickler.{h,cpp}` | Serialise enumerator state for checkpointing large runs. |

Enumerator returns `std::vector<std::shared_ptr<ROMol>>` products on demand and
can be paused/resumed.

-------------------------------------------------------------------------------

## 3. Medicinal chemistry helpers (ChemTransforms)

| File | Public API | Description |
|------|------------|-------------|
| `ChemTransforms.h / .cpp` | `addFunctionalGroup()`, `replaceCore()`, `deleteSubstructs()` | Wraps a set of predefined SMIRKS or uses `SubstructMatch` + `RWMol` edits to perform common transformations (deprotections, protecting group addition, etc.). |
| `MolFragmenter.{h,cpp}` | `fragmentOnBonds()` | Splits molecules at specified bonds, labels attachment points with dummy atoms. |
| `MolFragmenterJSONParser.{h,cpp}` | Reads JSON describing multiple fragmentation rules. |
| **Tests**: `testChemTransforms.cpp`, python wrappers under `rdkit/Chem` package (outside C++ scope).

-------------------------------------------------------------------------------

## 4. Data flow & relationships

```
Reaction SMARTS --> DaylightParser --> ChemicalReaction
ChemicalReaction::runReactants --> SubstructMatch (GraphMol/Substruct)
SubstructMatch --> MatchVectType list per reactant
CartesianProduct / EvenSamplePairs --> choose combination vector
combination vector + template product → RWMol edits → sanitise → product ROMol
```

-------------------------------------------------------------------------------

## 5. Dependencies

• Core `GraphMol` plus `Substruct` (matching) and `Fingerprints` (optional
  `BondFingerprints` for reagent selection).
• `DataStructs` (`ExplicitBitVect` for reactant caches).
• `Boost::random`, `Boost::serialization` for enumeration pickling.

-------------------------------------------------------------------------------

## 6. Rust-port notes

1. Define `struct ChemicalReaction { reactants: Vec<Molecule>, products: Vec<Molecule>, agents: Vec<Molecule>, mapping: HashMap<AtomId, AtomId>, ... }`.
2. Parsing: leverage `smiles` crate for SMIRKS or port minimal subset.
3. Enumeration uses iterator pattern: `impl Iterator<Item=Vec<Molecule>>` with
   `choose_strategy()` returning boxed trait objects.
4. For performance allow parallel enumeration with `rayon` and optional
   checkpoint serialisation using `bincode`.

-------------------------------------------------------------------------------

End of Reaction Chemistry map.
