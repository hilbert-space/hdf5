use ffi;
use std::{mem, slice};

use datatype::{self, Datatype};

/// Data.
pub trait Data {
    /// Return the data.
    fn as_bytes(&self) -> &[u8];

    /// Return the datatype.
    fn datatype(&self) -> Datatype;
}

macro_rules! implement(
    ($name:ty, $datatype:expr) => (
        impl Data for $name {
            #[inline]
            fn as_bytes(&self) -> &[u8] {
                unsafe {
                    slice::from_raw_parts(self as *const _ as *const u8, mem::size_of::<$name>())
                }
            }

            #[inline]
            fn datatype(&self) -> Datatype {
                datatype::native($datatype)
            }
        }
    );
);

implement!(f32, ffi::H5T_NATIVE_FLOAT);
implement!(f64, ffi::H5T_NATIVE_DOUBLE);

implement!(i8, ffi::H5T_NATIVE_INT8);
implement!(u8, ffi::H5T_NATIVE_UINT8);

implement!(i16, ffi::H5T_NATIVE_INT16);
implement!(u16, ffi::H5T_NATIVE_UINT16);

implement!(i32, ffi::H5T_NATIVE_INT32);
implement!(u32, ffi::H5T_NATIVE_UINT32);

implement!(i64, ffi::H5T_NATIVE_INT64);
implement!(u64, ffi::H5T_NATIVE_UINT64);

#[cfg(target_pointer_width = "32")]
implement!(isize, ffi::H5T_NATIVE_INT32);
#[cfg(target_pointer_width = "32")]
implement!(usize, ffi::H5T_NATIVE_UINT32);

#[cfg(target_pointer_width = "64")]
implement!(isize, ffi::H5T_NATIVE_INT64);
#[cfg(target_pointer_width = "64")]
implement!(usize, ffi::H5T_NATIVE_UINT64);
