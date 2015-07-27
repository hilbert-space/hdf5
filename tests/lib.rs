extern crate hdf5;
extern crate temporary;

#[cfg(feature = "serialize")]
extern crate rustc_serialize;

#[cfg(feature = "serialize")]
mod serialize;

use hdf5::File;
use temporary::Directory;

macro_rules! test(
    ($($name:ident := $value:expr,)*) => ({
        let directory = Directory::new("hdf5").unwrap();
        let file = File::new(directory.join("data.h5")).unwrap();
        $(file.write(stringify!($name), $value).unwrap();)*
    });
);

#[test]
fn write_boolean() {
    test!(
        a := true,
        b := false,
    );
}

#[test]
fn write_numeric_scalar() {
    test!(
        a := 42f32,
        b := 42f64,

        c := 42i8,
        d := 42u8,

        e := 42i16,
        f := 42u16,

        g := 42i32,
        h := 42u32,

        i := 42i64,
        j := 42u64,

        k := 42isize,
        l := 42usize,
    );
}

#[test]
fn write_numeric_vector() {
    test!(
        a := &vec![42f32, 69f32],
        b := &vec![42f64, 69f64],

        c := &vec![42i8, 69i8],
        d := &vec![42u8, 69u8],

        e := &vec![42i16, 69i16],
        f := &vec![42u16, 69u16],

        g := &vec![42i32, 69i32],
        h := &vec![42u32, 69u32],

        i := &vec![42i64, 69i64],
        j := &vec![42u64, 69u64],

        k := &vec![42isize, 69isize],
        l := &vec![42usize, 69usize],
    );
}

#[test]
fn write_overwrite() {
    test!(
        a := 42f32,
        a := 42f64,
    );
}

#[test]
fn write_text() {
    test!(
        a := '界',
        b := "Hello, 世界!",
    );
}

#[test]
fn version() {
    assert_eq!(hdf5::version().unwrap(), (1, 8, 15));
}
