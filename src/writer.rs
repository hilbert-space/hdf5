use std::marker::PhantomData;

use data::{Data, IntoData};
use dataset::{self, Dataset};
use dataspace;
use datatype::Datatype;
use file::File;
use link::Link;
use {ID, Identity, Result};

/// A writer.
pub struct Writer<'l> {
    state: State,
    phantom: PhantomData<&'l mut File>,
}

enum State {
    Setup { location: ID, name: String, dimensions: Vec<usize> },
    Ready(Inner),
}

struct Inner {
    dataset: Dataset,
    datatype: Datatype,
    dimensions: usize,
}

impl<'l> Writer<'l> {
    /// Create a writer.
    pub fn new(file: &'l mut File, name: &str, dimensions: &[usize]) -> Writer<'l> {
        Writer {
            state: State::Setup {
                location: file.id(),
                name: name.to_string(),
                dimensions: dimensions.to_vec(),
            },
            phantom: PhantomData,
        }
    }

    /// Write data.
    ///
    /// The function writes a chunk of data at a particular position with a
    /// particular size.
    pub fn write<T: IntoData>(&mut self, data: T, position: &[usize], size: &[usize])
                              -> Result<()> {

        let data = try!(data.into_data());
        let state = match self.state {
            State::Ready(ref mut inner) => return inner.write(data, position, size),
            State::Setup { location, ref name, ref dimensions } => {
                State::Ready(try!(Inner::new(location, name, data.datatype(), dimensions)))
            },
        };
        self.state = state;
        self.write(data, position, size)
    }
}

impl Inner {
    fn new(location: ID, name: &str, datatype: Datatype, dimensions: &[usize]) -> Result<Inner> {
        if try!(Link::exists(location, name)) {
            try!(Link::delete(location, name));
        }
        let dataspace = try!(dataspace::new(dimensions));
        let dataset = try!(dataset::new(location, name, datatype.id(), dataspace.id()));
        Ok(Inner { dataset: dataset, datatype: datatype, dimensions: dimensions.len() })
    }

    fn write<T: Data>(&mut self, data: T, position: &[usize], size: &[usize]) -> Result<()> {
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
