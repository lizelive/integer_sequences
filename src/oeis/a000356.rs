/// a(n) = 1*T(n) + 6
/// https://oeis.org/A000356

pub struct A000356;

impl crate::traits::IntegerSequence for A000356 {
    const NAME: &str = "a(n) = 1*T(n) + 6";

    const HEAD: &[crate::Value] = &[
        6, 7, 9, 12, 16, 21, 27, 34, 42, 51, 61, 72, 84, 97, 111, 126, 142, 159, 177, 196, 216, 237, 259, 282, 306
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000356";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_356(n)
    }
}

const fn tri_356(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000356>();
}
