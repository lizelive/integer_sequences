/// a(n) = n^3 + 6*n + 2
/// https://oeis.org/A000526

pub struct A000526;

impl crate::traits::IntegerSequence for A000526 {
    const NAME: &str = "a(n) = n^3 + 6*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 9, 22, 47, 90, 157, 254, 387, 562, 785, 1062, 1399, 1802, 2277, 2830, 3467, 4194, 5017, 5942, 6975, 8122, 9389, 10782, 12307, 13970, 15777, 17734, 19847, 22122, 24565
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000526";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_526(n)
    }
}

const fn poly_526(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000526>();
}
