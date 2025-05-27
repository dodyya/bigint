#[derive(Debug, Clone, Copy)]
pub struct Int {
    value: i64,
}

impl Int {
    pub fn new(value: i64) -> Self {
        Int { value }
    }

    pub fn add(&self, other: &Int) -> Int {
        Int::new(self.value.wrapping_add(other.value))
    }
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::ops::Add for Int {
    type Output = Int;

    fn add(self, other: Int) -> Int {
        Int::new(self.value.wrapping_add(other.value))
    }
}

impl std::ops::AddAssign for Int {
    fn add_assign(&mut self, other: Int) {
        self.value = self.value.wrapping_add(other.value);
    }
}

impl std::ops::Sub for Int {
    type Output = Int;

    fn sub(self, other: Int) -> Int {
        Int::new(self.value.wrapping_sub(other.value))
    }
}

impl std::ops::SubAssign for Int {
    fn sub_assign(&mut self, other: Int) {
        self.value = self.value.wrapping_sub(other.value);
    }
}

impl std::ops::Mul for Int {
    type Output = Int;

    fn mul(self, other: Int) -> Int {
        Int::new(self.value.wrapping_mul(other.value))
    }
}

impl std::ops::MulAssign for Int {
    fn mul_assign(&mut self, other: Int) {
        self.value = self.value.wrapping_mul(other.value);
    }
}

impl std::ops::Div for Int {
    type Output = Int;

    fn div(self, other: Int) -> Int {
        Int::new(self.value / other.value)
    }
}

impl std::ops::DivAssign for Int {
    fn div_assign(&mut self, other: Int) {
        self.value /= other.value;
    }
}

impl std::ops::Rem for Int {
    type Output = Int;

    fn rem(self, other: Int) -> Int {
        Int::new(self.value % other.value)
    }
}

impl std::ops::RemAssign for Int {
    fn rem_assign(&mut self, other: Int) {
        self.value %= other.value;
    }
}

impl std::ops::Neg for Int {
    type Output = Int;

    fn neg(self) -> Int {
        Int::new(-self.value)
    }
}

impl std::ops::BitAnd for Int {
    type Output = Int;

    fn bitand(self, other: Int) -> Int {
        Int::new(self.value & other.value)
    }
}

impl std::ops::BitAndAssign for Int {
    fn bitand_assign(&mut self, other: Int) {
        self.value &= other.value;
    }
}

impl std::ops::BitOr for Int {
    type Output = Int;

    fn bitor(self, other: Int) -> Int {
        Int::new(self.value | other.value)
    }
}

impl std::ops::BitOrAssign for Int {
    fn bitor_assign(&mut self, other: Int) {
        self.value |= other.value;
    }
}

impl std::ops::BitXor for Int {
    type Output = Int;

    fn bitxor(self, other: Int) -> Int {
        Int::new(self.value ^ other.value)
    }
}

impl std::ops::BitXorAssign for Int {
    fn bitxor_assign(&mut self, other: Int) {
        self.value ^= other.value;
    }
}

impl std::ops::Shl for Int {
    type Output = Int;

    fn shl(self, other: Int) -> Int {
        Int::new(self.value << other.value)
    }
}

impl std::ops::ShlAssign for Int {
    fn shl_assign(&mut self, other: Int) {
        self.value <<= other.value;
    }
}

impl std::ops::Shr for Int {
    type Output = Int;

    fn shr(self, other: Int) -> Int {
        Int::new(self.value >> other.value)
    }
}

impl std::ops::ShrAssign for Int {
    fn shr_assign(&mut self, other: Int) {
        self.value >>= other.value;
    }
}

impl core::cmp::Eq for Int {}

impl core::cmp::Ord for Int {
    fn cmp(&self, other: &Int) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl core::cmp::PartialEq for Int {
    fn eq(&self, other: &Int) -> bool {
        self.value == other.value
    }
}

impl core::cmp::PartialOrd for Int {
    fn partial_cmp(&self, other: &Int) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
