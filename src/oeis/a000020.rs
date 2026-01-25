/// Number of primitive polynomials of degree n over GF(2).
/// https://oeis.org/A000020

pub struct A000020;

impl crate::traits::IntegerSequence for A000020 {
    const NAME: &str = "Number of primitive polynomials of degree n over GF(2)";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 6, 6, 18, 16, 48, 60, 176, 144, 630, 756, 1800, 2048, 7710, 7776, 27594, 24000,
        84672, 120032, 356960, 276480, 1296000, 1719900, 4202496, 4741632, 18407808, 17820000,
        69273666, 67108864, 211016256, 319422144, 860289120, 786432000,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000020";

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
    crate::tester::test_sequance_formula_matchces_head::<A000020>();
}
