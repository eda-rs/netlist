use std::fmt::{self, Debug};
use std::{collections::HashMap, iter::Iterator};
use serde::{Deserialize, Serialize};


pub type NetIndex = usize;
pub type GateIndex = usize;
pub type PinIndex = usize;
pub type BlockIndex = usize;
type NodeIndex = usize;

#[derive(Default)]
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
    > Debug for NetList<W, N, G, B, P>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nets:")?;
        f.debug_list().entries(self.nets.iter()).finish()?;
        f.write_str("\n")?;
        write!(f, "gates:")?;
        f.debug_list().entries(self.gates.iter()).finish()?;
        f.write_str("\n")?;
        write!(f, "blocks:")?;
        f.debug_list().entries(self.blocks.iter()).finish()?;
        f.write_str("\n")?;
        write!(f, "pins:")?;
        f.debug_list().entries(self.pins.iter()).finish()?;
        f.write_str("\n")?;
        write!(f, "net_map:")?;
        f.debug_map().entries(self.net_map.iter()).finish()?;
        f.write_str("\n")?;
        write!(f, "gate_map:")?;
        f.debug_map().entries(self.gate_map.iter()).finish()?;
        f.write_str("\n")?;
        write!(f, "pin_map:")?;
        f.debug_list().entries(self.pin_map.iter()).finish()?;
        f.write_str("\n")
    }
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

#[derive(Default)]
pub struct Net<W> {
    pub name: String,
    pub bitwidth: u32,
    pub connection: Vec<NodeIndex>,
    pub data: W,
}

impl<W: Default + Debug> Debug for Net<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\t")?;
        f.debug_struct("Net")
            .field("name", &self.name)
            .field("bitwidth", &self.bitwidth)
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Default,PartialEq, Debug,Serialize, Deserialize)]
pub enum PinDirection {
    #[default]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PortList<P:Default> {
    pub items: Vec<Pin<P>>,
}

#[derive(Default,Serialize, Deserialize)]
pub struct Pin<P> {
    pub name: String,
    pub direction: PinDirection,
    pub bitwidth: u32,
    pub first_node: NodeIndex,
    pub data: P,
}

impl<P: Default + Debug> Debug for Pin<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\t")?;
        f.debug_struct("Pin")
            .field("name", &self.name)
            .field("direction", &self.direction)
            .field("bitwidth", &self.bitwidth)
            .field("data", &self.data)
            .finish()
    }
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

#[derive(Default)]
pub struct Block<B> {
    pub gates: Vec<GateIndex>,
    pub data: B,
}

impl<W: Default + Debug> Debug for Block<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\t")?;
        f.debug_struct("Block").field("data", &self.data).finish()
    }
}

#[derive(Default)]
pub struct Gate<G> {
    pub name: String,
    pub model: String,
    pub first_node: NodeIndex,
    pub data: G,
}

impl<W: Default + Debug> Debug for Gate<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\t")?;
        f.debug_struct("Gate")
            .field("name", &self.name)
            .field("model", &self.model)
            .field("data", &self.data)
            .finish()
    }
}

