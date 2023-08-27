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

use netlist::NetList;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &str = &args[1];
    println!("{} will be tested", file);
    let my_netlist = NetList::<Net, Node, Block, Component, Port,Scope>::verilog2netlist(file).unwrap();
    my_netlist.netlist2verilog("exported.v").unwrap();
}
