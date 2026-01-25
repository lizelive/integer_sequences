/// a(n) = 10*T(n)^3
/// https://oeis.org/A000829

pub struct A000829;

impl crate::traits::IntegerSequence for A000829 {
    const NAME: &str = "a(n) = 10*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 10, 270, 2160, 10000, 33750, 92610, 219520, 466560, 911250, 1663750, 2874960, 4745520, 7535710, 11576250, 17280000, 25154560, 35815770, 50002110, 68590000, 92610000, 123263910, 161942770, 210245760, 270000000, 343281250, 432435510, 540101520, 669234160, 823128750
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000829";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_829(n)
    }
}

const fn tri_pow_829(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    10 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000829>();
}
