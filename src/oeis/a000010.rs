/// Euler totient function phi(n): count numbers <= n and coprime to n.
/// https://oeis.org/A000010

pub struct A000010;

impl crate::traits::IntegerSequence for A000010 {
    const NAME: &str = "Euler totient function phi(n)";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20, 12, 18,
        12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46, 16, 42, 20,
        32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44, 24, 70, 24, 72,
        36, 40, 36, 60, 24, 78, 32, 54, 40, 82, 24, 64, 42, 56, 40, 88, 24, 72, 44, 60, 46, 72, 32,
        96, 42, 60, 40,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000010";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        euler_phi(n)
    }
}

const fn euler_phi(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    let mut result = n;
    let mut m = n;
    let mut p = 2;
    
    while p * p <= m {
        if m % p == 0 {
            while m % p == 0 {
                m /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    
    if m > 1 {
        result -= result / m;
    }
    
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000010>();
}
