/// a(n) = 1*T(n)^2 + 1
/// https://oeis.org/A000860

pub struct A000860;

impl crate::traits::IntegerSequence for A000860 {
    const NAME: &str = "a(n) = 1*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 10, 37, 101, 226, 442, 785, 1297, 2026, 3026, 4357, 6085, 8282, 11026, 14401, 18497, 23410, 29242, 36101, 44101, 53362, 64010, 76177, 90001, 105626, 123202, 142885, 164837, 189226
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000860";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_860(n)
    }
}

const fn tri_pow_860(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    1 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000860>();
}
