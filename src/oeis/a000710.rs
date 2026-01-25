/// a(n) = 1*n^2 + 1*n + 2
/// https://oeis.org/A000710

pub struct A000710;

impl crate::traits::IntegerSequence for A000710 {
    const NAME: &str = "a(n) = 1*n^2 + 1*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 4, 8, 14, 22, 32, 44, 58, 74, 92, 112, 134, 158, 184, 212, 242, 274, 308, 344, 382, 422, 464, 508, 554, 602, 652, 704, 758, 814, 872
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000710";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_710(n)
    }
}

const fn quad_710(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 1 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000710>();
}
