use bitwise_operators::Index;

// READ | CLEAR | SET | TOGGLE
#[derive(Clone, Copy)]
struct BitWise {
    value: u8,
}

impl BitWise {
    pub fn new() -> Self {
        BitWise { value: 0 }
    }

    pub fn clear(&mut self, index: Index) {
        self.value &= !(1u8 << index.get());
    }
  
    pub fn read(self,index: Index) -> bool {
        self.value >> index.get() & 1 == 1
    }

    pub fn set(&mut self,index: Index, new_value: bool ) {
        if new_value == true{
            self.value = self.value | (1 << index.get());
        }else{
            self.value = self.value & !(1 << index.get());
        }
    }

    pub fn toggle(&mut self, index: Index) {
        self.value ^= 1u8 << index.get();
    }
}

fn main() {
    let mut bol_example = BitWise::new();
    
    let bit = BitWise { value: 1 };
    for i in 0..=7 {
        println!(" index {i} = {}", bit.read(i.into()).unwrap());
    }

    println!("###### Toggle ######");
    println!("{:08b}", bol_example.value);
    bol_example.toggle(1.into());
    println!("{:08b}", bol_example.value);
    println!("####################");

    println!("###### Clear ######");
    println!("{:08b}", bol_example.value);
    bol_example.clear(1.into());
    println!("{:08b}", bol_example.value);
    println!("####################");
}
