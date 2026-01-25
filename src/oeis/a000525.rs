/// a(n) = n^3 + 5*n + 2
/// https://oeis.org/A000525

pub struct A000525;

impl crate::traits::IntegerSequence for A000525 {
    const NAME: &str = "a(n) = n^3 + 5*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 8, 20, 44, 86, 152, 248, 380, 554, 776, 1052, 1388, 1790, 2264, 2816, 3452, 4178, 5000, 5924, 6956, 8102, 9368, 10760, 12284, 13946, 15752, 17708, 19820, 22094, 24536
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000525";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_525(n)
    }
}

const fn poly_525(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000525>();
}
