use super::base::{number,identifier, tstring, ws};
use crate::model::PinDirection;
use nom::branch::alt;
use nom::bytes::complete::{tag,is_not};
use nom::character::complete::{alphanumeric1};
use nom::combinator::{map,value};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{pair,separated_pair,delimited, preceded, terminated, tuple};

use super::ParseRes;

type BindingT<'a> = (&'a str, &'a str);

pub fn instantiate_stmt(s: &str) -> ParseRes<&str, (&str, &str, Vec<BindingT>)> {
    context(
        "Instantiate Statement",
        terminated(
            tuple((
                tstring,
                identifier,
                delimited(
                    tag("("),
                    separated_list1(ws(tag(",")), binding_parser),
                    tag(")"),
                ),
            )),
            ws(tag(";")),
        ),
    )(s)
}

pub fn port_map_stmt(s: &str) -> ParseRes<&str, Vec<&str>> {
    context(
        "Port Map Statement",
        terminated(
            delimited(tag("("), separated_list1(tag(","), identifier), tag(")")),
            ws(tag(";")),
        ),
    )(s)

}

pub fn port_direction_declare_stmt(s: &str) -> ParseRes<&str, (PinDirection, &str)> {
    context(
        "Port Direction Declare Statement",
        terminated(
            alt((
                map(preceded(ws(tag("input")), identifier), |d| {
                    (PinDirection::Input, d)
                }),
                map(preceded(ws(tag("output")), identifier), |d| {
                    (PinDirection::Output, d)
                }),
            )),
            ws(tag(";")),
        ),
    )(s)
}

// return msb and lsb
pub fn port_bitwidth(s:&str) -> ParseRes<&str,(u32,u32)> {
    delimited(
        tag("["),
        separated_pair(number,tag(":"),number),
        tag("]"),
    )(s)
}

// verilog comment statment
pub fn comment(s: &str) -> ParseRes<&str, ()> {
  value(
    (), // Output is thrown away.
    pair(ws(tag("//")), is_not("\n\r"))
    )(s)
}

fn binding_parser(s: &str) -> ParseRes<&str, BindingT> {
    tuple((
        preceded(tag("."), alphanumeric1),
        delimited(ws(tag("(")), tstring, ws(tag(")"))),
    ))(s)
}
