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

pub fn new<L, T, S>(location: L, name: &str, datatype: T, dataspace: S) -> Result<Dataset>
    where L: Identity, T: Identity, S: Identity
{
    Ok(Dataset {
        id: ok!(ffi::H5Dcreate2(location.id(), str_to_cstr!(name).as_ptr(), datatype.id(),
                                dataspace.id(), ffi::H5P_DEFAULT, ffi::H5P_DEFAULT,
                                ffi::H5P_DEFAULT),
                "failed to create a dataset {:?}", name),
    })
}
