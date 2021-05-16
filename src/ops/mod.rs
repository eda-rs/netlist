use crate::error::NetListError;
use crate::model::NetList;
use crate::NResult;
impl<W: Default, N: Default, G: Default, P: Default> NetList<W, N, G, P> {
    // get internal net data by name
    pub fn get_net(&self, name: &str) -> NResult<&W> {
        match self.net_map.get(name) {
            Some(i) => Ok(&self.nets[*i].data),
            None => Err(Box::new(NetListError::NetNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }
    // get mutable internal net data by name
    pub fn get_mut_net(&mut self, name: &str) -> NResult<&mut W> {
        match self.net_map.get(name) {
            Some(i) => Ok(&mut self.nets[*i].data),
            None => Err(Box::new(NetListError::NetNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // update internal net data by name
    pub fn update_net(&mut self, name: &str, new_data: &W) -> NResult<()>
    where
        W: Clone,
    {
        match self.net_map.get(name) {
            Some(i) => self.nets[*i].data = new_data.clone(),
            None => {
                return Err(Box::new(NetListError::NetNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
        Ok(())
    }

    // get internal gate data by name
    pub fn get_gate(&self, name: &str) -> NResult<&G> {
        match self.gate_map.get(name) {
            Some(i) => Ok(&self.gates[*i].data),
            None => Err(Box::new(NetListError::GateNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get mutable internal gate data by name
    pub fn get_mut_gate(&mut self, name: &str) -> NResult<&mut G> {
        match self.gate_map.get(name) {
            Some(i) => Ok(&mut self.gates[*i].data),
            None => Err(Box::new(NetListError::GateNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // update internal gate data by name
    pub fn update_gate(&mut self, name: &str, new_data: &G) -> NResult<()>
    where
        G: Clone,
    {
        match self.gate_map.get(name) {
            Some(i) => self.gates[*i].data = new_data.clone(),
            None => {
                return Err(Box::new(NetListError::GateNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
        Ok(())
    }

    // get internal pin data by name
    pub fn get_pin(&self, name: &str) -> NResult<&P> {
        match self.pin_map.get(name) {
            Some(i) => Ok(&self.pins[*i].data),
            None => Err(Box::new(NetListError::PinNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // get internal pin data by name
    pub fn get_mut_pin(&mut self, name: &str) -> NResult<&mut P> {
        match self.pin_map.get(name) {
            Some(i) => Ok(&mut self.pins[*i].data),
            None => Err(Box::new(NetListError::PinNotFound(
                name.to_string(),
                self.name.clone(),
            ))),
        }
    }

    // update internal pin data by name
    pub fn update_pin(&mut self, name: &str, new_data: &P) -> NResult<()>
    where
        P: Clone,
    {
        match self.pin_map.get(name) {
            Some(i) => self.pins[*i].data = new_data.clone(),
            None => {
                return Err(Box::new(NetListError::PinNotFound(
                    name.to_string(),
                    self.name.clone(),
                )))
            }
        }
        Ok(())
    }
}
