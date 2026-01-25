/// a(n) = 1*n^3 + 4*n^2 + 2*n
/// https://oeis.org/A001054

pub struct A001054;

impl crate::traits::IntegerSequence for A001054 {
    const NAME: &str = "a(n) = 1*n^3 + 4*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 28, 69, 136, 235, 372, 553, 784, 1071, 1420, 1837, 2328, 2899, 3556, 4305, 5152, 6103, 7164, 8341, 9640, 11067, 12628, 14329, 16176, 18175, 20332, 22653, 25144, 27811
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001054";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1054(n)
    }
}

const fn cubic_1054(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 4 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001054>();
}
