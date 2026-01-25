/// a(n) = n^3 + 7*n + 1
/// https://oeis.org/A000517

pub struct A000517;

impl crate::traits::IntegerSequence for A000517 {
    const NAME: &str = "a(n) = n^3 + 7*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 23, 49, 93, 161, 259, 393, 569, 793, 1071, 1409, 1813, 2289, 2843, 3481, 4209, 5033, 5959, 6993, 8141, 9409, 10803, 12329, 13993, 15801, 17759, 19873, 22149, 24593
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000517";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_517(n)
    }
}

const fn poly_517(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000517>();
}
