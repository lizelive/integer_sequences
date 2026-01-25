/// a(n) = n^3 + 6*n + 1
/// https://oeis.org/A000516

pub struct A000516;

impl crate::traits::IntegerSequence for A000516 {
    const NAME: &str = "a(n) = n^3 + 6*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 21, 46, 89, 156, 253, 386, 561, 784, 1061, 1398, 1801, 2276, 2829, 3466, 4193, 5016, 5941, 6974, 8121, 9388, 10781, 12306, 13969, 15776, 17733, 19846, 22121, 24564
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000516";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_516(n)
    }
}

const fn poly_516(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000516>();
}
