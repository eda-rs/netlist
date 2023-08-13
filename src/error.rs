use thiserror::Error;
#[derive(Debug, Error)]
pub enum NetListError {
    #[error("Failed to operate from netlist data")]
    OpsErr {
        #[from]
        source: OpsErr,
    },
    #[error("Failed to parse verilog\nbecause `{0}`")]
    ParseErr(String),
    #[error("Failed to parse portlist from xml\nbecause ")]
    PortListErr{
        #[from]
        source: serde_xml_rs::Error,
    },    
    #[error("Failed to save verilog")]
    SaveErr {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Debug, Error)]
pub enum OpsErr {
    #[error("Net `{0}` not found in the netlist `{1}`")]
    NetNotFound(String, String),
    #[error("Gate `{0}` not found in the netlist `{1}`")]
    GateNotFound(String, String),
    #[error("Pin `{0}` not found in the netlist `{1}`")]
    PinNotFound(String, String),
    #[error("Sever Error in the system")]
    SeverError,
    #[error("Gate `{0}` already in the netlist `{1}` ")]
    GateAlreadyExist(String, String),
}
