/// a(n) = n^3 + 4*n^2 + 2*n + 0
/// https://oeis.org/A000214

pub struct A000214;

impl crate::traits::IntegerSequence for A000214 {
    const NAME: &str = "a(n) = n^3 + 4*n^2 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 28, 69, 136, 235, 372, 553, 784, 1071, 1420, 1837, 2328, 2899, 3556, 4305, 5152, 6103, 7164, 8341, 9640, 11067, 12628, 14329, 16176
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000214";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_214(n)
    }
}

const fn poly_214(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000214>();
}
