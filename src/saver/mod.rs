use crate::{model::{PinDirection,PortList}, NResult, NetList};
use std::{fs::File, io::Write, path::Path};
use serde::Serialize;
impl<W: Default, N: Default, G: Default, B: Default, P: Default> NetList<W, N, G, B, P> {
    pub fn netlist2verilog<Pth: AsRef<Path>>(&self, file: Pth) -> NResult<()> {
        let mut f = File::create(file)?;
        writeln!(f, "module {}", self.name)?;
        writeln!(
            f,
            "({});",
            self.pin_map
                .keys()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" , ")
        )?;
        for p in &self.pins {
            match p.direction {
                PinDirection::Input => {
                    if p.bitwidth == 1 {
                        writeln!(f, "input {};", p.name)?;
                    } else {
                        writeln!(f, "input [{}:0] {};", p.bitwidth - 1, p.name)?;
                    }
                }
                PinDirection::Output => {
                    if p.bitwidth == 1 {
                        writeln!(f, "output {};", p.name)?;
                    } else {
                        writeln!(f, "output [{}:0] {};", p.bitwidth - 1, p.name)?;
                    }
                }
            }
        }
        let mut p2n_list = Vec::new();
        for g in &self.gates {
            // insert first node
            let node = &self.nodes[g.first_node];
            p2n_list.push((&node.name, &self.nets[node.connection].name));
            // insert second node and so on
            for node_idx in self.get_gate_node(&g.name)? {
                let node = &self.nodes[node_idx];
                p2n_list.push((&node.name, &self.nets[node.connection].name));
            }
            writeln!(
                f,
                "{} {} ({});",
                g.model,
                g.name,
                p2n_list
                    .iter()
                    .map(|d| format!(".{} ( {} )", d.0, d.1))
                    .collect::<Vec<String>>()
                    .join(" , ")
            )?;
            p2n_list.clear();
        }

        writeln!(f, "endmodule")?;
        Ok(())
    }
}

impl<P: Default + Serialize> PortList<P> {
    pub fn portlist2xml<Pth: AsRef<Path>>(&self, file: Pth) -> NResult<()> {
        let mut f = File::create(file)?;
        let buff = serde_xml_rs::to_string(&self)?;
        write!(f,"{}",&buff)?;
        Ok(())
    }
}
