/// a(n) = 1*n^2 + 5*n + 0
/// https://oeis.org/A000704

pub struct A000704;

impl crate::traits::IntegerSequence for A000704 {
    const NAME: &str = "a(n) = 1*n^2 + 5*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 6, 14, 24, 36, 50, 66, 84, 104, 126, 150, 176, 204, 234, 266, 300, 336, 374, 414, 456, 500, 546, 594, 644, 696, 750, 806, 864, 924, 986
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000704";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_704(n)
    }
}

const fn quad_704(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 5 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000704>();
}
