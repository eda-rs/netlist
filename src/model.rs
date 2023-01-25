use std::fmt::Debug;
use std::{collections::HashMap, iter::Iterator};

pub type NetIndex = usize;
pub type GateIndex = usize;
pub type PinIndex = usize;
pub type BlockIndex = usize;
type NodeIndex = usize;

#[derive(Default, Debug)]
pub struct NetList<W, N, G, B, P> {
    pub name: String, // netlist name or module name
    pub nets: Vec<Net<W>>,
    pub gates: Vec<Gate<G>>,
    pub blocks: Vec<Block<B>>,
    pub pins: Vec<Pin<P>>,
    pub nodes: Vec<Node<N>>, // internal node, or gate pin
    //fast access
    pub net_map: HashMap<String, NetIndex>,
    pub gate_map: HashMap<String, GateIndex>,
    pub pin_map: HashMap<String, PinIndex>,
}

impl<
        W: Default + Debug,
        N: Default + Debug,
        G: Default + Debug,
        B: Default + Debug,
        P: Default + Debug,
    > NetList<W, N, G, B, P>
{
    pub fn new() -> Self {
        NetList::default()
    }
}

#[derive(Default, Debug)]
pub struct Net<W> {
    pub name: String,
    pub connection: Vec<NodeIndex>,
    pub data: W,
}

#[derive(PartialEq, Debug)]
pub enum PinDirection {
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub enum NodeOwner {
    GateInput(GateIndex),
    GateOutput(GateIndex),
    PinInput(PinIndex),
    PinOutput(PinIndex),
}

#[derive(Debug)]
pub struct Node<N> {
    pub name: String, // pin name
    pub owner: NodeOwner,
    pub connection: NetIndex,
    pub data: N,
    pub next_node: Option<NodeIndex>,
}

#[derive(Default, Debug)]
pub struct Pin<P> {
    pub name: String,
    pub direction: PinDirection,
    pub bitwidth: u32,
    pub first_node: NodeIndex,
    pub data: P,
}

pub struct NodeGraph<'a, W, N, G, B, P> {
    pub netlist: &'a NetList<W, N, G, B, P>,
    pub current_node_idx: NodeIndex,
}

impl<'a, W, N, G, B, P> Iterator for NodeGraph<'a, W, N, G, B, P> {
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

#[derive(Default, Debug)]
pub struct Block<B> {
    pub gates: Vec<GateIndex>,
    pub data: B,
}

#[derive(Default, Debug)]
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
