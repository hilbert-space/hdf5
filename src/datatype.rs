use ffi;
use libc;

use {Result, ID, Identity};

/// A datatype.
pub struct Datatype {
    id: ID,
    owned: bool,
}

identity!(Datatype);

impl Clone for Datatype {
    #[inline]
    fn clone(&self) -> Self {
        Datatype { id: self.id, owned: false }
    }
}

impl Drop for Datatype {
    fn drop(&mut self) {
        if self.owned {
            whatever!(ffi::H5Tclose(self.id));
        }
    }
}

impl PartialEq for Datatype {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[inline]
pub fn new_array<T: Identity>(datatype: T, dimensions: &[usize]) -> Result<Datatype> {
    let dimensions = dimensions.iter().map(|&dimension| dimension as ffi::hsize_t)
                                      .collect::<Vec<_>>();
    Ok(Datatype {
        id: ok!(ffi::H5Tarray_create2(datatype.id(), dimensions.len() as libc::c_uint,
                                      dimensions.as_ptr()),
                "failed to create a datatype"),
        owned: true,
    })
}

#[inline]
pub fn new_foreign(id: ID) -> Datatype {
    Datatype { id: id, owned: false }
}
