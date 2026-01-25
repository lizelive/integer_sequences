/// a(n) = 9*T(n)^1
/// https://oeis.org/A000808

pub struct A000808;

impl crate::traits::IntegerSequence for A000808 {
    const NAME: &str = "a(n) = 9*T(n)^1";

    const HEAD: &[crate::Value] = &[
        0, 9, 27, 54, 90, 135, 189, 252, 324, 405, 495, 594, 702, 819, 945, 1080, 1224, 1377, 1539, 1710, 1890, 2079, 2277, 2484, 2700, 2925, 3159, 3402, 3654, 3915
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000808";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_808(n)
    }
}

const fn tri_pow_808(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    9 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000808>();
}
