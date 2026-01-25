/// a(n) = 2*n^2 + 1
/// https://oeis.org/A000911

pub struct A000911;

impl crate::traits::IntegerSequence for A000911 {
    const NAME: &str = "a(n) = 2*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 9, 19, 33, 51, 73, 99, 129, 163, 201, 243, 289, 339, 393, 451, 513, 579, 649, 723, 801, 883, 969, 1059, 1153, 1251, 1353, 1459, 1569, 1683
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000911";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_911(n)
    }
}

const fn sq_911(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000911>();
}
