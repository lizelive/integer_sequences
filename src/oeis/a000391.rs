/// a(n) = 5*T(n) + 1
/// https://oeis.org/A000391

pub struct A000391;

impl crate::traits::IntegerSequence for A000391 {
    const NAME: &str = "a(n) = 5*T(n) + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 16, 31, 51, 76, 106, 141, 181, 226, 276, 331, 391, 456, 526, 601, 681, 766, 856, 951, 1051, 1156, 1266, 1381, 1501
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000391";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_391(n)
    }
}

const fn tri_391(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000391>();
}
