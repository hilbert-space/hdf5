use ffi;
use libc;
use std::rc::Rc;

use {Result, ID, Identity};

/// A datatype.
#[derive(Clone)]
pub struct Datatype(Rc<Inner>);

struct Inner {
    id: ID,
    owned: bool,
}

impl Datatype {
    /// Return the size in bytes.
    pub fn size(&self) -> Result<usize> {
        let size = unsafe { ffi::H5Tget_size(self.0.id) };
        if size <= 0 {
            raise!("failed to read the size");
        }
        Ok(size as usize)
    }
}

impl Identity for Datatype {
    #[inline]
    fn id(&self) -> ID {
        self.0.id
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        if self.owned {
            whatever!(ffi::H5Tclose(self.id));
        }
    }
}

impl PartialEq for Datatype {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

pub fn new_array<T: Identity>(datatype: T, dimensions: &[usize]) -> Result<Datatype> {
    let dimensions = dimensions.iter().map(|&size| size as ffi::hsize_t).collect::<Vec<_>>();
    let id = ok!(ffi::H5Tarray_create2(datatype.id(), dimensions.len() as libc::c_uint,
                                       dimensions.as_ptr()),
                 "failed to create a datatype");
    Ok(Datatype(Rc::new(Inner { id: id, owned: true })))
}

#[cfg(feature = "serialize")]
pub fn new_compound(fields: &[(String, Datatype, usize)]) -> Result<Datatype> {
    let size = fields.iter().fold(0, |sum, &(_, _, size)| sum + size) as libc::size_t;
    let id = ok!(ffi::H5Tcreate(ffi::H5T_COMPOUND, size), "failed to create a compound datatype");
    let mut offset = 0;
    for &(ref name, ref datatype, size) in fields.iter() {
        ok!(ffi::H5Tinsert(id, str_to_c_str!(&name[..]), offset as libc::size_t, datatype.id()));
        offset += size;
    }
    Ok(Datatype(Rc::new(Inner { id: id, owned: true })))
}

#[inline]
pub fn new_foreign(id: ID) -> Datatype {
    Datatype(Rc::new(Inner { id: id, owned: false }))
}
