use crate::{
    model::{Load, PinDirection},
    NetList,
};
use std::{error, io::Write,fs::File};
impl<N: Default, G: Default, P: Default> NetList<N, G, P> {
    pub fn netlist2verilog(&self, file:&str) -> Result<(), Box<dyn error::Error>> {
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
                PinDirection::Input => write!(f, "input {};\n", p.name)?,
                PinDirection::Output => write!(f, "output {};\n", p.name)?,
            }
        }
        for n in &self.nets {
            for m in &n.load_nodes {
                // match to find gate load
                let gate_load_node = &self.nodes[*m];
                if let Load::Gate(idx) = gate_load_node.load {
                    let gate = &self.gates[idx];
                    // prepare pin2net binding list
                    let mut p2n_list = vec![(&gate_load_node.name, &n.name)];
                    for gn_i in &gate.load_nodes {
                        let a_node = &self.nodes[*gn_i];
                        if let Load::Net(idx) = a_node.load {
                            p2n_list.push((&a_node.name, &self.nets[idx].name))
                        }
                    }
                    write!(
                        f,
                        "{} {} ({});\n",
                        gate.model,
                        gate.name,
                        p2n_list
                            .iter()
                            .map(|d| format!(".{} ( {} )", d.0, d.1))
                            .collect::<Vec<String>>()
                            .join(" , ")
                    )?;
                }
            }
        }
        write!(f, "endmodule\n")?;
        Ok(())
    }
}
