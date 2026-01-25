/// a(n) = 3*T(n)^1
/// https://oeis.org/A000802

pub struct A000802;

impl crate::traits::IntegerSequence for A000802 {
    const NAME: &str = "a(n) = 3*T(n)^1";

    const HEAD: &[crate::Value] = &[
        0, 3, 9, 18, 30, 45, 63, 84, 108, 135, 165, 198, 234, 273, 315, 360, 408, 459, 513, 570, 630, 693, 759, 828, 900, 975, 1053, 1134, 1218, 1305
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000802";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_802(n)
    }
}

const fn tri_pow_802(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    3 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000802>();
}
