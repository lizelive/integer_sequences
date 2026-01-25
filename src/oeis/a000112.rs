/// a(n) = n*(n+2) = (n+1)^2 - 1
/// https://oeis.org/A000112

pub struct A000112;

impl crate::traits::IntegerSequence for A000112 {
    const NAME: &str = "a(n) = n*(n+2)";

    const HEAD: &[crate::Value] = &[
        0, 3, 8, 15, 24, 35, 48, 63, 80, 99, 120, 143, 168, 195, 224, 255, 288, 323, 360, 399, 440
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000112";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        n_times_n_plus_2(n)
    }
}

const fn n_times_n_plus_2(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * (n + 2)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000112>();
}
