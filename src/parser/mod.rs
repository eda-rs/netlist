use nom::{
    error::{convert_error, VerboseError},
    Err, IResult,
};

mod base;
mod subparser;
mod verilog_parser;

pub type ParseRes<T, U> = IResult<T, U, VerboseError<T>>;

use super::model::NetList;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use verilog_parser::verilog_parser;

impl<N: Default, G: Default, P: Default> FromStr for NetList<N, G, P> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match verilog_parser(s) {
            Ok((_, u)) => Ok(u),
            Err(Err::Error(e)) => {
                println!("[VerilogParser] `VerboseError`:\n{}", convert_error(s, e));
                Err(Error::new(ErrorKind::InvalidData, "Invalid verilog File"))
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid verilog File")),
        }
    }
}

use std::{error, fs, path::Path};
impl<N: Default, G: Default, P: Default> NetList<N, G, P> {
    pub fn verilog2netlist(file: P) -> Result<Self, Box<dyn error::Error>>
    where
        P: AsRef<Path>,
    {
        Ok(fs::read_to_string(file)?.parse::<NetList<N, G, P>>()?)
    }
}
