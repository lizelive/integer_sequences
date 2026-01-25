/// a(n) = 3*n^2 + 5*n + 2
/// https://oeis.org/A000764

pub struct A000764;

impl crate::traits::IntegerSequence for A000764 {
    const NAME: &str = "a(n) = 3*n^2 + 5*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 10, 24, 44, 70, 102, 140, 184, 234, 290, 352, 420, 494, 574, 660, 752, 850, 954, 1064, 1180, 1302, 1430, 1564, 1704, 1850, 2002, 2160, 2324, 2494, 2670
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000764";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_764(n)
    }
}

const fn quad_764(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 5 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000764>();
}
