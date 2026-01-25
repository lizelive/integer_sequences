/// a(n) = 10*T(n)^1 + 1
/// https://oeis.org/A000859

pub struct A000859;

impl crate::traits::IntegerSequence for A000859 {
    const NAME: &str = "a(n) = 10*T(n)^1 + 1";

    const HEAD: &[crate::Value] = &[
        1, 11, 31, 61, 101, 151, 211, 281, 361, 451, 551, 661, 781, 911, 1051, 1201, 1361, 1531, 1711, 1901, 2101, 2311, 2531, 2761, 3001, 3251, 3511, 3781, 4061, 4351
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000859";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_859(n)
    }
}

const fn tri_pow_859(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    10 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000859>();
}
