use crate::big_uint::{BigUInt, ParseBigIntError};
use std::fmt;
use std::str::FromStr;

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

    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.negate())
    }

    pub fn mul(&self, other: &Self) -> Self {
        let new_magnitude = self.magnitude.mul(&other.magnitude);
        let new_sign = if self.sign == other.sign {
            Sign::Plus
        } else {
            Sign::Minus
        };
        
        BigInt { sign: new_sign, magnitude: new_magnitude }.normalize()
    }

    pub fn div(&self, other: &Self) -> (Self, Self) {
        let (quotient_magnitude, remainder_magnitude) = self.magnitude.div(&other.magnitude);
        let quotient_sign = if self.sign == other.sign {
            Sign::Plus
        } else {
            Sign::Minus
        };
        
        (BigInt { sign: quotient_sign, magnitude: quotient_magnitude }.normalize(),
         BigInt { sign: self.sign, magnitude: remainder_magnitude }.normalize())
    }
}

impl FromStr for BigInt {
    type Err = ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseBigIntError);
        }

        let (sign, magnitude_str) = if s.starts_with('-') {
            (Sign::Minus, &s[1..])
        } else if s.starts_with('+') {
            (Sign::Plus, &s[1..])
        } else {
            (Sign::Plus, s)
        };

        if magnitude_str.is_empty() {
            return Err(ParseBigIntError);
        }

        let magnitude = BigUInt::from_str(magnitude_str)?;
        Ok(BigInt { sign, magnitude }.normalize())
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
