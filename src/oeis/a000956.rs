/// a(n) = 7*n^2 + 5
/// https://oeis.org/A000956

pub struct A000956;

impl crate::traits::IntegerSequence for A000956 {
    const NAME: &str = "a(n) = 7*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 12, 33, 68, 117, 180, 257, 348, 453, 572, 705, 852, 1013, 1188, 1377, 1580, 1797, 2028, 2273, 2532, 2805, 3092, 3393, 3708, 4037, 4380, 4737, 5108, 5493, 5892
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000956";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_956(n)
    }
}

const fn sq_956(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000956>();
}
