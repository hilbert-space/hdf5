extern crate hdf5_sys as ffi;

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
    ok!(unsafe { ffi::H5get_libversion(&mut major as *mut _ as *mut _,
                                       &mut minor as *mut _ as *mut _,
                                       &mut patch as *mut _ as *mut _) });
    Ok((major, minor, patch))
}

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        assert_eq!(::version().unwrap(), (1, 8, 15));
    }
}
