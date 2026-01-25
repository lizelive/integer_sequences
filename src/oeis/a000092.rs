/// a(n) = floor(n^2/4).
/// https://oeis.org/A000092

pub struct A000092;

impl crate::traits::IntegerSequence for A000092 {
    const NAME: &str = "Primes p such that p + 6 is also prime";

    const HEAD: &[crate::Value] = &[
        5, 7, 11, 13, 17, 23, 31, 37, 41, 47, 53, 61, 67, 73, 83, 97, 101, 103, 107, 131, 151, 157,
        167, 173, 191, 193, 223, 227, 233, 241, 251, 257, 263, 271, 277, 307, 311, 331, 337, 347,
        353, 367, 373, 383, 433, 443, 457, 461, 503, 541, 547, 557, 563, 571, 587, 593, 601, 607,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000092";

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
    crate::tester::test_sequance_formula_matchces_head::<A000092>();
}
