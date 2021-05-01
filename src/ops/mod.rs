use crate::error::NetListError;
use crate::model::{Gate, Load, Net, NetList, Node};
use crate::NResult;
impl<N: Default, G: Default, P: Default> NetList<N, G, P> {
    // buffer is a non-functional gate that commonly used in physical design to
    // relax design timing
    // this function is usually to perform buffer insertion by between two Pin
    // Description:
    // source: (source gate name)
    // net: (net name)
    // gate: (gate name,gate model name,input pin, output pin)
    pub fn insert_buffer(&mut self, net: &str, gate: (&str, &str, &str, &str)) -> NResult<String> {
        match self.gate_map.get(gate.0) {
            None => {
                let n1_idx = self.nodes.len();
                let n2_idx = n1_idx + 1;
                let new_gate_idx = self.gates.len();
                let buffer = Gate {
                    name: gate.0.to_string(),
                    model: gate.1.to_string(),
                    load_nodes: vec![n2_idx],
                    ..Default::default()
                };
                let new_net_idx = self.nets.len();
                let new_net_name = format!("{}_{}", net, gate.0);
                match self.net_map.get(net) {
                    Some(net_idx) => {
                        let net_d = &mut self.nets[*net_idx];
                        let new_net = Net {
                            name: format!("{}_{}", net, gate.0),
                            load_nodes: net_d.load_nodes.clone(),
                            ..Default::default()
                        };
                        net_d.load_nodes = vec![n1_idx];

                        // update gate, node, net
                        self.gates.push(buffer);
                        self.nodes.push(Node {
                            name: gate.2.to_string(),
                            load: Load::Gate(new_gate_idx),
                        });
                        self.nodes.push(Node {
                            name: gate.3.to_string(),
                            load: Load::Net(new_net_idx),
                        });
                        self.nets.push(new_net);
                    }
                    None => {
                        return Err(Box::new(NetListError::NetNotFound(
                            net.to_string(),
                            self.name.clone(),
                        )))
                    }
                }
                self.net_map.insert(new_net_name.clone(), new_net_idx);
                Ok(new_net_name.clone())
            }
            Some(_) => Err(Box::new(NetListError::GateAlreadyExist(
                net.to_string(),
                self.name.clone(),
            ))),
        }
    }
    // get internal net data by name
    pub fn get_net(&self, name: &str) -> NResult<&N> {
        match self.net_map.get(name) {
            Some(i) => Ok(&self.nets[*i].data),
            None => Err(Box::new(NetListError::NetNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get internal net data by name
    pub fn get_mut_net(&mut self, name: &str) -> NResult<&mut N> {
        match self.net_map.get(name) {
            Some(i) => Ok(&mut self.nets[*i].data),
            None => Err(Box::new(NetListError::NetNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get internal gate data by name
    pub fn get_gate(&self, name: &str) -> NResult<&G> {
        match self.gate_map.get(name) {
            Some(i) => Ok(&self.gates[*i].data),
            None => Err(Box::new(NetListError::GateNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get mutable internal gate data by name
    pub fn get_mut_gate(&mut self, name: &str) -> NResult<&mut G> {
        match self.gate_map.get(name) {
            Some(i) => Ok(&mut self.gates[*i].data),
            None => Err(Box::new(NetListError::GateNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get internal pin data by name
    pub fn get_pin(&self, name: &str) -> NResult<&P> {
        match self.pin_map.get(name) {
            Some(i) => Ok(&self.pins[*i].data),
            None => Err(Box::new(NetListError::PinNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get mutable internal pin data by name
    pub fn get_mut_pin(&mut self, name: &str) -> NResult<&mut P> {
        match self.pin_map.get(name) {
            Some(i) => Ok(&mut self.pins[*i].data),
            None => Err(Box::new(NetListError::PinNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // update net internal data by name
    pub fn update_net(&mut self, name: &str, data: N) -> NResult<()> {
        match self.net_map.get(name) {
            Some(i) => self.nets[*i].data = data,
            None => {
                return Err(Box::new(NetListError::NetNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
        Ok(())
    }
    // update gate internal data by name
    pub fn update_gate(&mut self, name: &str, data: G) -> NResult<()> {
        match self.gate_map.get(name) {
            Some(i) => self.gates[*i].data = data,
            None => {
                return Err(Box::new(NetListError::GateNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
        Ok(())
    }
    // update pin internal data by name
    pub fn update_pin(&mut self, name: &str, data: P) -> NResult<()> {
        match self.pin_map.get(name) {
            Some(i) => self.pins[*i].data = data,
            None => {
                return Err(Box::new(NetListError::NetNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
        Ok(())
    }
    // get net's load gate , if load contains Pin, return Error
    pub fn get_net_load(&self, name: &str) -> NResult<Vec<Load>> {
        match self.net_map.get(name) {
            Some(net_i) => Ok(self.nets[*net_i]
                .load_nodes
                .iter()
                .map(|x| self.nodes[*x].load.clone())
                .collect()),
            None => Err(Box::new(NetListError::NetNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }
}
