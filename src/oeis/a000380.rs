/// a(n) = 4*T(n)
/// https://oeis.org/A000380

pub struct A000380;

impl crate::traits::IntegerSequence for A000380 {
    const NAME: &str = "a(n) = 4*T(n)";

    const HEAD: &[crate::Value] = &[
        0, 4, 12, 24, 40, 60, 84, 112, 144, 180, 220, 264, 312, 364, 420, 480, 544, 612, 684, 760, 840, 924, 1012, 1104, 1200
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000380";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_380(n)
    }
}

const fn tri_380(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000380>();
}
