mod big_int;
use big_int::BigInt;
use big_int::{one, two, zero};
fn main() {
    let start = &BigInt::from(1) << 128;

    let one = BigInt::from(1);
    let two = BigInt::from(2);
    let three = BigInt::from(3);

    let mut max_len = 0;
    let mut max_start = BigInt::from(0);
    for i in 0..(1 << 16) {
        let mut collatz = &start + &BigInt::from(i);
        let mut len = 0;
        while collatz != one {
            len += 1;
            if collatz.is_even() {
                collatz >>= 1;
            } else {
                collatz = &(&collatz * &three) + &one;
            }
        }
        if len > max_len {
            max_len = len;
            max_start = &start + &BigInt::from(i);
        }
    }
    println!("{} with {} steps", max_start, max_len);
}
