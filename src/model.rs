use std::{collections::HashMap, iter::Iterator};

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
    pub connection: Vec<NodeIndex>,
    pub data: W,
}

pub enum PinDirection {
    Input,
    Output,
}

#[derive(Clone)]
pub enum NodeOwner {
    GateInput(GateIndex),
    GateOutput(GateIndex),
    PinInput(PinIndex),
    PinOutput(PinIndex),
}

pub struct Node<N> {
    pub name: String, // pin name
    pub owner: NodeOwner,
    pub connection: NetIndex,
    pub data: N,
    pub next_node: Option<NodeIndex>,
}

#[derive(Default)]
pub struct Pin<P> {
    pub name: String,
    pub direction: PinDirection,
    pub bitwidth: u32,
    pub first_node: NodeIndex,
    pub data: P,
}

pub struct NodeGraph<'a, W, N, G, P> {
    pub netlist: &'a NetList<W, N, G, P>,
    pub current_node_idx: NodeIndex,
}

impl<'a, W, N, G, P> Iterator for NodeGraph<'a, W, N, G, P> {
    type Item = NodeIndex;
    fn next(&mut self) -> Option<NodeIndex> {
        match self.netlist.nodes[self.current_node_idx].next_node {
            None => None,
            Some(idx) => {
                self.current_node_idx = idx;
                Some(idx)
            }
        }
    }
}

#[derive(Default)]
pub struct Gate<G> {
    pub name: String,
    pub model: String,
    pub first_node: NodeIndex,
    pub data: G,
}

impl Default for PinDirection {
    fn default() -> Self {
        PinDirection::Input
    }
}
