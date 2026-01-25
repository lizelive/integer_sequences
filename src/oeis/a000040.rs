/// The prime numbers.
/// https://oeis.org/A000040

pub struct A000040;

impl crate::traits::IntegerSequence for A000040 {
    const NAME: &str = "The prime numbers";

    const HEAD: &[crate::Value] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000040";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        nth_prime(n)
    }
}

const fn is_prime(n: crate::Index) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

const fn nth_prime(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    let mut count = 0;
    let mut candidate = 2;
    while count < n {
        if is_prime(candidate) {
            count += 1;
            if count == n {
                return candidate;
            }
        }
        candidate += 1;
    }
    0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000040>();
}
