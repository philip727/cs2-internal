pub trait GetEntryIndex {
    fn index(&self) -> u32;

    fn is_valid(&self) -> bool {
        self.index() != 0xFFFFFFFF
    }

    fn get_entry_index(&self) -> i32;
}
