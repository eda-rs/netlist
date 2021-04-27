use super::base::{identifier, tstring, ws};
use crate::model::PinDirection;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, terminated, tuple};

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
                    separated_list1(tag(","), binding_parser),
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

fn binding_parser(s: &str) -> ParseRes<&str, BindingT> {
    tuple((
        preceded(tag("."), alphanumeric1),
        delimited(ws(tag("(")), tstring, ws(tag(")"))),
    ))(s)
}
