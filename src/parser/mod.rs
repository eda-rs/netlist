use nom::{error::VerboseError, IResult};

mod base;
mod subparser;
mod verilog_parser;

pub type ParseRes<T, U> = IResult<T, U, VerboseError<T>>;

use super::model::NetList;

use crate::NResult;
use std::{fs, path::Path};
use verilog_parser::verilog_parser;

impl<W: Default, N: Default, G: Default, B: Default, P: Default> NetList<W, N, G, B, P> {
    pub fn verilog2netlist<Pth: AsRef<Path>>(file: Pth) -> NResult<Self> {
        let buff = fs::read_to_string(file)?;
        verilog_parser(&buff)
    }
}
