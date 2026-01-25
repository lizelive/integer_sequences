/// a(n) = 2*n^2 + 4*n + 3
/// https://oeis.org/A000743

pub struct A000743;

impl crate::traits::IntegerSequence for A000743 {
    const NAME: &str = "a(n) = 2*n^2 + 4*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 9, 19, 33, 51, 73, 99, 129, 163, 201, 243, 289, 339, 393, 451, 513, 579, 649, 723, 801, 883, 969, 1059, 1153, 1251, 1353, 1459, 1569, 1683, 1801
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000743";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_743(n)
    }
}

const fn quad_743(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 4 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000743>();
}
