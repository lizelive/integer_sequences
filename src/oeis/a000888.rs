/// a(n) = 9*T(n)^4 + 1
/// https://oeis.org/A000888

pub struct A000888;

impl crate::traits::IntegerSequence for A000888 {
    const NAME: &str = "a(n) = 9*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 730, 11665, 90001, 455626, 1750330, 5531905, 15116545, 36905626, 82355626, 170772625, 333135505, 617174650, 1093955626, 1866240001, 3078918145, 4931831530, 7695324730, 11728890001, 17503290001, 25626566890, 36874368730, 52225046785, 72900000001, 100409765626, 136606377610, 183742537105, 244538162065, 322254905626
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000888";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_888(n)
    }
}

const fn tri_pow_888(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    9 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000888>();
}
