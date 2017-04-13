use utils::u256::U256;

#[repr(C)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct H256([u8; 32]);

impl Default for H256 {
    fn default() -> H256 {
        H256([0u8; 32])
    }
}

impl Into<U256> for H256 {
    fn into(self) -> U256 {
        U256::from(self.0.as_ref())
    }
}