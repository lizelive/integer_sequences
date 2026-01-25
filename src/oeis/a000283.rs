/// a(n) = 4*n^5
/// https://oeis.org/A000283

pub struct A000283;

impl crate::traits::IntegerSequence for A000283 {
    const NAME: &str = "a(n) = 4*n^5";

    const HEAD: &[crate::Value] = &[
        0, 4, 128, 972, 4096, 12500, 31104, 67228, 131072, 236196, 400000, 644204, 995328, 1485172, 2151296, 3037500, 4194304, 5679428, 7558272, 9904396, 12800000, 16336404, 20614528, 25745372, 31850496
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000283";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_283(n)
    }
}

const fn power_283(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    4 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000283>();
}
