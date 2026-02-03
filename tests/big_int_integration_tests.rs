use big_numbers::big_int::{BigInt, Sign};
use std::str::FromStr;

#[test]
fn test_big_int_from_u32() {
    let a = BigInt::from_u32(100);
    assert_eq!(a.sign, Sign::Plus);
    assert_eq!(a.to_string(), "100");
}

#[test]
fn test_big_int_from_str() {
    let cases = vec![
        ("100", Sign::Plus, "100"),
        ("-100", Sign::Minus, "-100"),
        ("+100", Sign::Plus, "100"),
        ("0", Sign::Plus, "0"),
        ("-0", Sign::Plus, "0"),
        ("+0", Sign::Plus, "0"),
    ];

    for (input, expected_sign, expected_str) in cases {
        let a = BigInt::from_str(input).unwrap();
        assert_eq!(a.sign, expected_sign, "Sign mismatch for {}", input);
        assert_eq!(a.to_string(), expected_str, "String mismatch for {}", input);
    }

    assert!(BigInt::from_str("").is_err());
    assert!(BigInt::from_str("-").is_err());
    assert!(BigInt::from_str("12a").is_err());
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

#[test]
fn test_big_int_add_exhaustive() {
    let cases = vec![
        ("10", "20", "30"),
        ("10", "-20", "-10"),
        ("-10", "20", "10"),
        ("-10", "-20", "-30"),
        ("0", "0", "0"),
        ("100", "-100", "0"),
        ("4294967295", "1", "4294967296"),
        ("-4294967296", "1", "-4294967295"),
        ("18446744073709551615", "1", "18446744073709551616"),
    ];

    for (a_str, b_str, expected) in cases {
        let a = BigInt::from_str(a_str).unwrap();
        let b = BigInt::from_str(b_str).unwrap();
        assert_eq!(a.add(&b).to_string(), expected, "Failed: {} + {}", a_str, b_str);
    }
}

#[test]
fn test_big_int_sub_exhaustive() {
    let cases = vec![
        ("30", "10", "20"),
        ("10", "30", "-20"),
        ("10", "-10", "20"),
        ("-10", "10", "-20"),
        ("-10", "-10", "0"),
        ("0", "10", "-10"),
        ("4294967296", "1", "4294967295"),
        ("1", "4294967296", "-4294967295"),
        ("18446744073709551616", "1", "18446744073709551615"),
    ];

    for (a_str, b_str, expected) in cases {
        let a = BigInt::from_str(a_str).unwrap();
        let b = BigInt::from_str(b_str).unwrap();
        assert_eq!(a.sub(&b).to_string(), expected, "Failed: {} - {}", a_str, b_str);
    }
}

#[test]
fn test_big_int_mul_exhaustive() {
    let cases = vec![
        ("10", "20", "200"),
        ("10", "-20", "-200"),
        ("-10", "20", "-200"),
        ("-10", "-20", "200"),
        ("0", "5", "0"),
        ("1", "100", "100"),
        ("-1", "100", "-100"),
        ("4294967296", "2", "8589934592"),
        ("-4294967296", "-4294967296", "18446744073709551616"),
    ];

    for (a_str, b_str, expected) in cases {
        let a = BigInt::from_str(a_str).expect(&format!("Failed to parse a: {}", a_str));
        let b = BigInt::from_str(b_str).expect(&format!("Failed to parse b: {}", b_str));
        assert_eq!(a.mul(&b).to_string(), expected, "Failed: {} * {}", a_str, b_str);
    }
}

#[test]
fn test_big_int_div_exhaustive() {
    let cases = vec![
        ("7", "3", "2", "1"),
        ("-7", "3", "-2", "-1"),
        ("7", "-3", "-2", "1"),
        ("-7", "-3", "2", "-1"),
        ("100", "5", "20", "0"),
        ("0", "10", "0", "0"),

        ("18446744073709551616", "4294967296", "4294967296", "0"),
        ("18446744073709551617", "4294967296", "4294967296", "1"),
    ];

    for (a_str, b_str, exp_q, exp_r) in cases {
        let a = BigInt::from_str(a_str).unwrap();
        let b = BigInt::from_str(b_str).unwrap();
        let (q, r) = a.div(&b);
        assert_eq!(q.to_string(), exp_q, "Quotient mismatch for {} / {}", a_str, b_str);
        assert_eq!(r.to_string(), exp_r, "Remainder mismatch for {} / {}", a_str, b_str);
    }
}


