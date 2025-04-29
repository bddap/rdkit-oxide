# Query Module Map

Location: `rdkit/Code/Query`

The **Query** module implements RDKit’s *generic query tree* abstraction used
everywhere a SMARTS-like matching expression is required (substructure search,
pharmacophore filters, stereochemistry checks, etc.).  It is a template-based
system that can be instantiated for arbitrary data types – the two most common
instantiations are:

* `Query<int>` – used for numeric atom or bond properties
* `Query<bool>` – used for logical predicates

Higher-level code (e.g. `QueryAtom`, `QueryBond`) embeds a tree of these query
objects and evaluates them against target atoms/bonds during matching.

-------------------------------------------------------------------------------

## 1. Core class – `Query<MatchFuncArg, DataFuncArg, needsConversion>`

Defined in `Query.h` as a template with default `DataFuncArg == MatchFuncArg`.

Key features:
• Stores optional function pointers `matchFunc` (predicate) and `dataFunc`
  (conversion).  
• Supports *negation* via `setNegation()`.  
• Children vector forms an **N-ary tree**; concrete algebra nodes override
  `Match()` to combine children results.  
• Provides `copy()` virtual clone used when duplicating molecules.

-------------------------------------------------------------------------------

## 2. Logical algebra helpers

| File | Class | Combines children by… |
|------|-------|-----------------------|
| `AndQuery.h` | `AndQuery` | logical AND (short-circuit) |
| `OrQuery.h`  | `OrQuery`  | logical OR |
| `NullQueryAlgebra.h` | `NullQuery` | always returns `true` (placeholder) |

Each of these inherits from `Query<>` and overrides `Match()` with the proper
boolean aggregation.  They are instantiated via helper factory functions such
as `makeAtomAndQuery()`, `makeBondOrQuery()` inside GraphMol.

-------------------------------------------------------------------------------

## 3. Comparison queries

Files: `EqualityQuery.h`, `GreaterQuery.h`, `GreaterEqualQuery.h`,
`LessQuery.h`, `LessEqualQuery.h`

Template `ComparisonQuery` holds `value` and optional `tolerance`; concrete
classes provide:

* `operator()` performing the comparison (e.g. `other == value`).
* `Match()` inherited from base calls that operator via `matchFunc` pointer.

Used extensively for numeric SMARTS attributes like `atomNumber`, `ringCount`
or `bondOrder`.

-------------------------------------------------------------------------------

## 4. Typedefs seen by GraphMol

GraphMol aliases two commonly used specialisations:

```
using QUERYATOM_QUERY = Queries::Query<int,int,false>*
using QUERYBOND_QUERY = Queries::Query<int,int,false>*
```

Construction helpers in `QueryOps.h` build deep trees like:

```
AndQuery
 ├─ EqualityQuery(atomicNum == 6)
 ├─ LessEqualQuery(formalCharge ≤ 0)
 └─ OrQuery
     ├─ EqualityQuery(isAromatic == 1)
     └─ EqualityQuery(bondOrder == 2)
```

-------------------------------------------------------------------------------

## 5. Tests / usage

No dedicated C++ tests live in this directory; correctness is exercised via
the large substructure-search test-suite in GraphMol.  The query classes
themselves are header-only, thus heavily template-instantiated.

-------------------------------------------------------------------------------

## 6. Dependencies

• `RDGeneral/Invariant.h` for runtime checks.  
• STL containers; no boost or third-party libs.

-------------------------------------------------------------------------------

## 7. Rust-port notes

1. Represent a query node as an enum:

```
enum QueryNode {
    And(Vec<QueryNode>),
    Or(Vec<QueryNode>),
    Not(Box<QueryNode>),
    Eq(i32),
    Gt(i32),
    Ge(i32),
    Lt(i32),
    Le(i32),
}

impl QueryNode {
    fn matches(&self, value:i32) -> bool { … }
}
```

2. Provide `clone_box()` trait to replicate `copy()` semantics.
3. High-level `QueryAtom` will own a tree boxed behind `Arc<QueryNode>`.

-------------------------------------------------------------------------------

End of Query map.
