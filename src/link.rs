use ffi;

use {ID, Result};

pub struct Link;

impl Link {
    pub fn exists(location: ID, name: &str) -> Result<bool> {
        let result = ok!(ffi::H5Lexists(location, str_to_cstr!(name).as_ptr(), ffi::H5P_DEFAULT),
                         "failed to check the existence of a link {:?}", name);
        Ok(result > 0)
    }

    pub fn delete(location: ID, name: &str) -> Result<()> {
        ok!(ffi::H5Ldelete(location, str_to_cstr!(name).as_ptr(), ffi::H5P_DEFAULT),
            "failed to delete a link {:?}", name);
        Ok(())
    }
}
