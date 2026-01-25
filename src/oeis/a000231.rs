/// a(n) = n^3 + 1*n^2 + 1*n + 1
/// https://oeis.org/A000231

pub struct A000231;

impl crate::traits::IntegerSequence for A000231 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 15, 40, 85, 156, 259, 400, 585, 820, 1111, 1464, 1885, 2380, 2955, 3616, 4369, 5220, 6175, 7240, 8421, 9724, 11155, 12720, 14425
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000231";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_231(n)
    }
}

const fn poly_231(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000231>();
}
