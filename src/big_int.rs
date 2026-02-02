#[derive(Debug, PartialEq)]
pub struct BigInt {
    pub limbs: Vec<u32>,
}

impl BigInt {
    pub fn new() -> Self {
        BigInt { limbs: vec![] }
    }

    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty() || self.limbs[0] == 0
    }

    pub fn from_u32(_n: u32) -> Self {
        BigInt { limbs: vec![_n] }
    }

    pub fn add(&self, _other: &Self) -> Self {
        let max_len = self.limbs.len().max(_other.limbs.len());
        let mut result = vec![];
        let mut carry: u32 = 0;

        for i in 0..max_len {
            let sum = (*self.limbs.get(i).unwrap_or(&0) as u64) + (*_other.limbs.get(i).unwrap_or(&0) as u64) + (carry as u64);
            carry = (sum >> 32) as u32;
            result.push(sum as u32);
        }

        if carry > 0 {
            result.push(carry);
        }

        BigInt { limbs: result }
    }

    pub fn sub(&self, _other: &Self) -> Self {
        !todo!()   
    }

    pub fn mul(&self, _other: &Self) -> Self {
        !todo!()   
    }

    // division is the trickiest of them all but I'll leave the stub for it here
    pub fn div(&self, _other: &Self) -> Self {
        !todo!()   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_zero() {
        let a = BigInt::from_u32(0);
        let b = BigInt::new();
        let c = BigInt::from_u32(15);
        assert!(a.is_zero());
        assert!(b.is_zero());
        assert!(!c.is_zero())
    }

    #[test]
    fn test_simple_addition() {
        let a = BigInt::from_u32(10);
        let b = BigInt::from_u32(20);
        let c = a.add(&b);
        
        assert_eq!(c.limbs, vec![30]); 
    }

    #[test]
    fn test_addition_with_carry() {
        let a = BigInt::from_u32(u32::MAX);
        let b = BigInt::from_u32(1);
        let c = a.add(&b);

        assert_eq!(c.limbs, vec![0, 1]);
    }

    #[test]
    fn test_cascade_carry() {
        let a = BigInt { limbs: vec![u32::MAX, u32::MAX] };
        let b = BigInt::from_u32(1);
        let c = a.add(&b);

        assert_eq!(c.limbs, vec![0, 0, 1]);
    }

    #[test]
    fn test_carry_mid_number() {
        let a = BigInt { limbs: vec![u32::MAX, 10] };
        let b = BigInt::from_u32(1);
        let c = a.add(&b);

        assert_eq!(c.limbs, vec![0, 11]);
    }

    #[test]
    fn test_debug_print() {
        let a = BigInt::from_u32(10);
        let a_repr = format!("{:?}", a); 

        let b = BigInt::from_u32(u32::MAX);
        let c = BigInt::from_u32(1);
        let sum_repr = format!("{:?}", b.add(&c));
        
        assert_eq!(a_repr, "BigInt { limbs: [10] }");
        assert_eq!(sum_repr, "BigInt { limbs: [0, 1] }");
    }
}
