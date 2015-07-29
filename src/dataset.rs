use ffi;

use data::Data;
use dataspace::{self, Dataspace};
use datatype::Datatype;
use {ID, Identity, Location, Result};

pub struct Dataset {
    id: ID,
}

identity!(Dataset);

impl Dataset {
    pub fn write<T: Data>(&self, data: T, memory_space: &Dataspace, file_space: &Dataspace)
                          -> Result<()> {

        ok!(ffi::H5Dwrite(self.id, data.datatype().id(), memory_space.id(), file_space.id(),
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

pub fn new<T: Location>(location: T, name: &str, datatype: &Datatype, dataspace: &Dataspace)
                        -> Result<Dataset> {

    Ok(Dataset {
        id: ok!(ffi::H5Dcreate2(location.id(), str_to_cstr!(name).as_ptr(), datatype.id(),
                                dataspace.id(), ffi::H5P_DEFAULT, ffi::H5P_DEFAULT,
                                ffi::H5P_DEFAULT),
                "failed to create a dataset {:?}", name),
    })
}
