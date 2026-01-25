/// a(n) = 3*n^2 + 1
/// https://oeis.org/A000912

pub struct A000912;

impl crate::traits::IntegerSequence for A000912 {
    const NAME: &str = "a(n) = 3*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 13, 28, 49, 76, 109, 148, 193, 244, 301, 364, 433, 508, 589, 676, 769, 868, 973, 1084, 1201, 1324, 1453, 1588, 1729, 1876, 2029, 2188, 2353, 2524
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000912";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_912(n)
    }
}

const fn sq_912(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000912>();
}
