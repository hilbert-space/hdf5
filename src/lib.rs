//! Interface to [HDF5][1].
//!
//! ## Example
//!
//! ```
//! extern crate hdf5;
//! # extern crate temporary;
//!
//! use hdf5::File;
//! # use temporary::Directory;
//!
//! # fn main() {
//! let path = "data.h5";
//! # let directory = Directory::new("hdf5").unwrap();
//! # let path = directory.join(path);
//! let file = File::new(path).unwrap();
//!
//! file.write("foo", 42).unwrap();
//! file.write("bar", &vec![42.0, 69.0]).unwrap();
//! # }
//! ```
//!
//! Structural data can be written using [`rustc-serialize`][2] as follows:
//!
//! ```
//! extern crate hdf5;
//! extern crate rustc_serialize;
//! # extern crate temporary;
//!
//! use hdf5::File;
//! # use temporary::Directory;
//!
//! #[derive(RustcEncodable)]
//! struct Foo {
//!     bar: Vec<f64>,
//!     baz: Baz,
//! }
//!
//! #[derive(RustcEncodable)]
//! struct Baz {
//!     qux: f64,
//! }
//!
//! # fn main() {
//! let foo = Foo {
//!     bar: vec![42.0],
//!     baz: Baz {
//!         qux: 69.0,
//!     },
//! };
//!
//! let path = "data.h5";
//! # let directory = Directory::new("hdf5").unwrap();
//! # let path = directory.join(path);
//! let file = File::new(path).unwrap();
//!
//! file.encode("foo", &foo).unwrap();
//! # }
//!
//! [1]: http://www.hdfgroup.org/HDF5
//! [2]: https://crates.io/crates/rustc-serialize

extern crate hdf5_sys as ffi;
extern crate libc;

#[cfg(feature = "serialize")]
extern crate rustc_serialize;

use std::{error, fmt};

/// An error.
#[derive(Clone, Debug)]
pub struct Error(String);

/// An identifier.
pub type ID = ffi::hid_t;

/// A type having an identifier.
pub trait Identity {
    /// Return the identifier.
    fn id(&self) -> ID;
}

/// A type representing a location.
pub trait Location: Identity {
}

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! identity(
    ($name:ident) => (
        impl ::Identity for $name {
            #[inline]
            fn id(&self) -> ::ID {
                self.id
            }
        }
    );
);

macro_rules! location(
    ($name:ident) => (
        impl ::Location for $name {
        }
    );
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

macro_rules! path_to_cstr(
    ($path:expr) => ({
        let path = $path;
        match path.to_str() {
            Some(path) => match ::std::ffi::CString::new(path) {
                Ok(string) => string,
                _ => raise!("failed to process a path {:?}", path),
            },
            _ => raise!("failed to process a path {:?}", path),
        }
    });
);

macro_rules! product(
    ($vector:expr) => (
        $vector.iter().fold(1, |result, &next| result * next)
    );
);

macro_rules! raise(
    ($($arg:tt)*) => (return Err(::Error(format!($($arg)*))));
);

macro_rules! str_to_cstr(
    ($string:expr) => ({
        let string = $string;
        match ::std::ffi::CString::new(string) {
            Ok(string) => string,
            _ => raise!("failed to process a string {:?}", string),
        }
    });
);

macro_rules! whatever(
    ($call:expr) => ({
        let _ = unsafe { $call };
    });
);

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

impl<'l, T: Identity> Identity for &'l T {
    #[inline]
    fn id(&self) -> ID {
        (*self).id()
    }
}

impl<'l, T: Location> Location for &'l T {
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
mod writer;

#[cfg(feature = "serialize")]
mod decoder;
#[cfg(feature = "serialize")]
mod encoder;

pub use data::{Data, IntoData, Slice};
pub use datatype::Datatype;
pub use file::File;
pub use writer::Writer;

#[cfg(feature = "serialize")]
pub use decoder::Decoder;
#[cfg(feature = "serialize")]
pub use encoder::Encoder;
