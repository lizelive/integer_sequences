/// a(n) = 3*n^3
/// https://oeis.org/A000262

pub struct A000262;

impl crate::traits::IntegerSequence for A000262 {
    const NAME: &str = "a(n) = 3*n^3";

    const HEAD: &[crate::Value] = &[
        0, 3, 24, 81, 192, 375, 648, 1029, 1536, 2187, 3000, 3993, 5184, 6591, 8232, 10125, 12288, 14739, 17496, 20577, 24000, 27783, 31944, 36501, 41472
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000262";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_262(n)
    }
}

const fn power_262(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000262>();
}
