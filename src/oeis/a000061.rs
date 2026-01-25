/// a(n) = (n-1)! = factorial(n-1).
/// https://oeis.org/A000061

pub struct A000061;

impl crate::traits::IntegerSequence for A000061 {
    const NAME: &str = "Primes p such that the cycle length of 1/p equals (p-1)/6";

    const HEAD: &[crate::Value] = &[
        7, 31, 43, 67, 79, 139, 181, 211, 229, 283, 307, 373, 421, 439, 457, 463, 487, 547, 571,
        607, 613, 631, 691, 709, 757, 787, 823, 829, 853, 859, 883, 907, 919, 997, 1033, 1051,
        1087, 1129, 1171, 1201, 1213, 1279, 1303, 1321, 1381, 1423, 1447, 1483, 1489, 1531, 1549,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000061";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000061>();
}
