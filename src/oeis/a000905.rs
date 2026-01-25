/// a(n) = 6*n^2 + 0
/// https://oeis.org/A000905

pub struct A000905;

impl crate::traits::IntegerSequence for A000905 {
    const NAME: &str = "a(n) = 6*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 6, 24, 54, 96, 150, 216, 294, 384, 486, 600, 726, 864, 1014, 1176, 1350, 1536, 1734, 1944, 2166, 2400, 2646, 2904, 3174, 3456, 3750, 4056, 4374, 4704, 5046
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000905";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_905(n)
    }
}

const fn sq_905(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000905>();
}
