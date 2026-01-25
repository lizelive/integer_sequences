/// a(n) = 2*T(n) + 6
/// https://oeis.org/A000366

pub struct A000366;

impl crate::traits::IntegerSequence for A000366 {
    const NAME: &str = "a(n) = 2*T(n) + 6";

    const HEAD: &[crate::Value] = &[
        6, 8, 12, 18, 26, 36, 48, 62, 78, 96, 116, 138, 162, 188, 216, 246, 278, 312, 348, 386, 426, 468, 512, 558, 606
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000366";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_366(n)
    }
}

const fn tri_366(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000366>();
}
