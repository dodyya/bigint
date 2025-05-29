const CHUNK_SIZE: usize = 8;
type ChunkType = u8;

use std::{cmp::min, ops::Index, sync::OnceLock};

// Global constant for BigInt with value 10
pub fn TEN() -> &'static BigInt {
    static TEN: OnceLock<BigInt> = OnceLock::new();
    TEN.get_or_init(|| BigInt::from(10))
}

pub fn ONE() -> &'static BigInt {
    static ONE: OnceLock<BigInt> = OnceLock::new();
    ONE.get_or_init(|| BigInt::from(1))
}

pub fn ZERO() -> &'static BigInt {
    static ZERO: OnceLock<BigInt> = OnceLock::new();
    ZERO.get_or_init(|| BigInt::from(0))
}

#[derive(Debug, Clone, PartialEq)]
pub struct BigInt {
    // least‐significant chunk first
    chunks: Vec<ChunkType>,
}

impl TryFrom<&str> for BigInt {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            if value.chars().any(|c| !matches!(c, '0'..='9' | '_')) {
                return Err("Invalid decimal string".to_string());
            };
            return Ok(Self::dec_parse(value));
        }
        match &value[0..2] {
            "0b" => {
                let bin_str = &value[2..];
                if bin_str.chars().any(|c| !matches!(c, '0' | '1' | '_')) {
                    return Err("Invalid binary string".to_string());
                }
                let mut chunks = Self::bin_parse(bin_str);
                trim(&mut chunks);
                Ok(BigInt { chunks })
            }
            "0x" => {
                let hex_str = &value[2..];
                if hex_str.chars().any(|c| {
                    !matches!(
                        c,
                        '0'..='9'
                            | 'a'..='f'
                            | 'A'..='F'
                            | '_'
                    )
                }) {
                    return Err("Invalid hexadecimal string".to_string());
                }
                let mut chunks = Self::hex_parse(hex_str);
                trim(&mut chunks);
                Ok(BigInt { chunks })
            }
            _ => {
                if value.chars().any(|c| !matches!(c, '0'..='9' | '_')) {
                    return Err("Invalid decimal string".to_string());
                };
                Ok(Self::dec_parse(value))
            }
        }
    }
}

fn trim(chunks: &mut Vec<ChunkType>) {
    if let Some(i) = chunks.iter().rposition(|x| *x != 0) {
        let new_len = i + 1;
        chunks.truncate(new_len);
    }
}

impl BigInt {
    pub fn trim(&mut self) {
        trim(&mut self.chunks);
    }

    pub fn from(value: ChunkType) -> Self {
        Self {
            chunks: vec![value],
        }
    }

    fn bin_parse(bit_str: &str) -> Vec<ChunkType> {
        bit_str
            .bytes()
            .filter(|&b| b != b'_')
            .map(|b| b - b'0')
            .collect::<Vec<_>>()
            .rchunks(CHUNK_SIZE)
            .map(|chunk| {
                chunk
                    .iter()
                    .fold(0, |acc, &bit| (acc << 1) | bit as ChunkType)
            })
            .collect()
    }

    fn hex_parse(hex_str: &str) -> Vec<ChunkType> {
        hex_str
            .bytes()
            .filter(|&b| b != b'_') // drop underscores inline
            .collect::<Vec<_>>()
            .rchunks(CHUNK_SIZE / 4)
            .map(|chunk| {
                chunk.iter().fold(0 as ChunkType, |acc, &byte| {
                    (acc << 4) | Self::hex_char(byte)
                })
            })
            .collect()
    }

    fn hex_char(byte: u8) -> ChunkType {
        match byte {
            b'0'..=b'9' => (byte - b'0') as ChunkType,
            b'A'..=b'F' => (byte - b'A' + 10) as ChunkType,
            b'a'..=b'f' => (byte - b'a' + 10) as ChunkType,
            _ => panic!("Invalid hex character{}", byte),
        }
    }
    fn dec_parse(dec_str: &str) -> BigInt {
        dec_str
            .trim_start_matches("0")
            .chars()
            .filter(|&x| x != '_')
            .fold(ZERO().clone(), |acc, c| {
                acc * TEN().clone() + BigInt::from(c as ChunkType - b'0' as ChunkType)
            })
    }
    pub fn bit(&self, index: usize) -> ChunkType {
        let chunk: ChunkType = self.chunks[index / CHUNK_SIZE].clone();

        return (chunk >> (index % CHUNK_SIZE)) & 1;
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        let chunk_index = index / CHUNK_SIZE;
        let bit_index = index % CHUNK_SIZE;

        if chunk_index >= self.chunks.len() {
            self.chunks.resize(chunk_index + 1, 0);
        }

        let chunk = self.chunks[chunk_index];
        let mask = 1 << bit_index;

        if value {
            self.chunks[chunk_index] = chunk | mask;
        } else {
            self.chunks[chunk_index] = chunk & !mask;
        }
    }

    pub fn len(&self) -> usize {
        self.chunks.len() * CHUNK_SIZE - self.chunks.last().unwrap().leading_zeros() as usize
    }

    fn digit(&self) -> ChunkType {
        (self.clone() % TEN().clone())
            .chunks
            .first()
            .unwrap_or(&0)
            .clone()
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // print most‐significant chunk first
        let mut temp = self.clone();
        let mut digit = self.digit();
        let mut out: String = "".into();
        while temp.len() > 0 {
            out.extend(digit.to_string().chars());
            temp /= TEN().clone();
            digit = temp.digit();
        }
        write!(f, "{}", out.chars().rev().collect::<String>())
    }
}

