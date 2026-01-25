/// a(n) = n^3 + 0*n + 6
/// https://oeis.org/A000560

pub struct A000560;

impl crate::traits::IntegerSequence for A000560 {
    const NAME: &str = "a(n) = n^3 + 0*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 7, 14, 33, 70, 131, 222, 349, 518, 735, 1006, 1337, 1734, 2203, 2750, 3381, 4102, 4919, 5838, 6865, 8006, 9267, 10654, 12173, 13830, 15631, 17582, 19689, 21958, 24395
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000560";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_560(n)
    }
}

const fn poly_560(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000560>();
}
