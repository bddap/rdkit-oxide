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

# GraphMol – Fingerprints relationships

GraphMol::Fingerprints::AtomPair::getFingerprint -> implemented in -> Code/GraphMol/Fingerprints/AtomPairGenerator.cpp
GraphMol::Fingerprints::AtomPair::getFingerprint -> calls -> DataStructs::SparseIntVect

GraphMol::Fingerprints::MorganGenerator::getFingerprint -> implemented in -> Code/GraphMol/Fingerprints/MorganGenerator.cpp
GraphMol::Fingerprints::MorganGenerator::getFingerprint -> calls -> GraphMol::Canon::getMorganCode

GraphMol::Fingerprints::RDKitFPGenerator::getFingerprint -> implemented in -> Code/GraphMol/Fingerprints/RDKitFPGenerator.cpp
GraphMol::Fingerprints::RDKitFPGenerator::getFingerprint -> uses -> DataStructs::ExplicitBitVect

# GraphMol – Descriptors relationships

GraphMol::Descriptors::CalcTPSA -> implemented in -> Code/GraphMol/Descriptors/MolSurf.cpp
GraphMol::Descriptors::CalcTPSA -> calls -> GraphMol::Descriptors::getAtomTR

GraphMol::Descriptors::BCUT2D -> implemented in -> Code/GraphMol/Descriptors/BCUT2D.cpp
GraphMol::Descriptors::BCUT2D -> calls -> Numerics::Matrix

GraphMol::Descriptors::RDF -> implemented in -> Code/GraphMol/Descriptors/RDF.cpp
GraphMol::Descriptors::RDF -> uses -> Geometry::UniformGrid3D

# ForceField relationships

ForceFields::ForceField -> implemented in -> Code/ForceField/ForceField.cpp
ForceFields::ForceField -> defined in -> Code/ForceField/ForceField.h

# Core API
ForceFields::ForceField::initialize -> calls -> std::vector<ContribPtr>::reserve
ForceFields::ForceField::calcEnergy -> calls -> ForceFields::ForceFieldContrib::getEnergy
ForceFields::ForceField::calcGrad -> calls -> ForceFields::ForceFieldContrib::getGrad
ForceFields::ForceField::minimize -> calls -> ForceFields::ForceField::calcGrad
ForceFields::ForceField::minimize -> calls -> ForceFields::ForceField::calcEnergy

# Generic contributor base
ForceFields::ForceFieldContrib -> defined in -> Code/ForceField/Contrib.h
ForceFields::ForceFieldContrib::getEnergy -> (pure virtual) implemented by -> derived constraint/term classes
ForceFields::ForceFieldContrib::getGrad   -> (pure virtual) implemented by -> derived constraint/term classes

# Geometric constraints
ForceFields::DistanceConstraintContrib -> implemented in -> Code/ForceField/DistanceConstraint.cpp
ForceFields::DistanceConstraintContrib -> defined in -> Code/ForceField/DistanceConstraint.h
ForceFields::DistanceConstraintContrib::getEnergy -> uses -> RDGeom::Point3D

ForceFields::AngleConstraintContrib    -> implemented in -> Code/ForceField/AngleConstraint.cpp
ForceFields::AngleConstraintContrib    -> defined in -> Code/ForceField/AngleConstraint.h

ForceFields::TorsionConstraintContrib  -> implemented in -> Code/ForceField/TorsionConstraint.cpp
ForceFields::TorsionConstraintContrib  -> defined in -> Code/ForceField/TorsionConstraint.h

ForceFields::PositionConstraintContrib -> implemented in -> Code/ForceField/PositionConstraint.cpp
ForceFields::PositionConstraintContrib -> defined in -> Code/ForceField/PositionConstraint.h

# UFF parameterised terms (selected)
ForceFields::UFF::BondStretchContrib -> implemented in -> Code/ForceField/UFF/BondStretch.cpp
ForceFields::UFF::BondStretchContrib -> defined in -> Code/ForceField/UFF/BondStretch.h
ForceFields::UFF::BondStretchContrib::getEnergy -> uses -> ForceFieldsHelper::computeDistance

