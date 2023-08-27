use netlist::NetList;
use std::env;
use std::path::PathBuf;

#[derive(Default)]
pub struct Net {}
#[derive(Default)]
pub struct Component {}
#[derive(Default)]
pub struct Port {}

#[derive(Default)]
pub struct Node {}

#[derive(Default)]
pub struct Block {}

#[derive(Default)]
pub struct Scope {}

fn get_testcase(filename: &str) -> Option<PathBuf> {
    match env::var("CARGO_MANIFEST_DIR") {
        Ok(v) => Some(
            [v, "testcases".to_string(), filename.to_string()]
                .iter()
                .collect(),
        ),
        Err(_) => None,
    }
}

#[test]
fn test_parse_verilog_1() {
    let p = get_testcase("s1238_placed.v").unwrap();
    let _ = NetList::<Net, Component, Block, Port, Node,Scope>::verilog2netlist(&p).unwrap();
}

#[test]
fn test_parse_verilog_2() {
    let p = get_testcase("Intro_TopNetlist.v").unwrap();
    let _ = NetList::<Net, Component, Block, Port, Node,Scope>::verilog2netlist(&p).unwrap();
}
