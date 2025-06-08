use std::ops::{DivAssign, Rem};

pub trait IntegerExt {
     fn divide_out(&self, divisor: Self) -> Self;
 }

 impl <I> IntegerExt for I where I: DivAssign + Rem<Output = Self> + From<u8> + Copy + PartialEq {
     fn divide_out(&self, divisor: Self) -> Self {
         let mut result = *self;

         while result % divisor == 0.into() {
             result /= divisor;
         }

         result
     }
 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_divide_out() {
        assert_eq!(12_i32.divide_out(3), 4);
        assert_eq!(24_i64.divide_out(2), 3);
    }
}