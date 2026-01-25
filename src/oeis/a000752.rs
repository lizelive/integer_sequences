/// a(n) = 3*n^2 + 3*n + 0
/// https://oeis.org/A000752

pub struct A000752;

impl crate::traits::IntegerSequence for A000752 {
    const NAME: &str = "a(n) = 3*n^2 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 6, 18, 36, 60, 90, 126, 168, 216, 270, 330, 396, 468, 546, 630, 720, 816, 918, 1026, 1140, 1260, 1386, 1518, 1656, 1800, 1950, 2106, 2268, 2436, 2610
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000752";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_752(n)
    }
}

const fn quad_752(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000752>();
}
