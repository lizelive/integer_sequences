/// a(n) = 6*T(n)^4
/// https://oeis.org/A000835

pub struct A000835;

impl crate::traits::IntegerSequence for A000835 {
    const NAME: &str = "a(n) = 6*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 6, 486, 7776, 60000, 303750, 1166886, 3687936, 10077696, 24603750, 54903750, 113848416, 222090336, 411449766, 729303750, 1244160000, 2052612096, 3287887686, 5130216486, 7819260000, 11668860000, 17084377926, 24582912486, 34816697856, 48600000000, 66939843750, 91070918406, 122495024736, 163025441376, 214836603750
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000835";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_835(n)
    }
}

const fn tri_pow_835(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    6 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000835>();
}
