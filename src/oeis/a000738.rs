/// a(n) = 2*n^2 + 4*n + 2
/// https://oeis.org/A000738

pub struct A000738;

impl crate::traits::IntegerSequence for A000738 {
    const NAME: &str = "a(n) = 2*n^2 + 4*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 8, 18, 32, 50, 72, 98, 128, 162, 200, 242, 288, 338, 392, 450, 512, 578, 648, 722, 800, 882, 968, 1058, 1152, 1250, 1352, 1458, 1568, 1682, 1800
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000738";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_738(n)
    }
}

const fn quad_738(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 4 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000738>();
}
