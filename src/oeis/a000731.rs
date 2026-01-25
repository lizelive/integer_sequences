/// a(n) = 2*n^2 + 2*n + 1
/// https://oeis.org/A000731

pub struct A000731;

impl crate::traits::IntegerSequence for A000731 {
    const NAME: &str = "a(n) = 2*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 13, 25, 41, 61, 85, 113, 145, 181, 221, 265, 313, 365, 421, 481, 545, 613, 685, 761, 841, 925, 1013, 1105, 1201, 1301, 1405, 1513, 1625, 1741
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000731";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_731(n)
    }
}

const fn quad_731(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000731>();
}
