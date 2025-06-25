#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PortMapping {
    external: u16,
    internal: u16
}

impl PortMapping {
    /// Add new port mapping of the type internal:internal
    pub fn new(external: u16, internal: u16) -> Self {
        PortMapping { external, internal }
    }

    /// Get host port
    pub fn get_external(&self) -> u16 {
        self.external
    }

    /// Get internal port
    pub fn get_internal(&self) -> u16 {
        self.internal
    }
}
