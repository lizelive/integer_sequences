/// a(n) = 4*T(n)^5
/// https://oeis.org/A000843

pub struct A000843;

impl crate::traits::IntegerSequence for A000843 {
    const NAME: &str = "a(n) = 4*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 4, 972, 31104, 400000, 3037500, 16336404, 68841472, 241864704, 738112500, 2013137500, 5009330304, 11548697472, 24961285804, 51051262500, 99532800000, 186103496704, 335364543972, 584844679404, 990439600000, 1633640400000, 2630994200604, 4146317905972, 6406272405504, 9720000000000, 14503632812500, 21310594907004, 30868746233472, 44125552799104, 62302615087500
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000843";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_843(n)
    }
}

const fn tri_pow_843(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    4 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000843>();
}
