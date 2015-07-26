extern crate hdf5_sys as ffi;
extern crate libc;

#[cfg(feature = "serialize")]
extern crate rustc_serialize;

use std::{error, fmt};

/// An error.
#[derive(Clone, Debug)]
pub struct Error(String);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

/// An identifier.
pub type ID = ffi::hid_t;

/// An identity.
pub trait Identity {
    /// Return the identifier.
    fn id(&self) -> ID;
}

macro_rules! identity(
    ($name:ident) => (
        impl ::Identity for $name {
            #[inline]
            fn id(&self) -> ::ID {
                self.id
            }
        }

        impl<'l> ::Identity for &'l $name {
            #[inline]
            fn id(&self) -> ::ID {
                self.id
            }
        }
    );
);

macro_rules! raise(
    ($($arg:tt)*) => (return Err(::Error(format!($($arg)*))));
);

macro_rules! ok(
    ($call:expr) => ({
        let result = unsafe { $call };
        if result < 0 {
            raise!("failed to call a native function (error code {})", result);
        }
        result
    });
    ($call:expr, $($arg:tt)+) => ({
        let result = unsafe { $call };
        if result < 0 {
            raise!($($arg)+);
        }
        result
    });
);

macro_rules! whatever(
    ($call:expr) => ({
        let _ = unsafe { $call };
    });
);

macro_rules! path_to_c_str(
    ($path:expr) => ({
        let path = $path;
        match path.to_str() {
            Some(path) => match ::std::ffi::CString::new(path) {
                Ok(string) => string.as_ptr(),
                _ => raise!("failed to process a path {:?}", path),
            },
            _ => raise!("failed to process a path {:?}", path),
        }
    });
);

macro_rules! str_to_c_str(
    ($string:expr) => ({
        let string = $string;
        match ::std::ffi::CString::new(string) {
            Ok(string) => string.as_ptr(),
            _ => raise!("failed to process a string {:?}", string),
        }
    });
);

impl Identity for ID {
    #[inline]
    fn id(&self) -> ID {
        *self
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.0
    }
}

/// Return the version number of HDF5.
pub fn version() -> Result<(usize, usize, usize)> {
    let (mut major, mut minor, mut patch) = (0, 0, 0);
    ok!(ffi::H5get_libversion(&mut major as *mut _ as *mut _, &mut minor as *mut _ as *mut _,
                              &mut patch as *mut _ as *mut _));
    Ok((major, minor, patch))
}

mod data;
mod dataset;
mod dataspace;
mod datatype;
mod file;
mod link;

#[cfg(feature = "serialize")]
mod decoder;
#[cfg(feature = "serialize")]
mod encoder;

pub use data::{Data, IntoData, Slice};
pub use datatype::Datatype;
pub use file::File;

#[cfg(feature = "serialize")]
pub use decoder::Decoder;
#[cfg(feature = "serialize")]
pub use encoder::Encoder;
