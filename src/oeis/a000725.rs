/// a(n) = 2*n^2 + 1*n + 0
/// https://oeis.org/A000725

pub struct A000725;

impl crate::traits::IntegerSequence for A000725 {
    const NAME: &str = "a(n) = 2*n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 3, 10, 21, 36, 55, 78, 105, 136, 171, 210, 253, 300, 351, 406, 465, 528, 595, 666, 741, 820, 903, 990, 1081, 1176, 1275, 1378, 1485, 1596, 1711
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000725";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_725(n)
    }
}

const fn quad_725(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000725>();
}
