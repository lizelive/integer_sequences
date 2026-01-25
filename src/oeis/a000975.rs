/// a(n) = 6*n^2 + 7
/// https://oeis.org/A000975

pub struct A000975;

impl crate::traits::IntegerSequence for A000975 {
    const NAME: &str = "a(n) = 6*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 13, 31, 61, 103, 157, 223, 301, 391, 493, 607, 733, 871, 1021, 1183, 1357, 1543, 1741, 1951, 2173, 2407, 2653, 2911, 3181, 3463, 3757, 4063, 4381, 4711, 5053
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000975";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_975(n)
    }
}

const fn sq_975(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000975>();
}
