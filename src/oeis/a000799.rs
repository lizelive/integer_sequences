/// a(n) = 4*n^2 + 5*n + 4
/// https://oeis.org/A000799

pub struct A000799;

impl crate::traits::IntegerSequence for A000799 {
    const NAME: &str = "a(n) = 4*n^2 + 5*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 13, 30, 55, 88, 129, 178, 235, 300, 373, 454, 543, 640, 745, 858, 979, 1108, 1245, 1390, 1543, 1704, 1873, 2050, 2235, 2428, 2629, 2838, 3055, 3280, 3513
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000799";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_799(n)
    }
}

const fn quad_799(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 5 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000799>();
}
