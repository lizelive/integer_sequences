/// a(n) = 3*T(n) + 8
/// https://oeis.org/A000378

pub struct A000378;

impl crate::traits::IntegerSequence for A000378 {
    const NAME: &str = "a(n) = 3*T(n) + 8";

    const HEAD: &[crate::Value] = &[
        8, 11, 17, 26, 38, 53, 71, 92, 116, 143, 173, 206, 242, 281, 323, 368, 416, 467, 521, 578, 638, 701, 767, 836, 908
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000378";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_378(n)
    }
}

const fn tri_378(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000378>();
}
