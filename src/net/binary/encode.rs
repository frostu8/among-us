/// A newtype struct that encapsulates a `Vec<u8>`.
pub struct CursorMut {
    inner: Vec<u8>,
}

impl CursorMut {
    /// Create a new, empty `CursorMut`.
    pub fn new() -> CursorMut {
        CursorMut {
            inner: Vec::with_capacity(512),
        }
    }

    /// Write a series of bytes to the `CursorMut`.
    pub fn write(&mut self, buf: &[u8]) {
        self.inner.extend(buf);
    }

    /// Encodes a type into the `CursorMut`.
    pub fn encode<T>(&mut self, ty: &T) -> Result<(), Error> 
    where T: Encode + ?Sized {
        ty.encode(self)
    }
}

impl Into<Vec<u8>> for CursorMut {
    fn into(self) -> Vec<u8> {
        self.inner
    }
}

/// An error type.
///
/// There isn't really anything that can go wrong, as bytes are a superset of
/// Rust types in this sense. This is only here for easy additions if it is
/// needed.
pub struct Error;

/// A type that can be encoded to a [`CursorMut`].
pub trait Encode {
    fn encode(&self, cursor: &mut CursorMut) -> Result<(), Error>;
}