ForceFields::UFF::AngleBendContrib   -> implemented in -> Code/ForceField/UFF/AngleBend.cpp
ForceFields::UFF::AngleBendContrib   -> defined in -> Code/ForceField/UFF/AngleBend.h

ForceFields::UFF::TorsionAngleContrib -> implemented in -> Code/ForceField/UFF/TorsionAngle.cpp
ForceFields::UFF::TorsionAngleContrib -> defined in -> Code/ForceField/UFF/TorsionAngle.h

ForceFields::UFF::NonbondedContrib -> implemented in -> Code/ForceField/UFF/Nonbonded.cpp
ForceFields::UFF::NonbondedContrib -> defined in -> Code/ForceField/UFF/Nonbonded.h

# Helper utilities
ForceFieldsHelper::computeDihedral -> defined in -> Code/ForceField/ForceField.h
ForceFieldsHelper::normalizeAngleDeg -> implemented in -> Code/ForceField/ForceField.cpp


# DistGeomHelpers relationships

DGeomHelpers::initBoundsMat -> implemented in -> Code/GraphMol/DistGeomHelpers/BoundsMatrixBuilder.cpp
DGeomHelpers::initBoundsMat -> defined in -> Code/GraphMol/DistGeomHelpers/BoundsMatrixBuilder.h

DGeomHelpers::setTopolBounds -> implemented in -> Code/GraphMol/DistGeomHelpers/BoundsMatrixBuilder.cpp
DGeomHelpers::setTopolBounds -> defined in -> Code/GraphMol/DistGeomHelpers/BoundsMatrixBuilder.h

DGeomHelpers::collectBondsAndAngles -> implemented in -> Code/GraphMol/DistGeomHelpers/BoundsMatrixBuilder.cpp

DGeomHelpers::EmbedMultipleConfs -> implemented in -> Code/GraphMol/DistGeomHelpers/Embedder.cpp
DGeomHelpers::EmbedMultipleConfs -> defined in -> Code/GraphMol/DistGeomHelpers/Embedder.h
DGeomHelpers::EmbedMultipleConfs -> calls -> DGeomHelpers::initBoundsMat
DGeomHelpers::EmbedMultipleConfs -> calls -> DistGeom::TriangleSmooth
DGeomHelpers::EmbedMultipleConfs -> calls -> ForceFields::ForceField::minimize

DGeomHelpers::EmbedMolecule -> inline defined in -> Code/GraphMol/DistGeomHelpers/Embedder.h
DGeomHelpers::EmbedMolecule -> calls -> DGeomHelpers::EmbedMultipleConfs

# ForceFieldHelpers relationships

ForceFieldsHelper::OptimizeMolecule -> defined inline in -> Code/GraphMol/ForceFieldHelpers/FFConvenience.h
ForceFieldsHelper::OptimizeMolecule -> calls -> ForceFields::ForceField::minimize
ForceFieldsHelper::OptimizeMolecule -> calls -> ForceFields::ForceField::calcEnergy

ForceFieldsHelper::OptimizeMoleculeConfs -> defined inline in -> Code/GraphMol/ForceFieldHelpers/FFConvenience.h
ForceFieldsHelper::OptimizeMoleculeConfs -> calls -> ForceFields::ForceField::initialize

# UFF force-field builder
RDKit::UFF::constructForceField -> implemented in -> Code/GraphMol/ForceFieldHelpers/UFF/Builder.cpp
RDKit::UFF::constructForceField -> defined in -> Code/GraphMol/ForceFieldHelpers/UFF/Builder.h
RDKit::UFF::constructForceField -> calls -> ForceFields::ForceField::addContrib
RDKit::UFF::constructForceField -> calls -> UFF::Tools::addBonds / addAngles / addTorsions / addNonbonded / addInversions

