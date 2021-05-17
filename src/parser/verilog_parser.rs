use super::{
    base::{tstring, ws},
    subparser::{
        comment, instantiate_stmt, port_direction_declare_stmt, port_map_stmt, wire_declare_stmt,
    },
    ParseRes,
};
use crate::model::{Gate, Net, NetList, Node, NodeOwner, Pin, PinDirection};
use nom::{
    branch::permutation,
    bytes::complete::tag,
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
};

pub fn verilog_parser<W: Default, N: Default, G: Default, P: Default>(
    s: &str,
) -> ParseRes<&str, NetList<W, N, G, P>> {
    preceded(many0(comment), module_parser)(s)
}

pub fn module_parser<W: Default, N: Default, G: Default, P: Default>(
    s: &str,
) -> ParseRes<&str, NetList<W, N, G, P>> {
    delimited(
        ws(tag("module")),
        tuple((
            tstring,
            port_map_stmt,
            permutation((
                many1(port_direction_declare_stmt),
                many1(instantiate_stmt),
                many0(wire_declare_stmt), // ignore it when updating netlist
                many0(comment),           // ignore
            )),
        )),
        ws(tag("endmodule")),
    )(s)
    .map(|(s, d)| {
        let mut netlist = NetList::default();
        netlist.name = d.0.to_string();
        // create new pin by port direction declare state
        for p in (d.2).0 {
            // node,pin,net index
            let mut node_id = netlist.nodes.len();
            let mut pin_id = netlist.pins.len();
            let mut net_id = netlist.nets.len();

            match p.0 {
                // Input pin means that
                // (1) new node with net as load will be created
                // (2) new input pin wih created node will be created
                // (3) new net with created node will be created
                PinDirection::Input => {
                    for pname in &p.2 {
                        let mut new_pin = Pin {
                            name: pname.to_string(),
                            bitwidth: 1,
                            first_node: node_id,
                            ..Default::default() // default as Input direction
                        };

                        if let Some((msb, lsb)) = p.1 {
                            new_pin.bitwidth = msb - lsb + 1;
                            for bit in lsb..=msb {
                                let net_name = &format!("{}[{}]", pname, bit);
                                netlist.nets.push(Net {
                                    name: net_name.to_string(),
                                    connection: vec![node_id],
                                    ..Default::default()
                                });
                                netlist.net_map.insert(net_name.to_string(), net_id);
                                if bit != msb {
                                    netlist.nodes.push(Node {
                                        name: net_name.to_string(),
                                        owner: NodeOwner::PinInput(pin_id),
                                        data: N::default(),
                                        connection: net_id,
                                        next_node: Some(node_id + 1),
                                    });
                                } else {
                                    netlist.nodes.push(Node {
                                        name: net_name.to_string(),
                                        owner: NodeOwner::PinInput(pin_id),
                                        data: N::default(),
                                        connection: net_id,
                                        next_node: None,
                                    });
                                }
                                node_id += 1;
                                pin_id += 1;
                                net_id += 1;
                            }
                        } else {
                            netlist.nets.push(Net {
                                name: pname.to_string(), // pname is also net name, according to verilog standard
                                connection: vec![node_id],
                                ..Default::default()
                            });
                            netlist.net_map.insert(pname.to_string(), net_id);
                            netlist.nodes.push(Node {
                                name: pname.to_string(),
                                owner: NodeOwner::PinInput(pin_id),
                                connection: net_id,
                                data: N::default(),
                                next_node: None,
                            });
                        }
                        netlist.pins.push(new_pin);
                        netlist.pin_map.insert(pname.to_string(), pin_id);
                    }
                }
                // Output pin means that
                // (1) new node with pin as load will be created
                // (2) new output pin with created node will be created
                // (3) new net that same name with pin name will be created, with created node
                PinDirection::Output => {
                    for pname in &p.2 {
                        let mut new_pin = Pin {
                            name: pname.to_string(),
                            bitwidth: 1,
                            direction: PinDirection::Output,
                            first_node: node_id,
                            ..Default::default()
                        };

                        if let Some((msb, lsb)) = p.1 {
                            new_pin.bitwidth = msb - lsb + 1;
                            for bit in lsb..=msb {
                                let net_name = &format!("{}[{}]", pname, bit);
                                netlist.nets.push(Net {
                                    name: net_name.to_string(),
                                    connection: vec![node_id],
                                    ..Default::default()
                                });
                                netlist.net_map.insert(net_name.to_string(), net_id);
                                if bit != msb {
                                    netlist.nodes.push(Node {
                                        name: net_name.to_string(),
                                        owner: NodeOwner::PinOutput(pin_id),
                                        data: N::default(),
                                        connection: net_id,
                                        next_node: Some(node_id + 1),
                                    });
                                } else {
                                    netlist.nodes.push(Node {
                                        name: net_name.to_string(),
                                        owner: NodeOwner::PinOutput(pin_id),

                                        data: N::default(),
                                        connection: net_id,
                                        next_node: None,
                                    });
                                }
                                node_id += 1;
                                pin_id += 1;
                                net_id += 1;
                            }
                        } else {
                            // when pin bitwidth == 1, pin_name = net_name = node_name
                            netlist.nets.push(Net {
                                name: pname.to_string(),
                                connection: vec![node_id],
                                ..Default::default()
                            });
                            netlist.net_map.insert(pname.to_string(), net_id);
                            netlist.nodes.push(Node {
                                name: pname.to_string(),
                                owner: NodeOwner::PinOutput(pin_id),
                                data: N::default(),
                                connection: net_id,
                                next_node: None,
                            });
                        }
                        netlist.pins.push(new_pin);
                        netlist.pin_map.insert(pname.to_string(), pin_id);
                    }
                }
            }
        }
        // create new gate and update its binded net
        for n in (d.2).1 {
            let gate_id = netlist.gates.len();
            let mut node_id = netlist.nodes.len();
            netlist.gate_map.insert(n.1.to_string(), gate_id);
            let new_gate: Gate<G> = Gate {
                name: n.1.to_string(),
                model: n.0.to_string(),
                first_node: node_id,
                ..Default::default()
            };
            netlist.gates.push(new_gate);
            // update its binded net
            // first check if net is already exists in the net_map,
            // if not , create new net

            // first node index in gate
            // second node index = first node index + 1, so on
            let new_node_num = n.2.len();
            for (i, d) in n.2.iter().enumerate() {
                let gate_port = d.0;
                let bind_net = d.1;

                match netlist.net_map.get(bind_net) {
                    // if bind_net already in net_map, it means that
                    // (1) no need to create new net in this step
                    // (2) update nodes in net
                    Some(net_id) => {
                        let net = &mut netlist.nets[*net_id];
                        net.connection.push(node_id);
                        // create new node
                        if i != new_node_num - 1 {
                            netlist.nodes.push(Node {
                                name: gate_port.to_string(),
                                owner: NodeOwner::GateInput(gate_id), // as we dont know node pin direction
                                data: N::default(),
                                connection: *net_id,
                                next_node: Some(node_id + 1),
                            })
                        } else {
                            netlist.nodes.push(Node {
                                name: gate_port.to_string(),
                                owner: NodeOwner::GateInput(gate_id), // as we dont know node pin direction
                                data: N::default(),
                                connection: *net_id,
                                next_node: None,
                            })
                        }
                    }
                    // if bind_net no in the net map, it means that
                    // (1) need to create new net in this step
                    None => {
                        let net_id = netlist.nets.len();
                        netlist.net_map.insert(bind_net.to_string(), net_id);
                        netlist.nets.push(Net {
                            name: bind_net.to_string(),
                            connection: vec![node_id],
                            ..Default::default()
                        });
                        // create new node
                        if i != new_node_num - 1 {
                            netlist.nodes.push(Node {
                                name: gate_port.to_string(),
                                owner: NodeOwner::GateInput(gate_id), // as we dont know node pin direction
                                data: N::default(),
                                connection: net_id,
                                next_node: Some(node_id + 1),
                            })
                        } else {
                            netlist.nodes.push(Node {
                                name: gate_port.to_string(),
                                owner: NodeOwner::GateInput(gate_id), // as we dont know node pin direction
                                data: N::default(),
                                connection: net_id,
                                next_node: None,
                            })
                        }
                    } // create new net
                }
                node_id += 1;
            }
        }
        (s, netlist)
    })
}
