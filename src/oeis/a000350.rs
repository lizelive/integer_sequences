/// a(n) = 1*T(n)
/// https://oeis.org/A000350

pub struct A000350;

impl crate::traits::IntegerSequence for A000350 {
    const NAME: &str = "a(n) = 1*T(n)";

    const HEAD: &[crate::Value] = &[
        0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231, 253, 276, 300
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000350";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_350(n)
    }
}

const fn tri_350(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000350>();
}
