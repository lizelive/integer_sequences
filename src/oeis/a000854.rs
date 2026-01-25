/// a(n) = 5*T(n)^1 + 1
/// https://oeis.org/A000854

pub struct A000854;

impl crate::traits::IntegerSequence for A000854 {
    const NAME: &str = "a(n) = 5*T(n)^1 + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 16, 31, 51, 76, 106, 141, 181, 226, 276, 331, 391, 456, 526, 601, 681, 766, 856, 951, 1051, 1156, 1266, 1381, 1501, 1626, 1756, 1891, 2031, 2176
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000854";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_854(n)
    }
}

const fn tri_pow_854(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    5 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000854>();
}
