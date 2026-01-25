/// a(n) = 3*n^2 + 3*n + 2
/// https://oeis.org/A000762

pub struct A000762;

impl crate::traits::IntegerSequence for A000762 {
    const NAME: &str = "a(n) = 3*n^2 + 3*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 8, 20, 38, 62, 92, 128, 170, 218, 272, 332, 398, 470, 548, 632, 722, 818, 920, 1028, 1142, 1262, 1388, 1520, 1658, 1802, 1952, 2108, 2270, 2438, 2612
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000762";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_762(n)
    }
}

const fn quad_762(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 3 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000762>();
}
