/// a(n) = 6*n^2 + 8
/// https://oeis.org/A000985

pub struct A000985;

impl crate::traits::IntegerSequence for A000985 {
    const NAME: &str = "a(n) = 6*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 14, 32, 62, 104, 158, 224, 302, 392, 494, 608, 734, 872, 1022, 1184, 1358, 1544, 1742, 1952, 2174, 2408, 2654, 2912, 3182, 3464, 3758, 4064, 4382, 4712, 5054
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000985";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_985(n)
    }
}

const fn sq_985(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000985>();
}
