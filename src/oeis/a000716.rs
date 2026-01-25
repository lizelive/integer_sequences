/// a(n) = 1*n^2 + 2*n + 3
/// https://oeis.org/A000716

pub struct A000716;

impl crate::traits::IntegerSequence for A000716 {
    const NAME: &str = "a(n) = 1*n^2 + 2*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 6, 11, 18, 27, 38, 51, 66, 83, 102, 123, 146, 171, 198, 227, 258, 291, 326, 363, 402, 443, 486, 531, 578, 627, 678, 731, 786, 843, 902
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000716";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_716(n)
    }
}

const fn quad_716(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 2 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000716>();
}
