/// a(n) = 2*n^2 + 5*n + 2
/// https://oeis.org/A000739

pub struct A000739;

impl crate::traits::IntegerSequence for A000739 {
    const NAME: &str = "a(n) = 2*n^2 + 5*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 9, 20, 35, 54, 77, 104, 135, 170, 209, 252, 299, 350, 405, 464, 527, 594, 665, 740, 819, 902, 989, 1080, 1175, 1274, 1377, 1484, 1595, 1710, 1829
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000739";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_739(n)
    }
}

const fn quad_739(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 5 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000739>();
}
