use ffi;

use data::Data;
use {ID, Identity, Result};

pub struct Dataset {
    id: ID,
}

identity!(Dataset);

impl Dataset {
    pub fn write<T: Data>(&self, data: T) -> Result<()> {
        ok!(ffi::H5Dwrite(self.id, data.datatype().id(), ffi::H5S_ALL, ffi::H5S_ALL,
                          ffi::H5P_DEFAULT, data.as_bytes().as_ptr() as *const _));
        Ok(())
    }
}

impl Drop for Dataset {
    fn drop(&mut self) {
        whatever!(ffi::H5Dclose(self.id));
    }
}

pub fn new(location: ID, name: &str, datatype: ID, dataspace: ID) -> Result<Dataset> {
    Ok(Dataset {
        id: ok!(ffi::H5Dcreate2(location, str_to_cstr!(name).as_ptr(), datatype, dataspace,
                                ffi::H5P_DEFAULT, ffi::H5P_DEFAULT, ffi::H5P_DEFAULT),
                "failed to create a dataset {:?}", name),
    })
}
