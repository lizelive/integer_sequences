/// a(n) = 5*T(n)
/// https://oeis.org/A000390

pub struct A000390;

impl crate::traits::IntegerSequence for A000390 {
    const NAME: &str = "a(n) = 5*T(n)";

    const HEAD: &[crate::Value] = &[
        0, 5, 15, 30, 50, 75, 105, 140, 180, 225, 275, 330, 390, 455, 525, 600, 680, 765, 855, 950, 1050, 1155, 1265, 1380, 1500
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000390";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_390(n)
    }
}

const fn tri_390(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000390>();
}
