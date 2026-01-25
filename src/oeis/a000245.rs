/// a(n) = n^3 + 0*n^2 + 4*n + 1
/// https://oeis.org/A000245

pub struct A000245;

impl crate::traits::IntegerSequence for A000245 {
    const NAME: &str = "a(n) = n^3 + 0*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 17, 40, 81, 146, 241, 372, 545, 766, 1041, 1376, 1777, 2250, 2801, 3436, 4161, 4982, 5905, 6936, 8081, 9346, 10737, 12260, 13921
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000245";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_245(n)
    }
}

const fn poly_245(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000245>();
}
