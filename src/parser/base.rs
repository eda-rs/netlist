use nom::branch::alt;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0};
use nom::combinator::recognize;

use nom::multi::{many0, many1};

use super::ParseRes;
use nom::sequence::{delimited, pair, tuple};
use std::str;

// basic parse.

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> ParseRes<&'a str, O>
where
    F: FnMut(&'a str) -> ParseRes<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// // typical string
// // ie. abcdef, de234, jkl_mn, ...
pub fn tstring(s: &str) -> ParseRes<&str, &str> {
    ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    )))(s)
}

/// module name identifier
/// this parser allow hierachical representation of module name
pub fn identifier(s: &str) -> ParseRes<&str, &str> {
    ws(recognize(pair(
        identifier,
        many0(alt((
            recognize(many1(tuple((tag("\\\\["), digit1, tag("\\\\]"))))),
            recognize(many1(tuple((tag("\\["), digit1, tag("\\]"))))),
            recognize(many1(tuple((tag("["), digit1, tag("]"))))),
            recognize(pair(tag("\\\\/"), identifier)),
            recognize(pair(tag("\\/"), identifier)),
            recognize(pair(tag("/"), identifier)),
        ))),
    )))(s)
}
