// run with cargo run --example galactic_test --release
use big_numbers::big_uint::BigUInt;
use std::time::Instant;

fn main() {
    // 2^16 limbs = 65,536 limbs (~2.1 Million bits)
    // it took 54 seconds on my pc! mul is very slow :(
    let size = 1 << 16;
    println!("Preparing numbers with {} limbs ({} bits)...", size, size * 32);
    
    let a = BigUInt { limbs: vec![u32::MAX; size] };
    let b = BigUInt { limbs: vec![u32::MAX; size] };

    println!("Starting Galactic Multiplication (O(N^2))...");
    let start = Instant::now();
    
    let _result = a.mul(&b);
    
    let duration = start.elapsed();
    println!("--- RESULT ---");
    println!("Multiplication completed in: {:?}", duration);
    println!("That's roughly {:.2} seconds for a single 2.1 million bit multiplication.", duration.as_secs_f64());
}
