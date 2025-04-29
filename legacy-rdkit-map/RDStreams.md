# RDStreams Module Map

Location: `rdkit/Code/RDStreams`

RDStreams is a *very small* helper module that wraps **Boost.IOStreams** gzip
filters so that the rest of RDKit can read compressed files transparently via
an `std::istream`-like interface.

-------------------------------------------------------------------------------

## 1. Contents

| File | Description |
|------|-------------|
| `streams.h` | Declares class `RDKit::gzstream` inheriting from `boost::iostreams::filtering_istream`. Constructor takes a filename, opens an `std::ifstream` in binary mode, pushes a gzip decompressor onto the filter stack, and finally the file stream.  Available only when `RDK_USE_BOOST_IOSTREAMS` is defined (CMake option `RDK_BUILDIOSTREAMS`). |
| `streams.cpp` | Defines the trivial constructor inline with the logic above. |

There are no additional classes, functions, or unit tests.

-------------------------------------------------------------------------------

## 2. Typical usage in RDKit

```
#include <RDStreams/streams.h>

RDKit::gzstream in("molecules.sdf.gz");
MolSupplier suppl(&in, true, false);
while (!suppl.atEnd()) {
    auto *mol = suppl.next();
    ...
}
```

If Boost.IOStreams is *not* available, callers fall back to regular
`std::ifstream` on uncompressed files.

-------------------------------------------------------------------------------

## 3. Dependencies

• Boost.IOStreams (`boost/iostreams/filtering_stream.hpp`,
  `boost/iostreams/filter/gzip.hpp`).  
• Standard `<fstream>`.

-------------------------------------------------------------------------------

## 4. Rust-port notes

In Rust this tiny abstraction can be replaced by the `flate2` crate:

```
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct GzStream(Box<dyn Read>);

impl GzStream {
    pub fn new(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let decoder = GzDecoder::new(BufReader::new(file));
        Ok(GzStream(Box::new(decoder)))
    }
}
```

Consumers like `SdfIterator` can accept any `impl Read` so no explicit module
may be needed.

-------------------------------------------------------------------------------

End of RDStreams map.
