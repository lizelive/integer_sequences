/// a(n) = 5*T(n) + 6
/// https://oeis.org/A000396

pub struct A000396;

impl crate::traits::IntegerSequence for A000396 {
    const NAME: &str = "a(n) = 5*T(n) + 6";

    const HEAD: &[crate::Value] = &[
        6, 11, 21, 36, 56, 81, 111, 146, 186, 231, 281, 336, 396, 461, 531, 606, 686, 771, 861, 956, 1056, 1161, 1271, 1386, 1506
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000396";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_396(n)
    }
}

const fn tri_396(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000396>();
}
