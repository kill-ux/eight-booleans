use std::{
    fmt::Binary,
    ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr},
};

pub trait UnsignedInt:
    Sized
    + Binary
    + PartialEq
    + Default
    + Shr<u8, Output = Self>
    + Shl<u8, Output = Self>
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Copy
{
    fn one() -> Self;
    fn count_zeros(self) -> u8 ;
}

impl UnsignedInt for u8 {
    fn one() -> Self {
        1
    }
    fn count_zeros(self) -> u8 {
        self.count_zeros() as u8
    }
}
impl UnsignedInt for u16 {
    fn one() -> Self {
        1
    }
    fn count_zeros(self) -> u8 {
        self.count_zeros() as u8
    }
}
impl UnsignedInt for u32 {
    fn one() -> Self {
        1
    }
    fn count_zeros(self) -> u8 {
        self.count_zeros() as u8
    }
}
impl UnsignedInt for u64 {
    fn one() -> Self {
        1
    }
    fn count_zeros(self) -> u8 {
        self.count_zeros() as u8
    }
}
impl UnsignedInt for u128 {
    fn one() -> Self {
        1
    }
    fn count_zeros(self) -> u8 {
        self.count_zeros() as u8
    }
}
impl UnsignedInt for usize {
    fn one() -> Self {
        1
    }
    fn count_zeros(self) -> u8 {
        self.count_zeros() as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index(u8);

impl Index {

    pub fn new<T: UnsignedInt>(value: u8, bite: T) -> Self {
        let max_value: u8 = bite.count_zeros() - 1;
        if value > max_value {
            panic!("Value {} is out of range for Index", value);
        }
        Index(value)
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

// READ | CLEAR | SET | TOGGLE
#[derive(Clone, Copy)]
pub struct ByteBool<T: UnsignedInt> {
    value: T,
}

impl Default for ByteBool<u8> {
    fn default() -> Self {
        ByteBool { value: 0 }
    }
}

impl<T: UnsignedInt> ByteBool<T> {
    pub fn new() -> Self {
        ByteBool {
            value: T::default(),
        }
    }

    pub fn clear(&mut self) {
        self.value = T::default();
    }

    pub fn read(self, index: u8) -> bool {
        let i: Index = Index::new(index, T::default());
        ((self.value >> i.get()) & T::one()) == T::one()
    }

    pub fn set(&mut self, index: u8, new_value: bool) {
        let i: Index = Index::new(index, T::default());
        if new_value == true {
            self.value = self.value | (T::one() << i.get());
        } else {
            self.value = self.value & !(T::one() << i.get());
        }
    }

    pub fn toggle(&mut self, index: u8) {
        let i: Index = Index::new(index, T::default());
        self.value = self.value ^ T::one() << i.get();
    }

    pub fn display(self) {
        let max = T::default().count_zeros() - 1;
        println!("Max bits for this type: {}", max);
        println!(
            "{:0width$b}",
            self.value,
            width = T::default().count_zeros() as usize
        );
    }
}