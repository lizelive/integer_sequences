/// a(n) = 4*n^2 + 4*n + 2
/// https://oeis.org/A000788

pub struct A000788;

impl crate::traits::IntegerSequence for A000788 {
    const NAME: &str = "a(n) = 4*n^2 + 4*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 10, 26, 50, 82, 122, 170, 226, 290, 362, 442, 530, 626, 730, 842, 962, 1090, 1226, 1370, 1522, 1682, 1850, 2026, 2210, 2402, 2602, 2810, 3026, 3250, 3482
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000788";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_788(n)
    }
}

const fn quad_788(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 4 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000788>();
}
