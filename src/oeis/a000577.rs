/// a(n) = n^3 + 7*n + 7
/// https://oeis.org/A000577

pub struct A000577;

impl crate::traits::IntegerSequence for A000577 {
    const NAME: &str = "a(n) = n^3 + 7*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 15, 29, 55, 99, 167, 265, 399, 575, 799, 1077, 1415, 1819, 2295, 2849, 3487, 4215, 5039, 5965, 6999, 8147, 9415, 10809, 12335, 13999, 15807, 17765, 19879, 22155, 24599
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000577";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_577(n)
    }
}

const fn poly_577(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000577>();
}
