# BigInt from Scratch

A toy implementation of arbitrary-precision integers, written entirely from scratch without external resources.
Built as an educational project to understand how integer arithmetic works and how it can be expanded.

## Features

- Supports construction from binary, decimal and hex strings, as well as basic integers
- Printing as binary, decimal, hex
- Semi-naive addition, subtraction, multiplication, division, bit shifts and GCD/LCM
- Fast exponentiation by repeated squaring
- Short-circuiting comparison operators

## Usage

```rust
let a = BigInt::try_from("12345678901234567890").unwrap();
let b = BigInt::from(45678).unwrap();
println!("{}", a * b);
println!("{:b}", a ^ 33);
println!("{:x}", gcd(b, a));
```

## Did you know?

The longest Collatz (3n+1) sequence between 340282366920938463463374607431768211456 (2^128) and 340282366920938463463374607431768276992 (2^128 + 2^16) is 340282366920938463463374607431768220415, with 1493 steps.
