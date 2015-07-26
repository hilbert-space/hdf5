use rustc_serialize;
use std::mem;

use data::{self, Array, Data};
use datatype::{self, Datatype};
use file::File;
use {Error, Result};

/// An encoder.
pub struct Encoder<'l> {
    file: &'l File,
    name: Option<String>,
    state: State,
}

struct Blob {
    data: Vec<u8>,
    datatype: Option<Datatype>,
}

enum State {
    Uncertain,
    Sequence(Blob),
}

impl<'l> Encoder<'l> {
    /// Create an encoder.
    pub fn new(file: &'l File, name: &str) -> Result<Encoder<'l>> {
        Ok(Encoder { file: file, name: Some(name.to_string()), state: State::Uncertain })
    }

    fn element<T: Data>(&mut self, data: T) -> Result<()> {
        match self.state {
            State::Uncertain => match self.name.take() {
                Some(ref name) => self.file.write(name, data),
                _ => raise!("cannot write data without a name"),
            },
            State::Sequence(ref mut blob) => {
                if let Some(ref datatype) = blob.datatype {
                    if datatype != &data.datatype() {
                        raise!("cannot mix datatypes in arrays");
                    }
                } else {
                    blob.datatype = Some(data.datatype());
                }
                blob.data.extend(data.as_bytes());
                Ok(())
            },
        }
    }

    fn sequence<F>(&mut self, next: F) -> Result<()> where F: FnOnce(&mut Self) -> Result<()> {
        let state = mem::replace(&mut self.state, State::Sequence(Blob::new()));
        try!(next(self));
        match mem::replace(&mut self.state, state) {
            State::Sequence(blob) => match self.name.take() {
                Some(ref name) => self.file.write(name, try!(blob.coagulate())),
                _ => raise!("cannot write data without a name"),
            },
            _ => unreachable!(),
        }
    }
}

impl Blob {
    #[inline]
    fn new() -> Blob {
        Blob { data: vec![], datatype: None }
    }

    fn coagulate(self) -> Result<Array<u8>> {
        let Blob { data, datatype } = self;
        let datatype = match datatype {
            Some(datatype) => {
                let size = try!(datatype.size());
                if data.len() % size != 0 {
                    raise!("encountered malformed array data");
                }
                try!(datatype::new_array(datatype, &[1, data.len() / size]))
            },
            _ => raise!("cannot infer the datatype of empty arrays"),
        };
        data::new_array(data, datatype)
    }
}

impl<'l> rustc_serialize::Encoder for Encoder<'l> {
    type Error = Error;

    fn emit_bool(&mut self, _: bool) -> Result<()> {
        panic!("HDF5 does not support booleans");
    }

    #[inline]
    fn emit_char(&mut self, value: char) -> Result<()> {
        self.element(value as u32)
    }

    fn emit_enum<F>(&mut self, _: &str, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support enums");
    }

    fn emit_enum_struct_variant<F>(&mut self, _: &str, _: usize, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support enums");
    }

    fn emit_enum_struct_variant_field<F>(&mut self, _: &str, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support enums");
    }

    fn emit_enum_variant<F>(&mut self, _: &str, _: usize, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support enums");
    }

    fn emit_enum_variant_arg<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support enums");
    }

    #[inline]
    fn emit_f32(&mut self, value: f32) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_f64(&mut self, value: f64) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_i8(&mut self, value: i8) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_i16(&mut self, value: i16) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_i32(&mut self, value: i32) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_i64(&mut self, value: i64) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_u64(&mut self, value: u64) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_isize(&mut self, value: isize) -> Result<()> {
        self.element(value)
    }

    fn emit_map<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_map_elt_key<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_map_elt_val<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    #[inline]
    fn emit_nil(&mut self) -> Result<()> {
        panic!("HDF5 does not support pointers");
    }

    fn emit_option<F>(&mut self, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support options");
    }

    fn emit_option_none(&mut self) -> Result<()> {
        panic!("HDF5 does not support options");
    }

    fn emit_option_some<F>(&mut self, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support options");
    }

    fn emit_seq<F>(&mut self, _: usize, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        self.sequence(next)
    }

    fn emit_seq_elt<F>(&mut self, _: usize, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        next(self)
    }

    fn emit_str(&mut self, value: &str) -> Result<()> {
        self.element(value)
    }

    fn emit_struct<F>(&mut self, _: &str, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_struct_field<F>(&mut self, _: &str, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_tuple<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support tuples");
    }

    fn emit_tuple_arg<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support tuples");
    }

    fn emit_tuple_struct<F>(&mut self, _: &str, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support tuple structs");
    }

    fn emit_tuple_struct_arg<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support tuple structs");
    }

    #[inline]
    fn emit_u8(&mut self, value: u8) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_u16(&mut self, value: u16) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_u32(&mut self, value: u32) -> Result<()> {
        self.element(value)
    }

    #[inline]
    fn emit_usize(&mut self, value: usize) -> Result<()> {
        self.element(value)
    }
}
