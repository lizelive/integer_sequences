/// a(n) = 9*T(n)^2
/// https://oeis.org/A000818

pub struct A000818;

impl crate::traits::IntegerSequence for A000818 {
    const NAME: &str = "a(n) = 9*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 9, 81, 324, 900, 2025, 3969, 7056, 11664, 18225, 27225, 39204, 54756, 74529, 99225, 129600, 166464, 210681, 263169, 324900, 396900, 480249, 576081, 685584, 810000, 950625, 1108809, 1285956, 1483524, 1703025
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000818";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_818(n)
    }
}

const fn tri_pow_818(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    9 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000818>();
}
