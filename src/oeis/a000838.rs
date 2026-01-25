/// a(n) = 9*T(n)^4
/// https://oeis.org/A000838

pub struct A000838;

impl crate::traits::IntegerSequence for A000838 {
    const NAME: &str = "a(n) = 9*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 9, 729, 11664, 90000, 455625, 1750329, 5531904, 15116544, 36905625, 82355625, 170772624, 333135504, 617174649, 1093955625, 1866240000, 3078918144, 4931831529, 7695324729, 11728890000, 17503290000, 25626566889, 36874368729, 52225046784, 72900000000, 100409765625, 136606377609, 183742537104, 244538162064, 322254905625
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000838";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_838(n)
    }
}

const fn tri_pow_838(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    9 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000838>();
}
