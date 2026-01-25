/// a(n) = 9*n^2 + 3
/// https://oeis.org/A000938

pub struct A000938;

impl crate::traits::IntegerSequence for A000938 {
    const NAME: &str = "a(n) = 9*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 12, 39, 84, 147, 228, 327, 444, 579, 732, 903, 1092, 1299, 1524, 1767, 2028, 2307, 2604, 2919, 3252, 3603, 3972, 4359, 4764, 5187, 5628, 6087, 6564, 7059, 7572
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000938";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_938(n)
    }
}

const fn sq_938(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000938>();
}
