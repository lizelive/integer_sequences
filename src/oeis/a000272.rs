/// a(n) = 3*n^4
/// https://oeis.org/A000272

pub struct A000272;

impl crate::traits::IntegerSequence for A000272 {
    const NAME: &str = "a(n) = 3*n^4";

    const HEAD: &[crate::Value] = &[
        0, 3, 48, 243, 768, 1875, 3888, 7203, 12288, 19683, 30000, 43923, 62208, 85683, 115248, 151875, 196608, 250563, 314928, 390963, 480000, 583443, 702768, 839523, 995328
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000272";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_272(n)
    }
}

const fn power_272(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000272>();
}
