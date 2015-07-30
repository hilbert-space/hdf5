use ffi;
use std::path::Path;

use data::{Data, IntoData};
use writer::Writer;
use {ID, Identity, Result};

#[cfg(feature = "serialize")]
use encoder::Encoder;

#[cfg(feature = "serialize")]
use rustc_serialize::Encodable;

/// A file.
pub struct File {
    id: ID,
}

identity!(File);
location!(File);

impl File {
    /// Create a new file.
    ///
    /// If the file already exists, its content will be truncated.
    pub fn new<T: AsRef<Path>>(path: T) -> Result<File> {
        Ok(File {
            id: ok!(ffi::H5Fcreate(path_to_cstr!(path.as_ref()).as_ptr(), ffi::H5F_ACC_TRUNC,
                                   ffi::H5P_DEFAULT, ffi::H5P_DEFAULT),
                    "failed to create a file {:?}", path.as_ref()),
        })
    }

    /// Open an existing file.
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
    pub fn encode<T: Encodable>(&self, name: &str, data: T) -> Result<()> {
        let mut encoder = Encoder::new(self, name);
        data.encode(&mut encoder)
    }

    /// Write data.
    ///
    /// The function is a shortcut for `Writer::new` followed by
    /// `Writer::write`.
    pub fn write<T: IntoData>(&self, name: &str, data: T) -> Result<()> {
        let data = try!(data.into_data());
        let dimensions = data.dimensions();
        let mut writer = Writer::new(self, name, dimensions);
        writer.write(&data, &vec![0; dimensions.len()], dimensions)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        whatever!(ffi::H5Fclose(self.id));
    }
}
