/// a(n) = n^3 + 8*n + 8
/// https://oeis.org/A000588

pub struct A000588;

impl crate::traits::IntegerSequence for A000588 {
    const NAME: &str = "a(n) = n^3 + 8*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 17, 32, 59, 104, 173, 272, 407, 584, 809, 1088, 1427, 1832, 2309, 2864, 3503, 4232, 5057, 5984, 7019, 8168, 9437, 10832, 12359, 14024, 15833, 17792, 19907, 22184, 24629
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000588";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_588(n)
    }
}

const fn poly_588(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000588>();
}
