use super::{
    base::{tstring, ws},
    subparser::{instantiate_stmt, port_direction_declare_stmt, port_map_stmt},
    ParseRes,
};
use crate::model::{Gate, Load, Net, NetList, Node, Pin, PinDirection};
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
            let pname: &str = p.1;
            // node,pin,net index
            let node_id = netlist.nodes.len();
            let pin_id = netlist.pins.len();
            let net_id = netlist.nets.len();
            // create new net,pin,node record in hashmap
            // pin net name eq pin name
            netlist.net_map.insert(pname.to_string(), net_id);
            netlist.pin_map.insert(pname.to_string(), pin_id);

            match p.0 {
                // Input pin means that
                // (1) new pin with load net will be created
                // (2) new net that same name with pin name will be created, without load pin
                PinDirection::Input => {
                    netlist.nodes.push(Node {
                        name: pname.to_string(),
                        load: Load::Net(net_id),
                    });
                    netlist.pins.push(Pin {
                        name: pname.to_string(),
                        direction: p.0,
                        load_node: Some(node_id),
                        ..Default::default()
                    });
                    netlist.nets.push(Net {
                        name: pname.to_string(), // pname is also net name, according to verilog standard
                        load_nodes: vec![],
                        ..Default::default()
                    });
                }
                // Output pin means that
                // (1) new pin without load net will be created
                // (2) new net that same name with pin name will be created, with load pin
                PinDirection::Output => {
                    netlist.nodes.push(Node {
                        name: pname.to_string(),
                        load: Load::Pin(pin_id),
                    });
                    netlist.pins.push(Pin {
                        name: pname.to_string(),
                        direction: p.0,
                        load_node: None,
                        ..Default::default()
                    });
                    netlist.nets.push(Net {
                        name: pname.to_string(), // pname is also net name, according to verilog standard
                        load_nodes: vec![node_id],
                        ..Default::default()
                    });
                }
            }
        }
        // create new gate and update its binded net
        for n in (d.2).1 {
            let gate_id = netlist.gates.len();
            netlist.gate_map.insert(n.1.to_string(), gate_id);
            let mut new_gate: Gate<G> = Gate {
                name: n.1.to_string(),
                model: n.0.to_string(),
                ..Default::default()
            };
            // update its binded net
            // first check if net is already exists in the net_map,
            // if not , create new net
            for (gate_port, bind_net) in n.2 {
                // create new node
                let node_id = netlist.nodes.len();
                match netlist.net_map.get(bind_net) {
                    // if bind_net already in net_map, it means that
                    // (1) no need to create new net in this step
                    // (2) the gate_port(node) is the endpoint of net
                    // (3) the gate_port(node) is the drive node of gate, or input pin of gate
                    Some(net_id) => {
                        let net = &mut netlist.nets[*net_id];
                        net.load_nodes.push(node_id);
                        netlist.nodes.push(Node {
                            name: gate_port.to_string(),
                            load: Load::Gate(gate_id),
                        })
                    }
                    // if bind_net no in the net map, it means that
                    // (1) need to create new net in this step
                    // (2) the gate_port(node) is the startpoint of net
                    // (3) the gate_port(node) is the load node of gate, or output pin of gate
                    None => {
                        let net_id = netlist.nets.len();
                        netlist.net_map.insert(bind_net.to_string(), net_id);
                        netlist.nets.push(Net {
                            name: bind_net.to_string(),
                            load_nodes: vec![],
                            ..Default::default()
                        });
                        netlist.nodes.push(Node {
                            name: gate_port.to_string(),
                            load: Load::Net(net_id),
                        });
                    } // create new net
                }
                new_gate.load_nodes.push(node_id);
            }
            netlist.gates.push(new_gate);
        }
        (s, netlist)
    })
}
