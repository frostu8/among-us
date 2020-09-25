use std::cmp::min;

/// The binary cursor.
///
/// The `Cursor` is designed to read a sequence of bytes sequentially.
pub struct Cursor<T>
where T: AsRef<[u8]> {
    inner: T,
    cursor: usize,
}

impl<T> Cursor<T>
where T: AsRef<[u8]> {
    /// Create a new binary cursor.
    pub fn new(inner: T) -> Cursor<T> {
        Cursor {
            inner,
            cursor: 0,
        }
    }

    /// Reads a sequence of bytes.
    ///
    /// This returns how many bytes were read from the cursor. In a networking
    /// scenario, it is implied that all source data will be destructed after
    /// the deserialize functions are called.
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let inner = self.inner.as_ref();

        // get the end point of the buffer
        let end = min(self.cursor + buf.len(), inner.len());

        // get the slice
        let slice = &inner[self.cursor..end];

        // copy the slice
        (&mut buf[..slice.len()]).copy_from_slice(slice);

        // return the length
        slice.len()
    }

    /// Decode a type from the `Cursor`.
    pub fn decode<U>(&mut self) -> Result<U, Error> 
    where U: Decode {
        U::decode(self)
    }
}

/// An error that can occur during decoding.
pub enum Error {
    /// An unexpected end to the bytes was reached.
    UnexpectedEnd,
    /// A Utf-8 error was found.
    Utf8(std::str::Utf8Error),
}

impl Error {
    /// Create a new unexpected end error.
    pub fn unexpected_end() -> Error {
        Error::UnexpectedEnd
    }

    /// Create a new Utf8 error.
    pub fn utf8(error: std::str::Utf8Error) -> Error {
        Error::Utf8(error)
    }
}

/// A type that can be decoded from a [`Cursor`].
pub trait Decode: Sized {
    /// Begin the deserialization.
    fn decode<T>(cursor: &mut Cursor<T>) -> Result<Self, Error>
    where T: AsRef<[u8]>;
}
