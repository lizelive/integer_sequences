/// a(n) = 7*T(n)^1
/// https://oeis.org/A000806

pub struct A000806;

impl crate::traits::IntegerSequence for A000806 {
    const NAME: &str = "a(n) = 7*T(n)^1";

    const HEAD: &[crate::Value] = &[
        0, 7, 21, 42, 70, 105, 147, 196, 252, 315, 385, 462, 546, 637, 735, 840, 952, 1071, 1197, 1330, 1470, 1617, 1771, 1932, 2100, 2275, 2457, 2646, 2842, 3045
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000806";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_806(n)
    }
}

const fn tri_pow_806(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    7 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000806>();
}
