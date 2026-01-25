/// a(n) = n^3 - n^2 + 1 = A000578(n) - A000290(n) + 1
/// https://oeis.org/A000128

pub struct A000128;

impl crate::traits::IntegerSequence for A000128 {
    const NAME: &str = "a(n) = n^3 - n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 1, 5, 19, 49, 101, 181, 295, 449, 649, 901, 1211, 1585, 2029, 2549, 3151, 3841, 4625, 5509, 6499, 7601, 8821, 10165, 11639, 13249, 15001, 16901, 18955, 21169, 23549
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000128";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_formula(n)
    }
}

const fn cubic_formula(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n - n * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000128>();
}
