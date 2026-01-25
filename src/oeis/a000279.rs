/// a(n) = 10*n^4
/// https://oeis.org/A000279

pub struct A000279;

impl crate::traits::IntegerSequence for A000279 {
    const NAME: &str = "a(n) = 10*n^4";

    const HEAD: &[crate::Value] = &[
        0, 10, 160, 810, 2560, 6250, 12960, 24010, 40960, 65610, 100000, 146410, 207360, 285610, 384160, 506250, 655360, 835210, 1049760, 1303210, 1600000, 1944810, 2342560, 2798410, 3317760
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000279";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_279(n)
    }
}

const fn power_279(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000279>();
}
