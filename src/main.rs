use bitwise_operators::Index;

// READ | CLEAR | SET | TOGGLE
#[derive(Clone, Copy)]
struct BitWise{
    pub value: u8,
}

impl BitWise {
    pub fn new(index: Index) -> Result<Self, String> {
        Ok(BitWise{value:1})
    }

    pub fn clear(&mut self, index: Index) {
        let mask: u8 = !(1u8 << index.get());
        self.value &= mask;
    }
  
    pub fn read(self,index: Index) -> Result<bool, String> {
        Ok(self.value >> index.get() & 1 == 1)
    }
}

fn main() {
    let mut bol_example = BitWise::new(1.into()).unwrap();
    println!("{:08b}", bol_example.value);
    bol_example.clear(1.into());
    println!("{:08b}", bol_example.value);
    let bit = BitWise {value:1};
    for i in 0..=7{
        println!(" index {i} = {}", bit.read(i.into()).unwrap());
    }
}
