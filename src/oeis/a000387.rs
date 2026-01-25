/// a(n) = 4*T(n) + 7
/// https://oeis.org/A000387

pub struct A000387;

impl crate::traits::IntegerSequence for A000387 {
    const NAME: &str = "a(n) = 4*T(n) + 7";

    const HEAD: &[crate::Value] = &[
        7, 11, 19, 31, 47, 67, 91, 119, 151, 187, 227, 271, 319, 371, 427, 487, 551, 619, 691, 767, 847, 931, 1019, 1111, 1207
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000387";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_387(n)
    }
}

const fn tri_387(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * (n + 1) / 2 + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000387>();
}
