use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::util::integer;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
struct ModInt {
    value: i64,
    modulus: i64,
}

impl ModInt {
    fn new(value: i64, modulus: i64) -> Self {
        ModInt {
            value: value % modulus,
            modulus,
        }
    }

    fn pow(self, power: i64) -> Self {
        ModInt {
            value: integer::pow_mod(self.value, power, self.modulus),
            ..self
        }
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

impl Neg for ModInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ModInt {
            value: self.modulus - self.value,
            ..self
        }
    }
}

// TODO: figure out lifetimes so I can implement Eq, PartialEq

// impl <T> PartialEq<T> for ModInt where &T: Into<i64> {
//     fn eq(&self, other: &T) -> bool {
//         self.value == <&T as Into<i64>>::into(other) % self.modulus
//     }
// }
//
// impl Eq for ModInt {}

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

        // assert_eq!(a, 6);
        // assert_eq!(a, 16);
    }
}