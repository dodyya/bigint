mod big_int;
use big_int::BigInt;
use std::time::Instant;
fn main() {
    let mut n = BigInt::try_from("0xadf140bb").unwrap();
    // n <<= 10000;
    println!("{:?}", n)
}
