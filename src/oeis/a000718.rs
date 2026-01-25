/// a(n) = 1*n^2 + 4*n + 3
/// https://oeis.org/A000718

pub struct A000718;

impl crate::traits::IntegerSequence for A000718 {
    const NAME: &str = "a(n) = 1*n^2 + 4*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 8, 15, 24, 35, 48, 63, 80, 99, 120, 143, 168, 195, 224, 255, 288, 323, 360, 399, 440, 483, 528, 575, 624, 675, 728, 783, 840, 899, 960
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000718";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_718(n)
    }
}

const fn quad_718(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 4 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000718>();
}
