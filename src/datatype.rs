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

macro_rules! new(
    ($id:expr, $owned: expr) => (
        Datatype(Rc::new(Inner { id: $id, owned: $owned }))
    );
);

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

#[inline]
pub fn from_raw_borrowed(id: ID) -> Datatype {
    new!(id, false)
}

#[cfg(feature = "serialize")]
pub fn new_compound(fields: &[(String, Datatype, usize)]) -> Result<Datatype> {
    let size = fields.iter().fold(0, |sum, &(_, _, size)| sum + size) as libc::size_t;
    let id = ok!(ffi::H5Tcreate(ffi::H5T_COMPOUND, size), "failed to create a compound datatype");
    let mut offset = 0;
    for &(ref name, ref datatype, size) in fields.iter() {
        ok!(ffi::H5Tinsert(id, str_to_cstr!(&name[..]).as_ptr(), offset as libc::size_t,
                           datatype.id()));
        offset += size;
    }
    Ok(new!(id, true))
}

pub fn new_string(length: usize) -> Result<Datatype> {
    let id = ok!(ffi::H5Tcopy(ffi::H5T_C_S1), "failed to create a string datatype");
    ok!(ffi::H5Tset_size(id, length as libc::size_t),
        "failed to set the size of a string datatype");
    ok!(ffi::H5Tset_cset(id, ffi::H5T_CSET_UTF8));
    Ok(new!(id, true))
}
