/// a(n) = 6*n^3
/// https://oeis.org/A000265

pub struct A000265;

impl crate::traits::IntegerSequence for A000265 {
    const NAME: &str = "a(n) = 6*n^3";

    const HEAD: &[crate::Value] = &[
        0, 6, 48, 162, 384, 750, 1296, 2058, 3072, 4374, 6000, 7986, 10368, 13182, 16464, 20250, 24576, 29478, 34992, 41154, 48000, 55566, 63888, 73002, 82944
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000265";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_265(n)
    }
}

const fn power_265(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    6 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000265>();
}
