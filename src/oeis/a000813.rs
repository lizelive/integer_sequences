/// a(n) = 4*T(n)^2
/// https://oeis.org/A000813

pub struct A000813;

impl crate::traits::IntegerSequence for A000813 {
    const NAME: &str = "a(n) = 4*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 4, 36, 144, 400, 900, 1764, 3136, 5184, 8100, 12100, 17424, 24336, 33124, 44100, 57600, 73984, 93636, 116964, 144400, 176400, 213444, 256036, 304704, 360000, 422500, 492804, 571536, 659344, 756900
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000813";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_813(n)
    }
}

const fn tri_pow_813(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    4 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000813>();
}
