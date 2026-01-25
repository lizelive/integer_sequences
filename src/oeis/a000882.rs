/// a(n) = 3*T(n)^4 + 1
/// https://oeis.org/A000882

pub struct A000882;

impl crate::traits::IntegerSequence for A000882 {
    const NAME: &str = "a(n) = 3*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 244, 3889, 30001, 151876, 583444, 1843969, 5038849, 12301876, 27451876, 56924209, 111045169, 205724884, 364651876, 622080001, 1026306049, 1643943844, 2565108244, 3909630001, 5834430001, 8542188964, 12291456244, 17408348929, 24300000001, 33469921876, 45535459204, 61247512369, 81512720689, 107418301876
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000882";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_882(n)
    }
}

const fn tri_pow_882(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    3 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000882>();
}
