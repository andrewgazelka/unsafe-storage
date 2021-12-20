/// [UnsafeStorage] is used to mark that there are some arbitrary invariants
/// which must be maintained in storing its inner value. Therefore, creation and
/// modifying of the inner value is an "unsafe" behavior. Although it might not
/// be unsafe in traditional Rust terms (no memory unsafety), behavior might be
/// "undefined"—or at least undocumented, because invariants are expected to be
/// upheld.
///
/// This is useful in macros which do not encapsulate their storage in modules.
/// This makes the macros for the end-user more ergonomic, as they can use the
/// macro multiple times in a single module.
#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Default)]
pub struct UnsafeStorage<T>(T);

impl<T> UnsafeStorage<T> {
    /// # Safety
    /// - See the broader scope that this is called in and which invariants are
    ///   mentioned
    pub unsafe fn new_unsafe(inner: T) -> Self {
        Self(inner)
    }

    /// # Safety
    /// This should be a safe operation assuming that when modifying T to T',
    /// UnsafeStorage::new_unsafe(T') is safe
    pub unsafe fn as_ref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Copy> UnsafeStorage<T> {
    /// Get the inner value
    pub fn inner(&self) -> T {
        self.0
    }
}
