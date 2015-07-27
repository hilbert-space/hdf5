extern crate hdf5;
extern crate temporary;

#[cfg(feature = "serialize")]
extern crate rustc_serialize;

use hdf5::File;
use temporary::Directory;

#[cfg(feature = "serialize")]
#[test]
fn encode_compound() {
    use hdf5::Encoder;
    use rustc_serialize::Encodable;

    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    #[derive(RustcEncodable)]
    struct Foo {
        a: f32,
        b: f64,

        c: i8,
        d: u8,

        e: i16,
        f: u16,

        g: i32,
        h: u32,

        i: i64,
        j: u64,

        k: isize,
        l: usize,
    }

    let foo = Foo {
        a: 42f32,
        b: 42f64,

        c: 42i8,
        d: 42u8,

        e: 42i16,
        f: 42u16,

        g: 42i32,
        h: 42u32,

        i: 42i64,
        j: 42u64,

        k: 42isize,
        l: 42usize,
    };

    let mut encoder = Encoder::new(&file, "foo").unwrap();
    foo.encode(&mut encoder).unwrap();
}

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

#[cfg(feature = "serialize")]
#[test]
fn encode_text() {
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

    test!("a", '界');
    test!("b", "Hello, 世界!");
}

#[cfg(feature = "serialize")]
#[test]
fn encode_vector() {
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

    test!("a", vec![42f32, 69f32]);
    test!("b", vec![42f64, 69f64]);

    test!("c", vec![42i8, 69i8]);
    test!("d", vec![42u8, 69u8]);

    test!("e", vec![42i16, 69i16]);
    test!("f", vec![42u16, 69u16]);

    test!("g", vec![42i32, 69i32]);
    test!("h", vec![42u32, 69u32]);

    test!("i", vec![42i64, 69i64]);
    test!("j", vec![42u64, 69u64]);

    test!("k", vec![42isize, 69isize]);
    test!("l", vec![42usize, 69usize]);
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
fn write_text() {
    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    macro_rules! test(
        ($name:expr, $value:expr) => (file.write($name, $value).unwrap());
    );

    test!("a", '界');
    test!("b", "Hello, 世界!");
}

#[test]
fn write_vector() {
    let directory = setup();
    let file = File::new(directory.join("data.h5")).unwrap();

    macro_rules! test(
        ($name:expr, $value:expr) => (file.write($name, $value).unwrap());
    );

    test!("a", &vec![42f32, 69f32]);
    test!("b", &vec![42f64, 69f64]);

    test!("c", &vec![42i8, 69i8]);
    test!("d", &vec![42u8, 69u8]);

    test!("e", &vec![42i16, 69i16]);
    test!("f", &vec![42u16, 69u16]);

    test!("g", &vec![42i32, 69i32]);
    test!("h", &vec![42u32, 69u32]);

    test!("i", &vec![42i64, 69i64]);
    test!("j", &vec![42u64, 69u64]);

    test!("k", &vec![42isize, 69isize]);
    test!("l", &vec![42usize, 69usize]);
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
