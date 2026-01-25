/// a(n) = 5*n^2 + 0
/// https://oeis.org/A000904

pub struct A000904;

impl crate::traits::IntegerSequence for A000904 {
    const NAME: &str = "a(n) = 5*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 20, 45, 80, 125, 180, 245, 320, 405, 500, 605, 720, 845, 980, 1125, 1280, 1445, 1620, 1805, 2000, 2205, 2420, 2645, 2880, 3125, 3380, 3645, 3920, 4205
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000904";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_904(n)
    }
}

const fn sq_904(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000904>();
}
