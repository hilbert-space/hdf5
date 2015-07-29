use rustc_serialize;
use std::mem;

use data::{Data, IntoData};
use datatype::{self, Datatype};
use file::File;
use {Error, Result};

/// An encoder.
pub struct Encoder<'l> {
    file: &'l mut File,
    name: Option<String>,
    state: State,
}

enum State {
    Uncertain,
    Sequence(Sequence),
    Structure(Structure),
}

struct Sequence {
    data: Vec<u8>,
    datatype: Option<Datatype>,
    length: usize,
}

struct Structure {
    data: Vec<u8>,
    fields: Vec<(String, Datatype, usize)>,
}

struct Blob {
    data: Vec<u8>,
    datatype: Datatype,
    dimensions: [usize; 1],
}

impl<'l> Encoder<'l> {
    /// Create an encoder.
    pub fn new(file: &'l mut File, name: &str) -> Result<Encoder<'l>> {
        Ok(Encoder { file: file, name: Some(name.to_string()), state: State::Uncertain })
    }

    fn element<T: Data>(&mut self, data: T) -> Result<()> {
        #[inline]
        fn copy(from: &[u8], into: &mut Vec<u8>) -> usize {
            use std::ptr::copy_nonoverlapping as copy;
            let (delta, current) = (from.len(), into.len());
            into.reserve(delta);
            unsafe {
                into.set_len(current + delta);
                copy(from.as_ptr(), into.as_mut_ptr().offset(current as isize), delta);
            }
            delta
        }

        match self.state {
            State::Uncertain => match self.name.take() {
                Some(ref name) => self.file.write(name, data),
                _ => raise!("cannot write data without a name"),
            },
            State::Sequence(ref mut sequence) => {
                if let Some(ref datatype) = sequence.datatype {
                    if datatype != &data.datatype() {
                        raise!("cannot mix datatypes in arrays");
                    }
                } else {
                    sequence.datatype = Some(data.datatype());
                }
                copy(data.as_bytes(), &mut sequence.data);
                sequence.length += 1;
                Ok(())
            },
            State::Structure(ref mut structure) => match self.name.take() {
                Some(name) => {
                    let size = copy(data.as_bytes(), &mut structure.data);
                    structure.fields.push((name, data.datatype(), size));
                    Ok(())
                },
                _ => raise!("cannot write a field without a name"),
            },
        }
    }

    fn sequence<F>(&mut self, next: F) -> Result<()> where F: FnOnce(&mut Self) -> Result<()> {
        let state = mem::replace(&mut self.state, State::Sequence(Sequence::new()));
        try!(next(self));
        match mem::replace(&mut self.state, state) {
            State::Sequence(sequence) => self.element(try!(sequence.coagulate())),
            _ => unreachable!(),
        }
    }

    fn structure<F>(&mut self, next: F) -> Result<()> where F: FnOnce(&mut Self) -> Result<()> {
        let state = mem::replace(&mut self.state, State::Structure(Structure::new()));
        try!(next(self));
        match mem::replace(&mut self.state, state) {
            State::Structure(structure) => self.element(try!(structure.coagulate())),
            _ => unreachable!(),
        }
    }

    fn structure_field<F>(&mut self, name: &str, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        let name = mem::replace(&mut self.name, Some(name.to_string()));
        try!(next(self));
        match mem::replace(&mut self.name, name) {
            None => Ok(()),
            _ => raise!("found a field without a value"),
        }
    }
}

impl Sequence {
    #[inline]
    fn new() -> Sequence {
        Sequence { data: vec![], datatype: None, length: 0 }
    }

    fn coagulate(self) -> Result<Blob> {
        let Sequence { data, datatype, length } = self;
        let datatype = match datatype {
            Some(datatype) => datatype,
            _ => raise!("cannot infer the datatype of empty arrays"),
        };
        debug_assert_eq!(length * datatype.size().unwrap(), data.len());
        Ok(Blob { data: data, datatype: datatype, dimensions: [length] })
    }
}

impl Structure {
    fn new() -> Structure {
        Structure { data: vec![], fields: vec![] }
    }

    fn coagulate(self) -> Result<Blob> {
        let Structure { data, fields } = self;
        let datatype = try!(datatype::new_compound(&fields));
        Ok(Blob { data: data, datatype: datatype, dimensions: [1] })
    }
}

impl Data for Blob {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    fn datatype(&self) -> Datatype {
        self.datatype.clone()
    }

    #[inline]
    fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }
}

impl<'l> rustc_serialize::Encoder for Encoder<'l> {
    type Error = Error;

    #[inline]
    fn emit_bool(&mut self, value: bool) -> Result<()> {
        self.element(value as u8)
    }

    #[inline]
    fn emit_char(&mut self, value: char) -> Result<()> {
        self.element(value as u32)
    }

    #[inline]
    fn emit_enum<F>(&mut self, _: &str, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        next(self)
    }

    fn emit_enum_struct_variant<F>(&mut self, _: &str, _: usize, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support enum structs");
    }

    fn emit_enum_struct_variant_field<F>(&mut self, _: &str, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support enum structs");
    }

    #[inline]
    fn emit_enum_variant<F>(&mut self, _: &str, id: usize, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        self.element(id)
    }

    fn emit_enum_variant_arg<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support enums with arguments");
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
        panic!("HDF5 does not support maps");
    }

    fn emit_map_elt_key<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support maps");
    }

    fn emit_map_elt_val<F>(&mut self, _: usize, _: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        panic!("HDF5 does not support maps");
    }

    fn emit_nil(&mut self) -> Result<()> {
        panic!("HDF5 does not support nils");
    }

    #[inline]
    fn emit_option<F>(&mut self, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        next(self)
    }

    #[inline]
    fn emit_option_none(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn emit_option_some<F>(&mut self, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        next(self)
    }

    #[inline]
    fn emit_seq<F>(&mut self, _: usize, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        self.sequence(next)
    }

    #[inline]
    fn emit_seq_elt<F>(&mut self, _: usize, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        next(self)
    }

    #[inline]
    fn emit_str(&mut self, value: &str) -> Result<()> {
        self.element(try!(value.into_data()))
    }

    #[inline]
    fn emit_struct<F>(&mut self, _: &str, _: usize, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        self.structure(next)
    }

    #[inline]
    fn emit_struct_field<F>(&mut self, name: &str, _: usize, next: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        self.structure_field(name, next)
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
