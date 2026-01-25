/// a(n) = 4*T(n) + 8
/// https://oeis.org/A000388

pub struct A000388;

impl crate::traits::IntegerSequence for A000388 {
    const NAME: &str = "a(n) = 4*T(n) + 8";

    const HEAD: &[crate::Value] = &[
        8, 12, 20, 32, 48, 68, 92, 120, 152, 188, 228, 272, 320, 372, 428, 488, 552, 620, 692, 768, 848, 932, 1020, 1112, 1208
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000388";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_388(n)
    }
}

const fn tri_388(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000388>();
}
