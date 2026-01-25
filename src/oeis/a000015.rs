/// Smallest prime power >= n.
/// https://oeis.org/A000015

pub struct A000015;

impl crate::traits::IntegerSequence for A000015 {
    const NAME: &str = "Smallest prime power >= n";

    const HEAD: &[crate::Value] = &[
        1, 2, 3, 4, 5, 7, 7, 8, 9, 11, 11, 13, 13, 16, 16, 16, 17, 19, 19, 23, 23, 23, 23, 25, 25,
        27, 27, 29, 29, 31, 31, 32, 37, 37, 37, 37, 37, 41, 41, 41, 41, 43, 43, 47, 47, 47, 47, 49,
        49, 53, 53, 53, 53, 59, 59, 59, 59, 59, 59, 61, 61, 64, 64, 64, 67, 67, 67, 71, 71, 71, 71,
        73, 73, 79, 79, 79, 79, 79, 79, 81, 81, 83, 83, 89, 89, 89, 89, 89, 89, 97,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000015";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        smallest_prime_power_geq(n)
    }
}

const fn is_prime_power(n: crate::Index) -> bool {
    if n < 1 {
        return false;
    }
    if n == 1 {
        return true;
    }
    
    // Check if n = p^k for some prime p and k >= 1
    let mut m = n;
    let mut p = 2;
    
    // Find smallest prime factor
    while p * p <= m {
        if m % p == 0 {
            // p is a prime factor, check if n is a power of p
            while m % p == 0 {
                m /= p;
            }
            return m == 1;
        }
        p += 1;
    }
    
    // n is prime
    true
}

const fn smallest_prime_power_geq(n: crate::Index) -> crate::Value {
    if n <= 1 {
        return 1;
    }
    let mut k = n;
    while !is_prime_power(k) {
        k += 1;
    }
    k
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000015>();
}
