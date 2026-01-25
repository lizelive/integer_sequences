/// a(n) = 5*n^2 + 3
/// https://oeis.org/A000934

pub struct A000934;

impl crate::traits::IntegerSequence for A000934 {
    const NAME: &str = "a(n) = 5*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 8, 23, 48, 83, 128, 183, 248, 323, 408, 503, 608, 723, 848, 983, 1128, 1283, 1448, 1623, 1808, 2003, 2208, 2423, 2648, 2883, 3128, 3383, 3648, 3923, 4208
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000934";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_934(n)
    }
}

const fn sq_934(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000934>();
}
