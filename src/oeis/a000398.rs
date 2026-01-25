/// a(n) = 5*T(n) + 8
/// https://oeis.org/A000398

pub struct A000398;

impl crate::traits::IntegerSequence for A000398 {
    const NAME: &str = "a(n) = 5*T(n) + 8";

    const HEAD: &[crate::Value] = &[
        8, 13, 23, 38, 58, 83, 113, 148, 188, 233, 283, 338, 398, 463, 533, 608, 688, 773, 863, 958, 1058, 1163, 1273, 1388, 1508
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000398";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_398(n)
    }
}

const fn tri_398(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000398>();
}
