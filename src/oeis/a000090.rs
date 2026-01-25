/// a(n) = A000005(n)^2 = d(n)^2.
/// https://oeis.org/A000090

pub struct A000090;

impl crate::traits::IntegerSequence for A000090 {
    const NAME: &str = "Number of solutions to x^4 == 1 (mod n)";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 4, 2, 2, 4, 2, 4, 2, 4, 4, 2, 8, 4, 4, 2, 2, 8, 4, 2, 2, 8, 4, 4, 2, 4, 4, 8,
        2, 4, 4, 4, 8, 4, 4, 2, 8, 16, 4, 4, 2, 4, 8, 2, 2, 8, 2, 4, 8, 8, 4, 2, 8, 8, 4, 4, 2,
        16, 4, 2, 4, 4, 16, 4, 2, 8, 4, 8, 2, 8, 4, 4, 8, 4, 4, 8, 2, 16, 2, 4, 2, 8, 16, 2, 8, 8,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000090";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000090>();
}
