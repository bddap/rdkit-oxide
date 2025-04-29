# GraphMol – ForceFieldHelpers Sub-module Map

Location: `rdkit/Code/GraphMol/ForceFieldHelpers`

This sub-tree provides **builder utilities that translate an RDKit molecule
into a classical force-field object** (`RDKit::ForceFields::ForceField`) ready
for energy minimisation or conformer refinement.  Three independent families
are implemented:

1. UFF – Universal Force Field (generic across the periodic table)
2. MMFF94 – Merck Molecular Force Field variants (MMFF94, MMFF94s)
3. CrystalFF – specialised torsion parameter set used by ETKDG

Each family follows the same pattern: an *atom typer* assigns parameter ids
while a *builder* adds harmonic terms (bond stretch, angle bend, torsion, vdW,
electrostatics) to the force field object.

--------------------------------------------------------------------------------

## 1. Common facade (`FFConvenience.h`)

Header-only helpers that hide the boilerplate of calling the specific builders:

```
bool MMFFHasAllMoleculeParams(const ROMol &mol);
std::unique_ptr<ForceFields::ForceField> UFFGetMoleculeForceField(
    const ROMol &mol, double vdwThresh, int confId = -1, bool ignoreInterfrag = false);
```

Used extensively by Python wrappers: `AllChem.UFFOptimizeMolecule()` etc.

--------------------------------------------------------------------------------

## 2. UFF folder

| File | Description |
|------|-------------|
| `AtomTyper.*` | Assigns UFF atom type (e.g. `C_3`, `O_2`) by inspecting atomic number, degree, aromaticity, ring membership – ~400 lines of `switch` logic. Public helper `UFFGetAtomType(...)`. |
| `Builder.*` | Translates typed molecule into `ForceField`. Adds stretch, angle, torsion, inversion and van-der-Waals terms according to UFF parameters hard-coded in `UFFTyperParams.h` (in `rdkit/Code/ForceField` core). |
| `UFF.h` | Thin wrapper exposing `UFFOptimizeMolecule()`, `UFFOptimizeMoleculeConfs()` convenience functions with convergence criteria. |

Tests: `testUFFHelpers.cpp` (+ Python variants) cover parameter coverage and optimisation success.

--------------------------------------------------------------------------------

## 3. MMFF folder

Very similar architecture but parameter tables are read **at runtime** from
`Data/MMFF/` CSV files generated from Merck’s publication.

| File | Key classes |
|------|-------------|
| `AtomTyper.*` | Follows Halgren’s rules yielding numeric *atom class* and partial charge; caches results in `MMFF::Properties`. |
| `Builder.*` | Adds terms for bond, angle, stretch-bend, torsion, out-of-plane, and vdW/electrostatic.  Respects `MMFFVariant` enum (94 vs 94s). |
| `MMFF.h` | Facade wrapper similar to UFF. |

Additional test `testMultiThread.cpp` verifies thread-safety of parameter cache when embedding many conformers in parallel.

--------------------------------------------------------------------------------

## 4. CrystalFF folder

Implements knowledge-based torsion preferences used by the ETKDG embedding
algorithm (see DistGeomHelpers map).

| File | Contents |
|------|----------|
| `TorsionPreferences.*` | Loads millions of torsion stats from `.in` files into a multi-level map keyed by SMARTS pattern ids. Provides lookup `getTorsionPreferences(mol, bondIdx)`. |
| `TorsionAngleContribs.*` | Custom `ForceFieldContrib` objects that add a periodic torsion potential to the force field according to the preference histogram. |
| `TorsionAngleM6.*` | Convenience builder injecting **M**odified **6-term** Fourier series contributions (cos(φ)^(n)). |

Used exclusively by `EmbedderUtils::addTorsionPreferenceContribs()`.

--------------------------------------------------------------------------------

## 5. Wrap folder

SWIG interface file exposes `«family»OptimizeMolecule*` functions and parameter
accessors (`MMFFGetMoleculeProperties()` etc.) to Python.

--------------------------------------------------------------------------------

## 6. Unit tests

* C++: `testUFFHelpers.cpp`, `testMMFFHelpers.cpp`, `testCrystalFF.cpp`.  
* Catch2 harness in root `catch_tests.cpp` for fast execution.  
* Python: multiple tests in `Wrap/rdForceFieldHelpers.py` (not listed here).

--------------------------------------------------------------------------------

## 7. Dependencies

• Core `ForceField` engine (`rdkit/Code/ForceField` module).  
• `SimDivPickers` for multi-conformer optimisation threads.  
• `Data` parameter CSVs loaded via `RDBase::getDataFilePath()`.

--------------------------------------------------------------------------------

## 8. Rust-port notes

1. Provide generic trait `ForceFieldBuilder` with `fn build(mol:&Molecule, conf:&mut Conformer) -> ForceField`.
2. Parameter tables can be embedded as `phf::Map` static constants (UFF) or lazily loaded CSV (MMFF – consider the `csv` crate).  
3. Potential terms implement `energy(&self, &coords) -> f64` + gradient; UFF/MMFF share harmonic functional forms so can reuse `harmonic` module.  
4. Multithreaded optimisation via `rayon`; thread-local parameter cache for
   MMFF to avoid contention.

--------------------------------------------------------------------------------

End of ForceFieldHelpers map.
