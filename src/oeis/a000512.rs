/// a(n) = n^3 + 2*n + 1
/// https://oeis.org/A000512

pub struct A000512;

impl crate::traits::IntegerSequence for A000512 {
    const NAME: &str = "a(n) = n^3 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 13, 34, 73, 136, 229, 358, 529, 748, 1021, 1354, 1753, 2224, 2773, 3406, 4129, 4948, 5869, 6898, 8041, 9304, 10693, 12214, 13873, 15676, 17629, 19738, 22009, 24448
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000512";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_512(n)
    }
}

const fn poly_512(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000512>();
}
