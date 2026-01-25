/// a(n) = n^3 + 4*n + 8
/// https://oeis.org/A000584

pub struct A000584;

impl crate::traits::IntegerSequence for A000584 {
    const NAME: &str = "a(n) = n^3 + 4*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 13, 24, 47, 88, 153, 248, 379, 552, 773, 1048, 1383, 1784, 2257, 2808, 3443, 4168, 4989, 5912, 6943, 8088, 9353, 10744, 12267, 13928, 15733, 17688, 19799, 22072, 24513
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000584";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_584(n)
    }
}

const fn poly_584(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000584>();
}
