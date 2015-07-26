use serialize;

use dataset;
use dataspace;
use file::File;
use link::Link;
use value::Value;
use {Error, Result};

/// An encoder.
pub struct Encoder<'l> {
    file: &'l File,
}

impl<'l> Encoder<'l> {
    /// Create an encoder.
    pub fn new(file: &'l File) -> Result<Encoder<'l>> {
        Ok(Encoder { file: file })
    }

    pub fn put<T: Value>(&mut self, name: &str, value: T) -> Result<()> {
        let dataspace = try!(dataspace::new());
        if try!(Link::exists(self.file, name)) {
            try!(Link::delete(self.file, name));
        }
        let _ = try!(dataset::new(self.file, name, value.datatype(), &dataspace));
        Ok(())
    }
}

#[allow(unused_variables)]
impl<'l> serialize::Encoder for Encoder<'l> {
    type Error = Error;

    fn emit_nil(&mut self) -> Result<()> {
        unimplemented!();
    }

    fn emit_usize(&mut self, v: usize) -> Result<()> {
        unimplemented!();
    }

    fn emit_u64(&mut self, v: u64) -> Result<()> {
        unimplemented!();
    }

    fn emit_u32(&mut self, v: u32) -> Result<()> {
        unimplemented!();
    }

    fn emit_u16(&mut self, v: u16) -> Result<()> {
        unimplemented!();
    }

    fn emit_u8(&mut self, v: u8) -> Result<()> {
        unimplemented!();
    }

    fn emit_isize(&mut self, v: isize) -> Result<()> {
        unimplemented!();
    }

    fn emit_i64(&mut self, v: i64) -> Result<()> {
        unimplemented!();
    }

    fn emit_i32(&mut self, v: i32) -> Result<()> {
        unimplemented!();
    }

    fn emit_i16(&mut self, v: i16) -> Result<()> {
        unimplemented!();
    }

    fn emit_i8(&mut self, v: i8) -> Result<()> {
        unimplemented!();
    }

    fn emit_bool(&mut self, v: bool) -> Result<()> {
        unimplemented!();
    }

    fn emit_f64(&mut self, v: f64) -> Result<()> {
        unimplemented!();
    }

    fn emit_f32(&mut self, v: f32) -> Result<()> {
        unimplemented!();
    }

    fn emit_char(&mut self, v: char) -> Result<()> {
        unimplemented!();
    }

    fn emit_str(&mut self, v: &str) -> Result<()> {
        unimplemented!();
    }

    fn emit_enum<F>(&mut self, name: &str, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_enum_variant<F>(&mut self, v_name: &str, v_id: usize, len: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_enum_variant_arg<F>(&mut self, a_idx: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_enum_struct_variant<F>(&mut self, v_name: &str, v_id: usize, len: usize, f: F)
                                   -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_enum_struct_variant_field<F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_struct<F>(&mut self, name: &str, len: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_struct_field<F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_tuple<F>(&mut self, len: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_tuple_arg<F>(&mut self, idx: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_tuple_struct<F>(&mut self, name: &str, len: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_tuple_struct_arg<F>(&mut self, f_idx: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_option<F>(&mut self, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_option_none(&mut self) -> Result<()> {
        unimplemented!();
    }

    fn emit_option_some<F>(&mut self, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_seq<F>(&mut self, len: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_seq_elt<F>(&mut self, idx: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_map<F>(&mut self, len: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_map_elt_key<F>(&mut self, idx: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }

    fn emit_map_elt_val<F>(&mut self, idx: usize, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        unimplemented!();
    }
}
