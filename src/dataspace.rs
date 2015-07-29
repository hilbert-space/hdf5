use ffi;
use libc;

use {ID, Result};

pub struct Dataspace {
    id: ID,
}

raw!(Dataspace);

impl Dataspace {
    pub fn select(&self, position: &[usize], size: &[usize]) -> Result<()> {
        ok!(ffi::H5Sselect_hyperslab(self.id, ffi::H5S_SELECT_SET, position.as_ptr() as *const _,
                                     0 as *const _, size.as_ptr() as *const _, 0 as *const _),
            "failed to select the hyperslab region");
        Ok(())
    }
}

impl Drop for Dataspace {
    fn drop(&mut self) {
        whatever!(ffi::H5Sclose(self.id));
    }
}

pub fn new(dimensions: &[usize]) -> Result<Dataspace> {
    Ok(Dataspace {
        id: ok!(ffi::H5Screate_simple(dimensions.len() as libc::c_int,
                                      dimensions.as_ptr() as *const _, 0 as *const _),
                "failed to create a dataspace"),
    })
}

pub fn from_raw(id: ID) -> Dataspace {
    Dataspace { id: id }
}
