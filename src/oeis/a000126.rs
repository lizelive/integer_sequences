/// a(n) = 2^n - n
/// https://oeis.org/A000126

pub struct A000126;

impl crate::traits::IntegerSequence for A000126 {
    const NAME: &str = "a(n) = 2^n - n";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 5, 12, 27, 58, 121, 248, 503, 1014, 2037, 4084, 8179, 16370, 32753, 65520, 131055, 262126, 524269, 1048556, 2097131, 4194282, 8388585, 16777192, 33554407, 67108838, 134217701, 268435428, 536870883
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000126";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power2_minus_n(n)
    }
}

const fn power2_minus_n(n: crate::Index) -> crate::Value {
    if n < 0 || n > 60 { return 0; }
    (1isize << n) - n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000126>();
}
