/// a(n) = 5*n^3 + 2*n^2 + 1*n
/// https://oeis.org/A001016

pub struct A001016;

impl crate::traits::IntegerSequence for A001016 {
    const NAME: &str = "a(n) = 5*n^3 + 2*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 50, 156, 356, 680, 1158, 1820, 2696, 3816, 5210, 6908, 8940, 11336, 14126, 17340, 21008, 25160, 29826, 35036, 40820, 47208, 54230, 61916, 70296, 79400, 89258, 99900, 111356, 123656
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001016";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1016(n)
    }
}

const fn cubic_1016(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 2 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001016>();
}
