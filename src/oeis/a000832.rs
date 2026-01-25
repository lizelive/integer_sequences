/// a(n) = 3*T(n)^4
/// https://oeis.org/A000832

pub struct A000832;

impl crate::traits::IntegerSequence for A000832 {
    const NAME: &str = "a(n) = 3*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 3, 243, 3888, 30000, 151875, 583443, 1843968, 5038848, 12301875, 27451875, 56924208, 111045168, 205724883, 364651875, 622080000, 1026306048, 1643943843, 2565108243, 3909630000, 5834430000, 8542188963, 12291456243, 17408348928, 24300000000, 33469921875, 45535459203, 61247512368, 81512720688, 107418301875
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000832";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_832(n)
    }
}

const fn tri_pow_832(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    3 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000832>();
}
