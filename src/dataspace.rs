use ffi;
use libc;

use {ID, Result};

pub struct Dataspace {
    id: ID,
}

identity!(Dataspace);

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
