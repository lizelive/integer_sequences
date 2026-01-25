/// a(n) = 4*n^2 + 5
/// https://oeis.org/A000953

pub struct A000953;

impl crate::traits::IntegerSequence for A000953 {
    const NAME: &str = "a(n) = 4*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 9, 21, 41, 69, 105, 149, 201, 261, 329, 405, 489, 581, 681, 789, 905, 1029, 1161, 1301, 1449, 1605, 1769, 1941, 2121, 2309, 2505, 2709, 2921, 3141, 3369
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000953";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_953(n)
    }
}

const fn sq_953(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000953>();
}
