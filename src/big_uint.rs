use std::cmp::Ordering;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBigIntError;

impl fmt::Display for ParseBigIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid digit in string")
    }
}

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
        let self_real_len = self.limbs.iter().rposition(|&l| l != 0).map_or(0, |p| p + 1).max(1);
        let other_real_len = other.limbs.iter().rposition(|&l| l != 0).map_or(0, |p| p + 1).max(1);

        let len_cmp = self_real_len.cmp(&other_real_len);
        if len_cmp != Ordering::Equal {
            return len_cmp;
        }

        for i in (0..self_real_len).rev() {
            let self_val = self.limbs.get(i).unwrap_or(&0);
            let other_val = other.limbs.get(i).unwrap_or(&0);
            let limb_cmp = self_val.cmp(other_val);
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
        self.limbs.iter().all(|&l| l == 0)
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

    // TODO: optimize multiplication, it's the slowest operation in the library
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

    pub fn div_single(&self, _divisor: u32) -> (Self, u32) {
        if _divisor == 0 {
            panic!("Division by zero");
        }

        let max_len = self.limbs.len();
        let mut result = vec![];
        let mut remainder: u64 = 0;

        for i in (0..max_len).rev() {
            let curr_val = (remainder << 32) + (*self.limbs.get(i).unwrap_or(&0) as u64);
            let quotient = curr_val / (_divisor as u64);
            remainder = curr_val % (_divisor as u64);
            result.push(quotient as u32);
        }

        result.reverse();
        (BigUInt { limbs: result }.truncate(), remainder as u32)
    }

    // TODO: optimize division by finding a way to get rid of .clone() calls
    pub fn div(&self, _other: &Self) -> (Self, Self) {
        if _other.is_zero() {
            panic!("Division by zero");
        }
        
        let n = self.limbs.len();
        let m = _other.limbs.len();

        if n < m {
            return (BigUInt::new(), self.clone());
        }

        let q_n = n - m + 1;
        let mut q = vec![0; q_n];

        let divisor_top = *_other.limbs.last().unwrap();

        let mut remainder = self.clone();
        remainder.limbs.push(0);

        for i in (0..q_n).rev() {
            let u_hi = *remainder.limbs.get(i + m).unwrap_or(&0);
            let u_lo = *remainder.limbs.get(i + m - 1).unwrap_or(&0);

            let transient = BigUInt { limbs: vec![u_lo, u_hi] }.truncate();

            let mut guess = *transient.div_single(divisor_top).0.limbs.get(0).unwrap_or(&0);

            q[i] = guess;

            let mut tmp = _other.mul_single(guess).shift_limbs(i);

            while tmp > remainder {
                guess -= 1;
                tmp = _other.mul_single(guess).shift_limbs(i);
            }

            remainder = remainder.sub(&tmp);
        }

        (BigUInt { limbs: q }.truncate(), remainder.truncate())
    }
}

impl FromStr for BigUInt {
    type Err = ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseBigIntError);
        }

        let mut result = BigUInt::from_u32(0);

        for c in s.chars() {
            let digit = c.to_digit(10).ok_or(ParseBigIntError)?;
            
            result = result.mul_single(10);
            result = result.add(&BigUInt::from_u32(digit));
        }

        Ok(result.truncate())
    }
}

impl fmt::Display for BigUInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let mut temp = self.clone();
        let mut digits = String::new();

        while !temp.is_zero() {
             let (q, r) = temp.div_single(10);
             temp = q;
             digits.push(std::char::from_digit(r, 10).unwrap());
         }

         write!(f, "{}", digits.chars().rev().collect::<String>())
    }
}




