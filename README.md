# HDF5 [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an interface to [HDF5][1]. Currently the package is only
capable of writing/encoding; the reading/decoding functionality is yet to be
implemented.

## [Documentation][doc]

## Example

```rust
extern crate hdf5;

use hdf5::File;

let path = "data.h5";
let file = File::new(path).unwrap();

file.write("foo", 42).unwrap();
file.write("bar", &vec![42.0, 69.0]).unwrap();
```

Structural data can be written using [`rustc-serialize`][2] as follows:

```rust
extern crate hdf5;
extern crate rustc_serialize;

use hdf5::File;

#[derive(RustcEncodable)]
struct Foo {
    bar: Vec<f64>,
    baz: Baz,
}

#[derive(RustcEncodable)]
struct Baz {
    qux: f64,
}

let foo = Foo {
    bar: vec![42.0],
    baz: Baz {
        qux: 69.0,
    },
};

let path = "data.h5";
let file = File::new(path).unwrap();

file.encode("foo", &foo).unwrap();
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: http://www.hdfgroup.org/HDF5
[2]: https://crates.io/crates/rustc-serialize

[doc]: https://stainless-steel.github.io/hdf5
[status-img]: https://travis-ci.org/stainless-steel/hdf5.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/hdf5
[version-img]: https://img.shields.io/crates/v/hdf5.svg
[version-url]: https://crates.io/crates/hdf5
