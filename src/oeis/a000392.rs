/// a(n) = 5*T(n) + 2
/// https://oeis.org/A000392

pub struct A000392;

impl crate::traits::IntegerSequence for A000392 {
    const NAME: &str = "a(n) = 5*T(n) + 2";

    const HEAD: &[crate::Value] = &[
        2, 7, 17, 32, 52, 77, 107, 142, 182, 227, 277, 332, 392, 457, 527, 602, 682, 767, 857, 952, 1052, 1157, 1267, 1382, 1502
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000392";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_392(n)
    }
}

const fn tri_392(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000392>();
}
