/// a(n) = 1*n^2 + 5
/// https://oeis.org/A000950

pub struct A000950;

impl crate::traits::IntegerSequence for A000950 {
    const NAME: &str = "a(n) = 1*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 6, 9, 14, 21, 30, 41, 54, 69, 86, 105, 126, 149, 174, 201, 230, 261, 294, 329, 366, 405, 446, 489, 534, 581, 630, 681, 734, 789, 846
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000950";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_950(n)
    }
}

const fn sq_950(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000950>();
}
