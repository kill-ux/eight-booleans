// READ | CLEAR | SET | TOGGLE
#[derive(Clone, Copy)]
struct BitWise{
    pub value: u8,
}

impl BitWise {
    pub fn read(self,index: u8) -> Result<bool, String> {
        if index > 7{
            return Err("Choose an index between 0 and 7".to_string());
        }
        Ok(self.value >> index & 1 == 1)
    }
}

fn main() {
    let bit = BitWise {value:1};
    for i in 0..=7{
        println!(" index {i} = {}", bit.read(i).unwrap());
    }
}
