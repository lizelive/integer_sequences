/// a(n) = n^3 + 4*n^2 + 4*n + 1
/// https://oeis.org/A000249

pub struct A000249;

impl crate::traits::IntegerSequence for A000249 {
    const NAME: &str = "a(n) = n^3 + 4*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 33, 76, 145, 246, 385, 568, 801, 1090, 1441, 1860, 2353, 2926, 3585, 4336, 5185, 6138, 7201, 8380, 9681, 11110, 12673, 14376, 16225
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000249";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_249(n)
    }
}

const fn poly_249(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000249>();
}
