use hdf5::{Encoder, File};
use rustc_serialize::Encodable;
use setup;

#[test]
fn encode_compound() {
    let directory = setup();
    let file = File::new("data.h5").unwrap();

    #[derive(RustcEncodable)]
    struct Foo {
        a: f64,
        b: i64,
        c: Vec<u64>,
        d: [usize; 2],
        e: Bar,
    }

    #[derive(RustcEncodable)]
    struct Bar {
        a: f32,
        b: u32,
        c: Vec<isize>,
    }

    let foo = Foo {
        a: 42.0,
        b: 42,
        c: vec![42, 69],
        d: [42, 69],
        e: Bar {
            a: 42.0,
            b: 42,
            c: vec![42, 69],
        },
    };

    let mut encoder = Encoder::new(&file, "foo").unwrap();
    foo.encode(&mut encoder).unwrap();
}

#[test]
fn encode_scalar() {
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
fn encode_text() {
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

#[test]
fn encode_vector() {
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
