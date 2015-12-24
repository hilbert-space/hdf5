extern crate hdf5;
extern crate temporary;

#[cfg(feature = "serialize")]
extern crate rustc_serialize;

#[cfg(feature = "serialize")]
mod encode;

mod write;

#[test]
fn version() {
    let (major, minor, _) = hdf5::version().unwrap();
    assert_eq!((major, minor), (1, 8));
}
