/// a(n) = n^3 + 6*n + 5
/// https://oeis.org/A000556

pub struct A000556;

impl crate::traits::IntegerSequence for A000556 {
    const NAME: &str = "a(n) = n^3 + 6*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 12, 25, 50, 93, 160, 257, 390, 565, 788, 1065, 1402, 1805, 2280, 2833, 3470, 4197, 5020, 5945, 6978, 8125, 9392, 10785, 12310, 13973, 15780, 17737, 19850, 22125, 24568
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000556";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_556(n)
    }
}

const fn poly_556(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000556>();
}
