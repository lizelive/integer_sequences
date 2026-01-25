/// a(n) = 8*n^4
/// https://oeis.org/A000277

pub struct A000277;

impl crate::traits::IntegerSequence for A000277 {
    const NAME: &str = "a(n) = 8*n^4";

    const HEAD: &[crate::Value] = &[
        0, 8, 128, 648, 2048, 5000, 10368, 19208, 32768, 52488, 80000, 117128, 165888, 228488, 307328, 405000, 524288, 668168, 839808, 1042568, 1280000, 1555848, 1874048, 2238728, 2654208
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000277";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_277(n)
    }
}

const fn power_277(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000277>();
}
