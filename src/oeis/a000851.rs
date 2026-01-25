/// a(n) = 2*T(n)^1 + 1
/// https://oeis.org/A000851

pub struct A000851;

impl crate::traits::IntegerSequence for A000851 {
    const NAME: &str = "a(n) = 2*T(n)^1 + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 7, 13, 21, 31, 43, 57, 73, 91, 111, 133, 157, 183, 211, 241, 273, 307, 343, 381, 421, 463, 507, 553, 601, 651, 703, 757, 813, 871
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000851";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_851(n)
    }
}

const fn tri_pow_851(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    2 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000851>();
}
