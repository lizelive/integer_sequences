/// Number of partitions of n with largest part at most 4.
/// https://oeis.org/A000077

pub struct A000077;

impl crate::traits::IntegerSequence for A000077 {
    const NAME: &str = "Primes p such that p+4 is also prime";

    const HEAD: &[crate::Value] = &[
        3, 7, 13, 19, 37, 43, 61, 67, 79, 97, 103, 109, 139, 151, 157, 163, 181, 193, 199, 211,
        223, 229, 241, 271, 277, 283, 313, 337, 349, 373, 379, 397, 409, 421, 433, 439, 457, 463,
        487, 499, 523, 541, 571, 577, 601, 607, 613, 619, 631, 643, 673, 709, 727, 733, 739, 751,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000077";

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
    crate::tester::test_sequance_formula_matchces_head::<A000077>();
}
