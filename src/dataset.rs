use ffi;

use {ID, Identity, Result};

pub struct Dataset {
    id: ID,
}

identity!(Dataset);

impl Drop for Dataset {
    fn drop(&mut self) {
        let _ = unsafe { ffi::H5Dclose(self.id) };
    }
}

pub fn new<L, T, S>(location: L, name: &str, datatype: T, dataspace: S) -> Result<Dataset>
    where L: Identity, T: Identity, S: Identity
{
    Ok(Dataset {
        id: ok!(ffi::H5Dcreate2(location.id(), str_to_c_str!(name), datatype.id(), dataspace.id(),
                                ffi::H5P_DEFAULT, ffi::H5P_DEFAULT, ffi::H5P_DEFAULT),
                "failed to create a dataset {:?}", name),
    })
}
