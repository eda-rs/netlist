use super::base::{identifier, number, tstring, ws};
use crate::model::PinDirection;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::combinator::{map, opt, value};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};

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
            delimited(
                tag("("),
                separated_list1(ws(tag(",")), identifier),
                tag(")"),
            ),
            ws(tag(";")),
        ),
    )(s)
}

pub fn port_direction_declare_stmt(
    s: &str,
) -> ParseRes<&str, (PinDirection, Option<(u32, u32)>, Vec<&str>)> {
    context(
        "Port Direction Declare Statement",
        alt((
            map(
                delimited(
                    ws(tag("input")),
                    tuple((opt(bitwidth), separated_list1(tag(","), identifier))),
                    ws(tag(";")),
                ),
                |d| (PinDirection::Input, d.0, d.1),
            ),
            map(
                delimited(
                    ws(tag("output")),
                    tuple((opt(bitwidth), separated_list1(tag(","), identifier))),
                    ws(tag(";")),
                ),
                |d| (PinDirection::Output, d.0, d.1),
            ),
        )),
    )(s)
}

// wire declare

// examples
// wire a1;
// wire [2:0] a2;
// wire a1, a2, a3;
pub fn wire_declare_stmt(s: &str) -> ParseRes<&str, (Option<(u32, u32)>, Vec<&str>)> {
    context(
        "Wire Declare Statement",
        delimited(
            ws(tag("wire")),
            tuple((opt(bitwidth), separated_list1(tag(","), identifier))),
            ws(tag(";")),
        ),
    )(s)
}

// bus bit in wire declare or port declare

// examples
// wire [11:0] a;
// input [2:0] b;
pub fn bitwidth(s: &str) -> ParseRes<&str, (u32, u32)> {
    delimited(
        ws(tag("[")),
        separated_pair(number, ws(tag(":")), number),
        ws(tag("]")),
    )(s)
}

// verilog comment statment
pub fn comment(s: &str) -> ParseRes<&str, ()> {
    value(
        (), // Output is thrown away.
        pair(ws(tag("//")), is_not("\n\r")),
    )(s)
}

// pin2net binding

// examples
// .A(n1)
fn binding_parser(s: &str) -> ParseRes<&str, BindingT> {
    tuple((
        preceded(ws(tag(".")), tstring),
        delimited(ws(tag("(")), identifier, ws(tag(")"))),
    ))(s)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_comment_1() {
        let input = "// Date      : Mon Jul 20 00:19:01 2020";
        let (_, _) = comment(input).unwrap();
    }
    #[test]
    fn test_comment_2() {
        let input = "///////////\n\r";
        let (_, _) = comment(input).unwrap();
    }
}
