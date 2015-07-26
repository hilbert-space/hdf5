use ffi;
use std::path::Path;

use data::Data;
use dataset;
use dataspace;
use link::Link;
use {ID, Result};

/// A file.
pub struct File {
    id: ID,
}

identity!(File);

impl File {
    /// Create a file.
    ///
    /// If the file already exists, it will be truncated.
    pub fn new<T: AsRef<Path>>(path: T) -> Result<File> {
        Ok(File {
            id: ok!(ffi::H5Fcreate(path_to_c_str!(path.as_ref()), ffi::H5F_ACC_TRUNC,
                                   ffi::H5P_DEFAULT, ffi::H5P_DEFAULT),
                    "failed to create a file {:?}", path.as_ref()),
        })
    }

    /// Open a file.
    ///
    /// The file should already exist.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<File> {
        Ok(File {
            id: ok!(ffi::H5Fopen(path_to_c_str!(path.as_ref()), ffi::H5F_ACC_RDWR,
                                 ffi::H5P_DEFAULT),
                    "failed to open a file {:?}", path.as_ref()),
        })
    }

    /// Write data.
    pub fn write<T: Data>(&self, name: &str, data: T) -> Result<()> {
        let dataspace = try!(dataspace::new(&[1]));
        if try!(Link::exists(self, name)) {
            try!(Link::delete(self, name));
        }
        let dataset = try!(dataset::new(self, name, &data.datatype(), &dataspace));
        try!(dataset.write(data));
        Ok(())
    }
}

impl Drop for File {
    fn drop(&mut self) {
        whatever!(ffi::H5Fclose(self.id));
    }
}
