// READ | CLEAR | SET | TOGGLE
#[derive(Clone, Copy)]
struct BitWise{
    pub value: u8,
}

impl BitWise {
    pub fn new(index: u8) -> Result<Self, String> {
        match index {
            0..8 => Ok(BitWise { value: 3 }),
            _ => Err("Choose an index between 0 and 7".to_string()),
        }
    }

    pub fn clear(&mut self, index: u8) {
        let mask: u8 = !(1u8 << index);
        self.value &= mask;
    }
  
    pub fn read(self,index: u8) -> Result<bool, String> {
        if index > 7{
            return Err("Choose an index between 0 and 7".to_string());
        }
        Ok(self.value >> index & 1 == 1)
    }
}

fn main() {
    let mut bol_example = BitWise::new(1).unwrap();
    println!("{:08b}", bol_example.value);
    bol_example.clear(1);
    println!("{:08b}", bol_example.value);
    let bit = BitWise {value:1};
    for i in 0..=7{
        println!(" index {i} = {}", bit.read(i).unwrap());
    }
}
