pub struct ConfigContext {
    pub esp_enabled: bool,
}

impl Default for ConfigContext {
    fn default() -> Self {
        Self {
            esp_enabled: false,
        }
    }
}
