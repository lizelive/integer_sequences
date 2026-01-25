/// a(n) = 3*n^2 + 9
/// https://oeis.org/A000992

pub struct A000992;

impl crate::traits::IntegerSequence for A000992 {
    const NAME: &str = "a(n) = 3*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 12, 21, 36, 57, 84, 117, 156, 201, 252, 309, 372, 441, 516, 597, 684, 777, 876, 981, 1092, 1209, 1332, 1461, 1596, 1737, 1884, 2037, 2196, 2361, 2532
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000992";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_992(n)
    }
}

const fn sq_992(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000992>();
}
