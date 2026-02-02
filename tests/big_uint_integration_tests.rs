use big_numbers::BigUInt;

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

#[test]
fn test_div_single_simple() {
    let a = BigUInt::from_u32(100);
    let (q, r) = a.div_single(5);
    
    assert_eq!(q.limbs, vec![20]);
    assert_eq!(r, 0);
}

#[test]
fn test_div_single_with_remainder() {
    let a = BigUInt::from_u32(102);
    let (q, r) = a.div_single(5);
    
    assert_eq!(q.limbs, vec![20]);
    assert_eq!(r, 2);
}

#[test]
fn test_div_single_large() {
    let a = BigUInt { limbs: vec![0, 1] };
    let (q, r) = a.div_single(2);
    
    assert_eq!(q.limbs, vec![2147483648]);
    assert_eq!(r, 0);
}

#[test]
fn test_div_single_by_one() {
    let a = BigUInt::from_u32(12345);
    let (q, r) = a.div_single(1);
    assert_eq!(q.limbs, vec![12345]);
    assert_eq!(r, 0);
}

#[test]
fn test_div_single_to_zero() {
    let a = BigUInt::from_u32(10);
    let (q, r) = a.div_single(20);
    assert!(q.is_zero());
    assert_eq!(r, 10);
}

#[test]
fn test_div_single_zero_nominator() {
    let a = BigUInt::from_u32(0);
    let (q, r) = a.div_single(5);
    assert!(q.is_zero());
    assert_eq!(r, 0);
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_div_single_by_zero() {
    let a = BigUInt::from_u32(100);
    let _ = a.div_single(0);
}

#[test]
fn test_div_simple() {
    let a = BigUInt::from_u32(200);
    let b = BigUInt::from_u32(20);
    let (q, r) = a.div(&b);
    
    assert_eq!(q.limbs, vec![10]);
    assert!(r.is_zero());
}

#[test]
fn test_div_with_remainder() {
    let a = BigUInt::from_u32(205);
    let b = BigUInt::from_u32(20);
    let (q, r) = a.div(&b);
    
    assert_eq!(q.limbs, vec![10]);
    assert_eq!(r.limbs, vec![5]);
}

#[test]
fn test_div_large() {
    let a = BigUInt::from_u32(1187);
    let b = BigUInt::from_u32(37);
    let (q, r) = a.div(&b);
    
    assert_eq!(q.limbs, vec![32]);
    assert_eq!(r.limbs, vec![3]);
}

#[test]
fn test_div_small_by_large() {
    let a = BigUInt::from_u32(10);
    let b = BigUInt::from_u32(20);
    let (q, r) = a.div(&b);
    
    assert!(q.is_zero());
    assert_eq!(r.limbs, vec![10]);
}

#[test]
fn test_div_multi_limb_quotient() {
    let a = BigUInt { limbs: vec![0, 2] };
    let b = BigUInt::from_u32(1);
    let (q, r) = a.div(&b);

    assert_eq!(q.limbs, vec![0, 2]);
    assert!(r.is_zero());
}

#[test]
fn test_div_self() {
    let a = BigUInt { limbs: vec![123, 456, 789] };
    let (q, r) = a.div(&a);

    assert_eq!(q.limbs, vec![1]);
    assert!(r.is_zero());
}

#[test]
fn test_div_exact_multiple_large() {
    let list = vec![u32::MAX, u32::MAX];
    let b = BigUInt { limbs: list.clone() };
    
    let a = BigUInt { limbs: vec![12345, 67890] };
    let product = a.mul(&b);
    
    let (q, r) = product.div(&b);

    assert_eq!(q.limbs, a.limbs);
    assert!(r.is_zero());
}

#[test]
fn test_div_correction_case() {
    let v_hi = 2147483648u32; // 2^31
    let divisor = BigUInt { limbs: vec![0, v_hi] };
    
    let a = BigUInt { limbs: vec![
        0, 0, 0, 
        u32::MAX, u32::MAX
    ]};
    let b = BigUInt { limbs: vec![u32::MAX] };
    
    let (q, r) = a.div(&b);
    
    let check = b.mul(&q).add(&r);

    assert_eq!(check.limbs, a.limbs);
    assert!(r < b);
}
