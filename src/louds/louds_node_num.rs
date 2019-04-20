use super::LoudsNodeNum;

impl super::LoudsNodeNum {
    pub fn new(value: u64) -> LoudsNodeNum {
        LoudsNodeNum { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}
