/// a(n) = 7*T(n)^3 + 1
/// https://oeis.org/A000876

pub struct A000876;

impl crate::traits::IntegerSequence for A000876 {
    const NAME: &str = "a(n) = 7*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 190, 1513, 7001, 23626, 64828, 153665, 326593, 637876, 1164626, 2012473, 3321865, 5274998, 8103376, 12096001, 17608193, 25071040, 35001478, 48013001, 64827001, 86284738, 113359940, 147172033, 189000001, 240296876, 302704858, 378071065, 468463913, 576190126
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000876";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_876(n)
    }
}

const fn tri_pow_876(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    7 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000876>();
}
