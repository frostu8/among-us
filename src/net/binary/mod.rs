pub mod encode;
pub mod decode;

macro_rules! impl_num_decode {
    ($N:ty) => {
        impl decode::Decode for $N {
            fn decode<T>(cursor: &mut decode::Cursor<T>) -> Result<Self, decode::Error> 
            where T: AsRef<[u8]> {
                let mut buf = [0; ::std::mem::size_of::<$N>()];

                if cursor.read(&mut buf) < ::std::mem::size_of::<$N>() {
                    Err(decode::Error::unexpected_end())
                } else {
                    Ok(<$N>::from_le_bytes(buf))
                }
            }
        }
    }
}

impl_num_decode!(u8);
impl_num_decode!(u16);
impl_num_decode!(u32);
impl_num_decode!(u64);
impl_num_decode!(u128);

impl_num_decode!(i8);
impl_num_decode!(i16);
impl_num_decode!(i32);
impl_num_decode!(i64);
impl_num_decode!(i128);

macro_rules! impl_num_encode {
    ($N:ty) => {
        impl encode::Encode for $N {
            fn encode(&self, cursor: &mut encode::CursorMut) -> Result<(), encode::Error> {
                cursor.write(&self.to_le_bytes());
                Ok(())
            }
        }
    }
}

impl_num_encode!(u8);
impl_num_encode!(u16);
impl_num_encode!(u32);
impl_num_encode!(u64);
impl_num_encode!(u128);

impl_num_encode!(i8);
impl_num_encode!(i16);
impl_num_encode!(i32);
impl_num_encode!(i64);
impl_num_encode!(i128);

use std::iter::FromIterator as _;

use std::convert::TryInto as _;

// TODO: fix a memory allocation security flaw here. It is possible to tell
// clients to allocate 65535 bytes in memory, which isn't too much of a problem,
// but with many, many packets, this could easily overflow memory.
impl decode::Decode for String {
    fn decode<T>(cursor: &mut decode::Cursor<T>) -> Result<Self, decode::Error> 
    where T: AsRef<[u8]> {
        let count = cursor.decode::<u16>()? as usize;
        let mut buf = Vec::from_iter((0..count).map(|_| 0));

        if cursor.read(&mut buf[..]) < count {
            Err(decode::Error::unexpected_end())
        } else {
            String::from_utf8(buf).map_err(|e| decode::Error::utf8(e.utf8_error()))
        }
    }
}

impl encode::Encode for String {
    fn encode(&self, cursor: &mut encode::CursorMut) -> Result<(), encode::Error> {
        let count: u16 = match self.len().try_into() {
            Ok(count) => count,
            Err(_) => return Err(encode::Error),
        };

        cursor.encode(&count)?;
        cursor.write(self.as_bytes());

        Ok(())
    }
}
