/// a(n) = 4*n^2 + 5*n + 0
/// https://oeis.org/A000779

pub struct A000779;

impl crate::traits::IntegerSequence for A000779 {
    const NAME: &str = "a(n) = 4*n^2 + 5*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 9, 26, 51, 84, 125, 174, 231, 296, 369, 450, 539, 636, 741, 854, 975, 1104, 1241, 1386, 1539, 1700, 1869, 2046, 2231, 2424, 2625, 2834, 3051, 3276, 3509
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000779";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_779(n)
    }
}

const fn quad_779(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 5 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000779>();
}
