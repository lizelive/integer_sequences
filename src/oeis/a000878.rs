/// a(n) = 9*T(n)^3 + 1
/// https://oeis.org/A000878

pub struct A000878;

impl crate::traits::IntegerSequence for A000878 {
    const NAME: &str = "a(n) = 9*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 244, 1945, 9001, 30376, 83350, 197569, 419905, 820126, 1497376, 2587465, 4270969, 6782140, 10418626, 15552001, 22639105, 32234194, 45001900, 61731001, 83349001, 110937520, 145748494, 189221185, 243000001, 308953126, 389191960, 486091369, 602310745, 740815876
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000878";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_878(n)
    }
}

const fn tri_pow_878(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    9 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000878>();
}
