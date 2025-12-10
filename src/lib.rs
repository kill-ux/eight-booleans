#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index(u8);

impl Index {
    const MAX_VALUE: u8 = 7;

    pub fn new(value: u8) -> Result<Self, String> {
        if value <= Self::MAX_VALUE {
            Ok(Index(value))
        } else {
            Err(format!(
                "Value must be between 0 and {}, but got {}", 
                Self::MAX_VALUE, 
                value
            ))
        }
    }

    pub fn new_unchecked(value: u8) -> Self {
        if value > Self::MAX_VALUE {
            panic!("Value {} is out of range for Index", value);
        }
        Index(value)
    }
    
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl From<Index> for u8 {
    fn from(item: Index) -> Self {
        item.0
    }
}

impl From<u8> for Index {
    fn from(item: u8) -> Self {
        Index::new_unchecked(item)
    }
}