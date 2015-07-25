use ffi;
use std::path::Path;

use Result;

/// A file.
pub struct File {
    raw: ffi::hid_t,
}

impl File {
    /// Create a new file.
    ///
    /// If the file already exists, it will be truncated.
    pub fn new<T: AsRef<Path>>(path: T) -> Result<File> {
        let raw = unsafe {
            ffi::H5Fcreate(path_to_c_str!(path.as_ref()), ffi::H5F_ACC_TRUNC, ffi::H5P_DEFAULT,
                           ffi::H5P_DEFAULT)
        };
        if raw < 0 {
            raise!("failed to create a file");
        }
        Ok(File { raw: raw })
    }

    /// Open an existing file.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<File> {
        let raw = unsafe {
            ffi::H5Fopen(path_to_c_str!(path.as_ref()), ffi::H5F_ACC_RDWR, ffi::H5P_DEFAULT)
        };
        if raw < 0 {
            raise!("failed to open a file");
        }
        Ok(File { raw: raw })
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = unsafe { ffi::H5Fclose(self.raw) };
    }
}
