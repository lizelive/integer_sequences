/// a(n) = 1*n^2 + 1
/// https://oeis.org/A000910

pub struct A000910;

impl crate::traits::IntegerSequence for A000910 {
    const NAME: &str = "a(n) = 1*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 10, 17, 26, 37, 50, 65, 82, 101, 122, 145, 170, 197, 226, 257, 290, 325, 362, 401, 442, 485, 530, 577, 626, 677, 730, 785, 842
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000910";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_910(n)
    }
}

const fn sq_910(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000910>();
}
