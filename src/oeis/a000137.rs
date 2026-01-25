/// a(n) = C(n+2,3) = n*(n+1)*(n+2)/6
/// https://oeis.org/A000137

pub struct A000137;

impl crate::traits::IntegerSequence for A000137 {
    const NAME: &str = "Tetrahedral numbers";

    const HEAD: &[crate::Value] = &[
        0, 1, 4, 10, 20, 35, 56, 84, 120, 165, 220, 286, 364, 455, 560, 680, 816, 969, 1140, 1330, 1540, 1771, 2024, 2300, 2600, 2925, 3276, 3654, 4060, 4495
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000137";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tetrahedral(n)
    }
}

const fn tetrahedral(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * (n + 1) * (n + 2) / 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000137>();
}
