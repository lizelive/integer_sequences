/// a(n) = 3*T(n) + 5
/// https://oeis.org/A000375

pub struct A000375;

impl crate::traits::IntegerSequence for A000375 {
    const NAME: &str = "a(n) = 3*T(n) + 5";

    const HEAD: &[crate::Value] = &[
        5, 8, 14, 23, 35, 50, 68, 89, 113, 140, 170, 203, 239, 278, 320, 365, 413, 464, 518, 575, 635, 698, 764, 833, 905
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000375";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_375(n)
    }
}

const fn tri_375(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000375>();
}
