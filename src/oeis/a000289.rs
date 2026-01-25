/// a(n) = 10*n^5
/// https://oeis.org/A000289

pub struct A000289;

impl crate::traits::IntegerSequence for A000289 {
    const NAME: &str = "a(n) = 10*n^5";

    const HEAD: &[crate::Value] = &[
        0, 10, 320, 2430, 10240, 31250, 77760, 168070, 327680, 590490, 1000000, 1610510, 2488320, 3712930, 5378240, 7593750, 10485760, 14198570, 18895680, 24760990, 32000000, 40841010, 51536320, 64363430, 79626240
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000289";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_289(n)
    }
}

const fn power_289(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000289>();
}