impl std::ops::Add for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        let len1 = self.chunks.len();
        let len2 = other.chunks.len();
        let max_len = std::cmp::max(len1, len2);

        let mut out: Vec<ChunkType> = Vec::with_capacity(max_len + 1);
        let mut carry: ChunkType = 0;

        for i in 0..max_len {
            let a = if i < len1 { self.chunks[i] } else { 0 };
            let b = if i < len2 { other.chunks[i] } else { 0 };
            let (sum1, ov1) = a.overflowing_add(b);
            let (sum2, ov2) = sum1.overflowing_add(carry);
            carry = (ov1 | ov2) as ChunkType;
            out.push(sum2);
        }

        if carry != 0 {
            out.push(carry);
        }

        BigInt { chunks: out }
    }
}

impl std::ops::Shl<u32> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: u32) -> BigInt {
        let big_offset = rhs / CHUNK_SIZE as u32;
        let little_offset = rhs % CHUNK_SIZE as u32;

        let mut out = self.chunks.clone();
        let mut new_overflow: ChunkType;
        let mut old_overflow: ChunkType = 0;

        if little_offset != 0 {
            for i in 0..out.len() {
                new_overflow = &out[i] >> (CHUNK_SIZE as u32 - little_offset); // Front bits that'd get shifted out
                out[i] <<= little_offset;
                out[i] |= old_overflow;
                old_overflow = new_overflow;
            }

            if old_overflow != 0 {
                out.push(old_overflow);
            }
        }

        for _ in 0..big_offset {
            out.insert(0, 0 as ChunkType);
        }

        BigInt { chunks: out }
    }
}

impl std::ops::ShlAssign<u32> for BigInt {
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.clone() << rhs;
    }
}

impl std::ops::Sub for BigInt {
    type Output = BigInt;

    fn sub(self, other: BigInt) -> BigInt {
        let len1 = self.chunks.len();
        let len2 = other.chunks.len();
        let max_len = std::cmp::max(len1, len2);

        let mut out: Vec<ChunkType> = Vec::with_capacity(max_len + 1);
        let mut borrow: ChunkType = 0;

        for i in 0..max_len {
            let a = if i < len1 { self.chunks[i] } else { 0 };
            let b = if i < len2 { other.chunks[i] } else { 0 };
            let (sum1, ov1) = a.overflowing_sub(b);
            let (sum2, ov2) = sum1.overflowing_sub(borrow);
            borrow = (ov1 | ov2) as ChunkType;
            out.push(sum2);
        }

        trim(&mut out);

        BigInt { chunks: out }
    }
}

impl std::ops::AddAssign for BigInt {
    fn add_assign(&mut self, other: BigInt) {
        *self = self.clone() + other;
    }
}

impl std::ops::SubAssign for BigInt {
    fn sub_assign(&mut self, other: BigInt) {
        *self = self.clone() - other;
    }
}

impl std::ops::Mul for BigInt {
    type Output = BigInt;

    fn mul(self, other: BigInt) -> BigInt {
        let mut out: BigInt = BigInt::from(0);
        let mut shift = 0;

        for i in 0..self.chunks.len() {
            for j in 0..CHUNK_SIZE {
                if self.chunks[i] & (1 << j) != 0 {
                    out += other.clone() << shift;
                }

                shift += 1;
            }
        }

        out
    }
}

impl std::ops::MulAssign for BigInt {
    fn mul_assign(&mut self, other: BigInt) {
        *self = self.clone() * other;
    }
}

impl std::cmp::PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.chunks.len() != other.chunks.len() {
            return Some(self.chunks.len().cmp(&other.chunks.len()));
        }

        for i in (0..self.chunks.len()).rev() {
            if self.chunks[i] != other.chunks[i] {
                return Some(self.chunks[i].cmp(&other.chunks[i]));
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl std::ops::Div for BigInt {
    type Output = BigInt;

    fn div(self, other: BigInt) -> BigInt {
        let len = self.len();
        let mut out: BigInt = BigInt::from(0);
        let mut temp = BigInt::from(0);

        for i in (0..len).rev() {
            temp <<= 1;
            temp += BigInt::from(self.bit(i));
            if temp >= other {
                temp -= other.clone();
                out.set_bit(i, true);
            }
        }

        out
    }
}

impl std::ops::DivAssign for BigInt {
    fn div_assign(&mut self, other: BigInt) {
        let len = self.len();
        let mut out: BigInt = BigInt::from(0);
        let mut temp = BigInt::from(0);

        for i in (0..len).rev() {
            temp <<= 1;
            temp += BigInt::from(self.bit(i));
            if temp >= other {
                temp -= other.clone();
                out.set_bit(i, true);
            }
        }

        self.chunks = out.chunks;
    }
}

impl std::ops::Rem for BigInt {
    type Output = BigInt;

    fn rem(self, other: BigInt) -> BigInt {
        let len = self.len();
        let mut temp = BigInt::from(0);

        for i in (0..len).rev() {
            temp <<= 1;
            temp += BigInt::from(self.bit(i));
            if temp >= other {
                temp -= other.clone();
            }
        }

        temp.trim();
        temp
    }
}

//   100010111
//   v   v vvv   -> 1000101110
//   10101110110 divided by 101:
//  -101
//     00111
//      -101
//        1001
//        -101
//         1001
//         -101
//          1000
//          -101
//            11
//
//
