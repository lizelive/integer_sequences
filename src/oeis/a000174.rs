/// a(n) = n^2 + 4*n + 2
/// https://oeis.org/A000174

pub struct A000174;

impl crate::traits::IntegerSequence for A000174 {
    const NAME: &str = "a(n) = n^2 + 4*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 7, 14, 23, 34, 47, 62, 79, 98, 119, 142, 167, 194, 223, 254, 287, 322, 359, 398, 439, 482, 527, 574, 623, 674
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000174";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_174(n)
    }
}

const fn poly_174(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 4 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000174>();
}
