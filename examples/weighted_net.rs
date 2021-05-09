
#[derive(Default)]
pub struct Net {
    weight:f32,
}
#[derive(Default)]
pub struct Component {
    location:Option<(i32,i32)>,
}
#[derive(Default)]
pub struct Port {
    location:Option<(i32,i32)>,
}

use netlist::NetList;
use std::env;
fn main() {
    let args:Vec<String> = env::args().collect();
    let file:&str = &args[1];
    println!("{} will be tested",file);
    let my_netlist = NetList::<Net,Component,Port>::verilog2netlist(file).unwrap();
    my_netlist.netlist2verilog("exported.v").unwrap();
}

