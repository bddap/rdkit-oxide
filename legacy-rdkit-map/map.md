# RDKit Relationship Graph (WIP)

Textual graph capturing high-level relationships between major modules and key
symbols.  Arrow notation:

```
module::SymbolA -> implemented in -> ./relative/path/file.cpp
module::SymbolA -> defined in -> ./relative/path/file.h
module::SymbolA -> calls -> module::SymbolB
```

-------------------------------------------------------------------------------

## Root modules

RDGeneral::rdLogger -> implemented in -> Code/RDGeneral/RDLog.cpp
RDGeneral::rdLogger -> defined in -> Code/RDGeneral/RDLog.h
RDGeneral::rdLogger -> calls -> std::ostream::operator<<

DataStructs::ExplicitBitVect -> implemented in -> Code/DataStructs/ExplicitBitVect.cpp
DataStructs::ExplicitBitVect -> defined in -> Code/DataStructs/ExplicitBitVect.h
DataStructs::ExplicitBitVect::getNumOnBits -> calls -> BitOps::countBits

DataStructs::SparseBitVect -> implemented in -> Code/DataStructs/SparseBitVect.cpp
DataStructs::SparseBitVect -> defined in -> Code/DataStructs/SparseBitVect.h
DataStructs::SparseBitVect::getTotalVal -> calls -> DataStructs::SparseIntVect::getTotalVal

DataStructs::BitOps::countBits -> implemented in -> Code/DataStructs/BitOps.cpp
DataStructs::BitOps::countBits -> defined in -> Code/DataStructs/BitOps.h

DataStructs::FPBReader -> implemented in -> Code/DataStructs/FPBReader.cpp
DataStructs::FPBReader -> defined in -> Code/DataStructs/FPBReader.h
DataStructs::FPBReader::getFingerprint -> calls -> DataStructs::BitOps::copyBitsToBuffer
DataStructs::MultiFPBReader -> implemented in -> Code/DataStructs/MultiFPBReader.cpp
DataStructs::MultiFPBReader::getFingerprint -> calls -> DataStructs::FPBReader::getFingerprint

GraphMol::ROMol -> implemented in -> Code/GraphMol/ROMol.cpp
GraphMol::ROMol -> defined in -> Code/GraphMol/ROMol.h
GraphMol::ROMol::addBond -> calls -> GraphMol::Bond

Geometry::Point3D -> implemented in -> Code/Geometry/point.cpp
Geometry::Point3D -> defined in -> Code/Geometry/point.h

ForceField::ForceField -> implemented in -> Code/ForceField/ForceField.cpp
ForceField::ForceField -> calls -> Numerics::Vector

# GraphMol – Core relationships

GraphMol::Atom -> implemented in -> Code/GraphMol/Atom.cpp
GraphMol::Atom -> defined in -> Code/GraphMol/Atom.h
GraphMol::Atom::getTotalValence -> calls -> GraphMol::Bond::getValenceContrib

GraphMol::Bond -> implemented in -> Code/GraphMol/Bond.cpp
GraphMol::Bond -> defined in -> Code/GraphMol/Bond.h
GraphMol::Bond::setStereoAtoms -> uses -> std::vector<unsigned int>

GraphMol::Conformer -> implemented in -> Code/GraphMol/Conformer.cpp
GraphMol::Conformer -> defined in -> Code/GraphMol/Conformer.h
GraphMol::Conformer::getAtomPos -> returns -> Geometry::Point3D

-------------------------------------------------------------------------------

More detailed per-function edges will be appended incrementally in subsequent
commits.
