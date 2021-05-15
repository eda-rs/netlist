use std::collections::HashMap;

pub type NetIndex = usize;
pub type GateIndex = usize;
pub type PinIndex = usize;
type NodeIndex = usize;

#[derive(Default)]
pub struct NetList<W, N, G, P> {
    pub(crate) name: String, // netlist name or module name
    pub(crate) nets: Vec<Net<W>>,
    pub(crate) gates: Vec<Gate<G>>,
    pub(crate) pins: Vec<Pin<P>>,
    pub(crate) nodes: Vec<Node<N>>, // internal node, or gate pin
    //fast access
    pub(crate) net_map: HashMap<String, NetIndex>,
    pub(crate) gate_map: HashMap<String, GateIndex>,
    pub(crate) pin_map: HashMap<String, PinIndex>,
}

#[derive(Default)]
pub(crate) struct Net<W> {
    pub name: String,
    pub nodes: Vec<NodeIndex>,
    pub data: W,
}

pub enum PinDirection {
    Input,
    Output,
}

#[derive(Clone)]
pub enum DriveLoad {
    Gate(GateIndex),
    Net(NetIndex),
    Pin(PinIndex),
}

pub struct Node<N> {
    pub name: String, // pin name
    pub from: DriveLoad,
    pub to: DriveLoad,
    pub data: N,
}

#[derive(Default)]
pub struct Pin<P> {
    pub name: String,
    pub direction: PinDirection,
    pub bitwidth: u32,
    pub node: Vec<NodeIndex>,
    pub data: P,
}

#[derive(Default)]
pub struct Gate<G> {
    pub name: String,
    pub model: String,
    pub nodes: Vec<NodeIndex>,
    pub data: G,
}

impl Default for PinDirection {
    fn default() -> Self {
        PinDirection::Input
    }
}
