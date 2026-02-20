use super::com::ServerComConfig;

#[derive(Default, Debug)]
pub struct FilesServerConfig {
    srv_com: ServerComConfig
}

impl FilesServerConfig {
    pub fn new(srv_com_cfg: ServerComConfig) -> Self {
        Self { srv_com: srv_com_cfg }
    }

    pub fn srv_com(&self) -> &ServerComConfig {
        &self.srv_com
    }

    pub fn srv_com_mut(&mut self) -> &mut ServerComConfig {
        &mut self.srv_com
    }
}