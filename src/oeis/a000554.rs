/// a(n) = n^3 + 4*n + 5
/// https://oeis.org/A000554

pub struct A000554;

impl crate::traits::IntegerSequence for A000554 {
    const NAME: &str = "a(n) = n^3 + 4*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 10, 21, 44, 85, 150, 245, 376, 549, 770, 1045, 1380, 1781, 2254, 2805, 3440, 4165, 4986, 5909, 6940, 8085, 9350, 10741, 12264, 13925, 15730, 17685, 19796, 22069, 24510
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000554";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_554(n)
    }
}

const fn poly_554(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000554>();
}
