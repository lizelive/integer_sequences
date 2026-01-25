/// a(n) = 9*n^3
/// https://oeis.org/A000268

pub struct A000268;

impl crate::traits::IntegerSequence for A000268 {
    const NAME: &str = "a(n) = 9*n^3";

    const HEAD: &[crate::Value] = &[
        0, 9, 72, 243, 576, 1125, 1944, 3087, 4608, 6561, 9000, 11979, 15552, 19773, 24696, 30375, 36864, 44217, 52488, 61731, 72000, 83349, 95832, 109503, 124416
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000268";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_268(n)
    }
}

const fn power_268(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000268>();
}
