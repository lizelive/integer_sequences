/// a(n) = 4*T(n) + 5
/// https://oeis.org/A000385

pub struct A000385;

impl crate::traits::IntegerSequence for A000385 {
    const NAME: &str = "a(n) = 4*T(n) + 5";

    const HEAD: &[crate::Value] = &[
        5, 9, 17, 29, 45, 65, 89, 117, 149, 185, 225, 269, 317, 369, 425, 485, 549, 617, 689, 765, 845, 929, 1017, 1109, 1205
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000385";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_385(n)
    }
}

const fn tri_385(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000385>();
}
