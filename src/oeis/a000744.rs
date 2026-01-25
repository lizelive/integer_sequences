/// a(n) = 2*n^2 + 5*n + 3
/// https://oeis.org/A000744

pub struct A000744;

impl crate::traits::IntegerSequence for A000744 {
    const NAME: &str = "a(n) = 2*n^2 + 5*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 10, 21, 36, 55, 78, 105, 136, 171, 210, 253, 300, 351, 406, 465, 528, 595, 666, 741, 820, 903, 990, 1081, 1176, 1275, 1378, 1485, 1596, 1711, 1830
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000744";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_744(n)
    }
}

const fn quad_744(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 5 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000744>();
}
