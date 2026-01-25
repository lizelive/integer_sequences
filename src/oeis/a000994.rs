/// a(n) = 5*n^2 + 9
/// https://oeis.org/A000994

pub struct A000994;

impl crate::traits::IntegerSequence for A000994 {
    const NAME: &str = "a(n) = 5*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 14, 29, 54, 89, 134, 189, 254, 329, 414, 509, 614, 729, 854, 989, 1134, 1289, 1454, 1629, 1814, 2009, 2214, 2429, 2654, 2889, 3134, 3389, 3654, 3929, 4214
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000994";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_994(n)
    }
}

const fn sq_994(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000994>();
}
