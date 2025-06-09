use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use crate::util::integer;

#[derive(Copy, Clone, Hash, Debug)]
pub struct ModInt {
    value: i64,
    modulus: i64,
}

impl ModInt {
    pub fn new(value: i64, modulus: i64) -> Self {
        ModInt {
            value: value % modulus,
            modulus,
        }
    }

    pub fn pow(self, power: i64) -> Self {
        ModInt {
            value: integer::pow_mod(self.value, power, self.modulus),
            ..self
        }
    }

    pub fn value(self) -> i64 {
        self.value
    }

    pub fn usize(self) -> usize {
        self.value as usize
    }

    pub fn assign(&mut self, value: i64) {
        self.value = value % self.modulus;
    }
}

impl Into<i64> for ModInt {
    fn into(self) -> i64 {
        self.value
    }
}

impl <T: Into<i64>> Add<T> for ModInt {
    type Output = Self;

    fn add(self, other: T) -> Self {
        ModInt {
            value: (self.value + other.into()) % self.modulus,
            ..self
        }
    }
}

impl <T: Into<i64>> Sub<T> for ModInt {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        ModInt {
            value: (self.value - other.into() + self.modulus) % self.modulus,
            ..self
        }
    }
}

impl <T: Into<i64>> Mul<T> for ModInt {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        ModInt {
            value: (self.value * other.into()) % self.modulus,
            ..self
        }
    }
}

impl <T: Into<i64>> Rem<T> for ModInt {
    type Output = Self;

    fn rem(self, other: T) -> Self {
        ModInt {
            value: self.value % other.into(),
            ..self
        }
    }
}

impl <T: Into<i64>> AddAssign<T> for ModInt {
    fn add_assign(&mut self, other: T) {
        self.value += other.into();
        self.value %= self.modulus;
    }
}

impl <T: Into<i64>> SubAssign<T> for ModInt {
    fn sub_assign(&mut self, other: T) {
        self.value -= other.into();
        self.value += self.modulus;
        self.value %= self.modulus;
    }
}

impl <T: Into<i64>> MulAssign<T> for ModInt {
    fn mul_assign(&mut self, other: T) {
        self.value *= other.into();
        self.value %= self.modulus;
    }
}

impl <T: Into<i64>> RemAssign<T> for ModInt {
    fn rem_assign(&mut self, other: T) {
        self.value %= other.into();
    }
}

impl Neg for ModInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ModInt {
            value: self.modulus - self.value,
            ..self
        }
    }
}

impl PartialEq for ModInt {
    fn eq(&self, other: &ModInt) -> bool {
        self.value == other.value
    }
}

impl PartialEq<i64> for ModInt {
    fn eq(&self, other: &i64) -> bool {
        self.value == *other % self.modulus
    }
}

impl Eq for ModInt {}

#[cfg(test)]
mod tests {
    use crate::util::integer::mod_int::ModInt;

    #[test]
    fn test_arithmetic() {
        let a = ModInt::new(7,10);
        let a = a + ModInt::new(5,10);
        assert_eq!(a.value, 2);

        let a = a + 24_i64;
        assert_eq!(a.value, 6);

        let a = a * 7;
        assert_eq!(a.value, 2);

        let a = -a;
        assert_eq!(a.value, 8);

        let mut a = a - 9;
        assert_eq!(a.value, 9);

        a += 2;
        assert_eq!(a.value, 1);

        a -= 7;
        assert_eq!(a.value, 4);

        a *= 9;
        assert_eq!(a.value, 6);

        assert_eq!(a, 6);
        assert_eq!(a, 16);

        a %= 4;
        assert_eq!(a.value, 2);
    }
}