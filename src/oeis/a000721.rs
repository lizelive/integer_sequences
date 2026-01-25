/// a(n) = 1*n^2 + 2*n + 4
/// https://oeis.org/A000721

pub struct A000721;

impl crate::traits::IntegerSequence for A000721 {
    const NAME: &str = "a(n) = 1*n^2 + 2*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 7, 12, 19, 28, 39, 52, 67, 84, 103, 124, 147, 172, 199, 228, 259, 292, 327, 364, 403, 444, 487, 532, 579, 628, 679, 732, 787, 844, 903
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000721";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_721(n)
    }
}

const fn quad_721(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 2 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000721>();
}
