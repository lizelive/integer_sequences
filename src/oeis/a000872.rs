/// a(n) = 3*T(n)^3 + 1
/// https://oeis.org/A000872

pub struct A000872;

impl crate::traits::IntegerSequence for A000872 {
    const NAME: &str = "a(n) = 3*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 82, 649, 3001, 10126, 27784, 65857, 139969, 273376, 499126, 862489, 1423657, 2260714, 3472876, 5184001, 7546369, 10744732, 15000634, 20577001, 27783001, 36979174, 48582832, 63073729, 81000001, 102984376, 129730654, 162030457, 200770249, 246938626
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000872";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_872(n)
    }
}

const fn tri_pow_872(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    3 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000872>();
}
