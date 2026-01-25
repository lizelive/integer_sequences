/// a(n) = n^2 + 3*n + 1
/// https://oeis.org/A000163

pub struct A000163;

impl crate::traits::IntegerSequence for A000163 {
    const NAME: &str = "a(n) = n^2 + 3*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 11, 19, 29, 41, 55, 71, 89, 109, 131, 155, 181, 209, 239, 271, 305, 341, 379, 419, 461, 505, 551, 599, 649
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000163";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_163(n)
    }
}

const fn poly_163(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 3 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000163>();
}
