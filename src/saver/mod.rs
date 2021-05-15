use crate::{
    error::NetListError,
    model::{DriveLoad, PinDirection},
    NetList,
};
use std::{error, fs::File, io::Write};
impl<W: Default, N: Default, G: Default, P: Default> NetList<W, N, G, P> {
    pub fn netlist2verilog(&self, file: &str) -> Result<(), Box<dyn error::Error>> {
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
            for n_idx in &g.nodes {
                let node = &self.nodes[*n_idx];
                if let DriveLoad::Net(idx) = node.from {
                    p2n_list.push((&node.name, &self.nets[idx].name));
                } else if let DriveLoad::Net(idx) = node.to {
                    p2n_list.push((&node.name, &self.nets[idx].name));
                } else {
                    return Err(Box::new(NetListError::SeverError));
                }
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
