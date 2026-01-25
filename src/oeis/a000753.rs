/// a(n) = 3*n^2 + 4*n + 0
/// https://oeis.org/A000753

pub struct A000753;

impl crate::traits::IntegerSequence for A000753 {
    const NAME: &str = "a(n) = 3*n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 20, 39, 64, 95, 132, 175, 224, 279, 340, 407, 480, 559, 644, 735, 832, 935, 1044, 1159, 1280, 1407, 1540, 1679, 1824, 1975, 2132, 2295, 2464, 2639
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000753";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_753(n)
    }
}

const fn quad_753(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000753>();
}
