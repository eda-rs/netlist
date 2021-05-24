use crate::{model::PinDirection, NResult, NetList};
use std::{fs::File, io::Write, path::Path};
impl<W: Default, N: Default, G: Default, P: Default> NetList<W, N, G, P> {
    pub fn netlist2verilog<Pth: AsRef<Path>>(&self, file: Pth) -> NResult<()> {
        let mut f = File::create(file)?;
        write!(f, "module {}\n", self.name)?;
        write!(
            f,
            "({});\n",
            self.pin_map
                .keys()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(" , ")
        )?;
        for p in &self.pins {
            match p.direction {
                PinDirection::Input => {
                    if p.bitwidth == 1 {
                        write!(f, "input {};\n", p.name)?;
                    } else {
                        write!(f, "input [{}:0] {};\n", p.bitwidth - 1, p.name)?;
                    }
                }
                PinDirection::Output => {
                    if p.bitwidth == 1 {
                        write!(f, "output {};\n", p.name)?;
                    } else {
                        write!(f, "output [{}:0] {};\n", p.bitwidth - 1, p.name)?;
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
            for node_idx in self.get_gate_node(&g.name)?.into_iter() {
                let node = &self.nodes[node_idx];
                p2n_list.push((&node.name, &self.nets[node.connection].name));
            }
            write!(
                f,
                "{} {} ({});\n",
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

        write!(f, "endmodule\n")?;
        Ok(())
    }
}
