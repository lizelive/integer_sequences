/// a(n) = n^2 + 7*n + 1
/// https://oeis.org/A000167

pub struct A000167;

impl crate::traits::IntegerSequence for A000167 {
    const NAME: &str = "a(n) = n^2 + 7*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 19, 31, 45, 61, 79, 99, 121, 145, 171, 199, 229, 261, 295, 331, 369, 409, 451, 495, 541, 589, 639, 691, 745
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000167";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_167(n)
    }
}

const fn poly_167(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 7 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000167>();
}
