/// a(n) = 2*n^2 + 0
/// https://oeis.org/A000901

pub struct A000901;

impl crate::traits::IntegerSequence for A000901 {
    const NAME: &str = "a(n) = 2*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 2, 8, 18, 32, 50, 72, 98, 128, 162, 200, 242, 288, 338, 392, 450, 512, 578, 648, 722, 800, 882, 968, 1058, 1152, 1250, 1352, 1458, 1568, 1682
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000901";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_901(n)
    }
}

const fn sq_901(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000901>();
}
