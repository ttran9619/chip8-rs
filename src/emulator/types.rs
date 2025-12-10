use derive_more::{From, Into, Add, BitOr, BitAnd, BitXor, Sub};

#[derive(Debug, PartialEq, Clone, Copy, From, Into)]
pub struct FourBitValue(pub u8);
#[derive(Debug, PartialEq, Clone, Copy, From, Into, Add, BitOr, BitAnd, BitXor, Sub)]
pub struct EightBitValue(pub u8);
#[derive(Debug, PartialEq, Clone, Copy, From, Into)]
pub struct TwelveBitValue(pub u16);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RegisterNumber(pub FourBitValue);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MemoryAddress(pub TwelveBitValue);

impl From<MemoryAddress> for u16 {
    fn from(item: MemoryAddress) -> Self {
        item.0.into()
    }
}

impl From<MemoryAddress> for usize {
    fn from(item: MemoryAddress) -> Self {
        let raw_value: u16 = item.0.into();
        raw_value.into()
    }
}

impl From<RegisterNumber> for usize {
    fn from(item: RegisterNumber) -> Self {
        let value: u8 = item.0.into();
        value as usize
    }
}

impl RegisterNumber {
    pub fn zero() -> Self{
        RegisterNumber(0.into())
    }
}
