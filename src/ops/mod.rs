use crate::model::{Gate, GateIndex, Net, NetIndex, NetList};
impl<N: Default, G: Default, P: Default> NetList<N, G, P> {
    // buffer is a non-functional gate that commonly used in physical design to
    // relax design timing
    // this function is usually to perform buffer insertion by between two Pin
    // Description:
    // source: (source gate name)
    // net: (net name)
    // gate: (gate name,gate model name,input pin, output pin)
    fn insert_buffer(
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

    fn change_net_name(&mut self, before: &str, after: &str) -> Option<NetIndex> {
        if self.net_map.contains_key(before) {
            match self.net_map.remove(before) {
                Some(idx) => {
                    self.net_map.insert(after.to_string(), idx);
                    return Some(idx);
                }
                None => return None,
            }
        } else {
            return None;
        }
    }
}
