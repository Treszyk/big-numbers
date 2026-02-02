pub struct BigInt {
    pub data: Vec<u32>,
}

impl BigInt {
    pub fn new() -> Self {
        BigInt { data: vec![] }
    }

    pub fn from_u32(n: u32) -> Self {
        BigInt { data: vec![] }
    }

    pub fn add(&self, _other: &Self) -> Self {
        BigInt { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let b = BigInt::new();
        assert_eq!(b.data.len(), 0);
    }

    #[test]
    fn test_simple_addition() {
        let a = BigInt::from_u32(10);
        let b = BigInt::from_u32(20);
        let c = a.add(&b);
        
        assert_eq!(c.data, vec![30]); 
    }

    #[test]
    fn test_addition_with_carry() {
        let a = BigInt::from_u32(u32::MAX);
        let b = BigInt::from_u32(1);
        let c = a.add(&b);

        assert_eq!(c.data, vec![0, 1]);
    }
}
