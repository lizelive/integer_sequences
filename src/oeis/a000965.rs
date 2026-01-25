/// a(n) = 6*n^2 + 6
/// https://oeis.org/A000965

pub struct A000965;

impl crate::traits::IntegerSequence for A000965 {
    const NAME: &str = "a(n) = 6*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 12, 30, 60, 102, 156, 222, 300, 390, 492, 606, 732, 870, 1020, 1182, 1356, 1542, 1740, 1950, 2172, 2406, 2652, 2910, 3180, 3462, 3756, 4062, 4380, 4710, 5052
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000965";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_965(n)
    }
}

const fn sq_965(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000965>();
}
