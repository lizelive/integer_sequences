/// a(n) = n^3 + 1*n^2 + 3*n + 0
/// https://oeis.org/A000216

pub struct A000216;

impl crate::traits::IntegerSequence for A000216 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 18, 45, 92, 165, 270, 413, 600, 837, 1130, 1485, 1908, 2405, 2982, 3645, 4400, 5253, 6210, 7277, 8460, 9765, 11198, 12765, 14472
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000216";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_216(n)
    }
}

const fn poly_216(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000216>();
}
