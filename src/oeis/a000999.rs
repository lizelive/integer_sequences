/// a(n) = 10*n^2 + 9
/// https://oeis.org/A000999

pub struct A000999;

impl crate::traits::IntegerSequence for A000999 {
    const NAME: &str = "a(n) = 10*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 19, 49, 99, 169, 259, 369, 499, 649, 819, 1009, 1219, 1449, 1699, 1969, 2259, 2569, 2899, 3249, 3619, 4009, 4419, 4849, 5299, 5769, 6259, 6769, 7299, 7849, 8419
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000999";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_999(n)
    }
}

const fn sq_999(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000999>();
}
