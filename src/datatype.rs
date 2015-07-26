use ffi;

use ID;

pub struct Datatype {
    id: ID,
    owned: bool,
}

identity!(Datatype);

impl Drop for Datatype {
    fn drop(&mut self) {
        if self.owned {
            whatever!(ffi::H5Tclose(self.id));
        }
    }
}

#[inline]
pub fn native(id: ID) -> Datatype {
    Datatype { id: id, owned: false }
}
