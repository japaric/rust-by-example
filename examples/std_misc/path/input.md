The `Path` struct represents file paths in the underlying filesystem. There are
two flavors of `Path`: `posix::Path`, for UNIX-like systems, and
`windows::Path`, for Windows. The prelude exports the appropriate
platform-specific `Path` variant.

A `Path` can be created from `OsStr` type and provides several methods to get
information from the file/directory the path points to.

Note that a `Path` is *not* internally represented as an UTF-8 string, but
instead is stored as a vector of bytes (`Vec<u8>`). Therefore, converting a
`Path` to a `&str` is *not* free and may fail (an `Option` is returned).

{path.play}

Be sure to check at other `Path` methods (`posix::Path` or `windows::Path`) and
the `Metadata` struct.

### See also

[OsStr][1] and [Metadata][2].

[1]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
[2]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
