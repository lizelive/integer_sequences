/// a(n) = n^3 + 3*n + 5
/// https://oeis.org/A000553

pub struct A000553;

impl crate::traits::IntegerSequence for A000553 {
    const NAME: &str = "a(n) = n^3 + 3*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 9, 19, 41, 81, 145, 239, 369, 541, 761, 1035, 1369, 1769, 2241, 2791, 3425, 4149, 4969, 5891, 6921, 8065, 9329, 10719, 12241, 13901, 15705, 17659, 19769, 22041, 24481
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000553";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_553(n)
    }
}

const fn poly_553(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000553>();
}
