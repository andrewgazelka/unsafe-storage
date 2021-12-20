# UnsafeStorage
`UnsafeStorage` is used to mark that there are some arbitrary invariants
which must be maintained in storing its inner value. Therefore, creation and
modifying of the inner value is an "unsafe" behavior. Although it might not
be unsafe in traditional Rust terms (no memory unsafety), behavior might be
"undefined"—or at least undocumented, because invariants are expected to be
upheld.

This is useful in macros which do not encapsulate their storage in modules.
This makes the macros for the end-user more ergonomic, as they can use the
macro multiple times in a single module.
