use std::marker::PhantomData;

use data::{Data, IntoData};
use dataset::{self, Dataset};
use dataspace;
use datatype::Datatype;
use file::File;
use link::Link;
use {Identity, Result};

/// A writer.
pub struct Writer<'l> {
    dataset: Dataset,
    datatype: Datatype,
    dimensions: usize,
    phantom: PhantomData<&'l mut File>,
}

impl<'l> Writer<'l> {
    /// Create a writer.
    pub fn new(file: &'l mut File, name: &str, datatype: Datatype, dimensions: &[usize])
               -> Result<Writer<'l>> {

        if try!(Link::exists(file.id(), name)) {
            try!(Link::delete(file.id(), name));
        }

        let dataspace = try!(dataspace::new(dimensions));
        let dataset = try!(dataset::new(file.id(), name, datatype.id(), dataspace.id()));

        Ok(Writer {
            dataset: dataset,
            datatype: datatype,
            dimensions: dimensions.len(),
            phantom: PhantomData,
        })
    }

    /// Write data.
    ///
    /// The function writes a patch of data at a particular position with a
    /// particular size.
    pub fn write<T: IntoData>(&mut self, data: T, position: &[usize], size: &[usize])
                              -> Result<()> {

        let data = try!(data.into_data());

        if self.datatype != data.datatype() {
            raise!("the data should have the claimed datatype");
        }
        if self.dimensions != position.len() {
            raise!("the position should have the claimed number of dimensions");
        }
        if self.dimensions != size.len() {
            raise!("the size should have the claimed number of dimensions");
        }
        if product!(data.dimensions()) != product!(size) {
            raise!("the data should have the claimed number of elements");
        }

        let memory_space = try!(dataspace::new(size));
        let file_space = try!(self.dataset.space());
        try!(file_space.select(position, size));

        self.dataset.write(data, memory_space.id(), file_space.id())
    }
}
