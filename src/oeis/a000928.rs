/// a(n) = 9*n^2 + 2
/// https://oeis.org/A000928

pub struct A000928;

impl crate::traits::IntegerSequence for A000928 {
    const NAME: &str = "a(n) = 9*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 11, 38, 83, 146, 227, 326, 443, 578, 731, 902, 1091, 1298, 1523, 1766, 2027, 2306, 2603, 2918, 3251, 3602, 3971, 4358, 4763, 5186, 5627, 6086, 6563, 7058, 7571
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000928";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_928(n)
    }
}

const fn sq_928(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000928>();
}
