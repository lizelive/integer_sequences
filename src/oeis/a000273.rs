/// a(n) = 4*n^4
/// https://oeis.org/A000273

pub struct A000273;

impl crate::traits::IntegerSequence for A000273 {
    const NAME: &str = "a(n) = 4*n^4";

    const HEAD: &[crate::Value] = &[
        0, 4, 64, 324, 1024, 2500, 5184, 9604, 16384, 26244, 40000, 58564, 82944, 114244, 153664, 202500, 262144, 334084, 419904, 521284, 640000, 777924, 937024, 1119364, 1327104
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000273";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_273(n)
    }
}

const fn power_273(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    4 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000273>();
}
