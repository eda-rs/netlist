use super::{
    base::{tstring, ws},
    subparser::{
        comment, instantiate_stmt, port_direction_declare_stmt, port_map_stmt, wire_declare_stmt,
    },
    ParseRes,
};
use crate::model::{DriveLoad, Gate, Net, NetList, Node, Pin, PinDirection};
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
                many0(wire_declare_stmt),
                many0(comment),
            )),
        )),
        ws(tag("endmodule")),
    )(s)
    .map(|(s, d)| {
        let mut netlist = NetList::default();
        netlist.name = d.0.to_string();

        // create new pin
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
                            ..Default::default() // default as Input direction
                        };
                        if let Some((msb, lsb)) = p.1 {
                            new_pin.bitwidth = msb - lsb + 1;
                            for bit in lsb..=msb {
                                let net_name = &format!("{}[{}]", pname, bit);
                                netlist.nets.push(Net {
                                    name: net_name.to_string(),
                                    nodes: vec![node_id],
                                    ..Default::default()
                                });
                                netlist.net_map.insert(net_name.to_string(), net_id);
                                netlist.nodes.push(Node {
                                    name: net_name.to_string(),
                                    from: DriveLoad::Pin(pin_id),
                                    to: DriveLoad::Net(net_id),
                                    data: N::default(),
                                });
                                new_pin.node.push(node_id);
                                node_id += 1;
                                pin_id += 1;
                                net_id += 1;
                            }
                        } else {
                            new_pin.node.push(node_id);
                            netlist.nets.push(Net {
                                name: pname.to_string(), // pname is also net name, according to verilog standard
                                nodes: vec![node_id],
                                ..Default::default()
                            });
                            netlist.net_map.insert(pname.to_string(), net_id);
                            netlist.nodes.push(Node {
                                name: pname.to_string(),
                                from: DriveLoad::Pin(pin_id),
                                to: DriveLoad::Net(net_id),
                                data: N::default(),
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
                            ..Default::default()
                        };
                        if let Some((msb, lsb)) = p.1 {
                            new_pin.bitwidth = msb - lsb + 1;
                            for bit in lsb..=msb {
                                let net_name = &format!("{}[{}]", pname, bit);
                                netlist.nets.push(Net {
                                    name: net_name.to_string(),
                                    nodes: vec![node_id],
                                    ..Default::default()
                                });
                                netlist.net_map.insert(net_name.to_string(), net_id);
                                netlist.nodes.push(Node {
                                    name: net_name.to_string(),
                                    from: DriveLoad::Net(net_id),
                                    to: DriveLoad::Pin(pin_id),
                                    data: N::default(),
                                });
                                new_pin.node.push(node_id);
                                node_id += 1;
                                pin_id += 1;
                                net_id += 1;
                            }
                        } else {
                            // when pin bitwidth == 1, pin_name = net_name = node_name
                            new_pin.node.push(node_id);
                            netlist.nets.push(Net {
                                name: pname.to_string(),
                                nodes: vec![node_id],
                                ..Default::default()
                            });
                            netlist.net_map.insert(pname.to_string(), net_id);
                            netlist.nodes.push(Node {
                                name: pname.to_string(),
                                from: DriveLoad::Net(net_id),
                                to: DriveLoad::Pin(pin_id),
                                data: N::default(),
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
            netlist.gate_map.insert(n.1.to_string(), gate_id);
            let mut new_gate: Gate<G> = Gate {
                name: n.1.to_string(),
                model: n.0.to_string(),
                ..Default::default()
            };
            // update its binded net
            // first check if net is already exists in the net_map,
            // if not , create new net

            // first node index in gate
            // second node index = first node index + 1, so on
            let mut node_id = netlist.nodes.len();
            for (gate_port, bind_net) in n.2 {
                // create new node
                match netlist.net_map.get(bind_net) {
                    // if bind_net already in net_map, it means that
                    // (1) no need to create new net in this step
                    // (2) new node will be created
                    // (3) as we dont know port direction from verilog, assume they are all output(net as load)
                    // (4) update nodes in net and gate
                    Some(net_id) => {
                        let net = &mut netlist.nets[*net_id];
                        net.nodes.push(node_id);
                        netlist.nodes.push(Node {
                            name: gate_port.to_string(),
                            from: DriveLoad::Gate(gate_id),
                            to: DriveLoad::Net(*net_id),
                            data: N::default(),
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
                            nodes: vec![node_id],
                            ..Default::default()
                        });
                        netlist.nodes.push(Node {
                            name: gate_port.to_string(),
                            from: DriveLoad::Gate(gate_id),
                            to: DriveLoad::Net(net_id),
                            data: N::default(),
                        });
                    } // create new net
                }
                new_gate.nodes.push(node_id);
                node_id += 1;
            }
            netlist.gates.push(new_gate);
        }
        (s, netlist)
    })
}
