#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index(u8);

impl Index {
    const MAX_VALUE: u8 = 7;

    pub fn new(value: u8) -> Self {
        if value > Self::MAX_VALUE {
            panic!("Value {} is out of range for Index", value);
        }
        Index(value)
    }
    
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Index {
    fn from(item: u8) -> Self {
        Index::new(item)
    }
}


// READ | CLEAR | SET | TOGGLE
#[derive(Clone, Copy)]
pub struct ByteBool {
    value: u8,
}

impl ByteBool {
    pub fn new() -> Self {
        ByteBool { value: 0 }
    }

    pub fn clear(&mut self) {
        self.value = 0;
    }

    pub fn read(self, index: u8) -> bool {
        let i: Index = index.into();
        self.value >> i.get() & 1 == 1
    }

    pub fn set(&mut self, index: u8, new_value: bool) {
        let i: Index = index.into();
        if new_value == true {
            self.value = self.value | (1 << i.get());
        } else {
            self.value = self.value & !(1 << i.get());
        }
    }

    pub fn toggle(&mut self, index: u8) {
        let i: Index = index.into();
        self.value ^= 1u8 << i.get();
    }

    pub fn display(self) {
        println!("{:08b}", self.value);
    }
}
