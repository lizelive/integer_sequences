/// a(n) = n^3 + 5*n + 1
/// https://oeis.org/A000515

pub struct A000515;

impl crate::traits::IntegerSequence for A000515 {
    const NAME: &str = "a(n) = n^3 + 5*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 19, 43, 85, 151, 247, 379, 553, 775, 1051, 1387, 1789, 2263, 2815, 3451, 4177, 4999, 5923, 6955, 8101, 9367, 10759, 12283, 13945, 15751, 17707, 19819, 22093, 24535
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000515";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_515(n)
    }
}

const fn poly_515(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000515>();
}
