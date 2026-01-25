/// a(n) = 10*T(n)^2
/// https://oeis.org/A000819

pub struct A000819;

impl crate::traits::IntegerSequence for A000819 {
    const NAME: &str = "a(n) = 10*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 10, 90, 360, 1000, 2250, 4410, 7840, 12960, 20250, 30250, 43560, 60840, 82810, 110250, 144000, 184960, 234090, 292410, 361000, 441000, 533610, 640090, 761760, 900000, 1056250, 1232010, 1428840, 1648360, 1892250
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000819";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_819(n)
    }
}

const fn tri_pow_819(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    10 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000819>();
}
