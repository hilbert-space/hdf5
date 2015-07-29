use ffi;

use data::Data;
use dataspace::{self, Dataspace};
use {ID, Raw, Result};

pub struct Dataset {
    id: ID,
}

raw!(Dataset);

impl Dataset {
    pub fn write<T: Data>(&self, data: T, memory_space: ID, file_space: ID) -> Result<()> {
        ok!(ffi::H5Dwrite(self.id, data.datatype().id(), memory_space, file_space,
                          ffi::H5P_DEFAULT, data.as_bytes().as_ptr() as *const _),
            "failed to write the data");
        Ok(())
    }

    pub fn space(&self) -> Result<Dataspace> {
        Ok(dataspace::from_raw(ok!(ffi::H5Dget_space(self.id), "failed to get the dataspace")))
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
