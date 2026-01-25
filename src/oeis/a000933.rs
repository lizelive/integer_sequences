/// a(n) = 4*n^2 + 3
/// https://oeis.org/A000933

pub struct A000933;

impl crate::traits::IntegerSequence for A000933 {
    const NAME: &str = "a(n) = 4*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 7, 19, 39, 67, 103, 147, 199, 259, 327, 403, 487, 579, 679, 787, 903, 1027, 1159, 1299, 1447, 1603, 1767, 1939, 2119, 2307, 2503, 2707, 2919, 3139, 3367
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000933";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_933(n)
    }
}

const fn sq_933(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000933>();
}
