use std::collections::HashMap;

pub type NetIndex = usize;
pub type GateIndex = usize;
pub type PinIndex = usize;
type NodeIndex = usize;

#[derive(Default)]
pub struct NetList<W, N, G, P> {
    pub name: String, // netlist name or module name
    pub nets: Vec<Net<W>>,
    pub gates: Vec<Gate<G>>,
    pub pins: Vec<Pin<P>>,
    pub nodes: Vec<Node<N>>, // internal node, or gate pin
    //fast access
    pub net_map: HashMap<String, NetIndex>,
    pub gate_map: HashMap<String, GateIndex>,
    pub pin_map: HashMap<String, PinIndex>,
}

#[derive(Default)]
pub struct Net<W> {
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
    pub node: NodeIndex,
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
