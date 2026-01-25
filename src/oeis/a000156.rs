/// a(n) = n^2 + 6*n + 0
/// https://oeis.org/A000156

pub struct A000156;

impl crate::traits::IntegerSequence for A000156 {
    const NAME: &str = "a(n) = n^2 + 6*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 16, 27, 40, 55, 72, 91, 112, 135, 160, 187, 216, 247, 280, 315, 352, 391, 432, 475, 520, 567, 616, 667, 720
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000156";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_156(n)
    }
}

const fn poly_156(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 6 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000156>();
}
