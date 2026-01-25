/// Sophie Germain primes p: 2p+1 is also prime.
/// https://oeis.org/A000053

pub struct A000053;

impl crate::traits::IntegerSequence for A000053 {
    const NAME: &str = "Sophie Germain primes p: 2p+1 is also prime";

    const HEAD: &[crate::Value] = &[
        1, 2, 3, 4, 5, 7, 9, 10, 12, 18, 24, 36, 37, 44, 51, 63, 66, 79, 82, 90, 96, 103, 107, 118,
        125, 136, 156, 172, 178, 189, 199, 213, 225, 228, 244, 256, 268, 282, 286, 291, 297, 310,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000053";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000053>();
}
