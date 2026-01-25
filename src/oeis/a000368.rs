/// a(n) = 2*T(n) + 8
/// https://oeis.org/A000368

pub struct A000368;

impl crate::traits::IntegerSequence for A000368 {
    const NAME: &str = "a(n) = 2*T(n) + 8";

    const HEAD: &[crate::Value] = &[
        8, 10, 14, 20, 28, 38, 50, 64, 80, 98, 118, 140, 164, 190, 218, 248, 280, 314, 350, 388, 428, 470, 514, 560, 608
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000368";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_368(n)
    }
}

const fn tri_368(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000368>();
}
