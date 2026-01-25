/// a(n) = 8*T(n)^1
/// https://oeis.org/A000807

pub struct A000807;

impl crate::traits::IntegerSequence for A000807 {
    const NAME: &str = "a(n) = 8*T(n)^1";

    const HEAD: &[crate::Value] = &[
        0, 8, 24, 48, 80, 120, 168, 224, 288, 360, 440, 528, 624, 728, 840, 960, 1088, 1224, 1368, 1520, 1680, 1848, 2024, 2208, 2400, 2600, 2808, 3024, 3248, 3480
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000807";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_807(n)
    }
}

const fn tri_pow_807(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    8 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000807>();
}
