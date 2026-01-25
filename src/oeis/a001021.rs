/// a(n) = 4*n^3 + 3*n^2 + 1*n
/// https://oeis.org/A001021

pub struct A001021;

impl crate::traits::IntegerSequence for A001021 {
    const NAME: &str = "a(n) = 4*n^3 + 3*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 46, 138, 308, 580, 978, 1526, 2248, 3168, 4310, 5698, 7356, 9308, 11578, 14190, 17168, 20536, 24318, 28538, 33220, 38388, 44066, 50278, 57048, 64400, 72358, 80946, 90188, 100108
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001021";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1021(n)
    }
}

const fn cubic_1021(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 3 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001021>();
}
