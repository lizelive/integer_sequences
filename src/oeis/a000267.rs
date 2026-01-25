/// a(n) = 8*n^3
/// https://oeis.org/A000267

pub struct A000267;

impl crate::traits::IntegerSequence for A000267 {
    const NAME: &str = "a(n) = 8*n^3";

    const HEAD: &[crate::Value] = &[
        0, 8, 64, 216, 512, 1000, 1728, 2744, 4096, 5832, 8000, 10648, 13824, 17576, 21952, 27000, 32768, 39304, 46656, 54872, 64000, 74088, 85184, 97336, 110592
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000267";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_267(n)
    }
}

const fn power_267(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000267>();
}
