/// a(n) = 9*n^2 + 4
/// https://oeis.org/A000948

pub struct A000948;

impl crate::traits::IntegerSequence for A000948 {
    const NAME: &str = "a(n) = 9*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 13, 40, 85, 148, 229, 328, 445, 580, 733, 904, 1093, 1300, 1525, 1768, 2029, 2308, 2605, 2920, 3253, 3604, 3973, 4360, 4765, 5188, 5629, 6088, 6565, 7060, 7573
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000948";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_948(n)
    }
}

const fn sq_948(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000948>();
}
