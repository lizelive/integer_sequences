/// a(n) = 4*T(n) + 1
/// https://oeis.org/A000381

pub struct A000381;

impl crate::traits::IntegerSequence for A000381 {
    const NAME: &str = "a(n) = 4*T(n) + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 13, 25, 41, 61, 85, 113, 145, 181, 221, 265, 313, 365, 421, 481, 545, 613, 685, 761, 841, 925, 1013, 1105, 1201
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000381";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_381(n)
    }
}

const fn tri_381(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000381>();
}
