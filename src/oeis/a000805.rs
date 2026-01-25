/// a(n) = 6*T(n)^1
/// https://oeis.org/A000805

pub struct A000805;

impl crate::traits::IntegerSequence for A000805 {
    const NAME: &str = "a(n) = 6*T(n)^1";

    const HEAD: &[crate::Value] = &[
        0, 6, 18, 36, 60, 90, 126, 168, 216, 270, 330, 396, 468, 546, 630, 720, 816, 918, 1026, 1140, 1260, 1386, 1518, 1656, 1800, 1950, 2106, 2268, 2436, 2610
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000805";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_805(n)
    }
}

const fn tri_pow_805(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    6 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000805>();
}
