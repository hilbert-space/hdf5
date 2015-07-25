extern crate hdf5_sys as ffi;
extern crate rustc_serialize as serialize;

use std::{error, fmt};

/// An error.
#[derive(Clone, Debug)]
pub struct Error(String);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error($message.to_string())));
);

macro_rules! ok(
    ($result:expr) => (if $result < 0 {
        raise!("received an error code from the HDF5 API");
    });
);

macro_rules! path_to_c_str(
    ($path:expr) => (match $path.to_str() {
        Some(path) => match ::std::ffi::CString::new(path) {
            Ok(string) => string.as_ptr(),
            _ => raise!("failed to process a path"),
        },
        _ => raise!("failed to process a path"),
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

/// Return the version number of HDF5.
pub fn version() -> Result<(usize, usize, usize)> {
    let (mut major, mut minor, mut patch) = (0, 0, 0);
    unsafe {
        ok!(ffi::H5get_libversion(&mut major as *mut _ as *mut _, &mut minor as *mut _ as *mut _,
                                  &mut patch as *mut _ as *mut _));
    }
    Ok((major, minor, patch))
}

mod decoder;
mod encoder;
mod file;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use file::File;
