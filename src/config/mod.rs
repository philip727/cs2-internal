pub struct ConfigContext {
    pub print_values: bool,
}

impl Default for ConfigContext {
    fn default() -> Self {
        Self {
            print_values: false,
        }
    }
}
