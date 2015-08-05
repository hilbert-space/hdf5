use ffi;
use std::{mem, slice};

use Result;
use datatype::{self, Datatype};

const SCALAR_DIMENSIONS: &'static [usize] = &[1];

/// An object suitable for storing.
pub trait Data {
    /// Return the raw data.
    fn as_bytes(&self) -> &[u8];

    /// Return the datatype.
    fn datatype(&self) -> Datatype;

    /// Return the dimensions.
    fn dimensions(&self) -> &[usize];
}

/// An object capable of converting into data.
pub trait IntoData {
    /// The target type.
    type Target: Data;

    /// Perform the conversion.
    fn into_data(self) -> Result<Self::Target>;
}

#[doc(hidden)]
pub struct Slice<'l, T: 'l> {
    data: &'l [T],
    datatype: Datatype,
    dimensions: [usize; 1],
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
                datatype::from_raw_borrowed($datatype)
            }

            #[inline]
            fn dimensions(&self) -> &[usize] {
                SCALAR_DIMENSIONS
            }
        }

        impl<'l> IntoData for &'l [$name] {
            type Target = Slice<'l, $name>;

            #[inline]
            fn into_data(self) -> Result<Self::Target> {
                Ok(Slice {
                    data: self,
                    datatype: datatype::from_raw_borrowed($datatype),
                    dimensions: [self.len()],
                })
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

implement!(bool, ffi::H5T_NATIVE_UINT8);

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

impl<'l, T: Data> Data for &'l T {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        (*self).as_bytes()
    }

    #[inline]
    fn datatype(&self) -> Datatype {
        (*self).datatype()
    }

    #[inline]
    fn dimensions(&self) -> &[usize] {
        (*self).dimensions()
    }
}

impl<'l, T> Data for Slice<'l, T> {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.data.as_ptr() as *const _,
                                  mem::size_of::<T>() * self.data.len())
        }
    }

    #[inline]
    fn datatype(&self) -> Datatype {
        self.datatype.clone()
    }

    #[inline]
    fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }
}

impl<T: Data> IntoData for T {
    type Target = T;

    #[inline]
    fn into_data(self) -> Result<Self::Target> {
        Ok(self)
    }
}

impl<'l> IntoData for &'l str {
    type Target = Slice<'l, u8>;

    #[inline]
    fn into_data(self) -> Result<Self::Target> {
        let datatype = try!(datatype::new_string(self.len()));
        Ok(Slice { data: self.as_bytes(), datatype: datatype, dimensions: [1] })
    }
}
