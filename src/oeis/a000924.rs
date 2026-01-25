/// a(n) = 5*n^2 + 2
/// https://oeis.org/A000924

pub struct A000924;

impl crate::traits::IntegerSequence for A000924 {
    const NAME: &str = "a(n) = 5*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 7, 22, 47, 82, 127, 182, 247, 322, 407, 502, 607, 722, 847, 982, 1127, 1282, 1447, 1622, 1807, 2002, 2207, 2422, 2647, 2882, 3127, 3382, 3647, 3922, 4207
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000924";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_924(n)
    }
}

const fn sq_924(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000924>();
}
