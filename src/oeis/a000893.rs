/// a(n) = 4*T(n)^5 + 1
/// https://oeis.org/A000893

pub struct A000893;

impl crate::traits::IntegerSequence for A000893 {
    const NAME: &str = "a(n) = 4*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 973, 31105, 400001, 3037501, 16336405, 68841473, 241864705, 738112501, 2013137501, 5009330305, 11548697473, 24961285805, 51051262501, 99532800001, 186103496705, 335364543973, 584844679405, 990439600001, 1633640400001, 2630994200605, 4146317905973, 6406272405505, 9720000000001, 14503632812501, 21310594907005, 30868746233473, 44125552799105, 62302615087501
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000893";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_893(n)
    }
}

const fn tri_pow_893(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    4 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000893>();
}
