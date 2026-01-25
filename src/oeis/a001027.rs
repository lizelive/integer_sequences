/// a(n) = 4*n^3 + 4*n^2 + 1*n
/// https://oeis.org/A001027

pub struct A001027;

impl crate::traits::IntegerSequence for A001027 {
    const NAME: &str = "a(n) = 4*n^3 + 4*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 9, 50, 147, 324, 605, 1014, 1575, 2312, 3249, 4410, 5819, 7500, 9477, 11774, 14415, 17424, 20825, 24642, 28899, 33620, 38829, 44550, 50807, 57624, 65025, 73034, 81675, 90972, 100949
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001027";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1027(n)
    }
}

const fn cubic_1027(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 4 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001027>();
}
