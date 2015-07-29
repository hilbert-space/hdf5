use ffi;
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

    /// Write a chunk of data.
    pub fn write<T: IntoData>(&mut self, data: T, position: &[usize]) -> Result<()> {
        let data = try!(data.into_data());

        if self.datatype != data.datatype() {
            raise!("the data should have the claimed datatype");
        }
        if self.dimensions != data.dimensions().len() {
            raise!("the data should have the claimed number of dimensions");
        }
        if self.dimensions != position.len() {
            raise!("the position should have the claimed number of dimensions");
        }

        let dataspace = ok!(ffi::H5Dget_space(self.dataset.id()), "failed to get the dataspace");
        ok!(ffi::H5Sselect_hyperslab(dataspace, ffi::H5S_SELECT_SET, position.as_ptr() as *const _,
                                     0 as *const _, data.dimensions().as_ptr() as *const _,
                                     0 as *const _),
            "failed to select the hyperslab region");

        self.dataset.write(data)
    }
}
