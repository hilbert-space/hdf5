use ffi;
use std::path::Path;

use data::{Data, IntoData};
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
    pub fn new<T: AsRef<Path>>(path: T) -> Result<File> {
        Ok(File {
            id: ok!(ffi::H5Fcreate(path_to_cstr!(path.as_ref()).as_ptr(), ffi::H5F_ACC_TRUNC,
                                   ffi::H5P_DEFAULT, ffi::H5P_DEFAULT),
                    "failed to create a file {:?}", path.as_ref()),
        })
    }

    /// Open a file.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<File> {
        Ok(File {
            id: ok!(ffi::H5Fopen(path_to_cstr!(path.as_ref()).as_ptr(), ffi::H5F_ACC_RDWR,
                                 ffi::H5P_DEFAULT),
                    "failed to open a file {:?}", path.as_ref()),
        })
    }

    /// Encode data.
    ///
    /// The function is a shortcut for `Encoder::new` followed by
    /// `Encodable::encode`.
    #[cfg(feature = "serialize")]
    pub fn encode<T: ::rustc_serialize::Encodable>(&self, name: &str, data: T) -> Result<()> {
        use encoder::Encoder;
        use rustc_serialize::Encodable;

        let mut encoder = try!(Encoder::new(self, name));
        data.encode(&mut encoder)
    }

    /// Write data.
    ///
    /// This function writes directly into the file without intermediate buffers
    /// as it is the case when using encoders.
    pub fn write<T: IntoData>(&self, name: &str, data: T) -> Result<()> {
        let dataspace = try!(dataspace::new(&[1]));
        if try!(Link::exists(self, name)) {
            try!(Link::delete(self, name));
        }
        let data = try!(data.into_data());
        let dataset = try!(dataset::new(self, name, data.datatype(), &dataspace));
        try!(dataset.write(data));
        Ok(())
    }
}

impl Drop for File {
    fn drop(&mut self) {
        whatever!(ffi::H5Fclose(self.id));
    }
}
