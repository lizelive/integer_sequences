/// a(n) = 10*n^2
/// https://oeis.org/A000259

pub struct A000259;

impl crate::traits::IntegerSequence for A000259 {
    const NAME: &str = "a(n) = 10*n^2";

    const HEAD: &[crate::Value] = &[
        0, 10, 40, 90, 160, 250, 360, 490, 640, 810, 1000, 1210, 1440, 1690, 1960, 2250, 2560, 2890, 3240, 3610, 4000, 4410, 4840, 5290, 5760
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000259";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_259(n)
    }
}

const fn power_259(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 2 {
        result *= n;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000259>();
}
