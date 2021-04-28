use crate::error::NetListError;
use crate::model::{Gate, GateIndex, Net, NetList};
use crate::NResult;
impl<N: Default, G: Default, P: Default> NetList<N, G, P> {
    // buffer is a non-functional gate that commonly used in physical design to
    // relax design timing
    // this function is usually to perform buffer insertion by between two Pin
    // Description:
    // source: (source gate name)
    // net: (net name)
    // gate: (gate name,gate model name,input pin, output pin)
    pub fn insert_buffer(
        &mut self,
        source: &str,
        net: &str,
        gate: (&str, &str, &str, &str),
    ) -> Option<GateIndex> {
        let mut buffer = Gate {
            model: gate.1.to_string(),
            ..Default::default()
        };
        let mut new_net = Net::default();
        let new_gate_idx = self.gates.len();
        let new_net_idx = self.nets.len();
        buffer.pin2net.insert(gate.3.to_string(), new_net_idx);

        if let Some(source_net_idx) = self.net_map.get(net) {
            buffer.pin2net.insert(gate.2.to_string(), *source_net_idx);
            // update gate2pin for new net
            let mut new_gate2pin = self.nets[*source_net_idx].gate2pin.clone();
            if let Some(source_gate_idx) = self.gate_map.get(source) {
                self.nets[*source_net_idx].gate2pin.clear();
                match new_gate2pin.remove(source_gate_idx) {
                    Some(p) => self.nets[*source_net_idx]
                        .gate2pin
                        .insert(*source_gate_idx, p),
                    None => return None,
                }
            } else {
                return None;
            };
            new_gate2pin.insert(new_gate_idx, gate.3.to_string());
            new_net.gate2pin = new_gate2pin;
        } else {
            return None;
        }
        self.nets.push(new_net);
        self.net_map
            .insert(format!("{}_{}", net, gate.0), new_net_idx);
        self.gate_map.insert(gate.1.to_string(), new_gate_idx);
        self.gates.push(buffer);

        return Some(new_gate_idx);
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

    // get internal net data by name
    pub fn get_pin(&self, name: &str) -> NResult<&P> {
        match self.pin_map.get(name) {
            Some(i) => Ok(&self.pins[*i].data),
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
}
