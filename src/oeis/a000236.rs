/// a(n) = n^3 + 1*n^2 + 2*n + 1
/// https://oeis.org/A000236

pub struct A000236;

impl crate::traits::IntegerSequence for A000236 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 17, 43, 89, 161, 265, 407, 593, 829, 1121, 1475, 1897, 2393, 2969, 3631, 4385, 5237, 6193, 7259, 8441, 9745, 11177, 12743, 14449
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000236";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_236(n)
    }
}

const fn poly_236(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000236>();
}
