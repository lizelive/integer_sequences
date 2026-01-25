/// a(n) = n^3 + 0*n + 7
/// https://oeis.org/A000570

pub struct A000570;

impl crate::traits::IntegerSequence for A000570 {
    const NAME: &str = "a(n) = n^3 + 0*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 8, 15, 34, 71, 132, 223, 350, 519, 736, 1007, 1338, 1735, 2204, 2751, 3382, 4103, 4920, 5839, 6866, 8007, 9268, 10655, 12174, 13831, 15632, 17583, 19690, 21959, 24396
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000570";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_570(n)
    }
}

const fn poly_570(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000570>();
}
