/// a(n) = n^3 + 0*n^2 + 2*n + 1
/// https://oeis.org/A000235

pub struct A000235;

impl crate::traits::IntegerSequence for A000235 {
    const NAME: &str = "a(n) = n^3 + 0*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 13, 34, 73, 136, 229, 358, 529, 748, 1021, 1354, 1753, 2224, 2773, 3406, 4129, 4948, 5869, 6898, 8041, 9304, 10693, 12214, 13873
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000235";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_235(n)
    }
}

const fn poly_235(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000235>();
}
