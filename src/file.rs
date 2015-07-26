use ffi;
use std::path::Path;

use {ID, Result};

/// A file.
pub struct File {
    id: ID,
}

identity!(File);

impl File {
    /// Create a new file.
    ///
    /// If the file already exists, it will be truncated.
    pub fn new<T: AsRef<Path>>(path: T) -> Result<File> {
        Ok(File {
            id: ok!(ffi::H5Fcreate(path_to_c_str!(path.as_ref()), ffi::H5F_ACC_TRUNC,
                                   ffi::H5P_DEFAULT, ffi::H5P_DEFAULT),
                    "failed to create a file {:?}", path.as_ref()),
        })
    }

    /// Open an existing file.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<File> {
        Ok(File {
            id: ok!(ffi::H5Fopen(path_to_c_str!(path.as_ref()), ffi::H5F_ACC_RDWR,
                                 ffi::H5P_DEFAULT),
                    "failed to open a file {:?}", path.as_ref()),
        })
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = unsafe { ffi::H5Fclose(self.id) };
    }
}
