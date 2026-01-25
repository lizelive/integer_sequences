/// a(n) = 4*n^2 + 1*n + 2
/// https://oeis.org/A000785

pub struct A000785;

impl crate::traits::IntegerSequence for A000785 {
    const NAME: &str = "a(n) = 4*n^2 + 1*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 7, 20, 41, 70, 107, 152, 205, 266, 335, 412, 497, 590, 691, 800, 917, 1042, 1175, 1316, 1465, 1622, 1787, 1960, 2141, 2330, 2527, 2732, 2945, 3166, 3395
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000785";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_785(n)
    }
}

const fn quad_785(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 1 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000785>();
}
