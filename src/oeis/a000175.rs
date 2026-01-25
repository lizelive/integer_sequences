/// a(n) = n^2 + 5*n + 2
/// https://oeis.org/A000175

pub struct A000175;

impl crate::traits::IntegerSequence for A000175 {
    const NAME: &str = "a(n) = n^2 + 5*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 8, 16, 26, 38, 52, 68, 86, 106, 128, 152, 178, 206, 236, 268, 302, 338, 376, 416, 458, 502, 548, 596, 646, 698
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000175";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_175(n)
    }
}

const fn poly_175(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 5 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000175>();
}
