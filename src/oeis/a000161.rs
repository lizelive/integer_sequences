/// a(n) = n^2 + 1*n + 1
/// https://oeis.org/A000161

pub struct A000161;

impl crate::traits::IntegerSequence for A000161 {
    const NAME: &str = "a(n) = n^2 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 7, 13, 21, 31, 43, 57, 73, 91, 111, 133, 157, 183, 211, 241, 273, 307, 343, 381, 421, 463, 507, 553, 601
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000161";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_161(n)
    }
}

const fn poly_161(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000161>();
}
