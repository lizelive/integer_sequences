/// a(n) = n^2 + 2*n + 0
/// https://oeis.org/A000152

pub struct A000152;

impl crate::traits::IntegerSequence for A000152 {
    const NAME: &str = "a(n) = n^2 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 3, 8, 15, 24, 35, 48, 63, 80, 99, 120, 143, 168, 195, 224, 255, 288, 323, 360, 399, 440, 483, 528, 575, 624
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000152";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_152(n)
    }
}

const fn poly_152(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000152>();
}
