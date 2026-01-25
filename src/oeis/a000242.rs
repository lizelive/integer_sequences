/// a(n) = n^3 + 2*n^2 + 3*n + 1
/// https://oeis.org/A000242

pub struct A000242;

impl crate::traits::IntegerSequence for A000242 {
    const NAME: &str = "a(n) = n^3 + 2*n^2 + 3*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 23, 55, 109, 191, 307, 463, 665, 919, 1231, 1607, 2053, 2575, 3179, 3871, 4657, 5543, 6535, 7639, 8861, 10207, 11683, 13295, 15049
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000242";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_242(n)
    }
}

const fn poly_242(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n * n + 3 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000242>();
}
