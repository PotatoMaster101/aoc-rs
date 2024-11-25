use num::Integer;
use num::integer::{gcd, lcm};

/// Computes [GCD](https://en.wikipedia.org/wiki/Greatest_common_divisor) on a range of numbers.
#[inline]
pub fn gcd_iter<T: Integer>(nums: impl IntoIterator<Item = T>) -> T {
    nums.into_iter().fold(T::zero(), |acc, x| gcd(acc, x))
}

/// Computes [LCM](https://en.wikipedia.org/wiki/Least_common_multiple) on a range of numbers.
#[inline]
pub fn lcm_iter<T: Integer>(nums: impl IntoIterator<Item = T>) -> T {
    nums.into_iter().fold(T::one(), |acc, x| lcm(acc, x))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd_iter() {
        let sut = [8];
        assert_eq!(gcd_iter(sut), 8);

        let sut = [8, 12];
        assert_eq!(gcd_iter(sut), 4);

        let sut = [48, 180, 240, 60];
        assert_eq!(gcd_iter(sut), 12);
    }

    #[test]
    fn test_lcm_iter() {
        let sut = [12];
        assert_eq!(lcm_iter(sut), 12);

        let sut = [12, 15];
        assert_eq!(lcm_iter(sut), 60);

        let sut = [48, 180, 240, 60];
        assert_eq!(lcm_iter(sut), 720);
    }
}
