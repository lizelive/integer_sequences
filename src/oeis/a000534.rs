/// a(n) = n^3 + 4*n + 3
/// https://oeis.org/A000534

pub struct A000534;

impl crate::traits::IntegerSequence for A000534 {
    const NAME: &str = "a(n) = n^3 + 4*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 8, 19, 42, 83, 148, 243, 374, 547, 768, 1043, 1378, 1779, 2252, 2803, 3438, 4163, 4984, 5907, 6938, 8083, 9348, 10739, 12262, 13923, 15728, 17683, 19794, 22067, 24508
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000534";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_534(n)
    }
}

const fn poly_534(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000534>();
}
