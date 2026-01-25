/// a(n) = 8*n^2 + 7
/// https://oeis.org/A000977

pub struct A000977;

impl crate::traits::IntegerSequence for A000977 {
    const NAME: &str = "a(n) = 8*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 15, 39, 79, 135, 207, 295, 399, 519, 655, 807, 975, 1159, 1359, 1575, 1807, 2055, 2319, 2599, 2895, 3207, 3535, 3879, 4239, 4615, 5007, 5415, 5839, 6279, 6735
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000977";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_977(n)
    }
}

const fn sq_977(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000977>();
}
