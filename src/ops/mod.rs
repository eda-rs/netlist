use crate::error::NetListError;
use crate::model::{Gate, Net, NetList, Node, NodeGraph, Pin};
use crate::NResult;

impl<W: Default, N: Default, G: Default, P: Default> NetList<W, N, G, P> {
    // get internal net data by name
    pub fn get_net(&self, name: &str) -> NResult<&Net<W>> {
        match self.net_map.get(name) {
            Some(i) => Ok(&self.nets[*i]),
            None => Err(Box::new(NetListError::NetNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }
    // get mutable internal net data by name
    pub fn get_mut_net(&mut self, name: &str) -> NResult<&mut Net<W>> {
        match self.net_map.get(name) {
            Some(i) => Ok(&mut self.nets[*i]),
            None => Err(Box::new(NetListError::NetNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get internal gate data by name
    pub fn get_gate(&self, name: &str) -> NResult<&Gate<G>> {
        match self.gate_map.get(name) {
            Some(i) => Ok(&self.gates[*i]),
            None => Err(Box::new(NetListError::GateNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get mutable internal gate data by name
    pub fn get_mut_gate(&mut self, name: &str) -> NResult<&mut Gate<G>> {
        match self.gate_map.get(name) {
            Some(i) => Ok(&mut self.gates[*i]),
            None => Err(Box::new(NetListError::GateNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get internal pin data by name
    pub fn get_pin(&self, name: &str) -> NResult<&Pin<P>> {
        match self.pin_map.get(name) {
            Some(i) => Ok(&self.pins[*i]),
            None => Err(Box::new(NetListError::PinNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get internal pin data by name
    pub fn get_mut_pin(&mut self, name: &str) -> NResult<&mut Pin<P>> {
        match self.pin_map.get(name) {
            Some(i) => Ok(&mut self.pins[*i]),
            None => Err(Box::new(NetListError::PinNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    pub fn get_gate_node(&self, name: &str) -> NResult<NodeGraph<W, N, G, P>> {
        match self.gate_map.get(name) {
            Some(idx) => {
                let successors = NodeGraph {
                    netlist: self,
                    current_node_idx: self.gates[*idx].first_node,
                };
                Ok(successors)
            }
            None => {
                return Err(Box::new(NetListError::NetNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
    }

    pub fn get_pin_node(&self, name: &str) -> NResult<NodeGraph<W, N, G, P>> {
        match self.pin_map.get(name) {
            Some(idx) => {
                let successors = NodeGraph {
                    netlist: self,
                    current_node_idx: self.pins[*idx].first_node,
                };
                Ok(successors)
            }
            None => {
                return Err(Box::new(NetListError::NetNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
    }

    pub fn get_net_connection(&self, name: &str) -> NResult<Vec<&Node<N>>> {
        match self.net_map.get(name) {
            Some(idx) => {
                let mut result = Vec::new();
                let net_conn = &self.nets[*idx].connection;
                for n in net_conn {
                    result.push(&self.nodes[*n]);
                }
                Ok(result)
            }
            None => {
                return Err(Box::new(NetListError::NetNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
    }
}
