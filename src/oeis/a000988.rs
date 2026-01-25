/// a(n) = 9*n^2 + 8
/// https://oeis.org/A000988

pub struct A000988;

impl crate::traits::IntegerSequence for A000988 {
    const NAME: &str = "a(n) = 9*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 17, 44, 89, 152, 233, 332, 449, 584, 737, 908, 1097, 1304, 1529, 1772, 2033, 2312, 2609, 2924, 3257, 3608, 3977, 4364, 4769, 5192, 5633, 6092, 6569, 7064, 7577
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000988";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_988(n)
    }
}

const fn sq_988(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000988>();
}
