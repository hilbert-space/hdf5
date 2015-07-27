use rustc_serialize;

use {Error, Result};

/// A decoder.
pub struct Decoder;

impl rustc_serialize::Decoder for Decoder {
    type Error = Error;

    fn error(&mut self, _: &str) -> Error {
        unimplemented!();
    }

    fn read_bool(&mut self) -> Result<bool> {
        unimplemented!();
    }

    fn read_char(&mut self) -> Result<char> {
        unimplemented!();
    }

    fn read_enum<T, F>(&mut self, _: &str, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        unimplemented!();
    }

    fn read_enum_struct_variant<T, F>(&mut self, _: &[&str], _: F) -> Result<T>
        where F: FnMut(&mut Self, usize) -> Result<T>
    {
        panic!("HDF5 does not support enum structs");
    }

    fn read_enum_struct_variant_field<T, F>(&mut self, _: &str, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        panic!("HDF5 does not support enum structs");
    }

    fn read_enum_variant<T, F>(&mut self, _: &[&str], _: F) -> Result<T>
        where F: FnMut(&mut Self, usize) -> Result<T>
    {
        unimplemented!();
    }

    fn read_enum_variant_arg<T, F>(&mut self, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        panic!("HDF5 does not support enums with arguments");
    }

    fn read_f64(&mut self) -> Result<f64> {
        unimplemented!();
    }

    fn read_f32(&mut self) -> Result<f32> {
        unimplemented!();
    }

    fn read_i8(&mut self) -> Result<i8> {
        unimplemented!();
    }

    fn read_i16(&mut self) -> Result<i16> {
        unimplemented!();
    }

    fn read_i32(&mut self) -> Result<i32> {
        unimplemented!();
    }

    fn read_i64(&mut self) -> Result<i64> {
        unimplemented!();
    }

    fn read_isize(&mut self) -> Result<isize> {
        unimplemented!();
    }

    fn read_map<T, F>(&mut self, _: F) -> Result<T>
        where F: FnOnce(&mut Self, usize) -> Result<T>
    {
        panic!("HDF5 does not support maps");
    }

    fn read_map_elt_key<T, F>(&mut self, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        panic!("HDF5 does not support maps");
    }

    fn read_map_elt_val<T, F>(&mut self, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        panic!("HDF5 does not support maps");
    }

    fn read_nil(&mut self) -> Result<()> {
        panic!("HDF5 does not support nils");
    }

    fn read_option<T, F>(&mut self, _: F) -> Result<T>
        where F: FnMut(&mut Self, bool) -> Result<T>
    {
        panic!("HDF5 does not support options");
    }

    fn read_seq<T, F>(&mut self, _: F) -> Result<T>
        where F: FnOnce(&mut Self, usize) -> Result<T>
    {
        unimplemented!();
    }

    fn read_seq_elt<T, F>(&mut self, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        unimplemented!();
    }

    fn read_str(&mut self) -> Result<String> {
        unimplemented!();
    }

    fn read_struct<T, F>(&mut self, _: &str, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        unimplemented!();
    }

    fn read_struct_field<T, F>(&mut self, _: &str, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        unimplemented!();
    }

    fn read_tuple<T, F>(&mut self, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        panic!("HDF5 does not support tuples");
    }

    fn read_tuple_arg<T, F>(&mut self, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        panic!("HDF5 does not support tuples");
    }

    fn read_tuple_struct<T, F>(&mut self, _: &str, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        panic!("HDF5 does not support tuples");
    }

    fn read_tuple_struct_arg<T, F>(&mut self, _: usize, _: F) -> Result<T>
        where F: FnOnce(&mut Self) -> Result<T>
    {
        panic!("HDF5 does not support tuples");
    }

    fn read_u8(&mut self) -> Result<u8> {
        unimplemented!();
    }

    fn read_u16(&mut self) -> Result<u16> {
        unimplemented!();
    }

    fn read_u32(&mut self) -> Result<u32> {
        unimplemented!();
    }

    fn read_u64(&mut self) -> Result<u64> {
        unimplemented!();
    }

    fn read_usize(&mut self) -> Result<usize> {
        unimplemented!();
    }
}
