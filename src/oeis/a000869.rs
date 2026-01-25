/// a(n) = 10*T(n)^2 + 1
/// https://oeis.org/A000869

pub struct A000869;

impl crate::traits::IntegerSequence for A000869 {
    const NAME: &str = "a(n) = 10*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 11, 91, 361, 1001, 2251, 4411, 7841, 12961, 20251, 30251, 43561, 60841, 82811, 110251, 144001, 184961, 234091, 292411, 361001, 441001, 533611, 640091, 761761, 900001, 1056251, 1232011, 1428841, 1648361, 1892251
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000869";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_869(n)
    }
}

const fn tri_pow_869(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    10 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000869>();
}
