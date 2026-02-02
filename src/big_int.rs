use crate::big_uint::BigUInt;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BigInt {
    pub sign: Sign,
    pub magnitude: BigUInt,
}

impl BigInt {
    pub fn new() -> Self {
        BigInt { sign: Sign::Plus, magnitude: BigUInt::new() }
    }

    pub fn from_u32(value: u32) -> Self {
        BigInt { sign: Sign::Plus, magnitude: BigUInt::from_u32(value) }.normalize()
    }

    pub fn from_i32(value: i32) -> Self {
        let sign = if value < 0 { Sign::Minus } else { Sign::Plus };
        let magnitude = BigUInt::from_u32(value.unsigned_abs());
        
        BigInt { sign, magnitude }.normalize()
    }   

    pub fn negate(&self) -> Self {
        if self.magnitude.is_zero() {
            return self.clone();
        }

        let new_sign = match self.sign {
            Sign::Plus => Sign::Minus,
            Sign::Minus => Sign::Plus,
        };

        BigInt { sign: new_sign, magnitude: self.magnitude.clone() }
    }

    fn normalize(mut self) -> Self {
        if self.magnitude.is_zero() {
            self.sign = Sign::Plus;
        }
        self
    }   

    pub fn add(&self, other: &Self) -> Self {
        if self.sign == other.sign {
            let new_magnitude = self.magnitude.add(&other.magnitude);
            return BigInt { sign: self.sign, magnitude: new_magnitude }.normalize();
        }

        let (larger, smaller) = if self.magnitude >= other.magnitude {
            (self, other)
        } else {
            (other, self)
        };

        let new_magnitude = larger.magnitude.sub(&smaller.magnitude);
        
        BigInt { sign: larger.sign, magnitude: new_magnitude }.normalize()
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sign == Sign::Minus && !self.magnitude.is_zero() {
            write!(f, "-")?;
        }
        write!(f, "{}", self.magnitude)
    }
}
