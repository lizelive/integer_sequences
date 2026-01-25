/// a(n) = n^2 + 0*n + 1
/// https://oeis.org/A000160

pub struct A000160;

impl crate::traits::IntegerSequence for A000160 {
    const NAME: &str = "a(n) = n^2 + 0*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 10, 17, 26, 37, 50, 65, 82, 101, 122, 145, 170, 197, 226, 257, 290, 325, 362, 401, 442, 485, 530, 577
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000160";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_160(n)
    }
}

const fn poly_160(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 0 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000160>();
}
