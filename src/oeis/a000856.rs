/// a(n) = 7*T(n)^1 + 1
/// https://oeis.org/A000856

pub struct A000856;

impl crate::traits::IntegerSequence for A000856 {
    const NAME: &str = "a(n) = 7*T(n)^1 + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 22, 43, 71, 106, 148, 197, 253, 316, 386, 463, 547, 638, 736, 841, 953, 1072, 1198, 1331, 1471, 1618, 1772, 1933, 2101, 2276, 2458, 2647, 2843, 3046
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000856";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_856(n)
    }
}

const fn tri_pow_856(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    7 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000856>();
}
