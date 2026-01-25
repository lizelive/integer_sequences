/// a(n) = 4*T(n) + 9
/// https://oeis.org/A000389

pub struct A000389;

impl crate::traits::IntegerSequence for A000389 {
    const NAME: &str = "a(n) = 4*T(n) + 9";

    const HEAD: &[crate::Value] = &[
        9, 13, 21, 33, 49, 69, 93, 121, 153, 189, 229, 273, 321, 373, 429, 489, 553, 621, 693, 769, 849, 933, 1021, 1113, 1209
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000389";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_389(n)
    }
}

const fn tri_389(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000389>();
}
