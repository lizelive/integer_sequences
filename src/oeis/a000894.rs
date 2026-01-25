/// a(n) = 5*T(n)^5 + 1
/// https://oeis.org/A000894

pub struct A000894;

impl crate::traits::IntegerSequence for A000894 {
    const NAME: &str = "a(n) = 5*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 1216, 38881, 500001, 3796876, 20420506, 86051841, 302330881, 922640626, 2516421876, 6261662881, 14435871841, 31201607256, 63814078126, 124416000001, 232629370881, 419205679966, 731055849256, 1238049500001, 2042050500001, 3288742750756, 5182897382466, 8007840506881, 12150000000001, 18129541015626, 26638243633756, 38585932791841, 55156940998881, 77878268859376
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000894";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_894(n)
    }
}

const fn tri_pow_894(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    5 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000894>();
}
