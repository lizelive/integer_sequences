/// a(n) = 5*n^3 + 2*n^2 + 2*n
/// https://oeis.org/A001046

pub struct A001046;

impl crate::traits::IntegerSequence for A001046 {
    const NAME: &str = "a(n) = 5*n^3 + 2*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 9, 52, 159, 360, 685, 1164, 1827, 2704, 3825, 5220, 6919, 8952, 11349, 14140, 17355, 21024, 25177, 29844, 35055, 40840, 47229, 54252, 61939, 70320, 79425, 89284, 99927, 111384, 123685
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001046";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1046(n)
    }
}

const fn cubic_1046(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 2 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001046>();
}
