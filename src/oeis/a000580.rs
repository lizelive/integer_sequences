/// a(n) = n^3 + 0*n + 8
/// https://oeis.org/A000580

pub struct A000580;

impl crate::traits::IntegerSequence for A000580 {
    const NAME: &str = "a(n) = n^3 + 0*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 9, 16, 35, 72, 133, 224, 351, 520, 737, 1008, 1339, 1736, 2205, 2752, 3383, 4104, 4921, 5840, 6867, 8008, 9269, 10656, 12175, 13832, 15633, 17584, 19691, 21960, 24397
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000580";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_580(n)
    }
}

const fn poly_580(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000580>();
}
