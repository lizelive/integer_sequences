/// a(n) = 3*n.
/// https://oeis.org/A000060

pub struct A000060;

impl crate::traits::IntegerSequence for A000060 {
    const NAME: &str = "Primes p such that the cycle length of 1/p equals (p-1)/4";

    const HEAD: &[crate::Value] = &[
        41, 73, 89, 97, 109, 113, 137, 149, 157, 193, 233, 241, 281, 313, 337, 353, 389, 401, 409,
        433, 449, 457, 521, 569, 577, 593, 601, 617, 641, 673, 701, 709, 733, 761, 769, 809, 857,
        877, 881, 929, 937, 953, 977, 1009, 1033, 1049, 1061, 1069, 1093, 1097, 1129, 1153, 1193,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000060";

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
    crate::tester::test_sequance_formula_matchces_head::<A000060>();
}
