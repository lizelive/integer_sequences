/// a(n) = 6*n^2 + 3
/// https://oeis.org/A000935

pub struct A000935;

impl crate::traits::IntegerSequence for A000935 {
    const NAME: &str = "a(n) = 6*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 9, 27, 57, 99, 153, 219, 297, 387, 489, 603, 729, 867, 1017, 1179, 1353, 1539, 1737, 1947, 2169, 2403, 2649, 2907, 3177, 3459, 3753, 4059, 4377, 4707, 5049
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000935";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_935(n)
    }
}

const fn sq_935(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000935>();
}
