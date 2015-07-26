use ffi;

use {ID, Result};

pub struct Dataspace {
    id: ID,
}

identity!(Dataspace);

impl Drop for Dataspace {
    fn drop(&mut self) {
        let _ = unsafe { ffi::H5Sclose(self.id) };
    }
}

pub fn new() -> Result<Dataspace> {
    Ok(Dataspace {
        id: ok!(ffi::H5Screate_simple(1, &1, 0 as *const _), "failed to create a dataspace"),
    })
}
