/// a(n) = 1*T(n)^2
/// https://oeis.org/A000810

pub struct A000810;

impl crate::traits::IntegerSequence for A000810 {
    const NAME: &str = "a(n) = 1*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 1, 9, 36, 100, 225, 441, 784, 1296, 2025, 3025, 4356, 6084, 8281, 11025, 14400, 18496, 23409, 29241, 36100, 44100, 53361, 64009, 76176, 90000, 105625, 123201, 142884, 164836, 189225
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000810";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_810(n)
    }
}

const fn tri_pow_810(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    1 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000810>();
}
