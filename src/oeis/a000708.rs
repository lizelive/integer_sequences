/// a(n) = 1*n^2 + 4*n + 1
/// https://oeis.org/A000708

pub struct A000708;

impl crate::traits::IntegerSequence for A000708 {
    const NAME: &str = "a(n) = 1*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 13, 22, 33, 46, 61, 78, 97, 118, 141, 166, 193, 222, 253, 286, 321, 358, 397, 438, 481, 526, 573, 622, 673, 726, 781, 838, 897, 958
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000708";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_708(n)
    }
}

const fn quad_708(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000708>();
}
