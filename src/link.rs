use ffi;

use {Identity, Result};

pub struct Link;

impl Link {
    pub fn exists<T: Identity>(location: T, name: &str) -> Result<bool> {
        let result = ok!(ffi::H5Lexists(location.id(), str_to_c_str!(name), ffi::H5P_DEFAULT),
                         "failed to check the existence of a link {:?}", name);
        Ok(result > 0)
    }

    pub fn delete<T: Identity>(location: T, name: &str) -> Result<()> {
        ok!(ffi::H5Ldelete(location.id(), str_to_c_str!(name), ffi::H5P_DEFAULT),
            "failed to delete a link {:?}", name);
        Ok(())
    }
}
