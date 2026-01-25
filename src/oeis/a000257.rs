/// a(n) = 8*n^2
/// https://oeis.org/A000257

pub struct A000257;

impl crate::traits::IntegerSequence for A000257 {
    const NAME: &str = "a(n) = 8*n^2";

    const HEAD: &[crate::Value] = &[
        0, 8, 32, 72, 128, 200, 288, 392, 512, 648, 800, 968, 1152, 1352, 1568, 1800, 2048, 2312, 2592, 2888, 3200, 3528, 3872, 4232, 4608
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000257";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_257(n)
    }
}

const fn power_257(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 2 {
        result *= n;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000257>();
}
