use hdf5::{Encoder, File};
use rustc_serialize::Encodable;
use temporary::Directory;

macro_rules! test(
    ($($name:ident := $value:expr,)*) => ({
        let directory = Directory::new("hdf5").unwrap();
        let file = File::new(directory.join("data.h5")).unwrap();
        $({
            let mut encoder = Encoder::new(&file, stringify!($name)).unwrap();
            $value.encode(&mut encoder).unwrap();
        })*
    });
);

#[test]
fn encode_boolean() {
    test!(
        a := true,
        b := false,
    );
}

#[test]
fn encode_enum() {
    let directory = Directory::new("hdf5").unwrap();
    let file = File::new(directory.join("data.h5")).unwrap();

    #[derive(RustcEncodable)]
    #[allow(dead_code)]
    enum Foo {
        Bar,
        Baz,
        Qux,
    }

    let foo = Foo::Bar;

    let mut encoder = Encoder::new(&file, "foo").unwrap();
    foo.encode(&mut encoder).unwrap();
}

#[test]
fn encode_compound() {
    let directory = Directory::new("hdf5").unwrap();
    let file = File::new(directory.join("data.h5")).unwrap();

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
fn encode_numberic_scalar() {
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
fn encode_numeric_vector() {
    test!(
        a := vec![42f32, 69f32],
        b := vec![42f64, 69f64],

        c := vec![42i8, 69i8],
        d := vec![42u8, 69u8],

        e := vec![42i16, 69i16],
        f := vec![42u16, 69u16],

        g := vec![42i32, 69i32],
        h := vec![42u32, 69u32],

        i := vec![42i64, 69i64],
        j := vec![42u64, 69u64],

        k := vec![42isize, 69isize],
        l := vec![42usize, 69usize],
    );
}

#[test]
fn encode_option() {
    test!(
        a := Some(42.0),
        b := Option::None::<u16>,
        c := Some(69u8),
    );
}

#[test]
fn encode_text() {
    test!(
        a := '界',
        b := "Hello, 世界!",
    );
}
