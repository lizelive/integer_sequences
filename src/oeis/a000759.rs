/// a(n) = 3*n^2 + 5*n + 1
/// https://oeis.org/A000759

pub struct A000759;

impl crate::traits::IntegerSequence for A000759 {
    const NAME: &str = "a(n) = 3*n^2 + 5*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 23, 43, 69, 101, 139, 183, 233, 289, 351, 419, 493, 573, 659, 751, 849, 953, 1063, 1179, 1301, 1429, 1563, 1703, 1849, 2001, 2159, 2323, 2493, 2669
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000759";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_759(n)
    }
}

const fn quad_759(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 5 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000759>();
}
