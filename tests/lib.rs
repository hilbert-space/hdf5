extern crate hdf5;
extern crate temporary;

#[cfg(feature = "serialize")]
extern crate rustc_serialize;

#[cfg(feature = "serialize")]
mod encode;

mod write;

#[test]
fn version() {
    assert_eq!(hdf5::version().unwrap(), (1, 8, 15));
}
