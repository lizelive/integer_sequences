/// a(n) = n^3 + 1*n + 2
/// https://oeis.org/A000521

pub struct A000521;

impl crate::traits::IntegerSequence for A000521 {
    const NAME: &str = "a(n) = n^3 + 1*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 4, 12, 32, 70, 132, 224, 352, 522, 740, 1012, 1344, 1742, 2212, 2760, 3392, 4114, 4932, 5852, 6880, 8022, 9284, 10672, 12192, 13850, 15652, 17604, 19712, 21982, 24420
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000521";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_521(n)
    }
}

const fn poly_521(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000521>();
}
