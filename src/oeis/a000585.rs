/// a(n) = n^3 + 5*n + 8
/// https://oeis.org/A000585

pub struct A000585;

impl crate::traits::IntegerSequence for A000585 {
    const NAME: &str = "a(n) = n^3 + 5*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 14, 26, 50, 92, 158, 254, 386, 560, 782, 1058, 1394, 1796, 2270, 2822, 3458, 4184, 5006, 5930, 6962, 8108, 9374, 10766, 12290, 13952, 15758, 17714, 19826, 22100, 24542
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000585";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_585(n)
    }
}

const fn poly_585(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000585>();
}
