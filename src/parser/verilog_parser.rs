use super::{
    base::{tstring, ws},
    subparser::{instantiate_stmt, port_direction_declare_stmt, port_map_stmt},
    ParseRes,
};
use crate::model::{Gate, Net, NetList, Pin};
use nom::{
    branch::permutation,
    bytes::complete::tag,
    multi::many1,
    sequence::{delimited, tuple},
};

pub fn verilog_parser<N: Default, G: Default, P: Default>(
    s: &str,
) -> ParseRes<&str, NetList<N, G, P>> {
    delimited(
        ws(tag("module")),
        tuple((
            tstring,
            port_map_stmt,
            permutation((many1(port_direction_declare_stmt), many1(instantiate_stmt))),
        )),
        ws(tag("endmodule")),
    )(s)
    .map(|(s, d)| {
        let mut netlist = NetList::default();
        netlist.name = d.0.to_string();
        // create new pin
        for p in (d.2).0 {
            let index = netlist.pins.len();
            netlist.pin_map.insert(p.1.to_string(), index);
            netlist.pins.push(Pin {
                direction: p.0,
                ..Default::default()
            });
        }
        // create new gate and update its binded net
        for n in (d.2).1 {
            let index = netlist.gates.len();
            netlist.gate_map.insert(n.1.to_string(), index);
            let mut new_gate: Gate<G> = Gate {
                model: n.0.to_string(),
                ..Default::default()
            };
            // update its binded net
            // first check if net is already exists in the net_map,
            // if not , create new net
            for (gate_port, bind_net) in n.2 {
                if !netlist.net_map.contains_key(bind_net) {
                    let net_index = netlist.nets.len();
                    netlist.net_map.insert(bind_net.to_string(), net_index);
                    netlist.nets.push(Net::default());
                    // update pin2net in gate
                    new_gate.pin2net.insert(gate_port.to_string(), net_index);
                } else {
                    // TODO , solve the unwrap
                    let net_index = netlist.net_map.get(bind_net).unwrap();
                    let net = &mut netlist.nets[*net_index];
                    net.gate2pin.insert(index, gate_port.to_string());
                    // update pin2net in gate
                    new_gate.pin2net.insert(gate_port.to_string(), *net_index);
                }
            }
            netlist.gates.push(new_gate);
        }
        (s, netlist)
    })
}
