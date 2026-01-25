/// a(n) = 2*n^3
/// https://oeis.org/A000261

pub struct A000261;

impl crate::traits::IntegerSequence for A000261 {
    const NAME: &str = "a(n) = 2*n^3";

    const HEAD: &[crate::Value] = &[
        0, 2, 16, 54, 128, 250, 432, 686, 1024, 1458, 2000, 2662, 3456, 4394, 5488, 6750, 8192, 9826, 11664, 13718, 16000, 18522, 21296, 24334, 27648
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000261";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_261(n)
    }
}

const fn power_261(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    2 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000261>();
}
