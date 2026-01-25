/// a(n) = n^2 + 8*n + 2
/// https://oeis.org/A000178

pub struct A000178;

impl crate::traits::IntegerSequence for A000178 {
    const NAME: &str = "a(n) = n^2 + 8*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 11, 22, 35, 50, 67, 86, 107, 130, 155, 182, 211, 242, 275, 310, 347, 386, 427, 470, 515, 562, 611, 662, 715, 770
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000178";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_178(n)
    }
}

const fn poly_178(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 8 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000178>();
}
