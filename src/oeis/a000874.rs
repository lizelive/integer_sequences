/// a(n) = 5*T(n)^3 + 1
/// https://oeis.org/A000874

pub struct A000874;

impl crate::traits::IntegerSequence for A000874 {
    const NAME: &str = "a(n) = 5*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 136, 1081, 5001, 16876, 46306, 109761, 233281, 455626, 831876, 1437481, 2372761, 3767856, 5788126, 8640001, 12577281, 17907886, 25001056, 34295001, 46305001, 61631956, 80971386, 105122881, 135000001, 171640626, 216217756, 270050761, 334617081, 411564376
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000874";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_874(n)
    }
}

const fn tri_pow_874(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    5 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000874>();
}
