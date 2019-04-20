use super::LoudsIndex;

impl super::LoudsIndex {
    pub fn new(value: u64) -> LoudsIndex {
        LoudsIndex { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}
