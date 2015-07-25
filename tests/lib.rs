extern crate hdf5;
extern crate temporary;

use hdf5::File;
use temporary::Directory;

#[test]
fn version() {
    assert_eq!(hdf5::version().unwrap(), (1, 8, 15));
}

#[test]
fn workflow() {
    let directory = setup();
    let _ = File::new(directory.join("data.h5")).unwrap();
}

fn setup() -> Directory {
    Directory::new("hdf5").unwrap()
}
