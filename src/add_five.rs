pub mod add_five {
    pub fn add_five(base: u32) -> u32 {
        base + 5
    }

    pub fn add_five_then_equal_one(base: u32) -> bool {
        crate::equal::equal_one::equal_one(add_five(base))
    }
}
