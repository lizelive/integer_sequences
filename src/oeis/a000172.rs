/// a(n) = n^2 + 2*n + 2
/// https://oeis.org/A000172

pub struct A000172;

impl crate::traits::IntegerSequence for A000172 {
    const NAME: &str = "a(n) = n^2 + 2*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 5, 10, 17, 26, 37, 50, 65, 82, 101, 122, 145, 170, 197, 226, 257, 290, 325, 362, 401, 442, 485, 530, 577, 626
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000172";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_172(n)
    }
}

const fn poly_172(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 2 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000172>();
}
