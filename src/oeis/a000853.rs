/// a(n) = 4*T(n)^1 + 1
/// https://oeis.org/A000853

pub struct A000853;

impl crate::traits::IntegerSequence for A000853 {
    const NAME: &str = "a(n) = 4*T(n)^1 + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 13, 25, 41, 61, 85, 113, 145, 181, 221, 265, 313, 365, 421, 481, 545, 613, 685, 761, 841, 925, 1013, 1105, 1201, 1301, 1405, 1513, 1625, 1741
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000853";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_853(n)
    }
}

const fn tri_pow_853(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    4 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000853>();
}