UFF::Tools::addBonds   -> implemented in -> Code/GraphMol/ForceFieldHelpers/UFF/Builder.cpp
UFF::Tools::addAngles  -> implemented in -> same cpp
UFF::Tools::addNonbonded -> implemented in -> same cpp

# MMFF force-field builder
RDKit::MMFF::constructForceField -> implemented in -> Code/GraphMol/ForceFieldHelpers/MMFF/Builder.cpp
RDKit::MMFF::constructForceField -> defined in -> Code/GraphMol/ForceFieldHelpers/MMFF/Builder.h
RDKit::MMFF::constructForceField -> calls -> ForceFields::ForceField::addContrib

MMFF::Tools::addStretchBend/angles/... -> implemented in -> same directory Builder.cpp

# Catalogs relationships

RDCatalog::Catalog (template) -> defined in -> Code/Catalogs/Catalog.h
RDCatalog::Catalog::addEntry -> (pure virtual) implemented by -> derived catalogs (e.g., HierarchCatalog)

RDCatalog::HierarchCatalog -> implemented in -> Code/Catalogs/Catalog.h (header-only) & Catalog.cpp for certain methods
RDCatalog::HierarchCatalog::addEntry -> calls -> boost::add_vertex
RDCatalog::HierarchCatalog::Serialize -> calls -> RDCatalog::CatalogEntry::toStream

RDCatalog::CatalogEntry -> implemented in -> Code/Catalogs/CatalogEntry.cpp
RDCatalog::CatalogEntry -> defined in -> Code/Catalogs/CatalogEntry.h
RDCatalog::CatalogEntry::Serialize -> pure virtual, implemented by domain-specific derived entries (e.g., functional‐group catalogs in GraphMol/FilterCatalog)

RDCatalog::CatalogParams -> implemented in -> Code/Catalogs/CatalogParams.cpp
RDCatalog::CatalogParams -> defined in -> Code/Catalogs/CatalogParams.h

RDCatalog::CatalogEntry::getBitId -> returns -> int (maps to fingerprint bit)

# SimDivPickers relationships

RDPickers::DistPicker::pick (abstract) -> defined in -> Code/SimDivPickers/DistPicker.h
RDPickers::getDistFromLTM -> implemented in -> Code/SimDivPickers/DistPicker.cpp

RDPickers::HierarchicalClusterPicker -> implemented in -> Code/SimDivPickers/HierarchicalClusterPicker.cpp
RDPickers::HierarchicalClusterPicker::pick -> calls -> RDPickers::HierarchicalClusterPicker::cluster
RDPickers::HierarchicalClusterPicker::cluster -> uses -> ML::Cluster::Murtagh algorithm (external in Code/ML)

RDPickers::MaxMinPicker -> implemented in -> Code/SimDivPickers/MaxMinPicker.cpp
RDPickers::MaxMinPicker::pick -> calls -> RDPickers::MaxMinPicker::lazyPick (template)
RDPickers::MaxMinPicker::lazyPick -> uses -> boost::random / std::mt19937

# Numerics relationships

Numerics::Vector -> defined in -> Code/Numerics/Vector.h
Numerics::Matrix -> defined in -> Code/Numerics/Matrix.h

Numerics::Optimizer::BFGSOptimize -> implemented in -> Code/Numerics/Optimizer/BFGSOpt.cpp
Numerics::Optimizer::BFGSOpt::minimize -> calls -> Numerics::Optimizer::LineSearch

# Geometry utilities

Geometry::Transform3D -> defined in -> Code/Geometry/Transform3D.h
Geometry::Transform3D::TransformPoints -> implemented in -> Code/Geometry/Transform3D.cpp

# DataManip relationships

RDDataManip::MetricMatrixCalc (template) -> defined in -> Code/DataManip/MetricMatrixCalc/MetricMatrixCalc.h
RDDataManip::MetricMatrixCalc::calcMetricMatrix -> calls -> user supplied metricFunc pointer

RDDataManip::EuclideanDistanceMetric -> implemented in -> Code/DataManip/MetricMatrixCalc/MetricFuncs.h
RDDataManip::TanimotoDistanceMetric   -> implemented in -> MetricFuncs.h, calls -> DataStructs::TanimotoSimilarity

