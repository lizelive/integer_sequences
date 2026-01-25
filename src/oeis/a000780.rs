/// a(n) = 4*n^2 + 1*n + 1
/// https://oeis.org/A000780

pub struct A000780;

impl crate::traits::IntegerSequence for A000780 {
    const NAME: &str = "a(n) = 4*n^2 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 19, 40, 69, 106, 151, 204, 265, 334, 411, 496, 589, 690, 799, 916, 1041, 1174, 1315, 1464, 1621, 1786, 1959, 2140, 2329, 2526, 2731, 2944, 3165, 3394
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000780";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_780(n)
    }
}

const fn quad_780(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000780>();
}
