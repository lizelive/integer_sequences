/// a(n) = 6*n^3 + 3*n^2 + 2*n
/// https://oeis.org/A001053

pub struct A001053;

impl crate::traits::IntegerSequence for A001053 {
    const NAME: &str = "a(n) = 6*n^3 + 3*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 11, 64, 195, 440, 835, 1416, 2219, 3280, 4635, 6320, 8371, 10824, 13715, 17080, 20955, 25376, 30379, 36000, 42275, 49240, 56931, 65384, 74635, 84720, 95675, 107536, 120339, 134120, 148915
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001053";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1053(n)
    }
}

const fn cubic_1053(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 3 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001053>();
}
