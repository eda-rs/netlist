use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetListError {
    #[error("Net `{0}` not found in the netlist `{1}`")]
    NetNotFound(String, String),
    #[error("Gate `{0}` not found in the netlist `{1}`")]
    GateNotFound(String, String),
    #[error("Pin `{0}` not found in the netlist `{1}`")]
    PinNotFound(String, String),
}
