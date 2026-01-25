/// a(n) = 3*n^2
/// https://oeis.org/A000252

pub struct A000252;

impl crate::traits::IntegerSequence for A000252 {
    const NAME: &str = "a(n) = 3*n^2";

    const HEAD: &[crate::Value] = &[
        0, 3, 12, 27, 48, 75, 108, 147, 192, 243, 300, 363, 432, 507, 588, 675, 768, 867, 972, 1083, 1200, 1323, 1452, 1587, 1728
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000252";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_252(n)
    }
}

const fn power_252(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 2 {
        result *= n;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000252>();
}
