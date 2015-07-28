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

pub fn new(dimensions: &[usize], limits: Option<&[usize]>) -> Result<Dataspace> {
    macro_rules! convert(
        ($dimensions:expr) => (
            $dimensions.iter().map(|&one| one as ffi::hsize_t).collect::<Vec<_>>()
        );
    );
    let dimensions = convert!(dimensions);
    let limits = limits.map(|limits| convert!(limits));
    let limits = limits.map(|limits| limits.as_ptr());
    Ok(Dataspace {
        id: ok!(ffi::H5Screate_simple(dimensions.len() as libc::c_int, dimensions.as_ptr(),
                                      limits.unwrap_or(0 as *const _)),
                "failed to create a dataspace"),
    })
}
