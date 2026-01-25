/// a(n) = 2*n^2 + 2
/// https://oeis.org/A000921

pub struct A000921;

impl crate::traits::IntegerSequence for A000921 {
    const NAME: &str = "a(n) = 2*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 4, 10, 20, 34, 52, 74, 100, 130, 164, 202, 244, 290, 340, 394, 452, 514, 580, 650, 724, 802, 884, 970, 1060, 1154, 1252, 1354, 1460, 1570, 1684
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000921";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_921(n)
    }
}

const fn sq_921(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000921>();
}
