use std::collections::HashMap;

pub type NetIndex = usize;
pub type GateIndex = usize;
pub type PinIndex = usize;
type NodeIndex = usize;

#[derive(Default)]
pub struct NetList<N, G, P> {
    pub name: String, // netlist name or module name
    pub(crate) nets: Vec<Net<N>>,
    pub(crate) gates: Vec<Gate<G>>,
    pub(crate) pins: Vec<Pin<P>>,
    pub(crate) nodes: Vec<Node>, // internal node, or gate pin
    //fast access
    pub(crate) net_map: HashMap<String, NetIndex>,
    pub(crate) gate_map: HashMap<String, GateIndex>,
    pub(crate) pin_map: HashMap<String, PinIndex>,
}

#[derive(Default)]
pub struct Net<N> {
    pub name: String,
    pub data: N,
    pub load_nodes: Vec<NodeIndex>,
}

pub enum PinDirection {
    Input,
    Output,
}

pub enum Load {
    Gate(GateIndex),
    Net(NetIndex),
    Pin(PinIndex),
}

pub struct Node {
    pub name: String, // pin name
    pub load: Load,
}

#[derive(Default)]
pub struct Pin<P> {
    pub name: String,
    pub direction: PinDirection,
    pub load_node: Option<NodeIndex>,
    pub data: P,
}

#[derive(Default)]
pub struct Gate<G> {
    pub name: String,
    pub model: String,
    pub load_nodes: Vec<NodeIndex>,
    pub data: G,
}

impl Default for PinDirection {
    fn default() -> Self {
        PinDirection::Input
    }
}
