/// a(n) = n^2 + 8*n + 3
/// https://oeis.org/A000188

pub struct A000188;

impl crate::traits::IntegerSequence for A000188 {
    const NAME: &str = "a(n) = n^2 + 8*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 12, 23, 36, 51, 68, 87, 108, 131, 156, 183, 212, 243, 276, 311, 348, 387, 428, 471, 516, 563, 612, 663, 716, 771
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000188";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_188(n)
    }
}

const fn poly_188(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 8 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000188>();
}
