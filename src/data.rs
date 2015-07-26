use ffi;
use std::{mem, slice};

use Result;
use datatype::{self, Datatype};

/// Data.
pub trait Data {
    /// Return the data.
    fn as_bytes(&self) -> &[u8];

    /// Return the datatype.
    fn datatype(&self) -> Datatype;
}

/// A type capable of converting itself into `Data`.
pub trait IntoData {
    /// The target type.
    type Target: Data;

    /// Perform the conversion.
    fn into_data(self) -> Result<Self::Target>;
}

/// An array.
pub struct Array<T: Data> {
    data: Vec<T>,
    datatype: Datatype,
}

/// A slice.
pub struct Slice<'l, T: Data + 'l> {
    data: &'l [T],
    datatype: Datatype,
}

macro_rules! implement(
    ($name:ty, $datatype:expr) => (
        impl Data for $name {
            #[inline]
            fn as_bytes(&self) -> &[u8] {
                unsafe {
                    slice::from_raw_parts(self as *const _ as *const _, mem::size_of::<$name>())
                }
            }

            #[inline]
            fn datatype(&self) -> Datatype {
                datatype::new_foreign($datatype)
            }
        }

        impl IntoData for Vec<$name> {
            type Target = Array<$name>;

            #[inline]
            fn into_data(self) -> Result<Self::Target> {
                let datatype = try!(datatype::new_array($datatype, &[1, self.len()]));
                Ok(Array { data: self, datatype: datatype })
            }
        }

        impl<'l> IntoData for &'l [$name] {
            type Target = Slice<'l, $name>;

            #[inline]
            fn into_data(self) -> Result<Self::Target> {
                let datatype = try!(datatype::new_array($datatype, &[1, self.len()]));
                Ok(Slice { data: self, datatype: datatype })
            }
        }

        impl<'l> IntoData for &'l Vec<$name> {
            type Target = Slice<'l, $name>;

            #[inline]
            fn into_data(self) -> Result<Self::Target> {
                (self as &[$name]).into_data()
            }
        }
    );
);

implement!(char, ffi::H5T_NATIVE_UINT32);

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

impl<T: Data> Data for Array<T> {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.data.as_ptr() as *const _ as *const _,
                                  mem::size_of::<T>() * self.data.len())
        }
    }

    #[inline]
    fn datatype(&self) -> Datatype {
        self.datatype.clone()
    }
}

impl<'l, T: Data> Data for Slice<'l, T> {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.data.as_ptr() as *const _ as *const _,
                                  mem::size_of::<T>() * self.data.len())
        }
    }

    #[inline]
    fn datatype(&self) -> Datatype {
        self.datatype.clone()
    }
}

impl<'l> Data for &'l str {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        str::as_bytes(self)
    }

    #[inline]
    fn datatype(&self) -> Datatype {
        datatype::new_foreign(ffi::H5T_C_S1)
    }
}

impl<T: Data> IntoData for T {
    type Target = T;

    #[inline]
    fn into_data(self) -> Result<Self::Target> {
        Ok(self)
    }
}

#[inline]
pub fn new_array<T: Data>(data: Vec<T>, datatype: Datatype) -> Result<Array<T>> {
    Ok(Array { data: data, datatype: datatype })
}
