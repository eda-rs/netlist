use nom::branch::alt;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0};
use nom::combinator::{map_res, opt, recognize};

use nom::multi::{many0, many1};

use super::ParseRes;
use nom::sequence::{delimited, pair, tuple};
use std::str::{self, FromStr};

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

fn identifier(input: &str) -> ParseRes<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

/// hierachical identifier
/// this parser allow hierachical representation of module name, net name or pin name
pub fn hier_identifier(s: &str) -> ParseRes<&str, &str> {
    ws(recognize(pair(
        pair(opt(tag("\\")), identifier),
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

// // unsigned integer number
// // ie, 100, 350
pub fn number(input: &str) -> ParseRes<&str, u32> {
    ws(map_res(recognize(digit1), u32::from_str))(input)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_identifier_1() {
        let input = "abc/net1";
        let (_, _) = identifier(input).unwrap();
    }
    #[test]
    fn test_identifier_2() {
        let input = "abc\\/net1";
        let (_, _) = identifier(input).unwrap();
    }
    #[test]
    fn test_identifier_3() {
        let input = "abc/net[1]";
        let (_, _) = identifier(input).unwrap();
    }
    #[test]
    fn test_identifier_4() {
        let input = "abc/net\\[1\\]";
        let (_, _) = identifier(input).unwrap();
    }
    #[test]
    fn test_identifier_5() {
        let input = "abc/def/net[1][2]";
        let (_, _) = identifier(input).unwrap();
    }
    // #[test]
    // fn test_identifier_6() {
    //     let input = "\\abc/def/net[1][2]";
    //     let (_, _) = identifier(input).unwrap();
    // }
}
