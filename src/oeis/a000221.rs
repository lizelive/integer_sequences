/// a(n) = n^3 + 1*n^2 + 4*n + 0
/// https://oeis.org/A000221

pub struct A000221;

impl crate::traits::IntegerSequence for A000221 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 6, 20, 48, 96, 170, 276, 420, 608, 846, 1140, 1496, 1920, 2418, 2996, 3660, 4416, 5270, 6228, 7296, 8480, 9786, 11220, 12788, 14496
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000221";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_221(n)
    }
}

const fn poly_221(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000221>();
}
