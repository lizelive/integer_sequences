/// Number of partitions with exactly k parts.
/// https://oeis.org/A000076

pub struct A000076;

impl crate::traits::IntegerSequence for A000076 {
    const NAME: &str = "Primes p such that p+2 is also prime (lesser of twin primes)";

    const HEAD: &[crate::Value] = &[
        3, 5, 11, 17, 29, 41, 59, 71, 101, 107, 137, 149, 179, 191, 197, 227, 239, 269, 281, 311,
        347, 419, 431, 461, 521, 569, 599, 617, 641, 659, 809, 821, 827, 857, 881, 1019, 1031,
        1049, 1061, 1091, 1151, 1229, 1277, 1289, 1301, 1319, 1427, 1451, 1481, 1487, 1607, 1619,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000076";

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
    crate::tester::test_sequance_formula_matchces_head::<A000076>();
}
