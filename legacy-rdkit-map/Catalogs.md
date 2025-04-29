# Catalogs Module – Detailed Map

Location: `rdkit/Code/Catalogs`

The *Catalogs* subsystem supplies a **generic, hierarchical catalogue container** that other modules (e.g., fragment catalogues, pharmacophore feature catalogues) specialise.  It is intentionally template-based and completely agnostic to chemical concepts; the *real* chemistry is supplied by the entry/parameter types plugged in by higher-level code.

Files in this directory build a static/shared library named **`Catalogs`** (see `CMakeLists.txt`).  The library depends only on `RDGeneral` and the C++ standard library.

---

## File-level breakdown

| File | Purpose |
|------|---------|
| `Catalog.h` (≈ 40–330) | Declares two key templates: `RDCatalog::Catalog<entryType, paramType>` and `RDCatalog::HierarchCatalog<entryType, paramType, orderType>`.  The header contains all logic – the companion `Catalog.cpp` is an empty TU to force template instantiation when building the library. |
| `Catalog.cpp` | Empty implementation file that simply `#include`s `Catalog.h`. |
| `CatalogEntry.h / .cpp` | Abstract base class for concrete catalogue entries.  Tracks a *bit id* integer and defines a required serialization interface. |
| `CatalogParams.h / .cpp` | Abstract base class for *parameter objects* used to configure a catalogue at construction time.  Carries a free-form `typeStr` and the (de)serialization interface. |
| `CMakeLists.txt` | Builds the `Catalogs` library and installs the public headers to `include/rdkit/Catalogs/`. |

Line-number references below come from RDKit 2023.09 on commit hash `…` (current workspace).

---

## Public symbols

### Namespace `RDCatalog`

1. `versionMajor`, `versionMinor`, `versionPatch`, `endianId` (Catalog.h: 15-18)
   • Build-time constants written into pickles for compatibility checks.

2. **`template<class E, class P> class Catalog`**  (Catalog.h: 39-131)
   • Abstract base for all catalogue containers.
   • Owns:
     – `unsigned int d_fpLength`: next *bit id* to assign / total fingerprint length.
     – `P* dp_cParams`: deep-copied parameter object (nullable until first `setCatalogParams`).
   • Key virtuals (pure):
     – `std::string Serialize() const` – must produce a portable binary pickle.
     – `unsigned int addEntry(E*, bool updateFpLength=true)` – add entry instance.
     – `const E* getEntryWithIdx(unsigned int) const` – index-based lookup.
     – `unsigned int getNumEntries() const` – entry count.
   • Non-virtual helpers: fingerprint length getters/setters and `setCatalogParams`.

3. **`template<class E, class P, class O> class HierarchCatalog`**  (Catalog.h: 135-327)
   Extends `Catalog` with a **directed graph** (Boost.Graph) that expresses parent/child relationships between entries.  Provides order-based queries.

   Internal types:
   • `vertex_entry_t` (tag for Boost property map).
   • `EntryProperty`, `CatalogGraph`, `*_ITER` aliases for BGL traits.

   Key operations:
   • Construction from params or a *pickle* (binary blob).
   • `toStream` / `Serialize` – Full binary output containing header, params, entries, and adjacency lists.
   • `initFromStream` / `initFromString` – Inverse of above.
   • `addEntry` – Inserts vertex, updates order map, assigns bit id if requested.
   • `addEdge` – Adds directional edge while avoiding duplicates.
   • Query helpers: `getEntryWithBitId`, `getIdOfEntryWithBitId`, `getDownEntryList`, `getEntriesOfOrder`.

   Data members:
   • `CatalogGraph d_graph` – Boost adjacency_list of entries.
   • `std::map<O, INT_VECT> d_orderMap` – Fast lookup of vertices by order.

4. **`class CatalogEntry`**  (CatalogEntry.h: 20-39)
   • Abstract polymorphic base for any data element stored in a catalogue.
   • Maintains `int d_bitId` (can be −1 for uninitialised).
   • Pure-virtual interface:
     – `std::string getDescription() const` – human-readable summary.
     – `void toStream(std::ostream&) const` / `std::string Serialize() const` / `initFrom*` – binary (de)serialization.

5. **`class CatalogParams`**  (CatalogParams.h: 18-44)
   • Abstract base holding catalogue-wide parameters (fragment size ranges, etc.).
   • Stores `std::string d_typeStr` identifying concrete subclass type.
   • Pure-virtual serialization interface identical to `CatalogEntry`.

---

## Inter-symbol relationships

```
RDCatalog::Catalog<E,P>::addEntry   -> expects pointer to E (subclass of CatalogEntry)
RDCatalog::Catalog<E,P>::dp_cParams -> owns deep copy of P (subclass of CatalogParams)

RDCatalog::HierarchCatalog<E,P,O>   -> public Catalog<E,P>
HierarchCatalog                     -> stores boost::adjacency_list graph of E*
HierarchCatalog::toStream           -> serialises: header → fpLength → entry count → P → entries (E) → edges
HierarchCatalog::initFromStream     -> inverse, constructs E and calls addEntry(); builds edges

CatalogEntry::Serialize             -> implemented by each concrete entry type elsewhere (not in this module)
CatalogParams::Serialize            -> implemented by each concrete params type elsewhere

(This module therefore forms a *framework layer*; chemistry-specific code lives in e.g. `GraphMol/FragmentCatalog`.)
```

---

## External dependencies

• Boost.Graph (header-only) – internal storage and traversal of hierarchy.  
• RDGeneral – logging/assertion macros (`PRECONDITION`, `URANGE_CHECK`, etc.), type aliases (`INT_VECT`), and binary stream helpers (`streamWrite`, `streamRead`).

No other RDKit modules are referenced here.

---

## Open questions / potential Rust-port notes

1. Replace Boost.Graph with petgraph or a bespoke adjacency-list; ensure edge-duplicate prevention.
2. Use `serde` for (de)serialization instead of bespoke binary streams.
3. `std::string` pickles are used throughout RDKit; we may instead expose `Vec<u8>`.
4. Generic type parameters (`E`, `P`, `O`) will map well to Rust generics with trait bounds (e.g. `Serialize + DeserializeOwned + Clone`).

---

End of module map.
