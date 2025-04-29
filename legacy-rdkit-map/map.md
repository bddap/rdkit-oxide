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

GraphMol::ROMol -> implemented in -> Code/GraphMol/ROMol.cpp
GraphMol::ROMol -> defined in -> Code/GraphMol/ROMol.h
GraphMol::ROMol::addBond -> calls -> GraphMol::Bond

Geometry::Point3D -> implemented in -> Code/Geometry/point.cpp
Geometry::Point3D -> defined in -> Code/Geometry/point.h

ForceField::ForceField -> implemented in -> Code/ForceField/ForceField.cpp
ForceField::ForceField -> calls -> Numerics::Vector

-------------------------------------------------------------------------------

More detailed per-function edges will be appended incrementally in subsequent
commits.