# Features relationships

RDFeatures::ExplicitFeature (template) -> defined in -> Code/Features/Feature.h
RDFeatures::ExplicitFeature::getLoc -> returns -> RDGeom::Point3D

RDFeatures::ImplicitFeature (template) -> defined in -> Code/Features/Feature.h
RDFeatures::ImplicitFeature::addPoint -> stores -> pointer to external location & weight
RDFeatures::ImplicitFeature::getLoc -> computes weighted centroid over points

# Geometry relationships

RDGeom::Point3D -> implemented in -> Code/Geometry/point.cpp
RDGeom::Point3D -> defined in -> Code/Geometry/point.h
RDGeom::Point3D::normalize -> calls -> std::sqrt

Geometry::Transform3D -> implemented in -> Code/Geometry/Transform3D.cpp
Geometry::Transform3D -> defined in -> Code/Geometry/Transform3D.h
Geometry::Transform3D::TransformPoint -> multiplies -> Numerics::Matrix

Geometry::UniformGrid3D -> implemented in -> Code/Geometry/UniformGrid3D.cpp
Geometry::UniformGrid3D -> defined in -> Code/Geometry/UniformGrid3D.h
Geometry::UniformGrid3D::addPoint -> stores -> RDGeom::Point3D in voxel map

# DistGeom core relationships

DistGeom::BoundsMatrix -> implemented in -> Code/DistGeom/BoundsMatrix.h (inline methods)
DistGeom::BoundsMatrix::setUpperBoundIfBetter -> ensures -> triangular inequality compliance

DistGeom::triangleSmoothBounds -> implemented in -> Code/DistGeom/TriangleSmooth.cpp
DistGeom::triangleSmoothBounds -> calls -> BoundsMatrix::setUpperBoundIfBetter / setLowerBoundIfBetter

DistGeom::pickRandomDistMat -> implemented in -> Code/DistGeom/DistGeomUtils.cpp
DistGeom::pickRandomDistMat -> calls -> DistGeom::BoundsMatrix::getUpperBound/getLowerBound

DistGeom::ChiralViolationContrib -> implemented in -> Code/DistGeom/ChiralViolationContribs.cpp
DistGeom::ChiralViolationContrib::getEnergy -> uses -> ForceFields::ForceFieldContrib base

# RDGeneral relationships

RDGeneral::rdLogger -> implemented in -> Code/RDGeneral/RDLog.cpp
RDGeneral::rdLogger -> defined in -> Code/RDGeneral/RDLog.h
RDGeneral::rdLogger::SetTee -> uses -> boost::iostreams::tee_device

RDGeneral::LogStateSetter -> implemented in -> Code/RDGeneral/RDLog.cpp

RDGeneral::BadFileException -> defined in -> Code/RDGeneral/BadFileException.h
RDGeneral::Invariant::check -> macros in -> Code/RDGeneral/Invariant.h; calls -> RDGeneral::Invariant::InvariantFailed

# RDStreams relationships

RDKit::gzstream -> implemented in -> Code/RDStreams/streams.cpp
RDKit::gzstream -> defined in -> Code/RDStreams/streams.h
RDKit::gzstream constructor -> composes -> boost::iostreams::filtering_istream with boost::iostreams::gzip_decompressor

# ChemicalFeatures relationships

ChemicalFeatures::ChemicalFeature (abstract) -> defined in -> Code/ChemicalFeatures/ChemicalFeature.h

ChemicalFeatures::FreeChemicalFeature -> implemented in -> Code/ChemicalFeatures/FreeChemicalFeature.cpp
ChemicalFeatures::FreeChemicalFeature -> defined in -> Code/ChemicalFeatures/FreeChemicalFeature.h
ChemicalFeatures::FreeChemicalFeature::toString -> serializes -> id, family, type, position



-------------------------------------------------------------------------------

More detailed per-function edges will be appended incrementally in subsequent
commits.
