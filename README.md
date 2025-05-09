# Source Map Cache

Manages source files for a compiler, providing mapping between file IDs, paths, and source locations (line/column).

## Overview

This crate provides the `SourceMap` struct, which is responsible for:

* **Mounting Directories:** Associates symbolic names ("mounts") with file system directories containing source code.
* **Reading & Caching:** Reads source files either by absolute path or relative to a mount point, caching their contents and metadata.
* **File IDs:** Assigns a unique `FileId` (u16) to each loaded file.
* **Location Mapping:** Efficiently converts byte offsets within a file (often obtained from parsers or lexers using crates like `source-map-node`) into human-readable line and column numbers.
* **Source Snippets:** Retrieves the source text corresponding to a given span (file ID, offset, length).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
source-map-cache = "0.0.2"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Copyright

Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/swamp
