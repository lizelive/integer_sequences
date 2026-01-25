/// a(n) = n^2 + 6*n + 4
/// https://oeis.org/A000196

pub struct A000196;

impl crate::traits::IntegerSequence for A000196 {
    const NAME: &str = "a(n) = n^2 + 6*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 11, 20, 31, 44, 59, 76, 95, 116, 139, 164, 191, 220, 251, 284, 319, 356, 395, 436, 479, 524, 571, 620, 671, 724
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000196";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_196(n)
    }
}

const fn poly_196(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 6 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000196>();
}
