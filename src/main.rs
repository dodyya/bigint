mod big_int;
use big_int::BigInt;
use big_int::one;
use std::time::Instant;
fn main() {
    let two = BigInt::from(2);
    let mut n = BigInt::from(1);
    println!("{:x}", one() << 1)
}
