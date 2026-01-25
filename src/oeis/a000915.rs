/// a(n) = 6*n^2 + 1
/// https://oeis.org/A000915

pub struct A000915;

impl crate::traits::IntegerSequence for A000915 {
    const NAME: &str = "a(n) = 6*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 25, 55, 97, 151, 217, 295, 385, 487, 601, 727, 865, 1015, 1177, 1351, 1537, 1735, 1945, 2167, 2401, 2647, 2905, 3175, 3457, 3751, 4057, 4375, 4705, 5047
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000915";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_915(n)
    }
}

const fn sq_915(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000915>();
}
