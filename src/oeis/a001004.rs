/// a(n) = 5*n^3 + 0*n^2 + 1*n
/// https://oeis.org/A001004

pub struct A001004;

impl crate::traits::IntegerSequence for A001004 {
    const NAME: &str = "a(n) = 5*n^3 + 0*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 42, 138, 324, 630, 1086, 1722, 2568, 3654, 5010, 6666, 8652, 10998, 13734, 16890, 20496, 24582, 29178, 34314, 40020, 46326, 53262, 60858, 69144, 78150, 87906, 98442, 109788, 121974
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001004";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1004(n)
    }
}

const fn cubic_1004(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 0 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001004>();
}
