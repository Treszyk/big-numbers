use big_numbers::big_int::{BigInt, Sign};

#[test]
fn test_big_int_from_u32() {
    let a = BigInt::from_u32(100);
    assert_eq!(a.sign, Sign::Plus);
    assert_eq!(a.to_string(), "100");
}

#[test]
fn test_big_int_negate() {
    let a = BigInt::from_u32(50);
    let b = a.negate();
    assert_eq!(b.sign, Sign::Minus);
    assert_eq!(b.to_string(), "-50");
    
    let c = b.negate();
    assert_eq!(c.sign, Sign::Plus);
    assert_eq!(c.to_string(), "50");
}

#[test]
fn test_big_int_negate_zero() {
    let a = BigInt::new();
    let b = a.negate();
    assert!(b.magnitude.is_zero());
    assert_eq!(b.to_string(), "0");
}

#[test]
fn test_big_int_add_same_sign() {
    let a = BigInt::from_u32(10);
    let b = BigInt::from_u32(20);
    assert_eq!(a.add(&b).to_string(), "30");

    let na = BigInt::from_u32(10).negate();
    let nb = BigInt::from_u32(20).negate();
    assert_eq!(na.add(&nb).to_string(), "-30");
}

#[test]
fn test_big_int_add_different_signs() {
    let a = BigInt::from_u32(100);
    let b = BigInt::from_u32(40).negate();
    assert_eq!(a.add(&b).to_string(), "60");

    let c = BigInt::from_u32(40);
    let d = BigInt::from_u32(100).negate();
    assert_eq!(c.add(&d).to_string(), "-60");
}

#[test]
fn test_big_int_add_zero_result() {
    let a = BigInt::from_u32(50);
    let b = BigInt::from_u32(50).negate();
    assert_eq!(a.add(&b).to_string(), "0");

    let c = BigInt::from_u32(50).negate();
    let d = BigInt::from_u32(50);
    assert_eq!(c.add(&d).to_string(), "0");
}

