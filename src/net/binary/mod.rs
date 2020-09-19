pub mod encode;
pub mod decode;

macro_rules! impl_num_encode {
    ($N:ty) => {
        impl decode::Decode for $N {
            type Error = !;

            fn decode<T>(cursor: &mut decode::Cursor<T>) -> Result<Self, decode::Error<Self::Error>> 
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
