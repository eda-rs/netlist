use std::collections::HashMap;

pub type NetIndex = usize;
pub type GateIndex = usize;
pub type PinIndex = usize;

#[derive(Default)]
pub struct NetList<N, G, P> {
    pub name: String, // netlist name or module name
    pub nets: Vec<Net<N>>,
    pub gates: Vec<Gate<G>>,
    pub pins: Vec<Pin<P>>,
    //fast access
    pub net_map: HashMap<String, NetIndex>,
    pub gate_map: HashMap<String, GateIndex>,
    pub pin_map: HashMap<String, PinIndex>,
}

#[derive(Default)]
pub struct Net<N> {
    pub data: N,
    pub gate2pin: HashMap<GateIndex, String>,
}

pub enum PinDirection {
    Input,
    Output,
}

#[derive(Default)]
pub struct Pin<P> {
    pub direction: PinDirection,
    pub data: P,
}

#[derive(Default)]
pub struct Gate<G> {
    pub model: String,
    pub pin2net: HashMap<String, NetIndex>,
    pub data: G,
}

impl Default for PinDirection {
    fn default() -> Self {
        PinDirection::Input
    }
}

// impl<N, G, P> NetList<N, G, P> {
//     fn verilog2netlist(file:) {}
//     fn netlist2verilog(&self) {}
// }
