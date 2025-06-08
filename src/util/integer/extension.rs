use std::ops::{AddAssign, DivAssign, Rem};

pub trait IntegerExt {
    fn divide_out(&self, divisor: Self) -> Self;
    fn digit_sum(&self) -> Self;
    fn sorted_digit_string(&self) -> String;
 }

// TODO: way to make these generic type descriptions better?
 impl <I> IntegerExt for I where I: DivAssign + Rem<Output = Self> + AddAssign + From<u8> + Copy + PartialEq + Default + ToString {
     fn divide_out(&self, divisor: Self) -> Self {
         let mut result = *self;

         while result % divisor == 0.into() {
             result /= divisor;
         }

         result
     }

     fn digit_sum(&self) -> Self {
         let mut running_value = *self;
         let mut result = Self::default();
         let ten = Self::from(10);

         while running_value != 0.into() {
             result += running_value % ten;
             running_value /= ten;
         }

         result
     }

    fn sorted_digit_string(&self) -> String {
        let mut ret = self.to_string().chars().collect::<Vec<_>>();
        ret.sort_unstable();

        ret.into_iter().collect()
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

    #[test]
    fn test_digit_sum() {
        assert_eq!(12_i32.digit_sum(), 3);
        assert_eq!(24_i64.digit_sum(), 6);
        assert_eq!(102030405_u64.digit_sum(), 15);
    }

    #[test]
    fn test_sorted_digit_string() {
        assert_eq!(132_i32.sorted_digit_string(), "123");
        assert_eq!(908070605_i32.sorted_digit_string(), "000056789");

    }
}