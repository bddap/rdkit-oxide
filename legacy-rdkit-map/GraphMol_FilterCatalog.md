# GraphMol – FilterCatalog Sub-module Map

Location: `rdkit/Code/GraphMol/FilterCatalog`

The **FilterCatalog** framework provides rule-based filtering of molecules for
library preparation, PAINS removal, Lipinski enforcement, etc. A filter is
defined by a `FilterCatalogEntry` consisting of a SMARTS pattern, an optional
callback, and metadata (name, description, family). `FilterCatalog` is a
container of entries that can be queried against a molecule, returning a list
of matches.

-------------------------------------------------------------------------------

## 1. Public classes (headers)

| Header | Class | Purpose |
|--------|-------|---------|
| `FilterCatalog.h` | `FilterCatalog` | Holds `std::vector<shared_ptr<FilterCatalogEntry>>`, provides `addEntry()`, `removeEntry()`, `getMatches(const ROMol&)`. |
| `FilterCatalogEntry.h` | `FilterCatalogEntry` | Stores SMARTS, `FilterMatcherBase` pointer, metadata, severity score. |
| `FilterMatcherBase.h` | `FilterMatcherBase` | Abstract base with `getMatches(const ROMol&)` returning `FilterMatch` structs. |
| `FilterMatchers.h` | `SmartsMatcher`, `RecursiveSmartsMatcher`, `FuncGroupMatcher` | Concrete matchers; main implementation in `FilterMatchers.cpp`. |

Supporting types:
* `FilterMatch` – struct containing `entryIdx`, `bitId`, list of `MatchVectType`.
* `CatalogEntryIdx` (unsigned int).

-------------------------------------------------------------------------------

## 2. Entry creation utilities (`Filters.cpp`)

The file exposes helper factories generating ready-made catalogues:

* `FilterCatalogParams PAINS_SmartsCatalogParams()` – loads PAINS A/B/C SMARTS.
* `FilterCatalogParams LipinskiCatalogParams()` – H-bond donor/acceptor rules.
* `FilterCatalogParams BRENKCatalogParams()` – problematic FG list.
* Each factory reads bundled TSV/CSV in `data/` sub-directory (parsed via
  `boost::tokenizer`).

Users build catalogue via:

```cpp
auto params = FilterCatalogParams::PAINS_SmartsCatalogParams();
FilterCatalog catalog(params);
```

`FilterCatalogParams` includes flags: `logicalCombination` (AND/OR) and
`recursive` to choose matcher type.

-------------------------------------------------------------------------------

## 3. Matcher implementation highlights

### `SmartsMatcher`
• Stores compiled query `ROMol` for each SMARTS.  
• `getMatches()` calls `SubstructMatch`; if any match, returns `FilterMatch`.

### `RecursiveSmartsMatcher`
• Handles SMARTS with recursive `$()` constructs; expands once and caches.  
• Uses `QueryOps.h` utilities.

### `FuncGroupMatcher`
• Recognises functional groups defined by name (e.g., *aldehyde*, *carboxylic acid*) using internal SMARTS table from `rdkit/Chem/FuncGroups.py`. The C++ side only stores group ID lists; actual SMARTS resolved in Python wrapper.

-------------------------------------------------------------------------------

## 4. Runner

`FilterCatalogRunner.cpp` defines helper function:

```cpp
std::vector<FilterMatch> getFilterMatches(const ROMol&, const FilterCatalog&);
```

It loops through entries, accumulates matches, and can short-circuit when
`logicalCombination == CatalogLogic::ANY` and a first match is found.

-------------------------------------------------------------------------------

## 5. Tests

* C++: `catch_tests.cpp` – validates PAINS catalogue, recursive SMARTS, logical
  combinations, serialization.
* Python (outside) has `rdkit.Chem.FilterCatalog` tests with PAINS examples.

-------------------------------------------------------------------------------

## 6. Dependencies

• `GraphMol/Substruct` for matching.  
• `DataStructs` for bitset indexes when exposing catalogue as fingerprint.  
• `RDGeneral/FileParseException` for data load errors.

-------------------------------------------------------------------------------

## 7. Rust-port notes

1. Represent `Catalog` as `Vec<Entry>` where `Entry` stores compiled SMARTS via
   `chemquery` AST and metadata.  
2. Provide `Matcher` trait with `fn matches(&self, mol: &Molecule) -> Option<MatchResult>`.
3. Preload built-in PAINS/Toxic/Fragment rules with `include_str!` at compile
   time; parse into `Entry` objects.

-------------------------------------------------------------------------------

End of FilterCatalog map.
