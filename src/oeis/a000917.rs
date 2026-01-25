/// a(n) = 8*n^2 + 1
/// https://oeis.org/A000917

pub struct A000917;

impl crate::traits::IntegerSequence for A000917 {
    const NAME: &str = "a(n) = 8*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 33, 73, 129, 201, 289, 393, 513, 649, 801, 969, 1153, 1353, 1569, 1801, 2049, 2313, 2593, 2889, 3201, 3529, 3873, 4233, 4609, 5001, 5409, 5833, 6273, 6729
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000917";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_917(n)
    }
}

const fn sq_917(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000917>();
}
