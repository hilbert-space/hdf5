extern crate hdf5;
extern crate temporary;

use hdf5::File;
use temporary::Directory;

#[test]
fn version() {
    assert_eq!(hdf5::version().unwrap(), (1, 8, 15));
}

#[test]
fn write_scalar() {
    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    file.write("a", 42f32).unwrap();
    file.write("b", 42f64).unwrap();

    file.write("c", 42i8).unwrap();
    file.write("d", 42u8).unwrap();

    file.write("e", 42i16).unwrap();
    file.write("f", 42u16).unwrap();

    file.write("g", 42i32).unwrap();
    file.write("h", 42u32).unwrap();

    file.write("i", 42i64).unwrap();
    file.write("j", 42u64).unwrap();

    file.write("k", 42isize).unwrap();
    file.write("l", 42usize).unwrap();
}

#[test]
fn write_overwrite() {
    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    file.write("a", 42f32).unwrap();
    file.write("a", 42f64).unwrap();
}

fn setup() -> Directory {
    Directory::new("hdf5").unwrap()
}
