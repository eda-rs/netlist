use crate::{error::NetListError, model::PinDirection, NetList};
use std::{collections::HashMap, error, io::Write};
impl<N: Default, G: Default, P: Default> NetList<N, G, P> {
    pub fn netlist2verilog<W: Write>(&self, mut f: W) -> Result<(), Box<dyn error::Error>> {
        write!(f, "module {}", self.name)?;
        write!(
            f,
            "({});",
            self.pin_map
                .keys()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(" , ")
        )?;
        for (k, v) in &self.pin_map {
            match self.pins[*v].direction {
                PinDirection::Input => write!(f, "input {};", k)?,
                PinDirection::Output => write!(f, "output {};", k)?,
            }
        }
        let mut idx2name_net_map = HashMap::new();
        for (k, v) in self.net_map.iter() {
            idx2name_net_map.insert(v, k);
        }
        for (k1, v1) in &self.gate_map {
            let gate = &self.gates[*v1];
            let mut pin2net_name = Vec::new();
            for (k2, v2) in gate.pin2net.iter() {
                if let Some(net_name) = idx2name_net_map.get(v2) {
                    pin2net_name.push((k2, net_name));
                } else {
                    return Err(Box::new(NetListError::SeverError));
                }
            }
            write!(
                f,
                "{} {} ({});",
                k1,
                gate.model,
                pin2net_name
                    .iter()
                    .map(|d| format!(".{} ( {} )", d.0, d.1))
                    .collect::<Vec<String>>()
                    .join(" , ")
            )?;
        }
        write!(f, "endmodule")?;
        Ok(())
    }
}
