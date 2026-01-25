/// a(n) = 1*T(n) + 5
/// https://oeis.org/A000355

pub struct A000355;

impl crate::traits::IntegerSequence for A000355 {
    const NAME: &str = "a(n) = 1*T(n) + 5";

    const HEAD: &[crate::Value] = &[
        5, 6, 8, 11, 15, 20, 26, 33, 41, 50, 60, 71, 83, 96, 110, 125, 141, 158, 176, 195, 215, 236, 258, 281, 305
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000355";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_355(n)
    }
}

const fn tri_355(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000355>();
}
