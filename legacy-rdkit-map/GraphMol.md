# GraphMol Module – Root‐Level Overview

Location: `rdkit/Code/GraphMol`

GraphMol is the **heart of RDKit**. It defines the molecular graph abstraction (atoms, bonds, conformers), cheminformatics algorithms (aromaticity, stereochemistry, substructure search), I/O parsers, descriptor calculators, fingerprints, reaction chemistry, and 2-D depiction.  Many of its components are themselves mini-libraries that have historically been split out but still live under the `GraphMol` umbrella.  Because of its size, the detailed mapping will be broken down *sub-directory by sub-directory* in later documents.  This file provides only a *top-level catalogue* so readers can orient themselves.

--------------------------------------------------------------------------------

## Core graph data structures

| Path | Purpose |
|------|---------|
| `Atom.*` | Atom class: element, charge, isotopes, query predicates. |
| `Bond.*` | Bond class: order, aromatic flag, query placeholders. |
| `Conformer.*` | Holds 3-D coordinates for a molecule (multiple per molecule). |
| `ROMol.*` (in `GraphMol/`) | “Read-only” molecule container aggregating atoms/bonds and metadata. |
| `RWMol.*` | Editable variant supporting atom/bond addition/removal. |

--------------------------------------------------------------------------------

## Functional sub-packages (first-level subdirectories)

Only key directories are listed; each will receive its own map later.

| Subdir | Synopsis |
|--------|----------|
| `Abbreviations` | Handles abbreviation (R-group) replacement for depiction. |
| `ChemReactions` | SMARTS-based reaction definition, application, and enumeration. |
| `ChemTransforms` | One-off reaction helpers for common functional-group transforms. |
| `CIPLabeler` | Assign R/S and E/Z stereochemical labels using CIP rules. |
| `Depictor` | 2-D coordinate generation (depiction). |
| `Descriptors` | Phys/chem descriptor calculators (TPSA, LogP, etc.). |
| `FileParsers` | Readers/Writers: SDF, SMILES, PDB, Mol2, etc. |
| `Fingerprints` | MACCS, Morgan (ECFP), AtomPair, TopologicalTorsion, etc. |
| `FilterCatalog` | Substructure-based filtering for PAINS, Lipinski, etc. |
| `FragCatalog` | Fragment and pharmacophore fragment catalogues built atop `Catalogs` module. |
| `FMCS` | Fast Maximum Common Substructure algorithm. |
| `GeneralizedSubstruct` | Query types and matcher for SMARTS+algebra queries. |
| `Pharm3D` | Pharmacophore 3-D alignment and scoring. |
| `RingInfo` (FindRings.cpp) | Ring perception (SSSR, symmetrized aromaticity). |
| `Aromaticity.cpp` | Aromaticity models (Kekulisation, Hückel). |
| `Chirality.*` / `FindStereo.cpp` | Stereochemistry perception. |
| `Subgraphs` | Subgraph enumeration utilities. |
| `Substruct` (`SubstructMatch.*`) | SMARTS matching, isomorphism solver. |
| `Query` | (link to `Code/Query`) templates used by query atoms/bonds.
| `ForceFieldHelpers` | Interfaces to UFF/MMFF for molecule minimisation. |
| `DistGeomHelpers` | Helpers to call the DistGeom embedder at molecule level. |
| `Atropisomers` | Specialized enumeration of atropisomer conformations. |
| `Basement` | Very old demos and legacy code (not built by default). |

Additionally, many stand-alone .cpp files add methods to the above core classes (e.g., `AtomIterators.*`, `BondIterators.*`, `Canon.*` for canonicalisation).

--------------------------------------------------------------------------------

## Build targets

`GraphMol` is built as multiple static libraries (`RDKitGraphMol`, `GraphMolWrap`, etc.) inside CMake, but the actual split is by functional group.  All components link against previously mapped modules: `DataStructs`, `Geometry`, `DistGeom`, `ForceField`, and core `RDGeneral` utilities.

--------------------------------------------------------------------------------

## Mapping plan

The following child summaries will be produced next (each bullet becomes a todo entry in progress.md):

* Atom/Bond/ROMol core
* FileParsers
* Fingerprints
* Descriptors
* Substructure search
* Reaction chemistry
* Depictor
* CIPLabeler & stereochemistry
* FilterCatalog
* FMCS
* DistGeomHelpers & ForceFieldHelpers

--------------------------------------------------------------------------------

End of root overview.
