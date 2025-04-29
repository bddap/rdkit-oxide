# GraphMol Core – Atom, Bond, ROMol/RWMol, Conformer

Location: `rdkit/Code/GraphMol` (top‐level .h/.cpp files)

This document describes the **foundational data structures** of RDKit’s molecular graph representation.  Understanding these classes is essential before diving into algorithms built on top of them.

--------------------------------------------------------------------------------

## 1. Atom (Atom.h / Atom.cpp)

Represents a vertex in the molecular graph.  Key data members (simplified):

| Field | Type | Meaning |
|-------|------|---------|
| `int d_atomicNum` | atomic number (0 = wildcard) |
| `unsigned int d_index` | position in owning molecule’s atom list |
| `ChiralType d_chiralTag` | enum of stereochemistry flag |
| `HybridizationType d_hybrid` | VB hybridisation model |
| `unsigned int d_explicitValence` / `d_implicitValence` | cached valence counts |
| `ROMol* dp_mol` | back-pointer to owning molecule (nullable until inserted) |
| `RDProps` base | flexible property dictionary |

Highlights:
• Implements lazy valence calculation via `calculateImplicitValence()` helper.  
• Query support: `QUERYATOM_QUERY` typedef aliasing `Queries::Query<int, Atom const*>` for SMARTS matching.  
• Copy, move constructors maintain or clear `dp_mol` pointer as appropriate.

Common methods:
```
getAtomicNum(), setFormalCharge(), getTotalNumHs(), isAromatic(),
setChiralTag(), invertStereo(),
```

--------------------------------------------------------------------------------

## 2. Bond (Bond.h / Bond.cpp)

A graph edge connecting two atoms.

Important enums:
• `BondType` – single, double, triple, aromatic, dative, etc.  
• `BondDir` – wedge/dash or cis/trans direction for drawing/stereo.  
• `BondStereo` – E/Z, CIS/TRANS, atropisomer etc.

Data members:
| Name | Purpose |
|------|---------|
| `unsigned int d_beginAtomIdx`, `d_endAtomIdx` | atom indices |
| `std::uint8_t d_bondType` | packed enum |
| `bool df_isAromatic`, `df_isConjugated` | flags |
| `BondStereo d_stereo` plus `std::vector<unsigned int>* dp_stereoAtoms` | stereo specification |
| `ROMol* dp_mol` | owner |

Provides query alias `QUERYBOND_QUERY` for SMARTS.

--------------------------------------------------------------------------------

## 3. Conformer (Conformer.h / Conformer.cpp)

Stores coordinates for each atom in a molecule.
• Vector of `RDGeom::Point3D` positions (`d_positions`).  
• `unsigned int d_id` conformer id; `bool is3D`.  
• Supports attachment of arbitrary data via `RDProps`.

--------------------------------------------------------------------------------

## 4. ROMol (ROMol.h / ROMol.cpp)

Immutable molecular graph (after construction).  Internally stores:

* `MolGraph d_graph` (typedef of `boost::adjacency_list<…>` with `Atom*` vertices and `Bond*` edges).
* `std::vector<Atom*> d_atmIdx` and `std::vector<Bond*> d_bondIdx` for O(1) index lookup.
* `boost::shared_ptr<RingInfo> dp_ringInfo` filled by `FindRings.cpp`.
* Containers of `Conformer`, `SubstanceGroup`, `StereoGroup`.
* Bookmark multimap 
  – `std::multimap<int, Atom*> d_atomBookmarks`,
  – same for bonds; used by parsers.

Key operations:
• Topology queries: `getAtomWithIdx()`, `getBondBetweenAtoms()`, iterators (`Atoms()`, `Bonds()`, `getAromaticAtoms()` etc.).  
• Chemistry algorithms call through helper files (aromaticity, stereo, kekulise).  
• Serialization via MolPickler.

--------------------------------------------------------------------------------

## 5. RWMol (RWMol.h / RWMol.cpp)

Mutable subclass of ROMol. Adds:
• Editing API: `addAtom()`, `addBond()`, `removeAtom()`, `removeBond()`, `beginBatchEdit()/commitBatchEdit()` transactions.  
• Ensures graphs and index vectors remain consistent.

--------------------------------------------------------------------------------

## 6. Iterators helpers

Files `AtomIterators.*` and `BondIterators.*` define `CXXAtomIterator`, `CXXBondIterator` wrappers enabling range‐for loops over atoms/bonds. They rely on BGL vertex/edge iterators internally.

--------------------------------------------------------------------------------

## Dependencies

• Boost.Graph for underlying adjacency list.  
• `Query/QueryObjects.h` for SMARTS query tree.  
• Core modules previously mapped: `Geometry`, `DataStructs` (bit ops for canonicalization), `RDGeneral` for props and invariants.

--------------------------------------------------------------------------------

## Rust-port notes

1. Represent molecule topology using `petgraph::Graph<Atom,Bond,Undirected>` with `NodeIndex` ↔ vector mapping for fast lookup.
2. Atom and Bond structs derive `Clone` but store indices only; back-references can be resolved via graph (`node_index()`
   is integer already).  
3. Property dictionaries can be implemented with `HashMap<String, PropertyValue>` where `PropertyValue` is an enum of common primitive types plus `Box<dyn Any>` feature-gated.
4. Iterators: provide `atoms()` and `bonds()` returning `impl Iterator<Item=&Atom>` similar to existing API.

--------------------------------------------------------------------------------

End of core GraphMol map.
