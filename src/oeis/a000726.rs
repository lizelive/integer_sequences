/// a(n) = 2*n^2 + 2*n + 0
/// https://oeis.org/A000726

pub struct A000726;

impl crate::traits::IntegerSequence for A000726 {
    const NAME: &str = "a(n) = 2*n^2 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 12, 24, 40, 60, 84, 112, 144, 180, 220, 264, 312, 364, 420, 480, 544, 612, 684, 760, 840, 924, 1012, 1104, 1200, 1300, 1404, 1512, 1624, 1740
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000726";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_726(n)
    }
}

const fn quad_726(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000726>();
}
