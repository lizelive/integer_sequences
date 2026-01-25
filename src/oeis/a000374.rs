/// a(n) = 3*T(n) + 4
/// https://oeis.org/A000374

pub struct A000374;

impl crate::traits::IntegerSequence for A000374 {
    const NAME: &str = "a(n) = 3*T(n) + 4";

    const HEAD: &[crate::Value] = &[
        4, 7, 13, 22, 34, 49, 67, 88, 112, 139, 169, 202, 238, 277, 319, 364, 412, 463, 517, 574, 634, 697, 763, 832, 904
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000374";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_374(n)
    }
}

const fn tri_374(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000374>();
}
