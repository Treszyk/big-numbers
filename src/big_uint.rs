use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BigUInt {
    pub limbs: Vec<u32>,
}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let len_cmp = self.limbs.len().cmp(&other.limbs.len());
        if len_cmp != Ordering::Equal {
            return len_cmp;
        }

        for i in (0..self.limbs.len()).rev() {
            let limb_cmp = self.limbs[i].cmp(&other.limbs[i]);
            if limb_cmp != Ordering::Equal {
                return limb_cmp;
            }
        }

        Ordering::Equal
    }
}

impl BigUInt {
    pub fn new() -> Self {
        BigUInt { limbs: vec![0] }
    }

    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty() || (self.limbs.len() == 1 && self.limbs[0] == 0)
    }

    pub fn from_u32(_n: u32) -> Self {
        BigUInt { limbs: vec![_n] }
    }

    pub fn truncate(&self) -> Self {
        let mut result = self.limbs.clone();

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigUInt { limbs: result }
    }

    pub fn shift_limbs(&self, n: usize) -> Self {
        if self.is_zero() {
            return self.clone();
        }

        let mut new_limbs = vec![0; n];
        new_limbs.extend(&self.limbs);
        
        BigUInt { limbs: new_limbs }
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

        BigUInt { limbs: result }.truncate()
    }

    // BigUInt only handles subtraction when a >= b
    pub fn sub(&self, _other: &Self) -> Self {
        if self < _other {
            panic!("Subtraction underflow");
        }

        let max_len = self.limbs.len().max(_other.limbs.len());
        let mut result = vec![];
        let mut borrow: u32 = 0;

        for i in 0..max_len {
            let mut val_self = *self.limbs.get(i).unwrap_or(&0) as u64;
            let val_other = (*_other.limbs.get(i).unwrap_or(&0) as u64) + (borrow as u64);
            
            if val_self < val_other {
                val_self += u32::MAX as u64 + 1;
                borrow = 1;
            } else {
                borrow = 0;
            }

            let val = val_self - val_other;

            result.push(val as u32);
        }
        
        BigUInt { limbs: result }.truncate()
    }

    pub fn mul_single(&self, _other: u32) -> Self {
        let max_len = self.limbs.len();
        let mut result = vec![];
        let mut carry: u32 = 0;

        for i in 0..max_len {
            let product = (*self.limbs.get(i).unwrap_or(&0) as u64) * (_other as u64) + (carry as u64);
            carry = (product >> 32) as u32;
            result.push(product as u32);
        }

        if carry > 0 {
            result.push(carry);
        }

        BigUInt { limbs: result }.truncate()
    }

    pub fn mul(&self, _other: &Self) -> Self {
        if self.is_zero() || _other.is_zero() {
            return BigUInt::new();
        }
        
        let parts_len = _other.limbs.len(); 
        let mut parts = vec![];

        for i in 0..parts_len {
            let part = self.mul_single(_other.limbs[i]);
            parts.push(part);
        }

        let mut result = parts[0].clone();

        for j in 1..parts_len {
            result = result.add(&parts[j].shift_limbs(j));
        }

        result.truncate()
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
        let a = BigUInt::from_u32(0);
        let b = BigUInt::new();
        let c = BigUInt::from_u32(15);
        assert!(a.is_zero());
        assert!(b.is_zero());
        assert!(!c.is_zero())
    }

    #[test]
    fn test_simple_addition() {
        let a = BigUInt::from_u32(10);
        let b = BigUInt::from_u32(20);
        let c = a.add(&b);
        
        assert_eq!(c.limbs, vec![30]); 
    }

    #[test]
    fn test_addition_with_carry() {
        let a = BigUInt::from_u32(u32::MAX);
        let b = BigUInt::from_u32(1);
        let c = a.add(&b);

        assert_eq!(c.limbs, vec![0, 1]);
    }

    #[test]
    fn test_cascade_carry() {
        let a = BigUInt { limbs: vec![u32::MAX, u32::MAX] };
        let b = BigUInt::from_u32(1);
        let c = a.add(&b);

        assert_eq!(c.limbs, vec![0, 0, 1]);
    }

    #[test]
    fn test_carry_mid_number() {
        let a = BigUInt { limbs: vec![u32::MAX, 10] };
        let b = BigUInt::from_u32(1);
        let c = a.add(&b);

        assert_eq!(c.limbs, vec![0, 11]);
    }

    #[test]
    fn test_debug_print() {
        let a = BigUInt::from_u32(10);
        let a_repr = format!("{:?}", a); 

        let b = BigUInt::from_u32(u32::MAX);
        let c = BigUInt::from_u32(1);
        let sum_repr = format!("{:?}", b.add(&c));
        
        assert_eq!(a_repr, "BigUInt { limbs: [10] }");
        assert_eq!(sum_repr, "BigUInt { limbs: [0, 1] }");
    }

    #[test]
    fn test_mul_single() {
        let a = BigUInt::from_u32(10);
        let c = a.mul_single(5);

        assert_eq!(c.limbs, vec![50]);

        let a = BigUInt::from_u32(u32::MAX);
        let c = a.mul_single(2);

        assert_eq!(c.limbs, vec![u32::MAX - 1, 1]);
    }

    #[test]
    fn test_mul_zero() {
        let a = BigUInt::from_u32(10);
        let b = BigUInt::new();
        let c = a.mul(&b);

        assert!(c.is_zero());
    }

    #[test]
    fn test_simple_multiplication() {
        let a = BigUInt::from_u32(10);
        let b = BigUInt::from_u32(20);
        let c = a.mul(&b);

        assert_eq!(c.limbs, vec![200]);
    }

    #[test]
    fn test_mul_large() {
        let a = BigUInt::from_u32(u32::MAX);
        let b = BigUInt::from_u32(u32::MAX);
        let c = a.mul(&b);

        assert_eq!(c.limbs, vec![1, u32::MAX - 1]);
    }
    
    #[test]
    fn test_mul_tripling_max() {
        let a = BigUInt::from_u32(u32::MAX);
        let b = BigUInt::from_u32(3);
        let c = a.mul(&b);

        assert_eq!(c.limbs, vec![u32::MAX - 2, 2]);
    }

    #[test]
    fn test_sub_simple() {
        let a = BigUInt::from_u32(20);
        let b = BigUInt::from_u32(10);
        let c = a.sub(&b);

        assert_eq!(c.limbs, vec![10]);
    }

    #[test]
    fn test_sub_borrow() {
        let a = BigUInt { limbs: vec![0, 1] };
        let b = BigUInt::from_u32(1);
        let c = a.sub(&b);

        assert_eq!(c.limbs, vec![u32::MAX]);
    }

    #[test]
    #[should_panic(expected = "Subtraction underflow")]
    fn test_sub_underflow() {
        let a = BigUInt::from_u32(10);
        let b = BigUInt::from_u32(20);
        let _c = a.sub(&b);
    }

    #[test]
    fn test_sub_cascade_borrow() {
        let a = BigUInt { limbs: vec![0, 0, 1] };
        let b = BigUInt::from_u32(1);
        let c = a.sub(&b);
        
        assert_eq!(c.limbs, vec![u32::MAX, u32::MAX]);
    }

    #[test]
    fn test_sub_large_diff() {
        let a = BigUInt { limbs: vec![10, 5] };
        let b = BigUInt::from_u32(20);
        let c = a.sub(&b);
        
        assert_eq!(c.limbs, vec![u32::MAX - 9, 4]);
    }
}
