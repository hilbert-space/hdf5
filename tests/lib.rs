extern crate hdf5;
extern crate temporary;

#[cfg(feature = "serialize")]
extern crate rustc_serialize;

use hdf5::File;
use temporary::Directory;

#[cfg(feature = "serialize")]
#[test]
fn encode_scalar() {
    use hdf5::Encoder;
    use rustc_serialize::Encodable;

    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    macro_rules! test(
        ($name:expr, $value:expr) => ({
            let mut encoder = Encoder::new(&file, $name).unwrap();
            $value.encode(&mut encoder).unwrap();
        });
    );

    test!("a", 42f32);
    test!("b", 42f64);

    test!("c", 42i8);
    test!("d", 42u8);

    test!("e", 42i16);
    test!("f", 42u16);

    test!("g", 42i32);
    test!("h", 42u32);

    test!("i", 42i64);
    test!("j", 42u64);

    test!("k", 42isize);
    test!("l", 42usize);
}

#[test]
fn write_scalar() {
    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    macro_rules! test(
        ($name:expr, $value:expr) => (file.write($name, $value).unwrap());
    );

    test!("a", 42f32);
    test!("b", 42f64);

    test!("c", 42i8);
    test!("d", 42u8);

    test!("e", 42i16);
    test!("f", 42u16);

    test!("g", 42i32);
    test!("h", 42u32);

    test!("i", 42i64);
    test!("j", 42u64);

    test!("k", 42isize);
    test!("l", 42usize);
}

#[test]
fn write_vector() {
    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    macro_rules! test(
        ($name:expr, $value:expr) => (file.write($name, $value).unwrap());
    );

    test!("a", &vec![42f32]);
    test!("b", &vec![42f64]);

    test!("c", &vec![42i8]);
    test!("d", &vec![42u8]);

    test!("e", &vec![42i16]);
    test!("f", &vec![42u16]);

    test!("g", &vec![42i32]);
    test!("h", &vec![42u32]);

    test!("i", &vec![42i64]);
    test!("j", &vec![42u64]);

    test!("k", &vec![42isize]);
    test!("l", &vec![42usize]);
}

#[test]
fn write_overwrite() {
    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    macro_rules! test(
        ($name:expr, $value:expr) => (file.write($name, $value).unwrap());
    );

    test!("a", 42f32);
    test!("a", 42f64);
}

#[test]
fn version() {
    assert_eq!(hdf5::version().unwrap(), (1, 8, 15));
}

fn setup() -> Directory {
    Directory::new("hdf5").unwrap()
}
