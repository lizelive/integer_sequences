/// a(n) = 6*n^5
/// https://oeis.org/A000285

pub struct A000285;

impl crate::traits::IntegerSequence for A000285 {
    const NAME: &str = "a(n) = 6*n^5";

    const HEAD: &[crate::Value] = &[
        0, 6, 192, 1458, 6144, 18750, 46656, 100842, 196608, 354294, 600000, 966306, 1492992, 2227758, 3226944, 4556250, 6291456, 8519142, 11337408, 14856594, 19200000, 24504606, 30921792, 38618058, 47775744
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000285";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_285(n)
    }
}

const fn power_285(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    6 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000285>();
}
